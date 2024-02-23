use serde::{Deserialize, Serialize};

/// Health Status
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Name of the service.
    name: String,
    /// Duration in milliseconds how long the health check took.
    ping: u64,
    /// Service status. Possible values can are: `pass`, `fail`
    status: String,
}
