// //! # Databases
// //!
// //! The Databases service allows you to create structured collections of
// //! documents, query and filter lists of documents
// use crate::{
//     api_params, app_json_header,
//     client::Client,
//     enumm::HttpMethod,
//     enums::{
//         index_type::IndexType, relation_mutate::RelationMutate, relationship_type::RelationshipType,
//     },
//     error::Error,
//     models::{
//         attribute_boolean::AttributeBoolean, attribute_datetime::AttributeDateTime,
//         attribute_email::AttributeEmail, attribute_enum::AttributeEnum,
//         attribute_float::AttributeFloat, attribute_integer::AttributeInteger,
//         attribute_ip::AttributeIp, attribute_list::AttributeList,
//         attribute_relationship::AttributeRelationship, attribute_string::AttributeString,
//         attribute_url::AttributeUrl, collection::Collection, collection_list::CollectionList,
//         database::Database, database_list::DatabaseList, document::Document,
//         document_list::DocumentList, index::Index, index_list::IndexList,
//     },
// };
// use serde_json::{json, Map, Value};

// pub struct Databases;

// impl Databases {
//     /// List databases
//     ///
//     /// Get a list of all databases from the current Appwrite project. You can use
//     /// the search parameter to filter your results.
//     pub async fn list(
//         client: &Client,
//         search: Option<String>,
//         queries: Option<Vec<String>>,
//     ) -> Result<DatabaseList, Error> {
//         const API_PATH: &str = "/databases";

//         let api_params = api_params!(
//             "search"=>search,
//             "queries"=>queries,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create database
//     ///
//     /// Create a new Database.
//     ///
//     pub async fn create(
//         client: &Client,
//         database_id: &str,
//         name: &str,
//         enabled: Option<bool>,
//     ) -> Result<Database, Error> {
//         const API_PATH: &str = "/databases";

//         let api_params = api_params!(
//             "databaseId"=> Some(database_id),
//             "name"=>Some(name),
//             "enabled"=>enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, API_PATH, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get database
//     ///
//     /// Get a database by its unique ID. This endpoint response returns a JSON
//     /// object with the database metadata.
//     pub async fn get(client: &Client, database_id: &str) -> Result<Database, Error> {
//         let api_path = "/databases/{databaseId}".replace("{databaseId}", database_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update database
//     ///
//     /// Update a database by its unique ID.
//     pub async fn update(
//         client: &Client,
//         database_id: &str,
//         name: &str,
//         enabled: Option<bool>,
//     ) -> Result<Database, Error> {
//         let api_path = "/databases/{databaseId}".replace("{databaseId}", database_id);

//         let api_params = api_params!(
//             "name"=>Some(name),
//             "enabled"=>enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PUT,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Delete database
//     ///
//     /// Delete a database by its unique ID. Only API keys with with databases.write
//     /// scope can delete a database.
//     pub async fn delete(client: &Client, database_id: &str) -> Result<(), Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}".replace("{databaseId}", database_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let _res = client
//             .call(
//                 HttpMethod::DELETE,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(())
//     }

//     /// List collections
//     ///
//     /// Get a list of all collections that belong to the provided databaseId. You
//     /// can use the search parameter to filter your results.
//     pub async fn list_collections(
//         client: &Client,
//         database_id: &str,
//         search: Option<String>,
//         queries: Option<Vec<String>>,
//     ) -> Result<CollectionList, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections".replace("{databaseId}", database_id);

//         let api_params = api_params!(
//             "search"=>search,
//             "queries"=>queries,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create collection
//     ///
//     /// Create a new Collection. Before using this route, you should create a new
//     /// database resource using either a [server
//     /// integration](https://appwrite.io/docs/server/databases#databasesCreateCollection)
//     /// API or directly from your database console.
//     pub async fn create_collection(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         name: &str,
//         permissions: Option<Vec<String>>,
//         document_security: Option<bool>,
//         enabled: Option<bool>,
//     ) -> Result<Collection, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections".replace("{databaseId}", database_id);

