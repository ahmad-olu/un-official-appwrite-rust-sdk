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
