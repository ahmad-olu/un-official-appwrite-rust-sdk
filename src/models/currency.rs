use serde::{Deserialize, Serialize};

/// Currency
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Currency {
    /// Currency symbol.
    pub symbol: String,

    /// Currency name.
    pub name: String,

    /// Currency native symbol.
    #[serde(rename = "symbolNative")]
    pub symbol_native: String,

    /// Number of decimal digits.
    #[serde(rename = "decimalDigits")]
    pub decimal_digits: u64,

    /// Currency digit rounding.
    pub rounding: f64,

    /// Currency code in [ISO 4217-1](http://en.wikipedia.org/wiki/ISO_4217) three-character format.
    pub code: String,

    /// Currency plural name
    #[serde(rename = "namePlural")]
    pub name_plural: String,
}
