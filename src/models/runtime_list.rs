use serde::{Deserialize, Serialize};

use super::runtime::Runtime;

/// Runtimes List
#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeList {
    /// Total number of runtimes documents that matched your query.
    total: u64,
    /// List of runtimes.
    runtimes: Vec<Runtime>,
}
