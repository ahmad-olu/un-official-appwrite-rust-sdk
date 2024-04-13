use serde::{Deserialize, Serialize};

/// MFA Factors
#[derive(Debug, Serialize, Deserialize)]
pub struct MfaFactors {
    /// TOTP
    pub totp: bool,
    /// Phone
    pub phone: bool,
    /// Email
    pub email: bool,
}
