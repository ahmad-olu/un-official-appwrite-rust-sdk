use serde::{Deserialize, Serialize};

/// AlgoScrypt
#[derive(Debug, Serialize, Deserialize)]
pub struct AlgoScrypt {
    /// algo type.
    #[serde(rename = "type")]
    algo_type: String,

    /// CPU complexity of computed hash.
    #[serde(rename = "costCpu")]
    cost_cpu: u64,

    /// Memory complexity of computed hash.
    #[serde(rename = "costMemory")]
    cost_memory: u64,

    /// Parallelization of computed hash.
    #[serde(rename = "costParallel")]
    cost_parallel: u64,

    /// Length used to compute hash.
    length: u64,
}
