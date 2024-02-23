use serde::{Deserialize, Serialize};
use serde_json::Value;

/// AttributeEnum
#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeEnum {
    /// Attribute Key.
    key: String,

    /// Attribute type.
    #[serde(rename = "type")]
    attribute_type: String,

    /// Attribute status. Possible values: `available`, `processing`, `deleting`, `stuck`, or `failed`
    status: String,

    /// Error message. Displays error generated on failure of creating or deleting an attribute.
    error: String,

    /// Is attribute required?
    xrequired: bool,

    /// Is attribute an array?
    array: Option<bool>,

    /// Array of elements in enumerated type.
    elements: Value,

    /// String format.
    format: String,

    /// Default value for attribute when not provided. Cannot be set when attribute is required.
    xdefault: Option<String>,
}
