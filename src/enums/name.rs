use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum Name {
    #[default]
    #[serde(rename = "v1-database")]
    V1Database,
    #[serde(rename = "v1-deletes")]
    V1Deletes,
    #[serde(rename = "v1-audits")]
    V1Audits,
    #[serde(rename = "v1-mails")]
    V1Mails,
    #[serde(rename = "v1-functions")]
    V1Functions,
    #[serde(rename = "v1-usage")]
    V1Usage,
    #[serde(rename = "v1-usage-dump")]
    V1UsageDump,
    #[serde(rename = "webhooksv1")]
    Webhooksv1,
    #[serde(rename = "v1-certificates")]
    V1Certificates,
    #[serde(rename = "v1-builds")]
    V1Builds,
    #[serde(rename = "v1-messaging")]
    V1Messaging,
    #[serde(rename = "v1-migrations")]
    V1Migrations,
    #[serde(rename = "hamsterv1")]
    Hamsterv1,
}

impl Name {
    /// Converts the enum to its serialized string representation
    pub fn as_serialized(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| panic!("Failed to serialize Name: {:?}", self))
            .trim_matches('"') // Remove surrounding quotes from the JSON string
            .to_string()
    }
}
