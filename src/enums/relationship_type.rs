use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum RelationshipType {
    #[default]
    #[serde(rename = "oneToOne")]
    OneToOne,
    #[serde(rename = "manyToOne")]
    ManyToOne,
    #[serde(rename = "manyToMany")]
    ManyToMany,
    #[serde(rename = "oneToMany")]
    OneToMany,
}

impl RelationshipType {
    /// Converts the enum to its serialized string representation
    pub fn as_serialized(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| panic!("Failed to serialize RelationshipType: {:?}", self))
            .trim_matches('"') // Remove surrounding quotes from the JSON string
            .to_string()
    }
}
