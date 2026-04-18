use std::str::FromStr;

use nom::{
  IResult, Parser,
  branch::alt,
  bytes::complete::tag,
  character::complete::{char, digit1, multispace1, space0, space1},
  combinator::{map, map_res, value},
};

use super::{ParseError, SessionParser};
use crate::{
  car::car_model::CarModel,
  qualifying_session::{QualifyingEntry, QualifyingSession, QualifyingTiming},
  track::model::Track,
};

/// Parser for PDF qualifying results from ACC.
pub struct PdfParser;

impl SessionParser for PdfParser {
  type Output = QualifyingSession;
  type Error = ParseError;

  fn parse(data: &[u8]) -> Result<Self::Output, Self::Error> {
    let text = pdf_extract::extract_text_from_mem(data).map_err(|e| ParseError::PdfError {
      message: e.to_string(),
    })?;

    parse_qualifying_pdf(&text)
  }
}

fn parse_qualifying_pdf(text: &str) -> Result<QualifyingSession, ParseError> {
  // Extract track name from the beginning
  let track = extract_track_name(text)?;

  // Parse all entries
  let entries = parse_all_entries(text)?;

  Ok(QualifyingSession::new(track, entries))
}

fn extract_track_name(text: &str) -> Result<Track, ParseError> {
  for line in text.lines() {
    let trimmed = line.trim();
    if trimmed.is_empty() {
      continue;
    }
    let track_lower = trimmed.to_lowercase();
    match track_lower.as_str() {
      "kyalami" => return Ok(Track::Kyalami),
      "monza" => return Ok(Track::Monza),
      "spa" | "spa-francorchamps" => return Ok(Track::Spa),
      "silverstone" => return Ok(Track::Silverstone),
      "nurburgring" | "nürburgring" => return Ok(Track::Nurburgring),
      "barcelona" | "circuit de barcelona-catalunya" => return Ok(Track::Barcelona),
      "brands hatch" => return Ok(Track::BrandsHatch),
      "hungaroring" => return Ok(Track::Hungaroring),
      "imola" => return Ok(Track::Imola),
      "laguna seca" => return Ok(Track::LagunaSeca),
      "misano" => return Ok(Track::Misano),
      "mount panorama" | "bathurst" => return Ok(Track::MountPanorama),
      "paul ricard" => return Ok(Track::PaulRicard),
      "suzuka" => return Ok(Track::Suzuka),
      "zandvoort" => return Ok(Track::Zandvoort),
      "zolder" => return Ok(Track::Zolder),
      "red bull ring" | "spielberg" => return Ok(Track::RedBullRing),
      "cota" | "circuit of the americas" => return Ok(Track::Cota),
      "indianapolis" => return Ok(Track::Indianapolis),
      "watkins glen" => return Ok(Track::WatkinsGlen),
      "valencia" => return Ok(Track::Valencia),
      _ => continue,
    }
  }
  Err(ParseError::InvalidTrackName {
    name: "Could not find track name in PDF".to_string(),
  })
}

/// Parse a time in MM:SS.mmm format to milliseconds
fn parse_time(input: &str) -> IResult<&str, u32> {
  map(
    (
      map_res(digit1, |s: &str| s.parse::<u32>()),
      char(':'),
      map_res(digit1, |s: &str| s.parse::<u32>()),
      char('.'),
      map_res(digit1, |s: &str| s.parse::<u32>()),
    ),
    |(minutes, _, seconds, _, millis)| (minutes * 60 + seconds) * 1000 + millis,
  )
  .parse(input)
}

/// Parse "No valid time" as None
fn parse_no_valid_time(input: &str) -> IResult<&str, Option<u32>> {
  value(
    None,
    (
      tag("No"),
      multispace1,
      tag("valid"),
      multispace1,
      tag("time"),
    ),
  )
  .parse(input)
}

/// Parse a boolean indicator (checkmark or cross)
fn parse_bool_indicator(input: &str) -> IResult<&str, bool> {
  alt((
    value(true, alt((tag("✔️"), tag("✔"), tag("✓")))),
    value(false, alt((tag("❌"), tag("✗"), tag("×")))),
  ))
  .parse(input)
}

/// Parse a u32 number
fn parse_u32(input: &str) -> IResult<&str, u32> {
  map_res(digit1, |s: &str| s.parse::<u32>()).parse(input)
}

