use serde::{Deserialize, Serialize};

use super::country::Country;

/// Countries List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct CountryList {
    /// Total number of countries documents that matched your query.
    pub total: u64,
    /// List of countries.
    pub countries: Vec<Country>,
}
