use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum AuthenticationFactor {
    #[serde(rename = "email")]
    EMAIL,
    #[serde(rename = "phone")]
    PHONE,
    #[serde(rename = "totp")]
    TOTP,
    #[serde(rename = "recoverycode")]
    RECOVERYCODE,
}
