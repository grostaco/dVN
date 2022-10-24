use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

// For use_async since reqwest::Error is not `Clone`
#[derive(ThisError, Clone, Debug, PartialEq)]
pub enum Error {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Not Found")]
    NotFound,

    #[error("Unprocessable Enttiy: {0:?}")]
    UnprocessableEntity(ErrorInfo),

    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Deserialize Error")]
    DeserializeError,

    #[error("Http Request Error")]
    RequestError,
}

pub type Result<T> = core::result::Result<T, Error>;
