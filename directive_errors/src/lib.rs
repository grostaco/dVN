use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("cannot parse {0} as {1}: {2}")]
    CannotParse(String, String, String),

    #[error("expected argument {0} at position {1}")]
    ExpectPositionalArgument(String, usize),

    #[error("{0}")]
    VerifyError(VerifyError),
}

#[derive(Debug, Error)]
pub enum VerifyError {
    #[error("{0}")]
    Custom(String),
}
pub trait Directive {
    fn parse(ctx: &str) -> Option<Result<Self, ParseError>>
    where
        Self: Sized;

    // If the directive is infallible, there is no reason to implement `verify`
    fn verify(&self) -> Result<(), VerifyError> {
        Ok(())
    }
}
