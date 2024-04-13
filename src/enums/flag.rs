use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub enum Flag {
    #[serde(rename = "af")]
    Afghanistan,
    #[serde(rename = "ao")]
    Angola,
    #[serde(rename = "al")]
    Albania,
    #[serde(rename = "ad")]
    Andorra,
    #[serde(rename = "ae")]
    UnitedArabEmirates,
    #[serde(rename = "ar")]
    Argentina,
    #[serde(rename = "am")]
    Armenia,
    #[serde(rename = "ag")]
    AntiguaAndBarbuda,
    #[serde(rename = "au")]
    Australia,
    #[serde(rename = "at")]
    Austria,
    #[serde(rename = "az")]
    Azerbaijan,
    #[serde(rename = "bi")]
    Burundi,
    #[serde(rename = "be")]
    Belgium,
    #[serde(rename = "bj")]
    Benin,
    #[serde(rename = "bf")]
    BurkinaFaso,
    #[serde(rename = "bd")]
    Bangladesh,
    #[serde(rename = "bg")]
    Bulgaria,
    #[serde(rename = "bh")]
    Bahrain,
    #[serde(rename = "bs")]
    Bahamas,
    #[serde(rename = "ba")]
    BosniaAndHerzegovina,
    #[serde(rename = "by")]
    Belarus,
    #[serde(rename = "bz")]
    Belize,
    #[serde(rename = "bo")]
    Bolivia,
    #[serde(rename = "br")]
    Brazil,
    #[serde(rename = "bb")]
    Barbados,
    #[serde(rename = "bn")]
    BruneiDarussalam,
    #[serde(rename = "bt")]
    Bhutan,
    #[serde(rename = "bw")]
    Botswana,
    #[serde(rename = "cf")]
    CentralAfricanRepublic,
    #[serde(rename = "ca")]
    Canada,
    #[serde(rename = "ch")]
    Switzerland,
    #[serde(rename = "cl")]
    Chile,
    #[serde(rename = "cn")]
    China,
    #[serde(rename = "ci")]
    CoteDIvoire,
    #[serde(rename = "cm")]
    Cameroon,
    #[serde(rename = "cd")]
    DemocraticRepublicOfTheCongo,
    #[serde(rename = "cg")]
    RepublicOfTheCongo,
    #[serde(rename = "co")]
    Colombia,
    #[serde(rename = "km")]
    Comoros,
    #[serde(rename = "cv")]
    CapeVerde,
    #[serde(rename = "cr")]
    CostaRica,
    #[serde(rename = "cu")]
    Cuba,
    #[serde(rename = "cy")]
    Cyprus,
    #[serde(rename = "cz")]
    CzechRepublic,
    #[serde(rename = "de")]
    Germany,
    #[serde(rename = "dj")]
    Djibouti,
    #[serde(rename = "dm")]
    Dominica,
    #[serde(rename = "dk")]
    Denmark,
    #[serde(rename = "do")]
    DominicanRepublic,
    #[serde(rename = "dz")]
    Algeria,
    #[serde(rename = "ec")]
    Ecuador,
    #[serde(rename = "eg")]
    Egypt,
    #[serde(rename = "er")]
    Eritrea,
    #[serde(rename = "es")]
    Spain,
    #[serde(rename = "ee")]
    Estonia,
    #[serde(rename = "et")]
    Ethiopia,
    #[serde(rename = "fi")]
    Finland,
    #[serde(rename = "fj")]
    Fiji,
    #[serde(rename = "fr")]
    France,
    #[serde(rename = "fm")]
    MicronesiaFederatedStatesOf,
    #[serde(rename = "ga")]
    Gabon,
    #[serde(rename = "gb")]
    UnitedKingdom,
    #[serde(rename = "ge")]
    Georgia,
    #[serde(rename = "gh")]
    Ghana,
    #[serde(rename = "gn")]
    Guinea,
    #[serde(rename = "gm")]
    Gambia,
    #[serde(rename = "gw")]
    GuineaBissau,
    #[serde(rename = "gq")]
    EquatorialGuinea,
    #[serde(rename = "gr")]
    Greece,
    #[serde(rename = "gd")]
    Grenada,
    #[serde(rename = "gt")]
    Guatemala,
    #[serde(rename = "gy")]
    Guyana,
    #[serde(rename = "hn")]
    Honduras,
    #[serde(rename = "hr")]
    Croatia,
    #[serde(rename = "ht")]
    Haiti,
    #[serde(rename = "hu")]
    Hungary,
    #[serde(rename = "id")]
    Indonesia,
    #[serde(rename = "in")]
    India,
    #[serde(rename = "ie")]
    Ireland,
    #[serde(rename = "ir")]
    IranIslamicRepublicOf,
    #[serde(rename = "iq")]
    Iraq,
    #[serde(rename = "is")]
    Iceland,
    #[serde(rename = "il")]
    Israel,
    #[serde(rename = "it")]
    Italy,
    #[serde(rename = "jm")]
    Jamaica,
    #[serde(rename = "jo")]
    Jordan,
    #[serde(rename = "jp")]
    Japan,
    #[serde(rename = "kz")]
    Kazakhstan,
    #[serde(rename = "ke")]
    Kenya,
    #[serde(rename = "kg")]
    Kyrgyzstan,
    #[serde(rename = "kh")]
    Cambodia,
    #[serde(rename = "ki")]
    Kiribati,
    #[serde(rename = "kn")]
    SaintKittsAndNevis,
    #[serde(rename = "kr")]
    SouthKorea,
    #[serde(rename = "kw")]
    Kuwait,
    #[serde(rename = "la")]
    LaoPeopleSDemocraticRepublic,
    #[serde(rename = "lb")]
    Lebanon,
    #[serde(rename = "lr")]
    Liberia,
    #[serde(rename = "ly")]
    Libya,
    #[serde(rename = "lc")]
    SaintLucia,
    #[serde(rename = "li")]
    Liechtenstein,
    #[serde(rename = "lk")]
    SriLanka,
    #[serde(rename = "ls")]
    Lesotho,
    #[serde(rename = "lt")]
    Lithuania,
    #[serde(rename = "lu")]
    Luxembourg,
    #[serde(rename = "lv")]
    Latvia,
    #[serde(rename = "ma")]
    Morocco,
    #[serde(rename = "mc")]
    Monaco,
    #[serde(rename = "md")]
    Moldova,
    #[serde(rename = "mg")]
    Madagascar,
    #[serde(rename = "mv")]
    Maldives,
    #[serde(rename = "mx")]
    Mexico,
    #[serde(rename = "mh")]
    MarshallIslands,
    #[serde(rename = "mk")]
    NorthMacedonia,
    #[serde(rename = "ml")]
    Mali,
    #[serde(rename = "mt")]
    Malta,
    #[serde(rename = "mm")]
    Myanmar,
    #[serde(rename = "me")]
    Montenegro,
    #[serde(rename = "mn")]
    Mongolia,
    #[serde(rename = "mz")]
    Mozambique,
    #[serde(rename = "mr")]
    Mauritania,
    #[serde(rename = "mu")]
    Mauritius,
    #[serde(rename = "mw")]
    Malawi,
    #[serde(rename = "my")]
    Malaysia,
    #[serde(rename = "na")]
    Namibia,
    #[serde(rename = "ne")]
    Niger,
    #[serde(rename = "ng")]
    Nigeria,
    #[serde(rename = "ni")]
    Nicaragua,
    #[serde(rename = "nl")]
    Netherlands,
    #[serde(rename = "no")]
    Norway,
    #[serde(rename = "np")]
    Nepal,
    #[serde(rename = "nr")]
    Nauru,
    #[serde(rename = "nz")]
    NewZealand,
    #[serde(rename = "om")]
    Oman,
    #[serde(rename = "pk")]
    Pakistan,
    #[serde(rename = "pa")]
    Panama,
    #[serde(rename = "pe")]
    Peru,
    #[serde(rename = "ph")]
    Philippines,
    #[serde(rename = "pw")]
    Palau,
    #[serde(rename = "pg")]
    PapuaNewGuinea,
    #[serde(rename = "pl")]
    Poland,
    #[serde(rename = "kp")]
    NorthKorea,
    #[serde(rename = "pt")]
    Portugal,
    #[serde(rename = "py")]
    Paraguay,
    #[serde(rename = "qa")]
    Qatar,
    #[serde(rename = "ro")]
    Romania,
    #[serde(rename = "ru")]
    Russia,
    #[serde(rename = "rw")]
    Rwanda,
    #[serde(rename = "sa")]
    SaudiArabia,
    #[serde(rename = "sd")]
    Sudan,
    #[serde(rename = "sn")]
    Senegal,
    #[serde(rename = "sg")]
    Singapore,
    #[serde(rename = "sb")]
    SolomonIslands,
    #[serde(rename = "sl")]
    SierraLeone,
    #[serde(rename = "sv")]
    ElSalvador,
    #[serde(rename = "sm")]
    SanMarino,
    #[serde(rename = "so")]
    Somalia,
    #[serde(rename = "rs")]
    Serbia,
    #[serde(rename = "ss")]
    SouthSudan,
    #[serde(rename = "st")]
    SaoTomeAndPrincipe,
    #[serde(rename = "sr")]
    Suriname,
    #[serde(rename = "sk")]
    Slovakia,
    #[serde(rename = "si")]
    Slovenia,
    #[serde(rename = "se")]
    Sweden,
    #[serde(rename = "sz")]
    Eswatini,
    #[serde(rename = "sc")]
    Seychelles,
    #[serde(rename = "sy")]
    Syria,
    #[serde(rename = "td")]
    Chad,
    #[serde(rename = "tg")]
    Togo,
    #[serde(rename = "th")]
    Thailand,
    #[serde(rename = "tj")]
    Tajikistan,
    #[serde(rename = "tm")]
    Turkmenistan,
    #[serde(rename = "tl")]
    TimorLeste,
    #[serde(rename = "to")]
    Tonga,
    #[serde(rename = "tt")]
    TrinidadAndTobago,
    #[serde(rename = "tn")]
    Tunisia,
    #[serde(rename = "tr")]
    Turkey,
    #[serde(rename = "tv")]
    Tuvalu,
    #[serde(rename = "tz")]
    Tanzania,
    #[serde(rename = "ug")]
    Uganda,
    #[serde(rename = "ua")]
    Ukraine,
    #[serde(rename = "uy")]
    Uruguay,
    #[serde(rename = "us")]
    UnitedStates,
    #[serde(rename = "uz")]
    Uzbekistan,
    #[serde(rename = "va")]
    VaticanCity,
    #[serde(rename = "vc")]
    SaintVincentAndTheGrenadines,
    #[serde(rename = "ve")]
    Venezuela,
    #[serde(rename = "vn")]
    Vietnam,
    #[serde(rename = "vu")]
    Vanuatu,
    #[serde(rename = "ws")]
    Samoa,
    #[serde(rename = "ye")]
    Yemen,
    #[serde(rename = "za")]
    SouthAfrica,
    #[serde(rename = "zm")]
    Zambia,
    #[serde(rename = "zw")]
    Zimbabwe,
}
