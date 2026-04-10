//! Parser for MoTeC .ld / .ldd binary telemetry files.
//!
//! File layout (all values little-endian):
//!   offset 0          – ldHead  (≈1762 bytes)
//!   head.event_ptr    – ldEvent (1154 bytes)
//!   event.venue_ptr   – ldVenue (1100 bytes)  [optional]
//!   head.meta_ptr     – ldChan linked list (124 bytes each for ACC)
//!   chan.data_ptr      – raw sample data blob

use std::io::{self, Cursor, Read, Seek, SeekFrom};

use crate::models::{LdChannel, LdSession};

// ── Error ────────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum LdError {
  #[error("IO error: {0}")]
  Io(#[from] io::Error),
  #[error("Not a MoTeC LD file (bad magic: expected 0x40, got {0:#x})")]
  BadMagic(u32),
}

// ── Public API ────────────────────────────────────────────────────────────────

pub fn parse_ld(data: &[u8]) -> Result<LdSession, LdError> {
  let mut r = Cursor::new(data);

  // ─ Header ────────────────────────────────────────────────────────────────

  let magic = ru32(&mut r)?;
  if magic != 0x40 {
    return Err(LdError::BadMagic(magic));
  }
  skip(&mut r, 4)?; // padding

  let meta_ptr = ru32(&mut r)? as usize;
  let _data_ptr = ru32(&mut r)? as usize;
  skip(&mut r, 20)?; // padding

  let event_ptr = ru32(&mut r)? as usize;
  skip(&mut r, 24)?; // padding

  skip(&mut r, 6)?; // unknown1/2/3 (three u16)
  let _device_serial = ru32(&mut r)?;
  let device_type = rstr(&mut r, 8)?;
  let _device_version = ru16(&mut r)?;
  skip(&mut r, 2)?; // unknown4
  let _num_channs = ru32(&mut r)?;
  skip(&mut r, 4)?; // padding

  let date = rstr(&mut r, 16)?;
  skip(&mut r, 16)?; // padding
  let time_of_day = rstr(&mut r, 16)?;
  skip(&mut r, 16)?; // padding

  let driver = rstr(&mut r, 64)?;
  let vehicle_id = rstr(&mut r, 64)?;
  skip(&mut r, 64)?; // padding
  let venue_from_header = rstr(&mut r, 64)?;
  skip(&mut r, 64)?; // padding
  skip(&mut r, 1024)?; // unknown block

  skip(&mut r, 4)?; // pro_logging_enable
  skip(&mut r, 66)?; // padding
  let short_comment = rstr(&mut r, 64)?;
  // (126 bytes trailing padding – not needed)

  // ─ Event block ───────────────────────────────────────────────────────────

  let (event_name, event_session, venue) = if event_ptr > 0 && event_ptr < data.len() {
    r.seek(SeekFrom::Start(event_ptr as u64))?;
    let name = rstr(&mut r, 64)?;
    let session = rstr(&mut r, 64)?;
    skip(&mut r, 1024)?; // comment
    let venue_ptr = ru16(&mut r)? as usize;

    // Venue block – prefer venue from event if available
    let venue = if venue_ptr > 0 && venue_ptr < data.len() {
      r.seek(SeekFrom::Start(venue_ptr as u64))?;
      let v = rstr(&mut r, 64)?;
      if v.is_empty() {
        venue_from_header.clone()
      } else {
        v
      }
    } else {
      venue_from_header.clone()
    };

    (name, session, venue)
  } else {
    (String::new(), String::new(), venue_from_header)
  };

  // ─ Channel linked list ───────────────────────────────────────────────────

  let mut channels = Vec::new();
  if meta_ptr > 0 && meta_ptr < data.len() {
    let mut ptr = meta_ptr;
    while ptr > 0 && ptr + 124 <= data.len() {
      r.seek(SeekFrom::Start(ptr as u64))?;

      let _prev = ru32(&mut r)?;
      let next = ru32(&mut r)? as usize;
      let chan_data_ptr = ru32(&mut r)? as usize;
      let data_len = ru32(&mut r)?;

      skip(&mut r, 2)?; // counter
      let dtype_a = ru16(&mut r)?;
      let dtype = ru16(&mut r)?;
      let rec_freq = ru16(&mut r)?;
      let shift = ri16(&mut r)?;
      let mul = ri16(&mut r)?;
      let scale = ri16(&mut r)?;
      let dec_places = ri16(&mut r)?;

      let name = rstr(&mut r, 32)?;
      let short_name = rstr(&mut r, 8)?;
      let unit = rstr(&mut r, 12)?;

      let channel = decode_channel(
        data,
        chan_data_ptr,
        data_len,
        ChanMeta {
          dtype_a,
          dtype,
          shift,
          mul,
          scale,
          dec_places,
          name,
          short_name,
          unit,
          frequency: rec_freq,
        },
      );
      channels.push(channel);

      ptr = next;
    }
  }

  Ok(LdSession {
    driver,
    vehicle_id,
    venue,
    date,
    time_of_day,
    short_comment,
    device_type,
    event_name,
    event_session,
    channels,
  })
}

