use serde::{Deserialize, Serialize};

/// Health Queue
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthQueue {
    /// Amount of actions in the queue.
    size: u64,
}
