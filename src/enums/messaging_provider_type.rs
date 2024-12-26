use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum MessagingProviderType {
    #[default]
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "push")]
    Push,
}

impl MessagingProviderType {
    /// Converts the enum to its serialized string representation
    pub fn as_serialized(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| panic!("Failed to serialize MessagingProviderType: {:?}", self))
            .trim_matches('"') // Remove surrounding quotes from the JSON string
            .to_string()
    }
}
