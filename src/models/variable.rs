use serde::{Deserialize, Serialize};

/// Variable
#[derive(Debug, Serialize, Deserialize)]
pub struct Variable {
    /// Variable ID.
    #[serde(rename = "$id")]
    id: String,

    /// Variable creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    created_at: String,

    /// Variable update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    updated_at: String,

    /// Variable key.
    key: String,

    /// Variable value.
    value: String,

    /// Service to which the variable belongs. Possible values are &quot;project&quot;, &quot;function&quot;
    #[serde(rename = "resourceType")]
    resource_type: String,

    /// ID of resource to which the variable belongs. If resourceType is &quot;project&quot;, it is empty. If resourceType is &quot;function&quot;, it is ID of the function.
    #[serde(rename = "resourceId")]
    resource_id: String,
}
