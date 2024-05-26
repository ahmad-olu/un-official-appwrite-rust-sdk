use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum AuthenticationType {
    #[serde(rename = "totp")]
    #[default]
    TOTP,
}
