use serde::{Deserialize, Serialize};

/// Health Antivirus
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthAntivirus {
    /// Antivirus version.
    version: String,
    /// Antivirus status. Possible values can are: `disabled`, `offline`, `online`
    status: String,
}
