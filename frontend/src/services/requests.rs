use std::fmt::Debug;

use dotenv_codegen::dotenv;
use reqwest::{Client, Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

use super::error::{Error, ErrorInfo};

const API_ROOT: &str = dotenv!("API_ROOT");

macro_rules! request {
    (get -> $url:expr) => {
        crate::services::requests::request_impl(reqwest::Method::GET, $url, ())
    };

    (delete -> $url:expr) => {
        crate::services::requests::request_impl(reqwest::Method::DELETE, $url, ())
    };

    (post -> $url:expr ; $body:expr) => {
        crate::services::requests::request_impl(reqwest::Method::POST, $url, $body)
    };
}

pub(crate) use request;

pub async fn request_impl<B, T>(method: Method, url: &str, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + Debug,
    B: Serialize + Debug,
{
    let allow_body = matches!(method, Method::POST | Method::PUT);
    let url = format!("{}{}", API_ROOT, url);

    let mut builder = Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");

    if allow_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    if let Ok(data) = response {
        match data.status() {
            StatusCode::OK => {
                let data = data.json::<T>().await;
                match data {
                    Ok(data) => Ok(data),
                    Err(_) => Err(Error::DeserializeError),
                }
            }
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            StatusCode::FORBIDDEN => Err(Error::Forbidden),
            StatusCode::NOT_FOUND => Err(Error::NotFound),
            StatusCode::INTERNAL_SERVER_ERROR => Err(Error::InternalServerError),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let data: Result<ErrorInfo, _> = data.json::<ErrorInfo>().await;
                match data {
                    Ok(data) => Err(Error::UnprocessableEntity(data)),
                    Err(_) => Err(Error::DeserializeError),
                }
            }
            _ => Err(Error::RequestError),
        }
    } else {
        Err(Error::RequestError)
    }
}
