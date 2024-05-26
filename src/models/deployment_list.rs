use serde::{Deserialize, Serialize};

use super::deployment::Deployment;

/// Deployment List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct DeploymentList {
    /// Total number of deployments documents that matched your query.
    pub total: u64,
    /// List of deployments.
    pub deployments: Vec<Deployment>,
}
