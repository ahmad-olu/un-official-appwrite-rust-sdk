use serde::{Deserialize, Serialize};

use super::variable::Variable;

/// Variables List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct VariableList {
    /// Total number of variables documents that matched your query.
    pub total: u64,
    /// List of variables.
    pub variables: Vec<Variable>,
}
