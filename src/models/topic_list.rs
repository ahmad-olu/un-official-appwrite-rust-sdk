use serde::{Deserialize, Serialize};

use super::topic::Topic;

/// Topic List
#[derive(Debug, Serialize, Deserialize)]
pub struct TopicList {
    /// Total number of topics documents that matched your query.
    pub total: usize,
    /// List of topics.
    pub topics: Vec<Topic>,
}
