use serde::{Deserialize, Serialize};

/// Language
#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    /// Language name.
    name: String,
    /// Language two-character ISO 639-1 codes.
    code: String,
    /// Language native name.
    #[serde(rename = "nativeName")]
    native_name: String,
}
