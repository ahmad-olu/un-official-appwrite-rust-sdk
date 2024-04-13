use serde::{Deserialize, Serialize};

use super::preferences::Preferences;

/// Team
#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    /// Team ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Team creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// Team update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// Team name.
    pub name: String,

    /// Total number of team members.
    pub total: u64,

    /// Team preferences as a key-value object
    #[serde(rename = "prefs")]
    pub preferences: Preferences,
}
