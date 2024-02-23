use serde::{Deserialize, Serialize};

/// AttributeString
#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeString {
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

    /// Attribute size.
    size: u64,

    /// Default value for attribute when not provided. Cannot be set when attribute is required.
    xdefault: Option<String>,
}
