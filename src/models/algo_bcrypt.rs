use serde::{Deserialize, Serialize};

/// AlgoBcrypt
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct AlgoBcrypt {
    /// algo type.
    #[serde(rename = "type")]
    pub algo_type: String,
}
