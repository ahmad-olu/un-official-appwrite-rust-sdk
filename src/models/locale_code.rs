use serde::{Deserialize, Serialize};

/// LocaleCode

#[derive(Debug, Serialize, Deserialize)]
pub struct LocaleCode {
    /// Locale codes in [ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes)
    pub code: String,
    /// Locale name
    pub name: String,
}
