use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Preferences
#[derive(Debug, Serialize, Deserialize)]
pub struct Preferences {
    #[serde(flatten)]
    data: Map<String, Value>,
}
