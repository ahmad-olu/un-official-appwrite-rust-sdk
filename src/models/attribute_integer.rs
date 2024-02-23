use serde::{Deserialize, Serialize};

/// AttributeInteger
#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeInteger {
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

    /// Minimum value to enforce for new documents.
    min: Option<u64>,

    /// Maximum value to enforce for new documents.
    max: Option<u64>,

    /// Default value for attribute when not provided. Cannot be set when attribute is required.
    xdefault: Option<u64>,
}
