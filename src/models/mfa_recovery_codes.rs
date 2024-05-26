use serde::{Deserialize, Serialize};
use serde_json::Value;

/// MFA Recovery Codes
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct MfaRecoveryCodes {
    /// Recovery codes.
    #[serde(rename = "recoveryCodes")]
    pub recovery_codes: Vec<Value>,
}
