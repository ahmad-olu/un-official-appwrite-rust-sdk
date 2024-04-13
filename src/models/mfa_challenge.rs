use serde::{Deserialize, Serialize};

/// MFA Challenge
#[derive(Debug, Serialize, Deserialize)]
pub struct MfaChallenge {
    /// Token ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Token creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// User ID.
    #[serde(rename = "userId")]
    pub user_id: String,

    /// Token expiration date in ISO 8601 format.
    pub expire: String,
}
