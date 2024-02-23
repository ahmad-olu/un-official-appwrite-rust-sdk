use serde::{Deserialize, Serialize};

/// Health Time
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthTime {
    /// Current unix timestamp on trustful remote server.
    #[serde(rename = "remoteTime")]
    remote_time: u64,
    /// Current unix timestamp of local server where Appwrite runs.
    #[serde(rename = "localTime")]
    local_time: u64,
    /// Difference of unix remote and local timestamps in milliseconds.
    diff: u64,
}
