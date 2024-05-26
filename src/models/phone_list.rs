use serde::{Deserialize, Serialize};

use super::phone::Phone;

/// Phones List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct PhoneList {
    /// Total number of phones documents that matched your query.
    pub total: u64,
    /// List of phones.
    pub sessions: Vec<Phone>,
}
