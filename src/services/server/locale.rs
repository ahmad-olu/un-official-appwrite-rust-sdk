// ///!Locale
// ///!
// ///! The Locale service allows you to customize your app based on your users&#039;
// ///! location.
// use crate::{
//     api_params, app_json_header,
//     client::Client,
//     enumm::HttpMethod,
//     error::Error,
//     models::{
//         continent_list::ContinentList, country_list::CountryList, currency_list::CurrencyList,
//         language_list::LanguageList, locale::Locale as MLocale, locale_code::LocaleCode,
//         phone_list::PhoneList,
//     },
// };

// pub struct Locale;

// impl Locale {
//     /// Get user locale
//     ///
//     /// Get the current user location based on IP. Returns an object with user
//     /// country code, country name, continent name, continent code, ip address and
//     /// suggested currency. You can use the locale header to get the data in a
//     /// supported language.
//     ///
//     /// ([IP Geolocation by DB-IP](https://db-ip.com))
//     pub async fn get(client: &Client) -> Result<MLocale, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/locale";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// List Locale Codes
//     ///
//     /// List of all locale codes in [ISO
//     /// 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).
//     pub async fn list_codes(client: &Client) -> Result<LocaleCode, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/locale/codes";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// List continents
//     ///
//     /// List of all continents. You can use the locale header to get the data in a
//     /// supported language.
//     pub async fn list_continents(client: &Client) -> Result<ContinentList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/locale/continents";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// List countries
//     ///
//     /// List of all countries. You can use the locale header to get the data in a
//     /// supported language.
//     pub async fn list_countries(client: &Client) -> Result<CountryList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/locale/countries";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// List EU countries
//     ///
//     /// List of all countries that are currently members of the EU. You can use the
//     /// locale header to get the data in a supported language.
//     pub async fn list_countries_eu(client: &Client) -> Result<CountryList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/locale/countries/eu";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// List countries phone codes
//     ///
//     /// List of all countries phone codes. You can use the locale header to get the
//     /// data in a supported language.
//     pub async fn list_countries_phones(client: &Client) -> Result<PhoneList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/locale/countries/phones";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// List currencies
//     ///
//     /// List of all currencies, including currency symbol, name, plural, and
//     /// decimal digits for all major and minor currencies. You can use the locale
//     /// header to get the data in a supported language.
//     pub async fn list_currencies(client: &Client) -> Result<CurrencyList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/locale/currencies";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// List languages
//     ///
//     /// List of all languages classified by ISO 639-1 including 2-letter code, name
//     /// in English, and name in the respective language.
//     pub async fn list_languages(client: &Client) -> Result<LanguageList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/locale/languages";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }
// }
