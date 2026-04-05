use std::str::FromStr;

use encoding_rs::{UTF_8, UTF_16BE, UTF_16LE};

use super::error::ParseError;
use crate::session::model::Session;

impl TryFrom<&[u8]> for Session {
  type Error = ParseError;

  fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
    // Detect and decode the encoding
    let decoded = if bytes.len() >= 2 {
      // Check for UTF-16 BOM or null bytes indicating UTF-16
      if bytes[0] == 0xFF && bytes[1] == 0xFE {
        // UTF-16 LE BOM
        let (cow, _, had_errors) = UTF_16LE.decode(bytes);
        if had_errors {
          return Err(ParseError::Utf8Error(
            std::str::from_utf8(bytes).unwrap_err(),
          ));
        }
        cow
      } else if bytes[0] == 0xFE && bytes[1] == 0xFF {
        // UTF-16 BE BOM
        let (cow, _, had_errors) = UTF_16BE.decode(bytes);
        if had_errors {
          return Err(ParseError::Utf8Error(
            std::str::from_utf8(bytes).unwrap_err(),
          ));
        }
        cow
      } else if bytes[1] == 0x00 && bytes[0] != 0x00 {
        // Likely UTF-16 LE without BOM (null bytes after chars)
        let (cow, _, had_errors) = UTF_16LE.decode(bytes);
        if had_errors {
          return Err(ParseError::Utf8Error(
            std::str::from_utf8(bytes).unwrap_err(),
          ));
        }
        cow
      } else if bytes[0] == 0x00 && bytes[1] != 0x00 {
        // Likely UTF-16 BE without BOM
        let (cow, _, had_errors) = UTF_16BE.decode(bytes);
        if had_errors {
          return Err(ParseError::Utf8Error(
            std::str::from_utf8(bytes).unwrap_err(),
          ));
        }
        cow
      } else {
        // Assume UTF-8
        let (cow, _, had_errors) = UTF_8.decode(bytes);
        if had_errors {
          return Err(ParseError::Utf8Error(
            std::str::from_utf8(bytes).unwrap_err(),
          ));
        }
        cow
      }
    } else {
      // Too short, assume UTF-8
      let (cow, _, had_errors) = UTF_8.decode(bytes);
      if had_errors {
        return Err(ParseError::Utf8Error(
          std::str::from_utf8(bytes).unwrap_err(),
        ));
      }
      cow
    };

    serde_json::from_str(&decoded).map_err(|e| ParseError::JsonError {
      source: e,
      span: None,
    })
  }
}

impl FromStr for Session {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    serde_json::from_str(s).map_err(|e| ParseError::JsonError {
      source: e,
      span: None,
    })
  }
}
