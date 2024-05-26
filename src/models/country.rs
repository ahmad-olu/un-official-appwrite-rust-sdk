use serde::{Deserialize, Serialize};

/// Country
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Country {
    /// Country name.
    pub name: String,
    /// Country two-character ISO 3166-1 alpha code.
    pub code: String,
}
