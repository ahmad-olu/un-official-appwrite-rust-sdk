use serde::{Deserialize, Serialize};

use super::session::Session;

/// Session List
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionList {
    /// Total number of sessions documents that matched your query.
    total: u64,
    /// List of sessions.
    sessions: Vec<Session>,
}
