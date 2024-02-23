use serde::{Deserialize, Serialize};

/// Deployment
#[derive(Debug, Serialize, Deserialize)]
pub struct Deployment {
    /// Deployment ID.
    #[serde(rename = "$id")]
    id: String,

    /// Deployment creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    created_at: String,

    /// Deployment update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    updated_at: String,

    /// Type of deployment.
    #[serde(rename = "type")]
    deployment_type: String,

    /// Resource ID.
    #[serde(rename = "resourceId")]
    resource_id: String,

    /// Resource type.
    #[serde(rename = "resourceType")]
    resource_type: String,

    /// The entrypoint file to use to execute the deployment code.
    entrypoint: String,

    /// The code size in bytes.
    size: u64,

    /// The current build ID.
    #[serde(rename = "buildId")]
    build_id: String,

    /// Whether the deployment should be automatically activated.
    activate: bool,

    /// The deployment status. Possible values are &quot;processing&quot;, &quot;building&quot;, &quot;waiting&quot;, &quot;ready&quot;, and &quot;failed&quot;.
    status: String,

    /// The build logs.
    #[serde(rename = "buildLogs")]
    build_logs: String,

    /// The current build time in seconds.
    #[serde(rename = "buildTime")]
    build_time: u64,

    /// The name of the vcs provider repository
    #[serde(rename = "providerRepositoryName")]
    provider_repository_name: String,

    /// The name of the vcs provider repository owner
    #[serde(rename = "providerRepositoryOwner")]
    provider_repository_owner: String,

    /// The url of the vcs provider repository
    #[serde(rename = "providerRepositoryUrl")]
    provider_repository_url: String,

    /// The branch of the vcs repository
    #[serde(rename = "providerBranch")]
    provider_branch: String,

    /// The commit hash of the vcs commit
    #[serde(rename = "providerCommitHash")]
    provider_commit_hash: String,

    /// The url of vcs commit author
    #[serde(rename = "providerCommitAuthorUrl")]
    provider_commit_author_url: String,

    /// The name of vcs commit author
    #[serde(rename = "providerCommitAuthor")]
    provider_commit_author: String,

    /// The commit message
    #[serde(rename = "providerCommitMessage")]
    provider_commit_message: String,

    /// The url of the vcs commit
    #[serde(rename = "providerCommitUrl")]
    provider_commit_url: String,

    /// The branch of the vcs repository
    #[serde(rename = "providerBranchUrl")]
    provider_branch_url: String,
}
