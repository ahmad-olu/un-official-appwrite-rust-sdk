use serde::{Deserialize, Serialize};

/// Continent
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Continent {
    /// continent name.
    pub name: String,
    /// continent two letter code.
    pub code: String,
}
