use serde::{Deserialize, Serialize};

/// AlgoScrypt
#[derive(Debug, Serialize, Deserialize)]
pub struct AlgoScrypt {
    /// algo type.
    #[serde(rename = "type")]
    pub algo_type: String,

    /// CPU complexity of computed hash.
    #[serde(rename = "costCpu")]
    pub cost_cpu: u64,

    /// Memory complexity of computed hash.
    #[serde(rename = "costMemory")]
    pub cost_memory: u64,

    /// Parallelization of computed hash.
    #[serde(rename = "costParallel")]
    pub cost_parallel: u64,

    /// Length used to compute hash.
    pub length: u64,
}
