use serde::{Deserialize, Serialize};

/// Health Queue
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct HealthQueue {
    /// Amount of actions in the queue.
    pub size: u64,
}
