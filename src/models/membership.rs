use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Membership
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Membership {
    /// Membership ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Membership creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// Membership update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// User ID.
    #[serde(rename = "userId")]
    pub user_id: String,

    /// User name.
    #[serde(rename = "userName")]
    pub user_name: String,

    /// User email address.
    #[serde(rename = "userEmail")]
    pub user_email: String,

    /// Team ID.
    #[serde(rename = "teamId")]
    pub team_id: String,

    /// Team name.
    #[serde(rename = "teamName")]
    pub team_name: String,

    /// Date, the user has been invited to join the team in ISO 8601 format.
    pub invited: String,

    /// Date, the user has accepted the invitation to join the team in ISO 8601 format.
    pub joined: String,

    /// User confirmation status, true if the user has joined the team or false otherwise.
    pub confirm: bool,

    /// Multi factor authentication status, true if the user has MFA enabled or false otherwise.
    pub mfa: bool,

    /// User list of roles
    pub roles: Vec<Value>,
}
