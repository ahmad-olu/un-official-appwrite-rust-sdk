use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Runtime
#[derive(Debug, Serialize, Deserialize)]
pub struct Runtime {
    /// Runtime ID.
    #[serde(rename = "$id")]
    id: String,

    /// Runtime Name.
    name: String,

    /// Runtime version.
    version: String,

    /// Base Docker image used to build the runtime.
    base: String,

    /// Image name of Docker Hub.
    image: String,

    /// Name of the logo image.
    logo: String,

    /// List of supported architectures.
    supports: Value,
}
