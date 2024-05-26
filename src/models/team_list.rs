use serde::{Deserialize, Serialize};

use super::team::Team;

/// Teams List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct TeamList {
    /// Total number of teams documents that matched your query.
   pub     total: u64,
    /// List of teams.
   pub     teams: Vec<Team>,
}
