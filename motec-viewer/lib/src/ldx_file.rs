//! Parser for MoTeC .ldx lap-marker XML sidecar files.
//!
//! The .ldx file is UTF-8 XML.  The actual nesting depth and element style
//! varies between MoTeC versions, so this parser handles both forms:
//!
//!   Attribute form:  <Marker Time="83400000" ... />
//!   Element form:    <Time>83400000</Time>

use quick_xml::Reader;
use quick_xml::events::Event;

use crate::models::LapInfo;

#[derive(Debug, thiserror::Error)]
pub enum LdxError {
  #[error("XML parse error: {0}")]
  Xml(#[from] quick_xml::Error),
  #[error("Attribute error: {0}")]
  Attr(#[from] quick_xml::events::attributes::AttrError),
}

/// Parse a .ldx XML file and return lap boundary information.
///
/// Times are stored as microseconds.  Returned `LapInfo` values are sorted
/// by start time; `duration_s` is `None` for the last lap.
pub fn parse_ldx(data: &[u8]) -> Result<Vec<LapInfo>, LdxError> {
  let mut reader = Reader::from_reader(data);
  reader.config_mut().trim_text(true);

  let mut raw_times_us: Vec<i64> = Vec::new();
  let mut buf = Vec::new();
  // Track whether the current element is a <Time> element so we can read
  // its text content.
  let mut inside_time_element = false;

  loop {
    match reader.read_event_into(&mut buf)? {
      Event::Eof => break,

      // Opening or self-closing element — look for a Time attribute.
      Event::Start(ref e) | Event::Empty(ref e) => {
        // Check if this element is named "Time"
        inside_time_element = e.name().as_ref() == b"Time";

        for attr in e.attributes() {
          let attr = attr?;
          if attr.key.as_ref() == b"Time" {
            let val = attr.unescape_value().unwrap_or_default();
            // Time values may be scientific notation (e.g. "1.62389e+08")
            if let Ok(us) = val.trim().parse::<f64>() {
              raw_times_us.push(us as i64);
            }
          }
        }
      }

      // Text content inside a <Time>…</Time> element.
      Event::Text(ref e) => {
        if inside_time_element {
          let val = e.unescape().unwrap_or_default();
          if let Ok(us) = val.trim().parse::<f64>() {
            raw_times_us.push(us as i64);
          }
          inside_time_element = false;
        }
      }

      Event::End(_) => {
        inside_time_element = false;
      }

      _ => {}
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
