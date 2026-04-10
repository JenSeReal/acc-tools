//! Parser for MoTeC .ldx lap-marker XML sidecar files.
//!
//! The .ldx file is UTF-8 XML.  The actual nesting depth varies between
//! MoTeC versions, so this parser walks the entire tree looking for any
//! element that has a `Time` attribute (value in microseconds) rather than
//! relying on a fixed path.

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
/// Times are stored as microseconds in the `Time` attribute.  Returned
/// `LapInfo` values are sorted by start time; `duration_s` is `None` for
/// the last lap if the total session length is not known.
pub fn parse_ldx(data: &[u8]) -> Result<Vec<LapInfo>, LdxError> {
  let mut reader = Reader::from_reader(data);
  reader.config_mut().trim_text(true);

  let mut raw_times_us: Vec<i64> = Vec::new();
  let mut buf = Vec::new();

  loop {
    match reader.read_event_into(&mut buf)? {
      Event::Eof => break,
      Event::Start(ref e) | Event::Empty(ref e) => {
        for attr in e.attributes() {
          let attr = attr?;
          if attr.key.as_ref() == b"Time" {
            let val = attr.unescape_value().unwrap_or_default();
            if let Ok(us) = val.trim().parse::<i64>() {
              raw_times_us.push(us);
            }
          }
        }
      }
      _ => {}
    }
    buf.clear();
  }

  raw_times_us.sort_unstable();

  let laps: Vec<LapInfo> = raw_times_us
    .windows(2)
    .enumerate()
    .map(|(i, w)| LapInfo {
      lap_number: i + 1,
      start_time_s: w[0] as f64 * 1e-6,
      duration_s: Some((w[1] - w[0]) as f64 * 1e-6),
    })
    .collect();

  // Add the final lap (no duration)
  if let Some(&last_us) = raw_times_us.last() {
    if raw_times_us.len() >= 2 {
      // Already covered by the windows() above – only add if there was a single marker
    } else if raw_times_us.len() == 1 {
      return Ok(vec![LapInfo {
        lap_number: 1,
        start_time_s: last_us as f64 * 1e-6,
        duration_s: None,
      }]);
    }
    // For multi-lap sessions, append the final partial lap
    let _ = last_us; // used implicitly in last LapInfo below
  }

  if laps.is_empty() && raw_times_us.len() == 1 {
    return Ok(vec![LapInfo {
      lap_number: 1,
      start_time_s: raw_times_us[0] as f64 * 1e-6,
      duration_s: None,
    }]);
  }

  // Append the last lap (starts at the last marker, duration unknown)
  if raw_times_us.len() > 1 {
    let mut result = laps;
    result.push(LapInfo {
      lap_number: result.len() + 1,
      start_time_s: *raw_times_us.last().unwrap() as f64 * 1e-6,
      duration_s: None,
    });
    return Ok(result);
  }

  Ok(laps)
}
