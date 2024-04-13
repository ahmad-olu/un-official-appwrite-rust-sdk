use serde::{Deserialize, Serialize};

use super::bucket::Bucket;

/// Buckets List
#[derive(Debug, Serialize, Deserialize)]
pub struct BucketList {
    /// Total number of buckets documents that matched your query.
    pub total: u64,
    /// List of buckets.
    pub buckets: Vec<Bucket>,
}
