use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::index::Index;

/// Collection
#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    /// Collection ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Collection creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// Collection update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// Collection permissions. [Learn more about permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions")]
    pub permissions: Vec<Value>,

    /// Database ID.
    #[serde(rename = "databaseId")]
    pub database_id: String,

    /// Collection name.
    pub name: String,

    /// Collection enabled. Can be &#039;enabled&#039; or &#039;disabled&#039;. When disabled, the collection is inaccessible to users, but remains accessible to Server SDKs using API keys.
    pub enabled: bool,

    /// Whether document-level permissions are enabled. [Learn more about permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "documentSecurity")]
    pub document_security: bool,

    /// Collection attributes.
    pub attributes: Vec<Value>,

    /// Collection indexes.
    pub indexes: Vec<Index>,
}
