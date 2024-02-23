use serde::{Deserialize, Serialize};

use super::deployment::Deployment;

/// Deployment List
#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentList {
    /// Total number of deployments documents that matched your query.
    total: u64,
    /// List of deployments.
    deployments: Vec<Deployment>,
}
