use serde::{Deserialize, Serialize};

use super::execution::Execution;

/// Executions List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct ExecutionList {
    /// Total number of executions documents that matched your query.
    pub total: u64,
    /// List of executions.
    pub executions: Vec<Execution>,
}
