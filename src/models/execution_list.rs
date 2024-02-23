use serde::{Deserialize, Serialize};

use super::execution::Execution;

/// Executions List
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionList {
    /// Total number of executions documents that matched your query.
    total: u64,
    /// List of executions.
    executions: Vec<Execution>,
}
