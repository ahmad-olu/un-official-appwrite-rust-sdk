//! # Databases
//!
//! The Databases service allows you to create structured collections of
//! documents, query and filter lists of documents
use std::collections::HashMap;

use crate::{
    app_json_header,
    client::Client,
    enumm::HttpMethod,
    error::Error,
    models::{
        attribute_boolean::AttributeBoolean, attribute_datetime::AttributeDateTime,
        attribute_email::AttributeEmail, attribute_enum::AttributeEnum,
        attribute_float::AttributeFloat, attribute_integer::AttributeInteger,
        attribute_ip::AttributeIp, attribute_list::AttributeList,
        attribute_relationship::AttributeRelationship, attribute_string::AttributeString,
        attribute_url::AttributeUrl, collection::Collection, collection_list::CollectionList,
        database::Database, database_list::DatabaseList, document::Document,
        document_list::DocumentList, index::Index, index_list::IndexList,
    },
};
use serde_json::Value;

pub struct Databases;

impl Databases {
    /// List databases
    ///
    /// Get a list of all databases from the current Appwrite project. You can use
    /// the search parameter to filter your results.
    ///* queries => vec(string)?
    ///* search => string?
    pub async fn list(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<DatabaseList, Error> {
        const API_PATH: &str = "/databases";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create database
    ///
    /// Create a new Database.
    ///
    ///* databaseId => string
    ///* name => string
    ///* enabled => bool?
    pub async fn create(client: &Client, args: HashMap<String, Value>) -> Result<Database, Error> {
        const API_PATH: &str = "/databases";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, API_PATH, api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Get database
    ///
    /// Get a database by its unique ID. This endpoint response returns a JSON
    /// object with the database metadata.
    pub async fn get(client: &Client, database_id: &str) -> Result<Database, Error> {
        let api_path = "/databases/{databaseId}".replace("{databaseId}", database_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update database
    ///
    /// Update a database by its unique ID.
    ///* name => string
    ///* enabled => bool?
    pub async fn update(
        client: &Client,
        database_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Database, Error> {
        let api_path = "/databases/{databaseId}".replace("{databaseId}", database_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PUT, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Delete database
    ///
    /// Delete a database by its unique ID. Only API keys with with databases.write
    /// scope can delete a database.
    pub async fn delete(client: &Client, database_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}".replace("{databaseId}", database_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(())
    }

    /// List collections
    ///
    /// Get a list of all collections that belong to the provided databaseId. You
    /// can use the search parameter to filter your results.
    ///* search => string?
    ///* queries => vec(string)?
    pub async fn list_collections(
        client: &Client,
        database_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<CollectionList, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections".replace("{databaseId}", database_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create collection
    ///
    /// Create a new Collection. Before using this route, you should create a new
    /// database resource using either a [server
    /// integration](https://appwrite.io/docs/server/databases#databasesCreateCollection)
    /// API or directly from your database console.
    ///* collectionId => string
    ///* name => string
    ///* permissions => option(vec)?
    ///* documentSecurity => bool?
    ///* enabled => bool?
    pub async fn create_collection(
        client: &Client,
        database_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Collection, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections".replace("{databaseId}", database_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Get collection
    ///
    /// Get a collection by its unique ID. This endpoint response returns a JSON
    /// object with the collection metadata.
    pub async fn get_collection(
        client: &Client,
        database_id: &str,
        collection_id: &str,
    ) -> Result<Collection, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update collection
    ///
    /// Update a collection by its unique ID.
    ///* name => string
    ///* permissions => vec(string)?
    ///* documentSecurity => bool?
    ///* enabled => bool?
    pub async fn update_collection(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Collection, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PUT, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Delete collection
    ///
    /// Delete a collection by its unique ID. Only users with write permissions
    /// have access to delete this resource.
    pub async fn delete_collection(
        client: &Client,
        database_id: &str,
        collection_id: &str,
    ) -> Result<(), Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(())
    }

    /// List attributes
    ///
    /// List attributes in the collection.
    ///* queries => vec(string)?
    pub async fn list_attributes(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeList, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create boolean attribute
    ///
    /// Create a boolean attribute.
    ///
    ///* key => string
    ///* required => bool
    ///* default => bool?
    ///* array => bool?
    pub async fn create_boolean_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeBoolean, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/boolean"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update boolean attribute
    ///
    ///  Update a boolean attribute. Changing the `default` value will not update
    /// already existing documents.
    ///* required => bool
    ///* default => bool?
    pub async fn update_boolean_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeBoolean, Error> {
        //const API_PATH: &str = "/databases";
        let api_path =
            "/databases/{databaseId}/collections/{collectionId}/attributes/boolean/{key}"
                .replace("{databaseId}", database_id)
                .replace("{collectionId}", collection_id)
                .replace("{key}", key);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create datetime attribute
    ///
    ///  Create a date time attribute according to the ISO 8601 standard.
    ///* key => string
    ///* required => bool
    ///* default => bool?
    ///* array => bool?
    pub async fn create_date_time_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeDateTime, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/datetime"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update dateTime attribute
    ///
    ///  Update a date time attribute. Changing the `default` value will not update
    /// already existing documents.
    ///* required => bool
    ///* default => bool?
    pub async fn update_date_time_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeDateTime, Error> {
        //const API_PATH: &str = "/databases";
        let api_path =
            "/databases/{databaseId}/collections/{collectionId}/attributes/datetime/{key}"
                .replace("{databaseId}", database_id)
                .replace("{collectionId}", collection_id)
                .replace("{key}", key);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create email attribute
    ///
    /// Create an email attribute.
    ///
    ///* key => string
    ///* required => bool
    ///* default => bool?
    ///* array => bool?
    pub async fn create_email_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeEmail, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/email"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update email attribute
    ///
    /// Update an email attribute. Changing the `default` value will not update
    /// already existing documents.
    ///
    ///* required => bool
    ///* default => string?
    pub async fn update_email_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeEmail, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/email/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create enum attribute
    ///
    ///* key => string
    ///* elements => vec(string)
    ///* required => bool
    ///* default => bool?
    ///* array => bool?
    pub async fn create_enum_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeEnum, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/enum"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update enum attribute
    ///
    /// Update an enum attribute. Changing the `default` value will not update
    /// already existing documents.
    ///
    ///* elements => vec(string)
    ///* required => bool
    ///* default => bool?
    pub async fn update_enum_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeEnum, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/enum/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create float attribute
    ///
    /// Create a float attribute. Optionally, minimum and maximum values can be
    /// provided.
    ///
    ///* key => string
    ///* required => bool
    ///* min => float?
    ///* max => float?
    ///* default => float?
    ///* array => bool?
    pub async fn create_float_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeFloat, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/float"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update float attribute
    ///
    /// Update a float attribute. Changing the `default` value will not update
    /// already existing documents.
    ///
    ///* required => bool
    ///* min => float?
    ///* max => float?
    ///* default => float?
    pub async fn update_float_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeFloat, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/float/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create integer attribute
    ///
    /// Create an integer attribute. Optionally, minimum and maximum values can be
    /// provided.
    ///
    ///* key => string
    ///* required => bool
    ///* min => number?
    ///* max => number?
    ///* default => number?
    ///* array => bool?
    pub async fn create_integer_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeInteger, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/integer"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update integer attribute
    ///
    /// Update an integer attribute. Changing the `default` value will not update
    /// already existing documents.
    ///
    ///* required => bool
    ///* min => number?
    ///* max => number?
    ///* default => number?
    pub async fn update_integer_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeInteger, Error> {
        //const API_PATH: &str = "/databases";
        let api_path =
            "/databases/{databaseId}/collections/{collectionId}/attributes/integer/{key}"
                .replace("{databaseId}", database_id)
                .replace("{collectionId}", collection_id)
                .replace("{key}", key);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create IP address attribute
    ///
    /// Create IP address attribute.
    ///
    ///* key => string
    ///* required => bool
    ///* default => string?
    ///* array => bool?
    pub async fn create_ip_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeIp, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/ip"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update IP address attribute
    ///
    /// Update an ip attribute. Changing the `default` value will not update
    /// already existing documents.
    ///
    ///* required => bool
    ///* default => string?
    pub async fn update_ip_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeIp, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/ip/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create relationship attribute
    ///
    /// Create relationship attribute. [Learn more about relationship
    /// attributes](https://appwrite.io/docs/databases-relationships#relationship-attributes).
    ///
    ///* relatedCollectionId => string
    ///* type => RelationshipType
    ///* twoWay => bool?
    ///* key => string?
    ///* twoWayKey => string?
    ///* onDelete => RelationMutate?
    pub async fn create_relationship_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeRelationship, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/relationship"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create string attribute
    ///
    /// Create a string attribute.
    ///
    ///* key => string
    ///* size => number
    ///* required => bool
    ///* default => string?
    ///* array => bool?
    ///* encrypt => bool?
    pub async fn create_string_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeString, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/string"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update string attribute
    ///
    /// Update a string attribute. Changing the `default` value will not update
    /// already existing documents.
    ///
    ///* required => bool
    ///* default => string?
    pub async fn update_string_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeString, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/string/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create URL attribute
    ///
    /// Create a URL attribute.
    ///
    ///* key => string
    ///* required => bool
    ///* default => string?
    ///* array => bool?
    pub async fn create_url_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeUrl, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/url"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update URL attribute
    ///
    /// Update an url attribute. Changing the `default` value will not update
    /// already existing documents.
    ///
    ///* required => bool
    ///* default => string?
    pub async fn update_url_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeUrl, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/url/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Get attribute
    ///
    /// Get attribute by ID.
    pub async fn get_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
    ) -> Result<Value, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json::<Value>().await?)
    }

    /// Delete attribute
    ///
    ///  Deletes an attribute.
    pub async fn delete_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
    ) -> Result<(), Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(())
    }

    /// Update relationship attribute
    ///
    /// Update relationship attribute. [Learn more about relationship
    /// attributes](https://appwrite.io/docs/databases-relationships#relationship-attributes).
    ///
    ///* onDelete => RelationMutate?
    pub async fn update_relationship_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        args: HashMap<String, Value>,
    ) -> Result<AttributeRelationship, Error> {
        //const API_PATH: &str = "/databases";
        let api_path =
            "/databases/{databaseId}/collections/{collectionId}/attributes/{key}/relationship"
                .replace("{databaseId}", database_id)
                .replace("{collectionId}", collection_id)
                .replace("{key}", key);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// List documents
    ///
    /// Get a list of all the user's documents in a given collection. You can use
    /// the query params to filter your results.
    ///* queries => vec(string)?
    pub async fn list_documents(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<DocumentList, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/documents"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create document
    ///
    /// Create a new Document. Before using this route, you should create a new
    /// collection resource using either a [server
    /// integration](https://appwrite.io/docs/server/databases#databasesCreateCollection)
    /// API or directly from your database console.
    ///* documentId => string
    ///* data => HashMap<String, Value>
    ///* permissions => vec(string)?
    pub async fn create_documents(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Document, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/documents"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Get document
    ///
    /// Get a document by its unique ID. This endpoint response returns a JSON
    /// object with the document data.
    ///* queries => vec(string)?
    pub async fn get_document(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        document_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Document, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/documents/{documentId}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{documentId}", document_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update document
    ///
    /// Update a document by its unique ID. Using the patch method you can pass
    /// only specific fields that will get updated.
    ///* data => HashMap<String,Value>?
    ///* permissions => vec(string)?
    pub async fn update_document(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        document_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Document, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/documents/{documentId}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{documentId}", document_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Delete document
    ///
    /// Delete a document by its unique ID.
    pub async fn delete_document(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        document_id: &str,
    ) -> Result<(), Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/documents/{documentId}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{documentId}", document_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(())
    }

    /// List indexes
    ///
    /// List indexes in the collection.
    ///* queries => vec(string)?
    pub async fn list_indexes(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<IndexList, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/indexes"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create index
    ///
    /// Creates an index on the attributes listed. Your index should include all
    /// the attributes you will query in a single request.
    /// Attributes can be `key`, `fulltext`, and `unique`.
    ///* key => string
    ///* type => IndexType
    ///* attributes => vec(string)
    ///* orders => vec(string)?
    pub async fn create_index(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Index, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/indexes"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Get index
    ///
    /// Get index by ID.
    pub async fn get_index(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
    ) -> Result<Index, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/indexes/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Delete index
    ///
    /// Delete an index.
    pub async fn delete_index(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
    ) -> Result<(), Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/indexes/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }
}

#[cfg(test)]
mod tests {

    use crate::{client::ClientBuilder, error::Error, id::ID, permission::Permission, role::Role};

    use super::Databases;

    //#[tokio::test]
    async fn test_databases() -> Result<(), Error> {
        let client = ClientBuilder::default()
            .set_endpoint("http://127.0.0.1/v1")?
            .set_project("676c2b7b000c834e1fce")?
            .set_key("standard_5d84014ebaf0de52308eff28946a43062921240c10b81c2fd037ab60b02f0257b7f0a53fe94065170fe7c7d0af2d4136d4cbf32a4055baeada3d27f2e323b70aeda87e97f676207cf10cbb18b7a80f8d1103803617454c89138f217dad701bbe9dc6950bc58853fdb2a0b4b67d2a8b8b6b7b9b2e6d9b94e0a2fcfee794688e2e")?
            //.set_self_signed(false)?
            .build()?;

        // ! create databases
        let create_databases = Databases::create(
            &client,
            maplit::hashmap! {
                "databaseId".into() => ID::unique(7).into(),
                "name".into()=> "dev".into(),
            },
        )
        .await?;
        assert_eq!(create_databases.name, "dev");

        // ! get database
        let get_database = Databases::get(&client, &create_databases.id).await?;
        assert_eq!(get_database.name, "dev");

        // ! update database
        let update_database = Databases::update(
            &client,
            &get_database.id,
            maplit::hashmap! {
                "name".into()=> "prod".into(),
            },
        )
        .await?;
        assert_eq!(update_database.name, "prod");

        // ! list databases
        let list_databases = Databases::list(&client, maplit::hashmap! {}).await?;
        assert_ne!(list_databases.databases, []);

        // ! create collection
        let create_col = Databases::create_collection(
            &client,
            &update_database.id,
            maplit::hashmap! {
                "collectionId".into() => ID::unique(7).into(),
                "name".into()=> "user".into(),
                "permissions".into()=> [Permission::read(&Role::any())].into(),
            },
        )
        .await?;
        assert_eq!(create_col.name, "user");

        // ! get collection
        let get_col =
            Databases::get_collection(&client, &update_database.id, &create_col.id).await?;
        assert_eq!(get_col.name, "user");

        // ! update collection
        let update_col = Databases::update_collection(
            &client,
            &update_database.id,
            &get_col.id,
            maplit::hashmap! {
                "name".into()=> "users".into(),
                "permissions".into()=> [Permission::read(&Role::any()),Permission::delete(&Role::any()),].into(),
            },
        )
        .await?;
        assert_eq!(update_col.name, "users");

        // ! list attribute
        let list_att = Databases::list_attributes(
            &client,
            &update_database.id,
            &get_col.id,
            maplit::hashmap! {},
        )
        .await?;
        assert_eq!(list_att.total, 0);

        // ! create boolean att
        let create_bool_att = Databases::create_boolean_attribute(
            &client,
            &update_database.id,
            &get_col.id,
            maplit::hashmap! {
                "key".into()=> "isAdmin".into(),
                "required".into()=> false.into(),
                "default".into()=> false.into(),
            },
        )
        .await?;
        assert_eq!(create_bool_att.key, "isAdmin");

        // ! delete collections
        let delete_collection =
            Databases::delete_collection(&client, &update_database.id, &get_col.id).await?;
        assert_eq!(delete_collection, ());

        // ! delete database
        let delete_database = Databases::delete(&client, &update_database.id).await?;
        assert_eq!(delete_database, ());

        Ok(())
    }
}
