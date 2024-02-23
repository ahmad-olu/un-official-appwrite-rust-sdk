use serde::{Deserialize, Serialize};

/// AlgoMD5
#[derive(Debug, Serialize, Deserialize)]
pub struct AlgoMd5 {
    /// algo type.
    #[serde(rename = "type")]
    algo_type: String,
}
