use serde::{Deserialize, Serialize};

/// Specification
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Specification {
    /// Memory size in MB.
    pub memory: u64,

    /// Number of CPUs.
    pub cpus: f64,

    /// Is size enabled.
    pub enabled: bool,

    /// Size slug.
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct SpecificationList {
    /// Total number of specification documents that matched your query.
    pub total: u64,
    /// List of specification.
    pub sessions: Vec<Specification>,
}
