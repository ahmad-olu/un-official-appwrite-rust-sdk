use serde::{Deserialize, Serialize};

/// AttributeString
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct AttributeRelationship {
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

    /// The ID of the related collection.
    #[serde(rename = "relatedCollection")]
    pub related_collection: String,

    /// The type of the relationship.
    #[serde(rename = "relationType")]
    pub relation_type: String,

    /// Is the relationship two-way?
    #[serde(rename = "twoWay")]
    pub two_way: bool,

    /// The key of the two-way relationship.
    #[serde(rename = "twoWayKey")]
    pub two_way_key: String,

    /// How deleting the parent document will propagate to child documents.
    #[serde(rename = "onDelete")]
    pub on_delete: String,

    /// Whether this is the parent or child side of the relationship
    pub side: String,
}
