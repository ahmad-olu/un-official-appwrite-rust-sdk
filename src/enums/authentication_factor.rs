use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum AuthenticationFactor {
    #[serde(rename = "email")]
    #[default]
    EMAIL,
    #[serde(rename = "phone")]
    PHONE,
    #[serde(rename = "totp")]
    TOTP,
    #[serde(rename = "recoverycode")]
    RECOVERYCODE,
}

impl AuthenticationFactor {
    /// Converts the enum to its serialized string representation
    pub fn as_serialized(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| panic!("Failed to serialize AuthenticationFactor: {:?}", self))
            .trim_matches('"') // Remove surrounding quotes from the JSON string
            .to_string()
    }
}
