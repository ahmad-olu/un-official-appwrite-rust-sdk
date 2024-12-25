use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Index
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Index {
    /// Index Key.
    pub key: String,
    /// Index type.
    #[serde(rename = "type")]
    pub index_type: String,
    /// Index status. Possible values: `available`, `processing`, `deleting`, `stuck`, or `failed`
    pub status: String,
    /// Error message. Displays error generated on failure of creating or deleting an index.
    pub error: String,
    /// Index attributes.
    pub attributes: Vec<Value>,
    /// Index orders.
    pub orders: Option<Vec<Value>>,
}