/// Parse all entries from the PDF text
fn parse_all_entries(text: &str) -> Result<Vec<QualifyingEntry>, ParseError> {
  let mut entries = Vec::new();

  // Normalize the text: join lines and handle multi-line entries
  let normalized = normalize_pdf_text(text);

  // Find entry patterns and parse them
  for line in normalized.lines() {
    if let Ok((_, entry)) = parse_entry_line(line) {
      entries.push(entry);
    }
  }

  Ok(entries)
}

/// Normalize PDF text by joining related lines
fn normalize_pdf_text(text: &str) -> String {
  let mut result = String::new();
  let mut current_entry = String::new();
  let mut in_entry = false;

  for line in text.lines() {
    let trimmed = line.trim();

    // Skip empty lines, headers, and page markers
    if trimmed.is_empty()
      || trimmed.contains("about:blank")
      || trimmed.contains("Pos.")
      || trimmed.contains("Rennnummer")
      || trimmed.contains("Gültige")
      || trimmed.contains("Runden")
      || trimmed.contains("Folge")
      || trimmed.contains("Qualifiziert")
      || trimmed == "(7)"
      || trimmed == "(min."
      || trimmed == "20)"
      || trimmed == "in"
    {
      continue;
    }

    // Check if this line starts a new entry (begins with a position number)
    let tokens: Vec<&str> = trimmed.split_whitespace().collect();
    if !tokens.is_empty()
      && let Ok(pos) = tokens[0].parse::<u32>()
      && (1..=100).contains(&pos)
      && tokens.len() >= 2
      && tokens[1].parse::<u32>().is_ok()
    {
      // This is a new entry
      if in_entry && !current_entry.is_empty() {
        result.push_str(&current_entry);
        result.push('\n');
      }
      current_entry = trimmed.to_string();
      in_entry = true;
      continue;
    }

    // This is a continuation of the current entry
    if in_entry {
      current_entry.push(' ');
      current_entry.push_str(trimmed);
    }
  }

  // Don't forget the last entry
  if !current_entry.is_empty() {
    result.push_str(&current_entry);
    result.push('\n');
  }

  result
}

/// Parse a single entry line
fn parse_entry_line(input: &str) -> IResult<&str, QualifyingEntry> {
  let (input, _) = space0(input)?;

  // Position
  let (input, position) = parse_u32(input)?;
  let (input, _) = space1(input)?;

  // Race number
  let (input, race_number) = parse_u32(input)?;
  let (input, _) = space1(input)?;

  // Now we need to parse: Name CarModel Times ValidFlag LapCount QualifiedFlag
  // The challenge is that both name and car model can have multiple words

  // Find the timing section by looking for time patterns
  let remaining = input;

  // Try to find where times start
  let (_, (name_car_part, timing_part)) = split_at_times(remaining)?;

  // Parse name and car from the first part
  let (driver_name, car_model) = parse_name_and_car(name_car_part)?;

  // Parse timing and flags from the second part
  let (_, (timing, valid_consecutive_laps, lap_count, qualified)) =
    parse_timing_and_flags(timing_part)?;

  Ok((
    "",
    QualifyingEntry {
      position,
      race_number,
      driver_name,
      car_model,
      timing,
      valid_consecutive_laps,
      lap_count,
      qualified,
    },
  ))
}

/// Split input at the point where times begin
fn split_at_times(input: &str) -> IResult<&str, (&str, &str)> {
  // Look for the first occurrence of a time pattern (MM:SS.mmm) or "No valid time"
  for i in 0..input.len() {
    let remaining = &input[i..];

    // Check for time pattern
    if parse_time(remaining).is_ok() || remaining.starts_with("No valid time") {
      let name_car = input[..i].trim();
      let timing = remaining.trim();
      return Ok(("", (name_car, timing)));
    }
  }

  Err(nom::Err::Error(nom::error::Error::new(
    input,
    nom::error::ErrorKind::TakeUntil,
  )))
}

