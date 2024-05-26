use serde::{Deserialize, Serialize};

/// Health Certificate
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct HealthCertificate {
    /// Certificate name
    pub name: String,

    /// Subject SN
    #[serde(rename = "subjectSN")]
    pub subject_sn: String,

    /// Issuer organisation
    #[serde(rename = "issuerOrganisation")]
    pub issuer_organization: String,

    /// Valid from
    #[serde(rename = "validFrom")]
    pub valid_from: String,

    /// Valid to
    #[serde(rename = "validTo")]
    pub valid_to: String,

    /// Signature type SN
    #[serde(rename = "signatureTypeSN")]
    pub signature_type_sn: String,
}
