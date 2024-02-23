use serde::{Deserialize, Serialize};

use super::continent::Continent;

/// Continent List
#[derive(Debug, Serialize, Deserialize)]
pub struct CountryList {
    /// Total number of continents documents that matched your query.
    total: u64,
    /// List of continents.
    continents: Vec<Continent>,
}
