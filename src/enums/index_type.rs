use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum IndexType {
    #[default]
    #[serde(rename = "key")]
    Key,
    #[serde(rename = "fulltext")]
    Fulltext,
    #[serde(rename = "unique")]
    Unique,
}

impl IndexType {
    /// Converts the enum to its serialized string representation
    pub fn as_serialized(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| panic!("Failed to serialize IndexType: {:?}", self))
            .trim_matches('"') // Remove surrounding quotes from the JSON string
            .to_string()
    }
}
