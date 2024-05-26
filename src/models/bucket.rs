use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Bucket
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Bucket {
    /// Bucket ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Bucket creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// Bucket update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// Bucket permissions. [Learn more about permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions")]
    pub permissions: Value,

    /// Whether file-level security is enabled. [Learn more about permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "fileSecurity")]
    pub file_security: bool,

    /// Bucket name.
    pub name: String,

    /// Bucket enabled.
    pub enabled: bool,

    /// Maximum file size supported.
    #[serde(rename = "maximumFileSize")]
    pub maximum_file_size: usize,

    /// Allowed file extensions.
    #[serde(rename = "allowedFileExtensions")]
    pub allowed_file_extensions: Vec<Value>,

    /// Compression algorithm choosen for compression. Will be one of none, [gzip](https://en.wikipedia.org/wiki/Gzip), or [zstd](https://en.wikipedia.org/wiki/Zstd).
    pub compression: String,

    /// Bucket is encrypted.
    pub encryption: bool,

    /// Virus scanning is enabled.
    pub antivirus: bool,
}
