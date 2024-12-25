use serde::{Deserialize, Serialize};

/// AttributeFloat
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct AttributeFloat {
    /// Attribute Key.
    pub key: String,

    /// Attribute type.
    #[serde(rename = "type")]
    pub attribute_type: String,

    /// Attribute status. Possible values: `available`, `processing`, `deleting`, `stuck`, or `failed`
    pub status: String,

    /// Error message. Displays error generated on failure of creating or deleting an attribute.
    pub error: String,

    /// Is attribute required?
    pub xrequired: bool,

    /// Is attribute an array?
    pub array: Option<bool>,

    /// Minimum value to enforce for new documents.
    pub min: Option<f64>,

    /// Maximum value to enforce for new documents.
    pub max: Option<f64>,

    /// Default value for attribute when not provided. Cannot be set when attribute is required.
    pub xdefault: Option<f64>,
}