//         let api_params = api_params!(
//             "collectionId"=>Some(collection_id),
//             "name"=>Some(name),
//             "permissions"=>permissions,
//             "documentSecurity"=>document_security,
//             "enabled"=>enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get collection
//     ///
//     /// Get a collection by its unique ID. This endpoint response returns a JSON
//     /// object with the collection metadata.
//     pub async fn get_collection(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//     ) -> Result<Collection, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update collection
//     ///
//     /// Update a collection by its unique ID.
//     pub async fn update_collection(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         name: &str,
//         permissions: Option<Vec<String>>,
//         document_security: Option<bool>,
//         enabled: Option<bool>,
//     ) -> Result<Collection, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "name"=>Some(name),
//             "permissions"=>permissions,
//             "documentSecurity"=>document_security,
//             "enabled"=>enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PUT,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Delete collection
//     ///
//     /// Delete a collection by its unique ID. Only users with write permissions
//     /// have access to delete this resource.
//     pub async fn delete_collection(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//     ) -> Result<(), Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let _res = client
//             .call(
//                 HttpMethod::DELETE,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(())
//     }

//     /// List attributes
//     ///
//     /// List attributes in the collection.
//     pub async fn list_attributes(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         queries: Option<Vec<String>>,
//     ) -> Result<AttributeList, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "queries"=>queries,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create boolean attribute
//     ///
//     /// Create a boolean attribute.
//     ///
//     pub async fn create_boolean_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         x_required: bool,
//         x_default: Option<bool>,
//         array: Option<bool>,
//     ) -> Result<AttributeBoolean, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/boolean"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "key"=>Some(key),
//             "required"=>Some(x_required),
//             "default"=>x_default,
//             "array"=>array,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update boolean attribute
//     ///
//     ///  Update a boolean attribute. Changing the `default` value will not update
//     /// already existing documents.
//     pub async fn update_boolean_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         x_required: bool,
//         x_default: Option<bool>,
//     ) -> Result<AttributeBoolean, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path =
//             "/databases/{databaseId}/collections/{collectionId}/attributes/boolean/{key}"
//                 .replace("{databaseId}", database_id)
//                 .replace("{collectionId}", collection_id)
//                 .replace("{key}", key);

//         let api_params = api_params!(
//             "required"=>Some(x_required),
//             "default"=>x_default,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create datetime attribute
//     ///
//     ///  Create a date time attribute according to the ISO 8601 standard.
//     pub async fn create_date_time_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         x_required: bool,
//         x_default: Option<bool>,
//         array: Option<bool>,
//     ) -> Result<AttributeDateTime, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/datetime"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "key"=>Some(key),
//             "required"=>Some(x_required),
//             "default"=>x_default,
//             "array"=>array,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update dateTime attribute
//     ///
//     ///  Update a date time attribute. Changing the `default` value will not update
//     /// already existing documents.
//     pub async fn update_date_time_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         x_required: bool,
//         x_default: Option<bool>,
//     ) -> Result<AttributeDateTime, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path =
//             "/databases/{databaseId}/collections/{collectionId}/attributes/datetime/{key}"
//                 .replace("{databaseId}", database_id)
//                 .replace("{collectionId}", collection_id)
//                 .replace("{key}", key);

//         let api_params = api_params!(
//             "required"=>Some(x_required),
//             "default"=>x_default,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create email attribute
//     ///
//     /// Create an email attribute.
//     ///
//     pub async fn create_email_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         x_required: bool,
//         x_default: Option<&str>,
//         array: Option<bool>,
//     ) -> Result<AttributeEmail, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/email"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "key"=>Some(key),
//             "required"=>Some(x_required),
//             "default"=>x_default,
//             "array"=>array,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update email attribute
//     ///
//     /// Update an email attribute. Changing the `default` value will not update
//     /// already existing documents.
//     ///
//     pub async fn update_email_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         x_required: bool,
//         x_default: Option<&str>,
//     ) -> Result<AttributeEmail, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/email/{key}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id)
//             .replace("{key}", key);

