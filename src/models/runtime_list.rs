use serde::{Deserialize, Serialize};

use super::runtime::Runtime;

/// Runtimes List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct RuntimeList {
    /// Total number of runtimes documents that matched your query.
    pub total: u64,
    /// List of runtimes.
    pub runtimes: Vec<Runtime>,
}
