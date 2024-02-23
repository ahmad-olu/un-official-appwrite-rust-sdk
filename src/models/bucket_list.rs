use serde::{Deserialize, Serialize};

use super::bucket::Bucket;

/// Buckets List
#[derive(Debug, Serialize, Deserialize)]
pub struct BucketList {
    /// Total number of buckets documents that matched your query.
    total: u64,
    /// List of buckets.
    buckets: Vec<Bucket>,
}
