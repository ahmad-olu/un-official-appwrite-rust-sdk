use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("file size error: {0:?}")]
    Io(#[from] std::io::Error),
    #[error("network error: {0:?}")]
    Network(#[from] reqwest::Error),
    #[error("AppWrite error: {0:?}")]
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
    pub message: String,

    code: Option<u64>,
    pub response: Option<String>,
    /// Error type.
    ///
    /// See [Error Types](https://appwrite.io/docs/response-codes#errorTypes)
    /// for more information.
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub version: Option<String>,
}

impl std::fmt::Display for AppWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}) {} ", self.code, self.message,)
    }
}

impl std::error::Error for AppWriteError {}
