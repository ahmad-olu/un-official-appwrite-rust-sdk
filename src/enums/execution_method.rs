use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum ExecutionMethod {
    #[default]
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "OPTIONS")]
    OPTIONS,
}

impl ExecutionMethod {
    /// Converts the enum to its serialized string representation
    pub fn as_serialized(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| panic!("Failed to serialize ExecutionMethod: {:?}", self))
            .trim_matches('"') // Remove surrounding quotes from the JSON string
            .to_string()
    }
}
