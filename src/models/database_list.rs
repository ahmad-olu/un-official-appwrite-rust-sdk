use serde::{Deserialize, Serialize};

use super::database::Database;

/// Databases List
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseList {
    /// Total number of databases documents that matched your query.
    total: u64,
    /// List of databases.
    databases: Vec<Database>,
}
