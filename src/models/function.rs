use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::variable::Variable;

/// Func
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Func {
    /// Function ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Function creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// Function update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// Execution permissions.
    pub execute: Value,

    /// Function name.
    pub name: String,

    /// Function enabled.
    pub enabled: bool,

    /// Is the function deployed with the latest configuration? This is set to false if you&#039;ve changed an environment variables, entrypoint, commands, or other settings that needs redeploy to be applied. When the value is false, redeploy the function to update it with the latest configuration.
    pub live: bool,

    /// Whether executions will be logged. When set to false, executions will not be logged, but will reduce resource used by your Appwrite project.
    pub logging: bool,

    /// Function execution runtime.
    pub runtime: String,

    /// Function&#039;s active deployment ID.
    pub deployment: String,

    /// Function variables.
    pub vars: Vec<Variable>,

    /// Function trigger events.
    pub events: Value,

    /// Function execution schedult in CRON format.
    pub schedule: String,

    /// Function execution timeout in seconds.
    pub timeout: u64,

    /// The entrypoint file used to execute the deployment.
    pub entrypoint: String,

    /// The build command used to build the deployment.
    pub commands: String,

    /// Version of Open Runtimes used for the function.
    pub version: String,

    /// Function VCS (Version Control System) installation id.
    #[serde(rename = "installationId")]
    pub installation_id: String,

    /// VCS (Version Control System) Repository ID
    #[serde(rename = "providerRepositoryId")]
    pub provider_repository_id: String,

    /// VCS (Version Control System) branch name
    #[serde(rename = "providerBranch")]
    pub provider_branch: String,

    /// Path to function in VCS (Version Control System) repository
    #[serde(rename = "providerRootDirectory")]
    pub provider_root_directory: String,

    /// Is VCS (Version Control System) connection is in silent mode? When in silence mode, no comments will be posted on the repository pull or merge requests
    #[serde(rename = "providerSilentMode")]
    pub provider_silent_mode: bool,
}
