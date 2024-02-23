use serde::{Deserialize, Serialize};

use super::currency::Currency;

/// Currencies List
#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyList {
    /// Total number of currencies documents that matched your query.
    total: u64,
    /// List of currencies.
    currencies: Vec<Currency>,
}
