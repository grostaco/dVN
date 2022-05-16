use thiserror::Error;

#[derive(Debug, Error)]
pub enum DirectiveParseError {
    #[error("cannot parse {0} as {1}: {2}")]
    CannotParse(String, String, String),

    #[error("expected argument {0} at position {1}")]
    ExpectPositionalArgument(String, usize),
}
