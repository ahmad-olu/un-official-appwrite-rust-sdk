use crate::client::Client;

extern crate reqwest;
extern crate serde_json;

#[macro_export]
macro_rules! api_params {
    ($( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut map = serde_json::Map::new();
        $(
            if let Some(value) = $value {
                map.insert($key.to_string(), serde_json::json!(value));
            }
        )*
        serde_json::Value::Object(map)
    }

    };
}

#[macro_export]
macro_rules! app_json_header {
    () => {{
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);
        headers
    }};
}

pub fn get_content_header_value<'a>(client: &'a Client, value: &'a str) -> Option<&'a str> {
    client
        .header
        .get(format!("x-appwrite-{value}"))
        .and_then(|g| g.to_str().ok())
}
