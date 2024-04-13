use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SmtpEncryption {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "ssl")]
    SSL,
    #[serde(rename = "tls")]
    Tls,
}
