//! # Graphql
//!
//! The GraphQL API allows you to query and mutate your Appwrite server using
//! GraphQL.

use std::collections::HashMap;

use serde_json::Value;

use crate::{client::Client, enumm::HttpMethod, error::Error};

pub struct Graphql;

impl Graphql {
    /// GraphQL endpoint
    ///
    /// Execute a GraphQL mutation.
    ///* query => HashMap<String, Value>
    pub async fn query(client: &Client, args: HashMap<String, Value>) -> Result<Value, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/graphql";

        let mut api_headers = reqwest::header::HeaderMap::new();
        api_headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);
        api_headers.insert("x-sdk-graphql", "true".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// GraphQL endpoint
    ///
    /// Execute a GraphQL mutation.
    ///* query => HashMap<String, Value>
    pub async fn mutation(client: &Client, args: HashMap<String, Value>) -> Result<Value, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/graphql/mutation";

        let mut api_headers = reqwest::header::HeaderMap::new();
        api_headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);
        api_headers.insert("x-sdk-graphql", "true".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }
}
