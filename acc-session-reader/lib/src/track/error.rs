use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum ParseError {
  #[error("Invalid track name: {0}")]
  #[diagnostic(
    code(parser::invalid_track_name),
    help("Valid track names are: Red Bull Ring")
  )]
  InvalidTrackName(String),

  #[error("IO error: {message}")]
  #[diagnostic(code(parser::io_error))]
  IoError { message: String },
}

impl From<std::io::Error> for ParseError {
  fn from(source: std::io::Error) -> Self {
    ParseError::IoError {
      message: source.to_string(),
    }
  }
}
