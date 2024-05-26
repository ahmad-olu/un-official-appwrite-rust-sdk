use serde::{Deserialize, Serialize};

use super::log::Log;

/// Log List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct LogList {
    /// Total number of logs documents that matched your query.
    pub total: u64,
    /// List of logs.
    pub sessions: Vec<Log>,
}
