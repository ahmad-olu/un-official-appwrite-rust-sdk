use serde::{Deserialize, Serialize};

use super::collection::Collection;

/// Collections List
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionList {
    /// Total number of collections documents that matched your query.
    total: u64,
    /// List of collections.
    collections: Vec<Collection>,
}
