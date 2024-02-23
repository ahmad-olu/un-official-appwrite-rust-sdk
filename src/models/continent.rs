use serde::{Deserialize, Serialize};

/// Continent
#[derive(Debug, Serialize, Deserialize)]
pub struct Continent {
    /// continent name.
    name: String,
    /// continent two letter code.
    code: String,
}
