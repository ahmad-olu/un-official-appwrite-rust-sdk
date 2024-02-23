use serde::{Deserialize, Serialize};

use super::log::Log;

/// Log List
#[derive(Debug, Serialize, Deserialize)]
pub struct LogList {
    /// Total number of logs documents that matched your query.
    total: u64,
    /// List of logs.
    sessions: Vec<Log>,
}
