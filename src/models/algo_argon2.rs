use serde::{Deserialize, Serialize};

/// AlgoArgon2
#[derive(Debug, Serialize, Deserialize)]
pub struct AlgoArgon2 {
    /// algo type.
    #[serde(rename = "type")]
    algo_type: String,

    /// Memory used to compute hash.
    #[serde(rename = "memoryCost")]
    memory_cost: u64,

    /// Amount of time consumed to compute hash
    #[serde(rename = "timeCost")]
    time_cost: u64,

    /// Number of threads used to compute hash.
    threads: u64,
}
