use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("file size error: {0:#?}")]
    Io(#[from] std::io::Error),
    #[error("network error: {0:#?}")]
    Network(#[from] reqwest::Error),
    #[error("AppWrite error: {0:#?}")]
    AppWriteError(AppWriteError),

    #[error("invalid header name: {0:#?}")]
    HeaderName(#[from] InvalidHeaderName),
    #[error("invalid header value: {0:#?}")]
    HeaderValue(#[from] InvalidHeaderValue),

    #[error("The file path `{0}` does not exist")]
    FilePathNotExist(String),

    #[error("Unknown error: probably a None Type")]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct AppWriteError {
    /// Error message.
    message: String,

    code: Option<u64>,
    response: String,
    /// Error type.
    ///
    /// See [Error Types](https://appwrite.io/docs/response-codes#errorTypes)
    /// for more information.
    #[serde(rename = "type")]
    error_type: Option<String>,
}
