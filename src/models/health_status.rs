use serde::{Deserialize, Serialize};

/// Health Status
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Name of the service.
    pub name: String,
    /// Duration in milliseconds how long the health check took.
    pub ping: u64,
    /// Service status. Possible values can are: `pass`, `fail`
    pub status: String,
}