// ── Channel decoding ──────────────────────────────────────────────────────────

struct ChanMeta {
  dtype_a: u16,
  dtype: u16,
  shift: i16,
  mul: i16,
  scale: i16,
  dec_places: i16,
  name: String,
  short_name: String,
  unit: String,
  frequency: u16,
}

fn decode_channel(data: &[u8], data_ptr: usize, sample_count: u32, meta: ChanMeta) -> LdChannel {
  let ChanMeta {
    dtype_a,
    dtype,
    shift,
    mul,
    scale,
    dec_places,
    name,
    short_name,
    unit,
    frequency,
  } = meta;
  let n = sample_count as usize;
  let end = data_ptr.saturating_add(n.saturating_mul(dtype as usize));

  if n == 0 || data_ptr >= data.len() || end > data.len() || dtype == 0 {
    return LdChannel {
      name,
      short_name,
      unit,
      frequency,
      sample_count,
      min: f64::NAN,
      max: f64::NAN,
      avg: f64::NAN,
      preview: vec![],
    };
  }

  // Read raw samples as f64 physical values
  let blob = &data[data_ptr..end];
  let mut sum = 0.0f64;
  let mut min = f64::INFINITY;
  let mut max = f64::NEG_INFINITY;

  // Build evenly-spaced preview (up to 256 points)
  let preview_count = n.min(256);
  let step = if n <= 256 { 1 } else { n / 256 };
  let mut preview = Vec::with_capacity(preview_count);

  for i in 0..n {
    let raw = read_raw_sample(blob, i, dtype_a, dtype);
    let phys = to_physical(raw, shift, mul, scale, dec_places);
    sum += phys;
    if phys < min {
      min = phys;
    }
    if phys > max {
      max = phys;
    }
    if i % step == 0 && preview.len() < 256 {
      preview.push(phys);
    }
  }

  LdChannel {
    name,
    short_name,
    unit,
    frequency,
    sample_count,
    min,
    max,
    avg: sum / n as f64,
    preview,
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

// ── Low-level reading helpers ─────────────────────────────────────────────────

fn ru32(r: &mut Cursor<&[u8]>) -> io::Result<u32> {
  let mut buf = [0u8; 4];
  r.read_exact(&mut buf)?;
  Ok(u32::from_le_bytes(buf))
}

fn ru16(r: &mut Cursor<&[u8]>) -> io::Result<u16> {
  let mut buf = [0u8; 2];
  r.read_exact(&mut buf)?;
  Ok(u16::from_le_bytes(buf))
}

fn ri16(r: &mut Cursor<&[u8]>) -> io::Result<i16> {
  let mut buf = [0u8; 2];
  r.read_exact(&mut buf)?;
  Ok(i16::from_le_bytes(buf))
}

fn skip(r: &mut Cursor<&[u8]>, n: u64) -> io::Result<()> {
  r.seek(SeekFrom::Current(n as i64))?;
  Ok(())
}

/// Read exactly `len` bytes and return them as a trimmed ASCII string.
fn rstr(r: &mut Cursor<&[u8]>, len: usize) -> io::Result<String> {
  let mut buf = vec![0u8; len];
  r.read_exact(&mut buf)?;
  // Trim null bytes and surrounding whitespace
  let s = buf
    .iter()
    .take_while(|&&b| b != 0)
    .copied()
    .collect::<Vec<_>>();
  Ok(String::from_utf8_lossy(&s).trim().to_string())
}
