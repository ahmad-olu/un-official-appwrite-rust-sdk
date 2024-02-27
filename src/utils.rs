use crate::client::Client;

pub fn get_content_header_value<'a>(client: &'a Client, value: &'a str) -> Option<&'a str> {
    client.header.get(value).and_then(|g| g.to_str().ok())
}
