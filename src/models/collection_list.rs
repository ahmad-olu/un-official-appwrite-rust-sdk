use serde::{Deserialize, Serialize};

use super::collection::Collection;

/// Collections List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct CollectionList {
    /// Total number of collections documents that matched your query.
    pub total: u64,
    /// List of collections.
    pub collections: Vec<Collection>,
}
