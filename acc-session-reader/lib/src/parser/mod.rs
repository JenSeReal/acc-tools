mod error;
mod json;
mod pdf;

pub use error::ParseError;
pub use json::JsonParser;
pub use pdf::PdfParser;

use crate::{
  qualifying_session::QualifyingSession, session::model::Session, session_type::model::SessionType,
  track::model::Track,
};

/// Trait for parsing session data from different formats.
///
/// Each parser implementation specifies its own output type, allowing
/// JSON and PDF parsers to return different session representations.
pub trait SessionParser {
  type Output;
  type Error;

  fn parse(data: &[u8]) -> Result<Self::Output, Self::Error>;
}

/// Trait for accessing common session information across different session types.
///
/// This allows generic code to work with both JSON-parsed `Session` and
/// PDF-parsed `QualifyingSession` types.
pub trait SessionInfo {
  fn track(&self) -> &Track;
  fn session_type(&self) -> SessionType;
  fn entry_count(&self) -> usize;
}

impl SessionInfo for Session {
  fn track(&self) -> &Track {
    &self.track_name
  }

  fn session_type(&self) -> SessionType {
    self.session_type
  }

  fn entry_count(&self) -> usize {
    self.session_result.leader_board_lines.len()
  }
}

impl SessionInfo for QualifyingSession {
  fn track(&self) -> &Track {
    &self.track_name
  }

  fn session_type(&self) -> SessionType {
    self.session_type
  }

  fn entry_count(&self) -> usize {
    self.entries.len()
  }
}
