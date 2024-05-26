use serde::{Deserialize, Serialize};

/// MFA Type
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct MfaType {
    /// Secret token used for TOTP factor.
    pub secret: String,
    /// URI for authenticator apps.
    pub uri: String,
}
