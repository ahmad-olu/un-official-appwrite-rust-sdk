use serde::{Deserialize, Serialize};

/// Target
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Target {
    /// Target ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Message creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// Message update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// Target Name.
    pub name: String,

    /// User ID.
    #[serde(rename = "userId")]
    pub user_id: String,

    /// Provider ID.
    #[serde(rename = "providerId")]
    pub provider_id: Option<String>,

    /// The target provider type. Can be one of the following: `email`, `sms` or `push`.
    #[serde(rename = "providerType")]
    pub provider_type: String,

    /// The target identifier.
    pub identifier: String,
}