/// Parse name and car model from a combined string
fn parse_name_and_car(
  input: &str,
) -> Result<(String, CarModel), nom::Err<nom::error::Error<&str>>> {
  let tokens: Vec<&str> = input.split_whitespace().collect();

  // Known car manufacturers
  let car_starters = [
    "Ford",
    "BMW",
    "Porsche",
    "McLaren",
    "Ferrari",
    "Mercedes",
    "Mercedes-AMG",
    "Audi",
    "Lamborghini",
    "Nissan",
    "Bentley",
    "AMR",
    "Reiter",
    "Emil",
    "Lexus",
    "Honda",
    "KTM",
    "Maserati",
    "Alpine",
    "Chevrolet",
    "Ginetta",
    "Toyota",
  ];

  // Find where car name starts
  let mut car_start_idx = None;
  for (i, token) in tokens.iter().enumerate() {
    if car_starters.contains(token) {
      car_start_idx = Some(i);
      break;
    }
  }

  let car_start_idx = car_start_idx
    .ok_or_else(|| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)))?;

  let name_parts = &tokens[..car_start_idx];
  let car_parts = &tokens[car_start_idx..];

  let driver_name = name_parts.join(" ");
  let car_string = car_parts.join(" ");

  let car_model = CarModel::from_str(&car_string)
    .map_err(|_| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)))?;

  Ok((driver_name, car_model))
}

/// Parse timing values and flags
fn parse_timing_and_flags(input: &str) -> IResult<&str, (QualifyingTiming, bool, u32, bool)> {
  // Check for "No valid time" case first
  if input.contains("No valid time") {
    // Parse: "No valid time No valid time No valid time ❌ 4 ❌" or similar
    let (input, _) = parse_no_valid_time(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = parse_no_valid_time(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = parse_no_valid_time(input)?;
    let (input, _) = space1(input)?;
    let (input, valid) = parse_bool_indicator(input)?;
    let (input, _) = space1(input)?;
    let (input, lap_count) = parse_u32(input)?;
    let (input, _) = space1(input)?;
    let (input, qualified) = parse_bool_indicator(input)?;

    return Ok((
      input,
      (
        QualifyingTiming::no_valid_time(),
        valid,
        lap_count,
        qualified,
      ),
    ));
  }

  // Normal case: three times
  let (input, best) = parse_time(input)?;
  let (input, _) = space1(input)?;
  let (input, avg) = parse_time(input)?;
  let (input, _) = space1(input)?;
  let (input, result) = parse_time(input)?;
  let (input, _) = space1(input)?;
  let (input, valid) = parse_bool_indicator(input)?;
  let (input, _) = space1(input)?;
  let (input, lap_count) = parse_u32(input)?;
  let (input, _) = space1(input)?;
  let (input, qualified) = parse_bool_indicator(input)?;

  Ok((
    input,
    (
      QualifyingTiming::new(Some(best), Some(avg), Some(result)),
      valid,
      lap_count,
      qualified,
    ),
  ))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_time() {
    assert_eq!(parse_time("01:41.115"), Ok(("", 101115)));
    assert_eq!(parse_time("01:40.702"), Ok(("", 100702)));
    assert_eq!(parse_time("01:42.062"), Ok(("", 102062)));
  }

  #[test]
  fn test_parse_bool_indicator() {
    assert_eq!(parse_bool_indicator("✔️"), Ok(("", true)));
    assert_eq!(parse_bool_indicator("❌"), Ok(("", false)));
  }

  #[test]
  fn test_parse_no_valid_time() {
    assert_eq!(parse_no_valid_time("No valid time"), Ok(("", None)));
  }

  #[test]
  fn test_parse_entry_line() {
    let line = "1 500 Andi Lederer Ford Mustang GT3 01:41.115 01:41.463 01:41.289 ❌ 18 ❌";
    let result = parse_entry_line(line);
    assert!(result.is_ok());
    let (_, entry) = result.unwrap();
    assert_eq!(entry.position, 1);
    assert_eq!(entry.race_number, 500);
    assert_eq!(entry.driver_name, "Andi Lederer");
    assert_eq!(entry.car_model, CarModel::FordMustangGT3);
    assert_eq!(entry.timing.best_lap, Some(101115));
    assert!(!entry.valid_consecutive_laps);
    assert_eq!(entry.lap_count, 18);
    assert!(!entry.qualified);
  }

  #[test]
  fn test_parse_entry_with_multiword_car() {
    let line =
      "4 401 Yannick Petry McLaren 720S GT3 Evo 2023 01:41.255 01:41.691 01:41.473 ❌ 12 ❌";
    let result = parse_entry_line(line);
    assert!(result.is_ok());
    let (_, entry) = result.unwrap();
    assert_eq!(entry.position, 4);
    assert_eq!(entry.driver_name, "Yannick Petry");
    assert_eq!(entry.car_model, CarModel::McLaren720SGT3Evo);
  }
}
