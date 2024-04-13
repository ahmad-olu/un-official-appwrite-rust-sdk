use serde::{Deserialize, Serialize};

/// Language
#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    /// Language name.
    pub name: String,
    /// Language two-character ISO 639-1 codes.
    pub code: String,
    /// Language native name.
    #[serde(rename = "nativeName")]
    pub native_name: String,
}
