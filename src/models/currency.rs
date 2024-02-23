use serde::{Deserialize, Serialize};

/// Currency
#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    /// Currency symbol.
    symbol: String,

    /// Currency name.
    name: String,

    /// Currency native symbol.
    #[serde(rename = "symbolNative")]
    symbol_native: String,

    /// Number of decimal digits.
    #[serde(rename = "decimalDigits")]
    decimal_digits: u64,

    /// Currency digit rounding.
    rounding: f64,

    /// Currency code in [ISO 4217-1](http://en.wikipedia.org/wiki/ISO_4217) three-character format.
    code: String,

    /// Currency plural name
    #[serde(rename = "namePlural")]
    name_plural: String,
}
