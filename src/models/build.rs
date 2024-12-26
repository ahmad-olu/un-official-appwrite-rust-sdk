use serde::{Deserialize, Serialize};

/// Build
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Build {
    /// Build ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// The deployment that created this build.
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,

    /// The build status. There are a few different types and each one means something different. \nFailed - The deployment build has failed. More details can usually be found in buildStderr\nReady - The deployment build was successful and the deployment is ready to be deployed\nProcessing - The deployment is currently waiting to have a build triggered\nBuilding - The deployment is currently being built
    pub status: String,

    /// The stdout of the build.
    pub stdout: String,

    /// The stderr of the build.
    pub stderr: String,

    /// The deployment creation date in ISO 8601 format.
    #[serde(rename = "startTime")]
    pub start_time: String,

    /// The time the build was finished in ISO 8601 format.
    #[serde(rename = "endTime")]
    pub end_time: String,

    /// The build duration in seconds.
    pub duration: u64,

    /// The code size in bytes.
    pub size: u64,
}
