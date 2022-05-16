use proc_macro::{Span, TokenTree};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KVParseError {
    #[error("Invalid punctuation {0} at {1:?}")]
    InvalidPunct(char, Span),
    #[error("Expected a value at {0:?}")]
    MissingValue(Span),
    #[error("Unexpected token tree {0}")]
    UnexpectedTokenTree(TokenTree),
    #[error("Expected a punctuation at {0:?}")]
    ExpectedPunct(Span),
    #[error("Expected a literal at {0:?}")]
    ExpectedLiteral(Span),
}

#[derive(Debug, Error)]
pub enum DirectiveParseError {
    #[error("Cannot parse {0} as {1}")]
    CannotParse(String, String),
}
