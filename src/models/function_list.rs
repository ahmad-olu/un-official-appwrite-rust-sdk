use serde::{Deserialize, Serialize};

use super::function::Func;

/// Functions List
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionList {
    /// Total number of functions documents that matched your query.
    total: u64,
    /// List of functions.
    functions: Vec<Func>,
}
