use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum CreditCard {
    #[serde(rename = "amex")]
    #[default]
    AmericaExpress,
    #[serde(rename = "argencard")]
    ArgenCard,
    #[serde(rename = "cabal")]
    Cabel,
    #[serde(rename = "censosud")]
    Consosud,
    #[serde(rename = "diners")]
    DinersClub,
    #[serde(rename = "discover")]
    Discover,
    #[serde(rename = "elo")]
    Elo,
    #[serde(rename = "hipercard")]
    HiperCard,
    #[serde(rename = "jcb")]
    Jcb,
    #[serde(rename = "mastercard")]
    MasterCard,
    #[serde(rename = "naranja")]
    Naranja,
    #[serde(rename = "targeta-shopping")]
    TarjetaShopping,
    #[serde(rename = "union-china-pay")]
    UnionChinaPay,
    #[serde(rename = "visa")]
    Visa,
    #[serde(rename = "mir")]
    Mir,
    #[serde(rename = "maestro")]
    Maestro,
}

impl CreditCard {
    /// Converts the enum to its serialized string representation
    pub fn as_serialized(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| panic!("Failed to serialize CreditCard: {:?}", self))
            .trim_matches('"') // Remove surrounding quotes from the JSON string
            .to_string()
    }
}
