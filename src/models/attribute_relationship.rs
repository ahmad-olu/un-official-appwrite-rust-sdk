use serde::{Deserialize, Serialize};

/// AttributeString
#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeRelationship {
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

    /// The ID of the related collection.
    #[serde(rename = "relatedCollection")]
    related_collection: String,

    /// The type of the relationship.
    #[serde(rename = "relationType")]
    relation_type: String,

    /// Is the relationship two-way?
    #[serde(rename = "twoWay")]
    two_way: bool,

    /// The key of the two-way relationship.
    #[serde(rename = "twoWayKey")]
    two_way_key: String,

    /// How deleting the parent document will propagate to child documents.
    #[serde(rename = "onDelete")]
    on_delete: String,

    /// Whether this is the parent or child side of the relationship
    #[serde(rename = "type")]
    side: String,
}
