use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum SmtpEncryption {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "ssl")]
    SSL,
    #[serde(rename = "tls")]
    Tls,
}

impl SmtpEncryption {
    /// Converts the enum to its serialized string representation
    pub fn as_serialized(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| panic!("Failed to serialize SmtpEncryption: {:?}", self))
            .trim_matches('"') // Remove surrounding quotes from the JSON string
            .to_string()
    }
}
