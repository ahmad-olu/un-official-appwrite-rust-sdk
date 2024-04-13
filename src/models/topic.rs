use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Topic
#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    /// Topic ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Message creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// Message update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// The name of the topic.
    pub name: String,

    /// Total count of email subscribers subscribed to the topic.
    #[serde(rename = "emailTotal")]
    pub email_total: usize,

    /// Total count of SMS subscribers subscribed to the topic.
    #[serde(rename = "smsTotal")]
    pub sms_total: usize,

    /// Total count of push subscribers subscribed to the topic.
    #[serde(rename = "pushTotal")]
    pub push_total: usize,

    /// Subscribe permissions.
    pub subscribe: Vec<Value>,
}
