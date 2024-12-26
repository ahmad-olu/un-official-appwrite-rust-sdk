use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum RelationMutate {
    #[default]
    #[serde(rename = "cascade")]
    Cascade,
    #[serde(rename = "restrict")]
    Restrict,
    #[serde(rename = "setNull")]
    SetNull,
}

impl RelationMutate {
    /// Converts the enum to its serialized string representation
    pub fn as_serialized(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| panic!("Failed to serialize RelationMutate: {:?}", self))
            .trim_matches('"') // Remove surrounding quotes from the JSON string
            .to_string()
    }
}
