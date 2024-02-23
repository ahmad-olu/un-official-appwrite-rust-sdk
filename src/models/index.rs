use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Index
#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    /// Index Key.
    key: String,
    /// Index type.
    #[serde(rename = "type")]
    index_type: String,
    /// Index status. Possible values: `available`, `processing`, `deleting`, `stuck`, or `failed`
    status: String,
    /// Error message. Displays error generated on failure of creating or deleting an index.
    error: String,
    /// Index attributes.
    attributes: Value,
    /// Index orders.
    orders: Option<Value>,
}
