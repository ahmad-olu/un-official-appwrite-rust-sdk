use serde::{Deserialize, Serialize};

/// AlgoArgon2
#[derive(Debug, Serialize, Deserialize)]
pub struct AlgoArgon2 {
    /// algo type.
    #[serde(rename = "type")]
    pub algo_type: String,

    /// Memory used to compute hash.
    #[serde(rename = "memoryCost")]
    pub memory_cost: u64,

    /// Amount of time consumed to compute hash
    #[serde(rename = "timeCost")]
    pub time_cost: u64,

    /// Number of threads used to compute hash.
    pub threads: u64,
}
