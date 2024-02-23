use serde::{Deserialize, Serialize};

/// AttributeFloat
#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeFloat {
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
    min: Option<f64>,

    /// Maximum value to enforce for new documents.
    max: Option<f64>,

    /// Default value for attribute when not provided. Cannot be set when attribute is required.
    xdefault: Option<f64>,
}
