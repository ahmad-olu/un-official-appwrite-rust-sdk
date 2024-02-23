use serde::{Deserialize, Serialize};

/// Token
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    /// Token ID.
    #[serde(rename = "$id")]
    id: String,

    /// Token creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    created_at: String,

    /// User ID.
    #[serde(rename = "userId")]
    user_id: String,

    /// Token secret key. This will return an empty string unless the response is returned using an API key or as part of a webhook payload.
    secret: String,

    /// Token expiration date in ISO 8601 format.
    expire: String,
}
