use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum MessagingProviderType {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "push")]
    Push,
}
