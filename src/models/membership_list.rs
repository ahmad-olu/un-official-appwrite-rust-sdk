use serde::{Deserialize, Serialize};

use super::membership::Membership;

/// Membership List
#[derive(Debug, Serialize, Deserialize)]
pub struct MembershipList {
    /// Total number of memberships documents that matched your query.
    pub total: u64,
    /// List of memberships.
    pub sessions: Vec<Membership>,
}