//         let api_params = api_params!(
//             "required"=>Some(x_required),
//             "default"=>x_default,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create enum attribute
//     ///
//     pub async fn create_enum_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         elements: Vec<&str>,
//         x_required: bool,
//         x_default: Option<bool>,
//         array: Option<bool>,
//     ) -> Result<AttributeEnum, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/enum"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "key"=>Some(key),
//             "elements"=>Some(elements),
//             "required"=>Some(x_required),
//             "default"=>x_default,
//             "array"=>array,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update enum attribute
//     ///
//     /// Update an enum attribute. Changing the `default` value will not update
//     /// already existing documents.
//     ///
//     pub async fn update_enum_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         elements: Vec<&str>,
//         x_required: bool,
//         x_default: Option<bool>,
//     ) -> Result<AttributeEnum, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/enum/{key}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id)
//             .replace("{key}", key);

//         let api_params = api_params!(
//             "elements"=>Some(elements),
//             "required"=>Some(x_required),
//             "default"=>x_default,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create float attribute
//     ///
//     /// Create a float attribute. Optionally, minimum and maximum values can be
//     /// provided.
//     ///
//     pub async fn create_float_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         x_required: bool,
//         min: Option<f64>,
//         max: Option<f64>,
//         x_default: Option<f64>,
//         array: Option<bool>,
//     ) -> Result<AttributeFloat, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/float"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "key"=>Some(key),
//             "required"=>Some(x_required),
//             "min"=>min,
//             "max"=>max,
//             "default"=>x_default,
//             "array"=> array,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update float attribute
//     ///
//     /// Update a float attribute. Changing the `default` value will not update
//     /// already existing documents.
//     ///
//     pub async fn update_float_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         min: Option<f64>,
//         max: Option<f64>,
//         x_required: bool,
//         x_default: Option<f64>,
//     ) -> Result<AttributeFloat, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/float/{key}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id)
//             .replace("{key}", key);

//         let api_params = api_params!(
//             "required"=>Some(x_required),
//             "min"=>min,
//             "max"=>max,
//             "default"=>x_default,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create integer attribute
//     ///
//     /// Create an integer attribute. Optionally, minimum and maximum values can be
//     /// provided.
//     ///
//     pub async fn create_integer_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         x_required: bool,
//         min: Option<u64>,
//         max: Option<u64>,
//         x_default: Option<u64>,
//         array: Option<bool>,
//     ) -> Result<AttributeInteger, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/integer"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "key"=> Some(key),
//             "required"=>Some(x_required),
//             "min"=>min,
//             "max"=>max,
//             "default"=>x_default,
//             "array"=> array,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update integer attribute
//     ///
//     /// Update an integer attribute. Changing the `default` value will not update
//     /// already existing documents.
//     ///
//     pub async fn update_integer_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         min: Option<u64>,
//         max: Option<u64>,
//         x_required: bool,
//         x_default: Option<u64>,
//     ) -> Result<AttributeInteger, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path =
//             "/databases/{databaseId}/collections/{collectionId}/attributes/integer/{key}"
//                 .replace("{databaseId}", database_id)
//                 .replace("{collectionId}", collection_id)
//                 .replace("{key}", key);

//         let api_params = api_params!(
//             "required"=>Some(x_required),
//             "min"=>min,
//             "max"=>max,
//             "default"=>x_default,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create IP address attribute
//     ///
//     /// Create IP address attribute.
//     ///
//     pub async fn create_ip_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         x_required: bool,
//         x_default: Option<&str>,
//         array: Option<bool>,
//     ) -> Result<AttributeIp, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/ip"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "key"=> Some(key),
//             "required"=>Some(x_required),
//             "default"=>x_default,
//             "array"=> array,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update IP address attribute
//     ///
//     /// Update an ip attribute. Changing the `default` value will not update
//     /// already existing documents.
//     ///
//     pub async fn update_ip_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         x_required: bool,
//         x_default: Option<&str>,
//     ) -> Result<AttributeIp, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/ip/{key}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id)
//             .replace("{key}", key);

