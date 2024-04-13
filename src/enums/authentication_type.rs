use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthenticationType {
    #[serde(rename = "totp")]
    TOTP,
}
