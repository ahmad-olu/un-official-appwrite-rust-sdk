use serde::{Deserialize, Serialize};

/// Progress of a File Upload
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct UploadProgress {
    /// ID of the file.
    #[serde(rename = "$id")]
    pub id: String,

    /// Progress percentage.
    pub progress: usize,

    /// Size uploaded in bytes.
    #[serde(rename = "sizeUploaded")]
    pub size_uploaded: usize,

    /// Total number of chunks.
    #[serde(rename = "chunksTotal")]
    pub chunks_total: usize,

    /// Number of chunks uploaded.
    #[serde(rename = "chunksUploaded")]
    pub chunks_uploaded: usize,
}
