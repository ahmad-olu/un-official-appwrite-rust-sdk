use serde::{Deserialize, Serialize};

/// AlgoPHPASS
#[derive(Debug, Serialize, Deserialize)]
pub struct AlgoPhpass {
    /// algo type.
    #[serde(rename = "type")]
    algo_type: String,
}
