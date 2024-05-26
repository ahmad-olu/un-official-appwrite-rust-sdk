use serde::{Deserialize, Serialize};

/// Variable
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Variable {
    /// Variable ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Variable creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// Variable update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// Variable key.
    pub key: String,

    /// Variable value.
    pub value: String,

    /// Service to which the variable belongs. Possible values are &quot;project&quot;, &quot;function&quot;
    #[serde(rename = "resourceType")]
    pub resource_type: String,

    /// ID of resource to which the variable belongs. If resourceType is &quot;project&quot;, it is empty. If resourceType is &quot;function&quot;, it is ID of the function.
    #[serde(rename = "resourceId")]
    pub resource_id: String,
}
