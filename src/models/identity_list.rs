use serde::{Deserialize, Serialize};

use super::identity::Identity;

/// Identity List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct IdentityList {
    /// Total number of identities documents that matched your query.
    pub total: u64,
    /// List of identities.
    pub identities: Vec<Identity>,
}
