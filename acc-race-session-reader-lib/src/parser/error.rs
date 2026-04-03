use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum ParseError {
  #[error("Failed to parse JSON")]
  #[diagnostic(code(parser::json_error), help("Check if the input is valid JSON"))]
  JsonError {
    #[source]
    source: serde_json::Error,
    #[label("error occurred here")]
    span: Option<SourceSpan>,
  },

  #[error("Failed to parse PDF: {message}")]
  #[diagnostic(
    code(parser::pdf_error),
    help("Check if the input is a valid PDF file")
  )]
  PdfError { message: String },

  #[error("Invalid session type: {session_type}")]
  #[diagnostic(
    code(parser::invalid_session_type),
    help("Valid session types are: FP, Q, R")
  )]
  InvalidSessionType {
    session_type: String,
    #[label("invalid session type")]
    span: Option<SourceSpan>,
  },

  #[error("Missing required field: {field}")]
  #[diagnostic(code(parser::missing_field))]
  MissingField {
    field: String,
    #[label("field not found")]
    span: Option<SourceSpan>,
  },

  #[error("Invalid car model: {model}")]
  #[diagnostic(code(parser::invalid_car_model))]
  InvalidCarModel {
    model: String,
    #[label("invalid model")]
    span: Option<SourceSpan>,
  },

  #[error("IO error: {message}")]
  #[diagnostic(code(parser::io_error))]
  IoError {
    message: String,
    #[source]
    source: std::io::Error,
  },

  #[error("Invalid lap data")]
  #[diagnostic(code(parser::invalid_lap), help("Lap splits should sum to lap time"))]
  InvalidLap {
    #[label("invalid lap")]
    span: Option<SourceSpan>,
  },

  #[error("Empty session data")]
  #[diagnostic(code(parser::empty_session))]
  EmptySession,

  #[error("Invalid driver index: {index}")]
  #[diagnostic(code(parser::invalid_driver_index))]
  InvalidDriverIndex { index: u32 },

  #[error("UTF-8 decoding error")]
  #[diagnostic(
    code(parser::utf8_error),
    help("The file may be encoded in UTF-16 or another encoding")
  )]
  Utf8Error(#[from] std::str::Utf8Error),

  #[error("Invalid time format: {time}")]
  #[diagnostic(code(parser::invalid_time), help("Expected format: MM:SS.mmm"))]
  InvalidTimeFormat { time: String },

  #[error("Invalid track name: {name}")]
  #[diagnostic(code(parser::invalid_track_name))]
  InvalidTrackName { name: String },
}

impl From<serde_json::Error> for ParseError {
  fn from(source: serde_json::Error) -> Self {
    ParseError::JsonError { source, span: None }
  }
}

impl From<std::io::Error> for ParseError {
  fn from(source: std::io::Error) -> Self {
    ParseError::IoError {
      message: source.to_string(),
      source,
    }
  }
}
