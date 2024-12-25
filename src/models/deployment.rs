use serde::{Deserialize, Serialize};

/// Deployment
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Deployment {
    /// Deployment ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Deployment creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// Deployment update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// Type of deployment.
    #[serde(rename = "type")]
    pub deployment_type: String,

    /// Resource ID.
    #[serde(rename = "resourceId")]
    pub resource_id: String,

    /// Resource type.
    #[serde(rename = "resourceType")]
    pub resource_type: String,

    /// The entrypoint file to use to execute the deployment code.
    pub entrypoint: String,

    /// The code size in bytes.
    pub size: u64,

    /// The current build ID.
    #[serde(rename = "buildSize")]
    pub build_size: u64,

    #[serde(rename = "buildId")]
    pub build_id: String,

    /// Whether the deployment should be automatically activated.
    pub activate: bool,

    /// The deployment status. Possible values are &quot;processing&quot;, &quot;building&quot;, &quot;waiting&quot;, &quot;ready&quot;, and &quot;failed&quot;.
    pub status: String,

    /// The build logs.
    #[serde(rename = "buildLogs")]
    pub build_logs: String,

    /// The current build time in seconds.
    #[serde(rename = "buildTime")]
    pub build_time: u64,

    /// The name of the vcs provider repository
    #[serde(rename = "providerRepositoryName")]
    pub provider_repository_name: String,

    /// The name of the vcs provider repository owner
    #[serde(rename = "providerRepositoryOwner")]
    pub provider_repository_owner: String,

    /// The url of the vcs provider repository
    #[serde(rename = "providerRepositoryUrl")]
    pub provider_repository_url: String,

    /// The branch of the vcs repository
    #[serde(rename = "providerBranch")]
    pub provider_branch: String,

    /// The commit hash of the vcs commit
    #[serde(rename = "providerCommitHash")]
    pub provider_commit_hash: String,

    /// The url of vcs commit author
    #[serde(rename = "providerCommitAuthorUrl")]
    pub provider_commit_author_url: String,

    /// The name of vcs commit author
    #[serde(rename = "providerCommitAuthor")]
    pub provider_commit_author: String,

    /// The commit message
    #[serde(rename = "providerCommitMessage")]
    pub provider_commit_message: String,

    /// The url of the vcs commit
    #[serde(rename = "providerCommitUrl")]
    pub provider_commit_url: String,

    /// The branch of the vcs repository
    #[serde(rename = "providerBranchUrl")]
    pub provider_branch_url: String,
}
