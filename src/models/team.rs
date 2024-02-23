use serde::{Deserialize, Serialize};

use super::preferences::Preferences;

/// Team
#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    /// Team ID.
    #[serde(rename = "$id")]
    id: String,

    /// Team creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    created_at: String,

    /// Team update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    updated_at: String,

    /// Team name.
    name: String,

    /// Total number of team members.
    total: u64,

    /// Team preferences as a key-value object
    #[serde(rename = "prefs")]
    preferences: Preferences,
}
