use serde::{Deserialize, Serialize};

use super::currency::Currency;

/// Currencies List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct CurrencyList {
    /// Total number of currencies documents that matched your query.
    pub total: u64,
    /// List of currencies.
    pub currencies: Vec<Currency>,
}
