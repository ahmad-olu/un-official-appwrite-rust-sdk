use serde::{Deserialize, Serialize};

use super::target::Target;

/// Target List
#[derive(Debug, Serialize, Deserialize)]
pub struct TargetList {
    /// Total number of targets documents that matched your query.
    total: usize,
    /// List of targets.
    targets: Vec<Target>,
}
