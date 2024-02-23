use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Membership
#[derive(Debug, Serialize, Deserialize)]
pub struct Membership {
    /// Membership ID.
    #[serde(rename = "$id")]
    id: String,

    /// Membership creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    created_at: String,

    /// Membership update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    updated_at: String,

    /// User ID.
    #[serde(rename = "userId")]
    user_id: String,

    /// User name.
    #[serde(rename = "userName")]
    user_name: String,

    /// User email address.
    #[serde(rename = "userEmail")]
    user_email: String,

    /// Team ID.
    #[serde(rename = "teamId")]
    team_id: String,

    /// Team name.
    #[serde(rename = "teamName")]
    team_name: String,

    /// Date, the user has been invited to join the team in ISO 8601 format.
    invited: String,

    /// Date, the user has accepted the invitation to join the team in ISO 8601 format.
    joined: String,

    /// User confirmation status, true if the user has joined the team or false otherwise.
    confirm: bool,

    /// User list of roles
    roles: Vec<String>,
}
