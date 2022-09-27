use std::fmt::Display;

use directive_errors::{ParseError, VerifyError};
use image::ImageError;
use thiserror::Error;

#[derive(Debug)]
pub struct Span {
    pub line: u64,
    pub column: u64,
}

#[derive(Debug)]
pub struct Error {
    pub file: String,
    pub span: Span,
    pub cause: ErrorCause,
}

#[derive(Debug, Error)]
pub enum ErrorCause {
    #[error("{0}")]
    Parse(#[from] ParseError),
    #[error("{0}")]
    Verify(#[from] VerifyError),

    #[error("unrecognized directive `{0}`")]
    Unrecognized(String),

    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    ImageError(#[from] ImageError),
}

impl Error {
    pub fn new<S: Into<String>>(file: S, span: Span, cause: ErrorCause) -> Self {
        Self {
            file: file.into(),
            span,
            cause,
        }
    }
}

impl Span {
    pub fn new(line: u64, column: u64) -> Self {
        Self { line, column }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{} error: {}",
            self.file, self.span.line, self.span.column, self.cause
        )
    }
}
