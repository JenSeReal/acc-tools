use std::str::FromStr;

use super::error::ParseError;

use super::model::SessionType;

impl FromStr for SessionType {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "FP" => Ok(Self::FreePractice),
      "Q" => Ok(Self::Qualifying),
      "R" => Ok(Self::Race),
      _ => Err(ParseError::InvalidSessionType(s.to_string())),
    }
  }
}
