use serde::{Deserialize, Serialize};

/// Headers
#[derive(Debug, Serialize, Deserialize)]
pub struct Headers {
    /// Header name.
    name: String,
    /// Header value.
    value: String,
}
