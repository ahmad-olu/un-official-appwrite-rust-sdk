use serde::{Deserialize, Serialize};

/// Health Antivirus
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct HealthAntivirus {
    /// Antivirus version.
    pub version: String,
    /// Antivirus status. Possible values can are: `disabled`, `offline`, `online`
    pub status: String,
}
