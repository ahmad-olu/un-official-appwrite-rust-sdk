use serde::{Deserialize, Serialize};

use super::index::Index;

/// Indexes List
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexList {
    /// Total number of indexes documents that matched your query.
   pub     total: u64,
    /// List of indexes.
   pub     indexes: Vec<Index>,
}
