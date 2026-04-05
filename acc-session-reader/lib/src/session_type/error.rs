use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum ParseError {
  #[error("Invalid session type: {0}")]
  #[diagnostic(
    code(parser::invalid_session_type),
    help("Valid session types are: FP, Q, R")
  )]
  InvalidSessionType(String),

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
