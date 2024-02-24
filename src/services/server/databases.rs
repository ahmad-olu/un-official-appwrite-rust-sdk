use crate::{
    client::Client,
    enums::HttpMethod,
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
use reqwest::header;
use serde_json::Value;

/// The Databases service allows you to create structured collections of
/// documents, query and filter lists of documents
struct Databases;

impl Databases {
    /// List databases
    ///
    /// Get a list of all databases from the current Appwrite project. You can use
    /// the search parameter to filter your results.
    async fn list(
        client: &Client,
        search: Option<&str>,
        queries: Option<Vec<&str>>,
    ) -> Result<DatabaseList, Error> {
        const API_PATH: &str = "/databases";

        let api_params = serde_json::json!({
            "queries":queries,
            "search":search,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create database
    ///
    /// Create a new Database.
    ///
    async fn create(
        client: &Client,
        database_id: &str,
        name: &str,
        enabled: Option<bool>,
    ) -> Result<Database, Error> {
        const API_PATH: &str = "/databases";

        let api_params = serde_json::json!({
            "databaseId":database_id,
            "name":name,
            "enabled":enabled
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Get database
    ///
    /// Get a database by its unique ID. This endpoint response returns a JSON
    /// object with the database metadata.
    async fn get(client: &Client, database_id: &str) -> Result<Database, Error> {
        let api_path = "/databases/{databaseId}".replace("{databaseId}", database_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update database
    ///
    /// Update a database by its unique ID.
    async fn update(
        client: &Client,
        database_id: &str,
        name: &str,
        enabled: Option<bool>,
    ) -> Result<Database, Error> {
        let api_path = "/databases/{databaseId}".replace("{databaseId}", database_id);

        let api_params = serde_json::json!({
            "name":name,
            "enabled":enabled,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PUT,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Delete database
    ///
    /// Delete a database by its unique ID. Only API keys with with databases.write
    /// scope can delete a database.
    async fn delete(client: &Client, database_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}".replace("{databaseId}", database_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// List collections
    ///
    /// Get a list of all collections that belong to the provided databaseId. You
    /// can use the search parameter to filter your results.
    async fn list_collections(
        client: &Client,
        database_id: &str,
        search: Option<&str>,
        queries: Option<Vec<&str>>,
    ) -> Result<CollectionList, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections".replace("{databaseId}", database_id);

        let api_params = serde_json::json!({
            "queries":queries,
            "search":search
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create collection
    ///
    /// Create a new Collection. Before using this route, you should create a new
    /// database resource using either a [server
    /// integration](https://appwrite.io/docs/server/databases#databasesCreateCollection)
    /// API or directly from your database console.
    async fn create_collection(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        name: &str,
        permissions: Option<Vec<&str>>,
        document_security: Option<bool>,
        enabled: Option<bool>,
    ) -> Result<Collection, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections".replace("{databaseId}", database_id);

        let api_params = serde_json::json!({
            "collectionId":collection_id,
            "name":name,
            "permissions":permissions,
            "documentSecurity":document_security,
            "enabled":enabled,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Get collection
    ///
    /// Get a collection by its unique ID. This endpoint response returns a JSON
    /// object with the collection metadata.
    async fn get_collection(
        client: &Client,
        database_id: &str,
        collection_id: &str,
    ) -> Result<Collection, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update collection
    ///
    /// Update a collection by its unique ID.
    async fn update_collection(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        name: &str,
        permissions: Option<Vec<&str>>,
        document_security: Option<bool>,
        enabled: Option<bool>,
    ) -> Result<Collection, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "name":name,
            "permissions":permissions,
            "documentSecurity":document_security,
            "enabled":enabled,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PUT,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Delete collection
    ///
    /// Delete a collection by its unique ID. Only users with write permissions
    /// have access to delete this resource.
    async fn delete_collection(
        client: &Client,
        database_id: &str,
        collection_id: &str,
    ) -> Result<(), Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(())
    }

    /// List attributes
    ///
    async fn list_attributes(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        queries: Option<Vec<&str>>,
    ) -> Result<AttributeList, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "queries":queries
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create boolean attribute
    ///
    /// Create a boolean attribute.
    ///
    async fn create_boolean_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        x_required: bool,
        x_default: Option<bool>,
        array: Option<bool>,
    ) -> Result<AttributeBoolean, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/boolean"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "key":key,
            "required": x_required,
            "default": x_default,
            "array":array,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update boolean attribute
    ///
    async fn update_boolean_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        x_required: bool,
        x_default: Option<bool>,
    ) -> Result<AttributeBoolean, Error> {
        //const API_PATH: &str = "/databases";
        let api_path =
            "/databases/{databaseId}/collections/{collectionId}/attributes/boolean/{key}"
                .replace("{databaseId}", database_id)
                .replace("{collectionId}", collection_id)
                .replace("{key}", key);

        let api_params = serde_json::json!({

            "required": x_required,
            "default": x_default,

        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create datetime attribute
    ///
    async fn create_date_time_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        x_required: bool,
        x_default: Option<bool>,
        array: Option<bool>,
    ) -> Result<AttributeDateTime, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/datetime"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "key":key,
            "required": x_required,
            "default": x_default,
            "array":array,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update dateTime attribute
    ///
    async fn update_date_time_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        x_required: bool,
        x_default: Option<bool>,
    ) -> Result<AttributeDateTime, Error> {
        //const API_PATH: &str = "/databases";
        let api_path =
            "/databases/{databaseId}/collections/{collectionId}/attributes/datetime/{key}"
                .replace("{databaseId}", database_id)
                .replace("{collectionId}", collection_id)
                .replace("{key}", key);

        let api_params = serde_json::json!({

            "required": x_required,
            "default": x_default,

        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create email attribute
    ///
    /// Create an email attribute.
    ///
    async fn create_email_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        x_required: bool,
        x_default: Option<bool>,
        array: Option<bool>,
    ) -> Result<AttributeEmail, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/email"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "key":key,
            "required": x_required,
            "default": x_default,
            "array":array,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
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
    async fn update_email_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        x_required: bool,
        x_default: Option<bool>,
    ) -> Result<AttributeEmail, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/email/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let api_params = serde_json::json!({

            "required": x_required,
            "default": x_default,

        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create enum attribute
    ///
    async fn create_enum_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        elements: Vec<&str>,
        x_required: bool,
        x_default: Option<bool>,
        array: Option<bool>,
    ) -> Result<AttributeEnum, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/enum"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "key":key,
            "elements":elements,
            "required": x_required,
            "default": x_default,
            "array":array,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
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
    async fn update_enum_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        elements: Vec<&str>,
        x_required: bool,
        x_default: Option<bool>,
    ) -> Result<AttributeEnum, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/enum/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let api_params = serde_json::json!({
            "elements":elements,
            "required": x_required,
            "default": x_default,

        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &api_params,
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
    async fn create_float_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        x_required: bool,
        min: Option<f64>,
        max: Option<f64>,
        x_default: Option<f64>,
        array: Option<bool>,
    ) -> Result<AttributeFloat, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/float"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "key":key,
            "required": x_required,
            "min":min,
            "max":max,
            "default": x_default,
            "array":array,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
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
    async fn update_float_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        min: Option<f64>,
        max: Option<f64>,
        x_required: bool,
        x_default: Option<f64>,
    ) -> Result<AttributeFloat, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/float/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let api_params = serde_json::json!({
            "min":min,
            "max":max,
            "required": x_required,
            "default": x_default,

        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &api_params,
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
    async fn create_integer_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        x_required: bool,
        min: Option<u64>,
        max: Option<u64>,
        x_default: Option<u64>,
        array: Option<bool>,
    ) -> Result<AttributeInteger, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/integer"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "key":key,
            "required": x_required,
            "min":min,
            "max":max,
            "default": x_default,
            "array":array,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
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
    async fn update_integer_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        min: Option<u64>,
        max: Option<u64>,
        x_required: bool,
        x_default: Option<u64>,
    ) -> Result<AttributeInteger, Error> {
        //const API_PATH: &str = "/databases";
        let api_path =
            "/databases/{databaseId}/collections/{collectionId}/attributes/integer/{key}"
                .replace("{databaseId}", database_id)
                .replace("{collectionId}", collection_id)
                .replace("{key}", key);

        let api_params = serde_json::json!({
            "min":min,
            "max":max,
            "required": x_required,
            "default": x_default,

        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create IP address attribute
    ///
    /// Create IP address attribute.
    ///
    async fn create_ip_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        x_required: bool,
        x_default: Option<&str>,
        array: Option<bool>,
    ) -> Result<AttributeIp, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/ip"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "key":key,
            "required": x_required,
            "default": x_default,
            "array":array,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
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
    async fn update_ip_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        x_required: bool,
        x_default: Option<&str>,
    ) -> Result<AttributeIp, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/ip/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let api_params = serde_json::json!({
            "required": x_required,
            "default": x_default,

        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &api_params,
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
    async fn create_relationship_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        related_collection_id: &str,
        relationship_type: &str,
        two_way: Option<bool>,
        key: Option<&str>,
        two_way_key: Option<&str>,
        on_delete: Option<&str>,
    ) -> Result<AttributeRelationship, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/relationship"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "relatedCollectionId":related_collection_id,
            "type":relationship_type,
            "twoWay":two_way,
            "key":key,
            "twoWayKey":two_way_key,
            "onDelete": on_delete,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create string attribute
    ///
    /// Create a string attribute.
    ///
    async fn create_string_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        size: u64,
        x_required: bool,
        x_default: Option<&str>,
        array: Option<bool>,
        encrypt: Option<bool>,
    ) -> Result<AttributeString, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/string"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "key":key,
            "size":size,
            "required": x_required,
            "default": x_default,
            "array":array,
            "encrypt":encrypt,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
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
    async fn update_string_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,

        x_required: bool,
        x_default: Option<&str>,
    ) -> Result<AttributeString, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/string/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let api_params = serde_json::json!({
            "required": x_required,
            "default": x_default,

        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create URL attribute
    ///
    /// Create a URL attribute.
    ///
    async fn create_url_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        x_required: bool,
        x_default: Option<&str>,
        array: Option<bool>,
    ) -> Result<AttributeUrl, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/url"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "key":key,
            "required": x_required,
            "default": x_default,
            "array":array,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
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
    async fn update_url_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        x_required: bool,
        x_default: Option<&str>,
    ) -> Result<AttributeUrl, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/url/{key}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{key}", key);

        let api_params = serde_json::json!({
            "required": x_required,
            "default": x_default,

        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Get attribute
    ///
    async fn get_attribute(
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

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json::<Value>().await?)
    }

    /// Delete attribute
    ///
    async fn delete_attribute(
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

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &api_params,
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
    async fn update_relationship_attribute(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        on_delete: Option<&str>,
    ) -> Result<AttributeRelationship, Error> {
        //const API_PATH: &str = "/databases";
        let api_path =
            "/databases/{databaseId}/collections/{collectionId}/attributes/{key}/relationship"
                .replace("{databaseId}", database_id)
                .replace("{collectionId}", collection_id)
                .replace("{key}", key);

        let api_params = serde_json::json!({
            "onDelete":on_delete,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// List documents
    ///
    /// Get a list of all the user's documents in a given collection. You can use
    /// the query params to filter your results.
    async fn list_documents(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        queries: Option<Vec<&str>>,
    ) -> Result<DocumentList, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/documents"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "queries":queries,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create document
    ///
    /// Create a new Document. Before using this route, you should create a new
    /// collection resource using either a [server
    /// integration](https://appwrite.io/docs/server/databases#databasesCreateCollection)
    /// API or directly from your database console.
    async fn create_documents(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        document_id: &str,
        data: Value,
        permissions: Option<Vec<&str>>,
    ) -> Result<Document, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/documents"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "documentId":document_id,
            "data":data,
            "permissions":permissions,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Get document
    ///
    /// Get a document by its unique ID. This endpoint response returns a JSON
    /// object with the document data.
    async fn get_document(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        document_id: &str,
        queries: Option<Vec<&str>>,
    ) -> Result<Document, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/documents/{documentId}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{documentId}", document_id);

        let api_params = serde_json::json!({
            "queries":queries,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update document
    ///
    /// Update a document by its unique ID. Using the patch method you can pass
    /// only specific fields that will get updated.
    async fn update_document(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        document_id: &str,
        data: Option<Value>,
        permissions: Option<Vec<&str>>,
    ) -> Result<Document, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/documents/{documentId}"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id)
            .replace("{documentId}", document_id);

        let api_params = serde_json::json!({
            "data":data,
            "permissions":permissions,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Delete document
    ///
    /// Delete a document by its unique ID.
    async fn delete_document(
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

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(())
    }

    /// List indexes
    ///
    async fn list_indexes(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        queries: Option<Vec<&str>>,
    ) -> Result<IndexList, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/indexes"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "queries": queries,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create index
    ///
    async fn create_index(
        client: &Client,
        database_id: &str,
        collection_id: &str,
        key: &str,
        index_type: &str,
        attributes: Vec<&str>,
        orders: Option<Vec<&str>>,
    ) -> Result<Index, Error> {
        //const API_PATH: &str = "/databases";
        let api_path = "/databases/{databaseId}/collections/{collectionId}/indexes"
            .replace("{databaseId}", database_id)
            .replace("{collectionId}", collection_id);

        let api_params = serde_json::json!({
            "key":key,
            "type":index_type,
            "attributes":attributes,
            "orders":orders,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Get index
    ///
    async fn get_index(
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

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    async fn delete_index(
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

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }
}
