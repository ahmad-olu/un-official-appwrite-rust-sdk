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
