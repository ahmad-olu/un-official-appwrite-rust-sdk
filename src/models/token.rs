use serde::{Deserialize, Serialize};

/// Token
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    /// Token ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Token creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// User ID.
    #[serde(rename = "userId")]
    pub user_id: String,

    /// Token secret key. This will return an empty string unless the response is returned using an API key or as part of a webhook payload.
    pub secret: String,

    /// Token expiration date in ISO 8601 format.
    pub expire: String,

    /// Security phrase of a token. Empty if security phrase was not requested when creating a token. It includes randomly generated phrase which is also sent in the external resource such as email.
    pub phrase: String,
}
