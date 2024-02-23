use serde::{Deserialize, Serialize};

use super::variable::Variable;

/// Variables List
#[derive(Debug, Serialize, Deserialize)]
pub struct VariableList {
    /// Total number of variables documents that matched your query.
    total: u64,
    /// List of variables.
    variables: Vec<Variable>,
}
