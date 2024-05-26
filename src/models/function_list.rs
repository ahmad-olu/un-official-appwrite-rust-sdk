use serde::{Deserialize, Serialize};

use super::function::Func;

/// Functions List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct FunctionList {
    /// Total number of functions documents that matched your query.
    pub total: u64,
    /// List of functions.
    pub functions: Vec<Func>,
}
