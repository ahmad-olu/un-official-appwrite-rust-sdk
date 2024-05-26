use serde::{Deserialize, Serialize};

use super::message::Message;

/// Message list
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct MessageList {
    /// Total number of messages documents that matched your query.
    pub total: usize,
    /// List of messages.
    pub messages: Vec<Message>,
}
