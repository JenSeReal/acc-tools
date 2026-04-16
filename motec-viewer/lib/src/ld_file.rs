//! Parser for MoTeC .ld / .ldd binary telemetry files.
//!
//! File layout (all values little-endian):
//!   offset 0          – ldHead  (≈1762 bytes)
//!   head.event_ptr    – ldEvent (1154 bytes)
//!   event.venue_ptr   – ldVenue (1100 bytes)  [optional]
//!   head.meta_ptr     – ldChan linked list (124 bytes each for ACC)
//!   chan.data_ptr      – raw sample data blob

use nom::{
  IResult,
  bytes::complete::take,
  number::complete::{le_i16, le_u16, le_u32},
};

use crate::models::{LdChannel, LdSession};

// ── Error ────────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum LdError {
  #[error("Not a MoTeC LD file (bad magic: expected 0x40, got {0:#x})")]
  BadMagic(u32),
  #[error("File too short to parse")]
  TooShort,
  #[error("Parse error")]
  Parse,
}

fn nom_err(_: nom::Err<nom::error::Error<&[u8]>>) -> LdError {
  LdError::Parse
}

// ── Public API ────────────────────────────────────────────────────────────────

pub fn parse_ld(data: &[u8]) -> Result<LdSession, LdError> {
  if data.len() < 4 {
    return Err(LdError::TooShort);
  }

  // Check magic before nom parsing so we can return a descriptive error.
  let magic = u32::from_le_bytes(data[..4].try_into().unwrap());
  if magic != 0x40 {
    return Err(LdError::BadMagic(magic));
  }

  let (_, header) = parse_header(data).map_err(nom_err)?;

  // ─ Event block ───────────────────────────────────────────────────────────

  let (event_name, event_session, venue) = if header.event_ptr > 0 && header.event_ptr < data.len()
  {
    let (_, (name, session, venue_ptr)) =
      parse_event(&data[header.event_ptr..]).map_err(nom_err)?;

    let venue = if venue_ptr > 0 && venue_ptr < data.len() {
      let (_, v) = fixed_str(64)(&data[venue_ptr..]).map_err(nom_err)?;
      if v.is_empty() {
        header.venue.clone()
      } else {
        v
      }
    } else {
      header.venue.clone()
    };

    (name, session, venue)
  } else {
    (String::new(), String::new(), header.venue.clone())
  };

  // ─ Channel linked list ───────────────────────────────────────────────────

  let mut channels = Vec::new();
  let mut ptr = header.meta_ptr;

  while ptr > 0 && ptr + 124 <= data.len() {
    let (_, entry) = parse_channel_entry(&data[ptr..]).map_err(nom_err)?;
    channels.push(decode_channel(data, &entry));
    ptr = entry.next;
  }

  Ok(LdSession {
    driver: header.driver,
    vehicle_id: header.vehicle_id,
    venue,
    date: header.date,
    time_of_day: header.time_of_day,
    short_comment: header.short_comment,
    device_type: header.device_type,
    event_name,
    event_session,
    channels,
  })
}

// ── Header ────────────────────────────────────────────────────────────────────

struct LdHeader {
  meta_ptr: usize,
  event_ptr: usize,
  device_type: String,
  date: String,
  time_of_day: String,
  driver: String,
  vehicle_id: String,
  venue: String,
  short_comment: String,
}

