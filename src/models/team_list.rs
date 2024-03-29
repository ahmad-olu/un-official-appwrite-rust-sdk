use serde::{Deserialize, Serialize};

use super::team::Team;

/// Teams List
#[derive(Debug, Serialize, Deserialize)]
pub struct TeamList {
    /// Total number of teams documents that matched your query.
    total: u64,
    /// List of teams.
    teams: Vec<Team>,
}
