use serde::{Deserialize, Serialize};

use super::user::User;

/// User List
#[derive(Debug, Serialize, Deserialize)]
pub struct UserList {
    /// Total number of users documents that matched your query.
    pub total: u64,
    /// List of users.
    pub users: Vec<User>,
}
