use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Provider
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Provider {
    /// Provider ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Message creation time in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// Message update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// The name for the provider instance.
    pub name: String,

    /// The name of the provider service.
    pub provider: String,

    /// Is provider enabled?
    pub enabled: bool,

    /// Type of provider.
    #[serde(rename = "type")]
    pub provider_type: String,

    /// Provider credentials.
    pub credentials: HashMap<String, Value>,

    /// Provider options.
    pub options: Option<HashMap<String, Value>>,
}