//         let api_params = api_params!(
//             "required"=>Some(x_required),
//             "default"=>x_default,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create relationship attribute
//     ///
//     /// Create relationship attribute. [Learn more about relationship
//     /// attributes](https://appwrite.io/docs/databases-relationships#relationship-attributes).
//     ///
//     pub async fn create_relationship_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         related_collection_id: &str,
//         relationship_type: RelationshipType,
//         two_way: Option<bool>,
//         key: Option<&str>,
//         two_way_key: Option<&str>,
//         on_delete: Option<RelationMutate>,
//     ) -> Result<AttributeRelationship, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/relationship"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "relatedCollectionId"=> Some(related_collection_id),
//             "type"=>Some(relationship_type),
//             "twoWay"=>two_way,
//             "key"=>key,
//             "twoWayKey"=> two_way_key,
//             "onDelete"=> on_delete,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create string attribute
//     ///
//     /// Create a string attribute.
//     ///
//     pub async fn create_string_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         size: u64,
//         x_required: bool,
//         x_default: Option<&str>,
//         array: Option<bool>,
//         encrypt: Option<bool>,
//     ) -> Result<AttributeString, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/string"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "key"=> Some(key),
//             "size"=>Some(size),
//             "required"=>Some(x_required),
//             "default"=>x_default,
//             "array"=> array,
//             "encrypt"=> encrypt,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update string attribute
//     ///
//     /// Update a string attribute. Changing the `default` value will not update
//     /// already existing documents.
//     ///
//     pub async fn update_string_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         x_required: bool,
//         x_default: Option<&str>,
//     ) -> Result<AttributeString, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/string/{key}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id)
//             .replace("{key}", key);

//         let mut api_params = serde_json::Map::new();
//         api_params.insert("required".to_string(), json!(x_required));
//         if let Some(default_val) = x_default {
//             api_params.insert("default".to_string(), json!(default_val));
//         }

//         let api_params = api_params!(
//             "required"=> Some(x_required),
//             "default"=>x_default,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create URL attribute
//     ///
//     /// Create a URL attribute.
//     ///
//     pub async fn create_url_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         x_required: bool,
//         x_default: Option<&str>,
//         array: Option<bool>,
//     ) -> Result<AttributeUrl, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/url"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "key"=> Some(key),
//             "required"=>Some(x_required),
//             "default"=>x_default,
//             "array"=>array,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update URL attribute
//     ///
//     /// Update an url attribute. Changing the `default` value will not update
//     /// already existing documents.
//     ///
//     pub async fn update_url_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         x_required: bool,
//         x_default: Option<&str>,
//     ) -> Result<AttributeUrl, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/url/{key}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id)
//             .replace("{key}", key);

//         let api_params = api_params!(
//             "required"=> Some(x_required),
//             "default"=>x_default,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get attribute
//     ///
//     /// Get attribute by ID.
//     pub async fn get_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//     ) -> Result<Value, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/{key}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id)
//             .replace("{key}", key);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json::<Value>().await?)
//     }

//     /// Delete attribute
//     ///
//     ///  Deletes an attribute.
//     pub async fn delete_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//     ) -> Result<(), Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/attributes/{key}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id)
//             .replace("{key}", key);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let _res = client
//             .call(
//                 HttpMethod::DELETE,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(())
//     }

