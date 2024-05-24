use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("file size error: {0:?}")]
    Io(#[from] std::io::Error),
    #[error("network error: {0:?}")]
    Network(#[from] reqwest::Error),
    #[error("AppWrite error:-> code:{code:?} -> message:{message:?} -> response:{response:?} -> type:{error_type:?}")]
    AppWriteError {
        message: String,
        code: Option<u64>,
        response: Option<String>,
        error_type: Option<String>,
    },

    #[error("invalid header name: {0:#?}")]
    HeaderName(#[from] InvalidHeaderName),
    #[error("invalid header value: {0:#?}")]
    HeaderValue(#[from] InvalidHeaderValue),

    #[error("The file path `{0}` does not exist")]
    FilePathNotExist(String),

    #[error("Unknown error: probably a None Type")]
    Unknown,

    #[error("wrong upload type")]
    WrongUploadType,

    #[error("Custom error: {0}")]
    Custom(String),
}

#[derive(Debug, Deserialize)]
pub struct AppWriteError {
    /// Error message.
    pub message: String,

    pub code: Option<u64>,
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
