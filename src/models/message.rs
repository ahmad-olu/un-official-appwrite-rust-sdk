use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Message
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Message {
    /// Message ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Message creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// Message update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// Message provider type.
    #[serde(rename = "providerType")]
    pub provider_type: String,

    /// Topic IDs set as recipients.
    pub topics: Vec<Value>,

    /// User IDs set as recipients.
    pub users: Vec<Value>,

    /// Target IDs set as recipients.
    pub targets: Vec<Value>,

    /// The scheduled time for message.
    #[serde(rename = "scheduledAt")]
    pub scheduled_at: Option<String>,

    /// The time when the message was delivered.
    #[serde(rename = "deliveredAt")]
    pub delivered_at: Option<String>,

    /// Delivery errors if any.
    #[serde(rename = "deliveryErrors")]
    pub delivery_errors: Option<Vec<Value>>,

    /// Number of recipients the message was delivered to.
    #[serde(rename = "deliveredTotal")]
    pub delivered_total: usize,

    /// Data of the message.
    pub data: HashMap<String, Value>,

    /// Status of delivery.
    pub status: String,
}
