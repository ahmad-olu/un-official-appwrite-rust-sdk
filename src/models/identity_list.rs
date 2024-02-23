use serde::{Deserialize, Serialize};

use super::identity::Identity;

/// Identity List
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityList {
    /// Total number of identities documents that matched your query.
    total: u64,
    /// List of identities.
    identities: Vec<Identity>,
}