fn parse_header(input: &[u8]) -> IResult<&[u8], LdHeader> {
  let (input, _magic) = le_u32(input)?;
  let (input, _) = take(4usize)(input)?; // padding

  let (input, meta_ptr) = le_u32(input)?;
  let (input, _data_ptr) = le_u32(input)?;
  let (input, _) = take(20usize)(input)?; // padding

  let (input, event_ptr) = le_u32(input)?;
  let (input, _) = take(24usize)(input)?; // padding

  let (input, _) = take(6usize)(input)?; // unknown1/2/3 (three u16)
  let (input, _device_serial) = le_u32(input)?;
  let (input, device_type) = fixed_str(8)(input)?;
  let (input, _device_version) = le_u16(input)?;
  let (input, _) = take(2usize)(input)?; // unknown4
  let (input, _num_channs) = le_u32(input)?;
  let (input, _) = take(4usize)(input)?; // padding

  let (input, date) = fixed_str(16)(input)?;
  let (input, _) = take(16usize)(input)?; // padding
  let (input, time_of_day) = fixed_str(16)(input)?;
  let (input, _) = take(16usize)(input)?; // padding

  let (input, driver) = fixed_str(64)(input)?;
  let (input, vehicle_id) = fixed_str(64)(input)?;
  let (input, _) = take(64usize)(input)?; // padding
  let (input, venue) = fixed_str(64)(input)?;
  let (input, _) = take(64usize)(input)?; // padding
  let (input, _) = take(1024usize)(input)?; // unknown block

  let (input, _) = take(4usize)(input)?; // pro_logging_enable
  let (input, _) = take(66usize)(input)?; // padding
  let (input, short_comment) = fixed_str(64)(input)?;

  Ok((
    input,
    LdHeader {
      meta_ptr: meta_ptr as usize,
      event_ptr: event_ptr as usize,
      device_type,
      date,
      time_of_day,
      driver,
      vehicle_id,
      venue,
      short_comment,
    },
  ))
}

// ── Event block ───────────────────────────────────────────────────────────────

fn parse_event(input: &[u8]) -> IResult<&[u8], (String, String, usize)> {
  let (input, name) = fixed_str(64)(input)?;
  let (input, session) = fixed_str(64)(input)?;
  let (input, _) = take(1024usize)(input)?; // comment
  let (input, venue_ptr) = le_u16(input)?;
  Ok((input, (name, session, venue_ptr as usize)))
}

// ── Channel linked list entry ─────────────────────────────────────────────────

struct ChanEntry {
  next: usize,
  data_ptr: usize,
  data_len: u32,
  dtype_a: u16,
  dtype: u16,
  frequency: u16,
  shift: i16,
  mul: i16,
  scale: i16,
  dec_places: i16,
  name: String,
  short_name: String,
  unit: String,
}

fn parse_channel_entry(input: &[u8]) -> IResult<&[u8], ChanEntry> {
  let (input, _prev) = le_u32(input)?;
  let (input, next) = le_u32(input)?;
  let (input, data_ptr) = le_u32(input)?;
  let (input, data_len) = le_u32(input)?;

  let (input, _) = take(2usize)(input)?; // counter
  let (input, dtype_a) = le_u16(input)?;
  let (input, dtype) = le_u16(input)?;
  let (input, frequency) = le_u16(input)?;
  let (input, shift) = le_i16(input)?;
  let (input, mul) = le_i16(input)?;
  let (input, scale) = le_i16(input)?;
  let (input, dec_places) = le_i16(input)?;

  let (input, name) = fixed_str(32)(input)?;
  let (input, short_name) = fixed_str(8)(input)?;
  let (input, unit) = fixed_str(12)(input)?;

  Ok((
    input,
    ChanEntry {
      next: next as usize,
      data_ptr: data_ptr as usize,
      data_len,
      dtype_a,
      dtype,
      frequency,
      shift,
      mul,
      scale,
      dec_places,
      name,
      short_name,
      unit,
    },
  ))
}

// ── Channel decoding ──────────────────────────────────────────────────────────

