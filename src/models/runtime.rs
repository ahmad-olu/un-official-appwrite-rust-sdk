use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Runtime
#[derive(Debug, Serialize, Deserialize)]
pub struct Runtime {
    /// Runtime ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Runtime Name.
    pub name: String,

    /// Runtime version.
    pub version: String,

    /// Base Docker image used to build the runtime.
    pub base: String,

    /// Image name of Docker Hub.
    pub image: String,

    /// Name of the logo image.
    pub logo: String,

    /// List of supported architectures.
    pub supports: Vec<Value>,
}
