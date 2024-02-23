use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Bucket
#[derive(Debug, Serialize, Deserialize)]
pub struct Bucket {
    /// Bucket ID.
    #[serde(rename = "$id")]
    id: String,

    /// Bucket creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    created_at: String,

    /// Bucket update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    updated_at: String,

    /// Bucket permissions. [Learn more about permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions")]
    permissions: Value,

    /// Whether file-level security is enabled. [Learn more about permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "fileSecurity")]
    file_security: bool,

    /// Bucket name.
    name: String,

    /// Bucket enabled.
    enabled: bool,

    /// Maximum file size supported.
    #[serde(rename = "maximumFileSize")]
    maximum_file_size: u64,

    /// Allowed file extensions.
    #[serde(rename = "allowedFileExtensions")]
    allowed_file_extensions: Value,

    /// Compression algorithm choosen for compression. Will be one of none, [gzip](https://en.wikipedia.org/wiki/Gzip), or [zstd](https://en.wikipedia.org/wiki/Zstd).
    compression: String,

    /// Bucket is encrypted.
    encryption: bool,

    /// Virus scanning is enabled.
    antivirus: bool,
}
