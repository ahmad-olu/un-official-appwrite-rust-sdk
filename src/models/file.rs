use serde::{Deserialize, Serialize};

/// File
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct File {
    /// File ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Bucket ID.
    #[serde(rename = "bucketId")]
    pub bucket_id: String,

    /// File creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// File update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// File permissions. [Learn more about permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions")]
    pub permissions: Vec<String>,

    /// File name.
    pub name: String,

    /// File MD5 signature.
    pub signature: String,

    /// File mime type.
    #[serde(rename = "mimeType")]
    pub mime_type: String,

    /// File original size in bytes.
    #[serde(rename = "sizeOriginal")]
    pub size_original: usize,
    /// Total number of chunks available
    #[serde(rename = "chunksTotal")]
    pub chunks_total: usize,

    /// Total number of chunks uploaded
    #[serde(rename = "chunksUploaded")]
    pub chunks_uploaded: usize,
}
