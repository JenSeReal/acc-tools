//! Parser for MoTeC .ldx lap-marker XML sidecar files.
//!
//! The .ldx file is UTF-8 XML.  The actual nesting depth and element style
//! varies between MoTeC versions, so this parser handles both forms:
//!
//!   Attribute form:  <Marker Time="83400000" ... />
//!   Element form:    <Time>83400000</Time>

use quick_xml::{Reader, events::Event};

use crate::models::LapInfo;

#[derive(Debug, thiserror::Error)]
pub enum LdxError {
  #[error("Parse error: {0}")]
  Parse(String),
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a .ldx XML file and return lap boundary information.
///
/// Times are stored as microseconds.  Returned `LapInfo` values are sorted
/// by start time; `duration_s` is `None` for the last lap.
pub fn parse_ldx(data: &[u8]) -> Result<Vec<LapInfo>, LdxError> {
  let mut raw_times_us: Vec<i64> = Vec::new();

  let mut reader = Reader::from_reader(data);
  reader.config_mut().trim_text(true);

  let mut buf = Vec::new();
  let mut inside_time_elem = false;

  loop {
    match reader.read_event_into(&mut buf) {
      Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
        let name = e.local_name();
        if name.as_ref() == b"Time" {
          // Element form: collect the text content on the next Text event.
          // (Empty elements with this name are ignored — they carry no value.)
          inside_time_elem = true;
        }

        // Attribute form: any element may carry a Time="…" attribute.
        for attr in e.attributes().flatten() {
          if attr.key.local_name().as_ref() == b"Time"
            && let Ok(val) = attr.unescape_value()
            && let Some(us) = parse_time_str(val.trim())
          {
            raw_times_us.push(us);
          }
        }
      }

      Ok(Event::Text(ref e)) if inside_time_elem => {
        if let Ok(text) = std::str::from_utf8(e.as_ref())
          && let Some(us) = parse_time_str(text.trim())
        {
          raw_times_us.push(us);
        }
        inside_time_elem = false;
      }

      Ok(Event::End(_)) => {
        inside_time_elem = false;
      }

      Ok(Event::Eof) => break,

      Err(e) => return Err(LdxError::Parse(e.to_string())),

      _ => {
        inside_time_elem = false;
      }
    }
    buf.clear();
  }

  raw_times_us.sort_unstable();
  // Deduplicate identical timestamps (some files emit start+end for same lap)
  raw_times_us.dedup();

  if raw_times_us.is_empty() {
    return Ok(vec![]);
  }

  if raw_times_us.len() == 1 {
    return Ok(vec![LapInfo {
      lap_number: 1,
      start_time_s: raw_times_us[0] as f64 * 1e-6,
      duration_s: None,
    }]);
  }

  // Build one LapInfo per consecutive pair of timestamps.
  let mut result: Vec<LapInfo> = raw_times_us
    .windows(2)
    .enumerate()
    .map(|(i, w)| LapInfo {
      lap_number: i + 1,
      start_time_s: w[0] as f64 * 1e-6,
      duration_s: Some((w[1] - w[0]) as f64 * 1e-6),
    })
    .collect();

  // Append the final lap (starts at the last marker, duration unknown).
  result.push(LapInfo {
    lap_number: result.len() + 1,
    start_time_s: *raw_times_us.last().unwrap() as f64 * 1e-6,
    duration_s: None,
  });

  Ok(result)
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Parse a decimal or scientific-notation string to microseconds (i64).
fn parse_time_str(s: &str) -> Option<i64> {
  s.parse::<f64>().ok().map(|v| v as i64)
}
