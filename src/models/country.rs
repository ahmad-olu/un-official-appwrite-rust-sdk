use serde::{Deserialize, Serialize};

/// Country
#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    /// Country name.
    name: String,
    /// Country two-character ISO 3166-1 alpha code.
    code: String,
}