//     /// Update relationship attribute
//     ///
//     /// Update relationship attribute. [Learn more about relationship
//     /// attributes](https://appwrite.io/docs/databases-relationships#relationship-attributes).
//     ///
//     pub async fn update_relationship_attribute(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         on_delete: Option<RelationMutate>,
//     ) -> Result<AttributeRelationship, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path =
//             "/databases/{databaseId}/collections/{collectionId}/attributes/{key}/relationship"
//                 .replace("{databaseId}", database_id)
//                 .replace("{collectionId}", collection_id)
//                 .replace("{key}", key);

//         let api_params = api_params!(
//             "onDelete"=> on_delete,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// List documents
//     ///
//     /// Get a list of all the user's documents in a given collection. You can use
//     /// the query params to filter your results.
//     pub async fn list_documents(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         queries: Option<Vec<String>>,
//     ) -> Result<DocumentList, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/documents"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "queries"=> queries,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create document
//     ///
//     /// Create a new Document. Before using this route, you should create a new
//     /// collection resource using either a [server
//     /// integration](https://appwrite.io/docs/server/databases#databasesCreateCollection)
//     /// API or directly from your database console.
//     pub async fn create_documents(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         document_id: &str,
//         data: Map<String, Value>,
//         permissions: Option<Vec<String>>,
//     ) -> Result<Document, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/documents"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "documentId"=> Some(document_id),
//             "data"=>Some(data),
//             "permissions"=>permissions,

//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get document
//     ///
//     /// Get a document by its unique ID. This endpoint response returns a JSON
//     /// object with the document data.
//     pub async fn get_document(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         document_id: &str,
//         queries: Option<Vec<String>>,
//     ) -> Result<Document, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/documents/{documentId}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id)
//             .replace("{documentId}", document_id);

//         let api_params = api_params!(
//             "queries"=> queries,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update document
//     ///
//     /// Update a document by its unique ID. Using the patch method you can pass
//     /// only specific fields that will get updated.
//     pub async fn update_document(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         document_id: &str,
//         data: Option<Map<String, Value>>,
//         permissions: Option<Vec<String>>,
//     ) -> Result<Document, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/documents/{documentId}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id)
//             .replace("{documentId}", document_id);

//         let api_params = api_params!(
//             "data"=> Some(data),
//             "permissions"=>Some(permissions),
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Delete document
//     ///
//     /// Delete a document by its unique ID.
//     pub async fn delete_document(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         document_id: &str,
//     ) -> Result<(), Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/documents/{documentId}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id)
//             .replace("{documentId}", document_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let _res = client
//             .call(
//                 HttpMethod::DELETE,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(())
//     }

//     /// List indexes
//     ///
//     /// List indexes in the collection.
//     pub async fn list_indexes(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         queries: Option<Vec<String>>,
//     ) -> Result<IndexList, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/indexes"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "queries"=> queries,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create index
//     ///
//     /// Creates an index on the attributes listed. Your index should include all
//     /// the attributes you will query in a single request.
//     /// Attributes can be `key`, `fulltext`, and `unique`.
//     pub async fn create_index(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//         index_type: IndexType,
//         attributes: Vec<&str>,
//         orders: Option<Vec<&str>>,
//     ) -> Result<Index, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/indexes"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id);

//         let api_params = api_params!(
//             "key"=> Some(key),
//             "type"=>Some(index_type),
//             "attributes"=>Some(attributes),
//             "orders"=>orders,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get index
//     ///
//     /// Get index by ID.
//     pub async fn get_index(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//     ) -> Result<Index, Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/indexes/{key}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id)
//             .replace("{key}", key);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Delete index
//     ///
//     /// Delete an index.
//     pub async fn delete_index(
//         client: &Client,
//         database_id: &str,
//         collection_id: &str,
//         key: &str,
//     ) -> Result<(), Error> {
//         //const API_PATH: &str = "/databases";
//         let api_path = "/databases/{databaseId}/collections/{collectionId}/indexes/{key}"
//             .replace("{databaseId}", database_id)
//             .replace("{collectionId}", collection_id)
//             .replace("{key}", key);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::DELETE,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }
// }
