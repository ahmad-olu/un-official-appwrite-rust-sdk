use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CreditCard {
    #[serde(rename = "amex")]
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
