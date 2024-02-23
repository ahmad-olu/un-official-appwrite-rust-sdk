use serde::{Deserialize, Serialize};

/// JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct JWT {
    /// JWT encoded string.
    jwt: String,
}
