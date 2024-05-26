use serde::{Deserialize, Serialize};

/// Locale
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Locale {
    /// User IP address.
    pub ip: String,

    /// Country code in [ISO 3166-1](http://en.wikipedia.org/wiki/ISO_3166-1) two-character format
    #[serde(rename = "countryCode")]
    pub country_code: String,

    /// Country name. This field support localization.
    pub country: String,

    /// Continent code. A two character continent code &quot,AF&quot, for Africa, &quot,AN&quot, for Antarctica, &quot,AS&quot, for Asia, &quot,EU&quot, for Europe, &quot,NA&quot, for North America, &quot,OC&quot, for Oceania, and &quot,SA&quot, for South America.
    #[serde(rename = "continentCode")]
    pub continent_code: String,

    /// Continent name. This field support localization.
    pub continent: String,

    /// True if country is part of the European Union.
    pub eu: bool,

    /// Currency code in [ISO 4217-1](http://en.wikipedia.org/wiki/ISO_4217) three-character format
    pub currency: String,
}