fn decode_channel(data: &[u8], entry: &ChanEntry) -> LdChannel {
  let n = entry.data_len as usize;
  let end = entry
    .data_ptr
    .saturating_add(n.saturating_mul(entry.dtype as usize));

  if n == 0 || entry.data_ptr >= data.len() || end > data.len() || entry.dtype == 0 {
    return LdChannel {
      name: entry.name.clone(),
      short_name: entry.short_name.clone(),
      unit: entry.unit.clone(),
      frequency: entry.frequency,
      sample_count: entry.data_len,
      min: f64::NAN,
      max: f64::NAN,
      avg: f64::NAN,
      samples: vec![],
    };
  }

  let blob = &data[entry.data_ptr..end];
  let mut sum = 0.0f64;
  let mut min = f64::INFINITY;
  let mut max = f64::NEG_INFINITY;
  let mut samples = Vec::with_capacity(n);

  for i in 0..n {
    let raw = read_raw_sample(blob, i, entry.dtype_a, entry.dtype);
    let phys = to_physical(raw, entry.shift, entry.mul, entry.scale, entry.dec_places);
    sum += phys;
    if phys < min {
      min = phys;
    }
    if phys > max {
      max = phys;
    }
    samples.push(phys);
  }

  LdChannel {
    name: entry.name.clone(),
    short_name: entry.short_name.clone(),
    unit: entry.unit.clone(),
    frequency: entry.frequency,
    sample_count: entry.data_len,
    min,
    max,
    avg: sum / n as f64,
    samples,
  }
}

/// Read a single raw sample as f64 from the blob.
fn read_raw_sample(blob: &[u8], index: usize, dtype_a: u16, dtype: u16) -> f64 {
  // dtype_a=0x07 → floating point; anything else → signed integer
  // dtype        → byte width (2 or 4 for most channels, 8 for f64)
  let off = index * dtype as usize;
  match (dtype_a, dtype) {
    (0x07, 2) => {
      let bits = u16::from_le_bytes([blob[off], blob[off + 1]]);
      f16_to_f64(bits)
    }
    (0x07, 4) => {
      let bits = u32::from_le_bytes([blob[off], blob[off + 1], blob[off + 2], blob[off + 3]]);
      f32::from_bits(bits) as f64
    }
    (0x08, 8) => {
      let bits = u64::from_le_bytes(blob[off..off + 8].try_into().unwrap_or([0; 8]));
      f64::from_bits(bits)
    }
    (_, 2) => i16::from_le_bytes([blob[off], blob[off + 1]]) as f64,
    (_, 4) => i32::from_le_bytes([blob[off], blob[off + 1], blob[off + 2], blob[off + 3]]) as f64,
    _ => 0.0,
  }
}

/// Convert raw integer/float value to physical engineering value.
///
/// Formula (from ldparser.py):
///   physical = (raw / scale * 10^(-dec_places) + shift) * mul
fn to_physical(raw: f64, shift: i16, mul: i16, scale: i16, dec_places: i16) -> f64 {
  if mul == 0 {
    return 0.0;
  }
  let s = if scale == 0 { 1.0 } else { scale as f64 };
  let factor = 10.0f64.powi(-(dec_places as i32));
  (raw / s * factor + shift as f64) * mul as f64
}

/// Convert an IEEE 754 half-precision float (binary16) to f64.
fn f16_to_f64(bits: u16) -> f64 {
  let sign: f64 = if bits >> 15 != 0 { -1.0 } else { 1.0 };
  let exp = ((bits >> 10) & 0x1f) as i32;
  let mant = (bits & 0x3ff) as f64;
  match exp {
    0 => sign * mant * 2.0f64.powi(-24),
    31 => {
      if mant == 0.0 {
        sign * f64::INFINITY
      } else {
        f64::NAN
      }
    }
    e => sign * (1.0 + mant / 1024.0) * 2.0f64.powi(e - 15),
  }
}

// ── nom helpers ───────────────────────────────────────────────────────────────

/// Parse exactly `len` bytes as a null-terminated, trimmed ASCII string.
fn fixed_str(len: usize) -> impl Fn(&[u8]) -> IResult<&[u8], String> {
  move |input: &[u8]| {
    let (input, bytes) = take(len)(input)?;
    let s: Vec<u8> = bytes.iter().copied().take_while(|&b| b != 0).collect();
    Ok((input, String::from_utf8_lossy(&s).trim().to_string()))
  }
}
