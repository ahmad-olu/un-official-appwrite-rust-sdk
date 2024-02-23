use serde::{Deserialize, Serialize};

/// Database
#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    /// Database ID.
    #[serde(rename = "$id")]
    id: String,

    /// Database name.
    name: String,

    /// Database creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    created_at: String,

    /// Database update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    updated_at: String,

    /// If database is enabled. Can be &#039;enabled&#039; or &#039;disabled&#039;. When disabled, the database is inaccessible to users, but remains accessible to Server SDKs using API keys.
    enabled: bool,
}
