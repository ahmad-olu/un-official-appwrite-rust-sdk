//! # Unofficial Appwrite
//!
//! `unofficial_appwrite` was created to help developers interested in using appwrite for their rust
//! web servers.
//!
//! Appwrite is an open-source backend as a service server that abstract and simplify complex
//! and repetitive development tasks behind a very simple to use REST API. Appwrite aims to
//! help you develop your apps faster and in a more secure way. Use the Dart SDK to integrate
//! your app with the Appwrite server to easily start interacting with all of Appwrite
//! backend APIs and tools. For full API documentation and tutorials go
//! to https://appwrite.io/docs

//! ### Initialize & Make API Request
//!
//! ``` rust
//!
//!use unofficial_appwrite::client::ClientBuilder;
//!use unofficial_appwrite::error::Error;
//!use unofficial_appwrite::services::server::users::Users;
//!use unofficial_appwrite::id::ID;
//!
//!#[tokio::main]
//!async fn main() -> Result<(), Error> {
//!    let  client = ClientBuilder::default()
//!    .set_endpoint("http://[HOSTNAME_OR_IP]/v1") // Make sure your endpoint is accessible
//!    .set_project("5ff3379a01d25")? // Your project ID
//!    .set_key("cd868c7af8bdc893b4...93b7535db89")?
//!    //.set_self_signed(false)? // Use only on dev mode with a self-signed SSL cert
//!    .build()?;
//!
//! let create_user = Users::create(
//!        &client,
//!        ID::unique(),
//!        Some("email@example.com"),
//!        None,
//!        Some("password"),
//!        Some("Walter O'Brien"),
//!    )
//!    .await?;
//!    dbg!("{:?}", create_user);
//!}
//!```
//! --------------------------
//!
//!  #### User Examples
//! ```rust
//!
//!use unofficial_appwrite::client::ClientBuilder;
//!use unofficial_appwrite::error::Error;
//!use unofficial_appwrite::services::server::users::Users;
//!use serde_json::{json, Map};
//!
//!#[tokio::main]
//!async fn main() -> Result<(), Error> {
//!    let  client = ClientBuilder::default()
//!    .set_endpoint("http://[HOSTNAME_OR_IP]/v1") // Make sure your endpoint is accessible
//!    .set_project("5ff3379a01d25")? // Your project ID
//!    .set_key("cd868c7af8bdc893b4...93b7535db89")?
//!    //.set_self_signed(false)? // Use only on dev mode with a self-signed SSL cert
//!    .build()?;
//!
//!let user_prefs = Users::get_prefs(&client, "6...64").await?;
//!    dbg!("{:#?}", user_prefs);
//!
//!    let mut prefs = Map::new();
//!    prefs.insert("isAdmin".to_string(), json!(true));
//!    let user_update_prefs = Users::update_prefs(&client, "6...4", prefs).await?;
//!    dbg!("{:#?}", user_update_prefs);
//!}
//!
//!```
//! --------------------------
//!
//! #### Database Examples
//!
//! ```rust
//!use unofficial_appwrite::client::ClientBuilder;
//!use unofficial_appwrite::error::Error;
//!use serde_json::{json, Map};
//!use unofficial_appwrite::id::ID;
//!use unofficial_appwrite::services::server::databases::Databases;
//!use permission::Permission;
//!use unofficial_appwrite::enums::relationship_type::RelationshipType;
//!
//!#[tokio::main]
//!async fn main() -> Result<(), Error> {
//!
//!     let  client = ClientBuilder::default()
//!    .set_endpoint("http://[HOSTNAME_OR_IP]/v1") // Make sure your endpoint is accessible
//!    .set_project("5ff3379a01d25")? // Your project ID
//!    .set_key("cd868c7af8bdc893b4...93b7535db89")?
//!    //.set_self_signed(false)? // Use only on dev mode with a self-signed SSL cert
//!    .build()?;
//!
//!     let create_db = Databases::create(&client, ID::unique(), "test_db", None).await?;
//!    dbg!(create_db);
//!
//!    let create_collection = Databases::create_collection(
//!        &client,"6618...76",ID::unique(),"test_collection_1",
//!        Some(vec![
//!            Permission::read("any").as_str(),
//!            Permission::create("user:22222346").as_str(),
//!        ]),
//!        None,None,).await?;
//!    dbg!(create_collection);
//!
//!    let att = Databases::create_boolean_attribute(
//!        &client,"6618...76","6618...d4","isAdmin",true,None,None,).await?;
//!    dbg!(att);
//!    let name = Databases::create_string_attribute(
//!        &client,"6618...76","6618...d4","title",255,true,None,None,None,).await?;
//!    dbg!(name);
//!
//!    let mut data = Map::new();
//!    data.insert(String::from("isAdmin"), json!(false));
//!    data.insert(String::from("title"), json!("next1"));
//!    let create_doc = Databases::create_documents(
//!        &client,"6618...76","6618...d4",ID::unique(),data,None,).await?;
//!    dbg!(create_doc);
//!
//!    let relationship = Databases::create_relationship_attribute(
//!        &client,"6618...76","6618...d4","6619...1e",
//!        RelationshipType::OneToOne,
//!        None,Some("test_col_2"),None,None,).await?;
//!    dbg!(relationship);
//!}
//!
//!```
//!
//!-----------------------------------
//!
//! #### Storage Examples
//!
//!```rust
//!use unofficial_appwrite::client::ClientBuilder;
//!use unofficial_appwrite::error::Error;
//!use unofficial_appwrite::id::ID;
//!use unofficial_appwrite::services::server::storage::Storage,;
//!use std::fs;
//!
//!#[tokio::main]
//!async fn main() -> Result<(), Error> {
//!
//!     let  client = ClientBuilder::default()
//!    .set_endpoint("http://[HOSTNAME_OR_IP]/v1") // Make sure your endpoint is accessible
//!    .set_project("5ff3379a01d25")? // Your project ID
//!    .set_key("cd868c7af8bdc893b4...93b7535db89")?
//!    //.set_self_signed(false)? // Use only on dev mode with a self-signed SSL cert
//!    .build()?;
//!
//!let create_buk = Storage::create_bucket(
//!        &client,ID::unique(),"My Bucket",None,None,None,None,None,None,None,None,).await?;
//!    dbg!(create_buk);
//!
//!    let create_file_less_than_5_mb = Storage::create_files(
//!        &client,"661...e9",ID::unique(),
//!       r"C:\Users\pc\Downloads\Documents\test\WEB700 Assignment 5.pdf",None, None,
//!    ).await?;
//!    dbg!(create_file_less_than_5_mb);
//!
//!    let create_file_greater_than_5_mb = Storage::create_files(
//!        &client,"661...e9",ID::unique(),
//!        r"C:\Users\pc\Downloads\Documents\test\Lovefool Vintage Jazz Cardigans Cover ft Haley Reinhart.mp4",None,
//!        Some(|prog| {
//!            println!("{}:{}:{}", prog.id, prog.progress, prog.size_uploaded);
//!        }),
//!    )
//!    .await?;
//!    dbg!(create_file_greater_than_5_mb);
//!
//!    let get_file =
//!        Storage::get_file(&client, "661...e9", "661...71e").await?;
//!    dbg!(get_file);
//!
//!    let get_file_download =
//!        Storage::get_file_download(&client, "661...e9", "661...71e").await?;
//!    fs::write(
//!        r"C:\Users\pc\Downloads\Documents\test\second\new.mp4",
//!        get_file_download,
//!    )
//!    .expect("didn't work");
//!}
//!
//!```
//!
//!----------------------------------------
//!
//! NOTE 🎶: for other examples. check out the official docs or sdk of official sdk as a guide to using this sdk.

pub mod client;
pub mod enumm;
pub mod enums;
pub mod error;
pub mod id;
pub mod models;
pub mod permission;
pub mod query;
mod realtime;
pub mod role;
pub mod services;
pub mod upload_progress;
pub mod utils;
