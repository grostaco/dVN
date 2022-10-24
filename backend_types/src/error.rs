#[cfg(feature = "rocket")]
use rocket::serde::json::Json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Error {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ErrorOr<T> {
    Response(T),
    Error(Error),
}

#[cfg(feature = "rocket")]
pub type Result<T> = Json<ErrorOr<T>>;

impl<T> ErrorOr<T> {
    #[inline]
    #[cfg(feature = "rocket")]
    pub fn json_error_from<E: ToString>(e: E) -> Json<Self> {
        Json(Self::Error(Error {
            message: e.to_string(),
        }))
    }

    #[inline]
    #[cfg(feature = "rocket")]
    pub fn json_response_from(response: T) -> Json<Self> {
        Json(Self::Response(response))
    }
}
