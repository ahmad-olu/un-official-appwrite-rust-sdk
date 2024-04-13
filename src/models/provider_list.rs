use serde::{Deserialize, Serialize};

use super::provider::Provider;

/// Provider List
#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderList {
    /// Total number of providers documents that matched your query.
    pub total: usize,
    /// List of providers.
    pub providers: Vec<Provider>,
}
