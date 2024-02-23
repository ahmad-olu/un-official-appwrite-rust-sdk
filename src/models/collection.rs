use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::index::Index;

/// Collection
#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    /// Collection ID.
    #[serde(rename = "$id")]
    id: String,

    /// Collection creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    created_at: String,

    /// Collection update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    updated_at: String,

    /// Collection permissions. [Learn more about permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions")]
    permissions: Value,

    /// Database ID.
    #[serde(rename = "databaseId")]
    database_id: String,

    /// Collection name.
    name: String,

    /// Collection enabled. Can be &#039;enabled&#039; or &#039;disabled&#039;. When disabled, the collection is inaccessible to users, but remains accessible to Server SDKs using API keys.
    enabled: bool,

    /// Whether document-level permissions are enabled. [Learn more about permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "documentSecurity")]
    document_security: bool,

    /// Collection attributes.
    attributes: Value,

    /// Collection indexes.
    indexes: Vec<Index>,
}
