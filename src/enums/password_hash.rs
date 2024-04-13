use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum PasswordHash {
    #[serde(rename = "sha1")]
    Sha1,
    #[serde(rename = "sha224")]
    Sha224,
    #[serde(rename = "sha256")]
    Sha256,
    #[serde(rename = "sha384")]
    Sha384,
    #[serde(rename = "sha512/224")]
    Sha512224,
    #[serde(rename = "sha512/256")]
    Sha512256,
    #[serde(rename = "sha512")]
    Sha512,
    #[serde(rename = "sha3-224")]
    Sha3224,
    #[serde(rename = "sha3-256")]
    Sha3256,
    #[serde(rename = "sha3-384")]
    Sha3384,
    #[serde(rename = "sha3-512")]
    Sha3512,
}
