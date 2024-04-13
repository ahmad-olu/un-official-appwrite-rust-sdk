use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Document
#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    /// Document ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Collection ID.
    #[serde(rename = "$collectionId")]
    pub collection_id: String,

    /// Database ID.
    #[serde(rename = "$databaseId")]
    pub database_id: String,

    /// Document creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// Document update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// Document permissions. [Learn more about permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions")]
    pub permissions: Vec<String>,

    #[serde(flatten)]
    pub data: Map<String, Value>,
}
