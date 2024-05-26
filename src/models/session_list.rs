use serde::{Deserialize, Serialize};

use super::session::Session;

/// Session List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct SessionList {
    /// Total number of sessions documents that matched your query.
    pub total: u64,
    /// List of sessions.
    pub sessions: Vec<Session>,
}
