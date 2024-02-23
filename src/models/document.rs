use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Document
#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    /// Document ID.
    #[serde(rename = "$id")]
    id: String,

    /// Collection ID.
    #[serde(rename = "$collectionId")]
    collection_id: String,

    /// Database ID.
    #[serde(rename = "$databaseId")]
    database_id: String,

    /// Document creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    created_at: String,

    /// Document update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    updated_at: String,

    /// Document permissions. [Learn more about permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions")]
    permissions: Vec<String>,

    #[serde(flatten)]
    data: Map<String, Value>,
}
