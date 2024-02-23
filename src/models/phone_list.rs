use serde::{Deserialize, Serialize};

use super::phone::Phone;

/// Phones List
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneList {
    /// Total number of phones documents that matched your query.
    total: u64,
    /// List of phones.
    sessions: Vec<Phone>,
}
