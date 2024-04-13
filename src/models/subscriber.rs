use serde::{Deserialize, Serialize};

use super::target::Target;

/// Subscriber
#[derive(Debug, Serialize, Deserialize)]
pub struct Subscriber {
    /// Subscriber ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Message creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// Message update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// Target ID.
    #[serde(rename = "targetId")]
    pub target_id: String,

    /// Target.
    pub target: Target,

    /// Topic ID.
    #[serde(rename = "userId")]
    pub user_id: String,

    /// User Name.
    #[serde(rename = "userName")]
    pub user_name: String,

    /// Topic ID.
    #[serde(rename = "topicId")]
    pub topic_id: String,

    /// The target provider type. Can be one of the following: `email`, `sms` or `push`.
    #[serde(rename = "providerType")]
    pub provider_type: String,
}
