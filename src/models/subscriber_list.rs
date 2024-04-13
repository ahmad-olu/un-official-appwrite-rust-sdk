use serde::{Deserialize, Serialize};

use super::subscriber::Subscriber;

/// Subscriber List
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberList {
    /// Total number of subscribers documents that matched your query.
    pub total: usize,
    /// List of subscribers.
    pub subscribers: Vec<Subscriber>,
}
