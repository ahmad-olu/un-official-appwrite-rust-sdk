use serde::{Deserialize, Serialize};

use super::index::Index;

/// Indexes List
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexList {
    /// Total number of indexes documents that matched your query.
    total: u64,
    /// List of indexes.
    indexes: Vec<Index>,
}
