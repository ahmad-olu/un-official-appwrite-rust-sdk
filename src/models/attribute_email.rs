use serde::{Deserialize, Serialize};

/// AttributeEmail
#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeEmail {
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
    pub xrequired: Option<bool>,

    /// Is attribute an array?
    pub array: Option<bool>,

    /// String format.
    pub format: String,

    /// Default value for attribute when not provided. Cannot be set when attribute is required.
    pub xdefault: Option<String>,
}
