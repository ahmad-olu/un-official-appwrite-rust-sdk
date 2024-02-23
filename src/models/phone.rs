use serde::{Deserialize, Serialize};

/// Phone
#[derive(Debug, Serialize, Deserialize)]
pub struct Phone {
    /// Phone code.
    code: String,

    /// Country two-character ISO 3166-1 alpha code.
    #[serde(rename = "countryCode")]
    country_code: String,

    /// Country name.
    #[serde(rename = "countryName")]
    country_name: String,
}
