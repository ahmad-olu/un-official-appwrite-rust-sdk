use serde::{Deserialize, Serialize};

/// Database
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Database {
    /// Database ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Database name.
    pub name: String,

    /// Database creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// Database update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// If database is enabled. Can be &#039;enabled&#039; or &#039;disabled&#039;. When disabled, the database is inaccessible to users, but remains accessible to Server SDKs using API keys.
    pub enabled: bool,
}
