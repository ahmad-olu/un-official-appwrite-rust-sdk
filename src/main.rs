use std::{fs, path::Path};

use reqwest::{
    header::{self, CONTENT_TYPE},
    multipart::{self, Part},
    StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use futures_util::{future, pin_mut, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

mod client;
pub mod enums;
mod error;
pub mod id;
pub mod models;
pub mod permission;
pub mod query;
pub mod role;
mod services;
pub mod upload_progress;
pub mod utils;

const BASE_URL: &str = "https://cloud.appwrite.io";

enum AccountUrl {
    Create,
    CreateEmailSession,
    CreateAnonymousSession,
}

/// HTTP methods.
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl AccountUrl {
    fn url(&self) -> (&str, HttpMethod) {
        match self {
            AccountUrl::Create => ("/v1/account", HttpMethod::POST),
            AccountUrl::CreateEmailSession => ("/v1/account/sessions/email", HttpMethod::POST),
            AccountUrl::CreateAnonymousSession => {
                ("/v1/account/sessions/anonymous", HttpMethod::POST)
            }
        }
    }
}

async fn run_account_client<T: Serialize + ?Sized>(
    url: AccountUrl,
    base_url: &str,
    json: Option<&T>,
) -> Result<String, reqwest::Error> {
    let acc = url.url();
    let res = reqwest::Client::new();

    let res = match acc.1 {
        HttpMethod::GET => res.get(format!("{}{}", base_url, acc.0)),
        HttpMethod::POST => res.post(format!("{}{}", base_url, acc.0)),
        HttpMethod::PUT => res.put(format!("{}{}", base_url, acc.0)),
        HttpMethod::DELETE => res.delete(format!("{}{}", base_url, acc.0)),
        HttpMethod::PATCH => res.patch(format!("{}{}", base_url, acc.0)),
    };

    let res = match json {
        Some(data) => res.json(data),
        None => res.json(&serde_json::json!({})),
    };

    let res = res
        .header(CONTENT_TYPE, "application/json")
        .header("X-Appwrite-Project", "65d20d389f2b36778b8b")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

#[derive(Debug)]
struct OnProgress {
    id: String,
    progress: usize,
    size_uploaded: usize,
    chunks_total: usize,
    chunks_uploaded: usize,
}

async fn create_file(on_progress: Option<fn(OnProgress)>) -> Result<File, AppWriteError> {
    const CHUNK_SIZE: usize = 5 * 1024 * 1024; // 5 MB
    const FILE_PART: &str =
        r"C:\Users\pc\Documents\books\soft books\cheatsheet\AI-Neural-Networks.pdf";
    const FILE_NAME: &str = "AI-Neural-Networks.pdf";

    // const FILE_PART: &str = r"C:\Users\pc\Pictures\Imagine\6676892.jpg";
    // const FILE_NAME: &str = "6676892.jpg";

    let file = match fs::read(FILE_PART) {
        Ok(size) => size,
        Err(err) => return Err(AppWriteError::FileSizeError(err)),
    };

    let file_size = file.len();

    // API Path Formation
    let api_path = format!("/storage/buckets/{}/files", "65d20d5c8096032a03cd");
    let base_url = "https://cloud.appwrite.io/v1";
    let uri = format!("{}{}", base_url, api_path);

    // File Size Check and Upload
    if file_size <= CHUNK_SIZE {
        // Single-request upload
        let part = Part::bytes(file).file_name(FILE_NAME);
        let form = multipart::Form::new()
            .text("fileId", "unique()")
            .part("file", part);

        let file = reqwest::Client::new()
            .post(uri)
            .header(CONTENT_TYPE, "multipart/form-data; boundary=111122223333")
            .header("X-Appwrite-Project", "65d20d389f2b36778b8b")
            .multipart(form)
            .send()
            .await
            .map_err(|e| AppWriteError::NetworkError(e))?
            .json::<File>()
            .await
            .map_err(|e| AppWriteError::InvalidResponse(e))?;
        return Ok(file);
    }

    // Large File Chunking
    let mut offset: usize = 0;
    let boundary = "111122223333";
    let mut first_upload = true; // Track if it's the first upload for x-appwrite-id
    let mut x_appwrite_id: Option<String> = None;

    let mut res: Option<File> = None;

    while offset < file_size {
        let end = std::cmp::min(offset + CHUNK_SIZE, file_size);
        let chunk = &file[offset..end];
        let content_range = format!("bytes {}-{}/{}", offset, end - 1, file_size);
        println!("{}", content_range);
        // Build the multipart form request
        let mut headers = header::HeaderMap::new();

        let mut chunk_form = multipart::Form::new();

        chunk_form = chunk_form
            .text("fileId", "unique()")
            .part("file", Part::bytes(chunk.to_vec()).file_name(FILE_NAME));

        if !first_upload {
            headers.insert(
                "x-appwrite-id",
                x_appwrite_id.clone().unwrap().as_str().parse().unwrap(),
            );
        }

        let response = reqwest::Client::new()
            .post(uri.clone())
            .json(&serde_json::json!({
                "permissions": &[] as &[String]
            }))
            .header(
                reqwest::header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={}", boundary),
            )
            .header("X-Appwrite-Project", "65d20d389f2b36778b8b")
            .header("Content-Range", content_range)
            .headers(headers)
            .multipart(chunk_form)
            .send()
            .await
            .map_err(AppWriteError::NetworkError)?;

        if response.status() != StatusCode::CREATED {
            println!("{:#?}", response.text().await);
            return Err(AppWriteError::AppwriteError(
                "Invalid response from server".to_string(),
            ));
        }
        let response = response
            .json::<File>()
            .await
            .map_err(|_| AppWriteError::AppwriteError("Conversion to file error".to_string()))?;

        if first_upload {
            if let Some(id) = response.clone().id {
                println!("id-----{}", id);
                x_appwrite_id = Some(id);
            }
            first_upload = false;
        }

        if let Some(ref on_progress) = on_progress {
            on_progress(OnProgress {
                id: response.clone().id.unwrap(),
                progress: std::cmp::min(offset, CHUNK_SIZE) / CHUNK_SIZE * 100,
                size_uploaded: std::cmp::min(offset, CHUNK_SIZE),
                chunks_total: response.chunks_total,
                chunks_uploaded: response.chunks_uploaded,
            });
        }

        res = Some(response.clone());

        offset += CHUNK_SIZE;
    }

    Ok(res.unwrap())
}

#[tokio::main]
async fn main() -> Result<(), AppWriteError> {
    // let data = serde_json::json!({
    //     "userId":"unique()",
    //     "email":"ace2@gmail.com",
    //     "password":"password",
    // });

    // let res = run_account_client(AccountUrl::Create, BASE_URL, Some(&data)).await?;
    // println!("{:#?}", res);

    // let res = run_account_client(AccountUrl::CreateAnonymousSession, BASE_URL, None::<&Value>)
    //     .await
    //     .expect("msg");
    // println!("{:#?}", res);

    //

    // fn prog(p: OnProgress) {
    //     println!("{:?}", p)
    // }
    // let file = create_file(Some(|p| println!("prog>>-------->{:?}", p))).await?;
    // println!("{:#?}", file);

    //realtime

    // let (ws_stream, _response)= connect_async("wss://cloud.appwrite.io/v1/realtime?project=65d20d389f2b36778b8b&channels[]=documents&channels[]=databases.65d6ca3f97569a4c7ea8.collections").await.expect("Can't connect");

    // // //socket.send(Message::Text("Hello, Test".into())).unwrap();

    // let (mut _write, mut read) = ws_stream.split();

    // if let Some(message) = read.next().await {
    //     let msg = message.expect("Failed to read the message");
    //     let msg = match msg {
    //         Message::Text(s) => s,
    //         _ => {
    //             panic!()
    //         }
    //     };
    //     let parsed: Value = serde_json::from_str(&msg).expect("Unable to parse json");
    //     println!("{:#?}", parsed);
    // }

    // let a = Path::new(r"C:\Users\pc\Pictures\Imagine\6676892.jpg")
    //     .file_name()
    //     .unwrap();
    // println!("{}", format!("{:?}", a));

    Ok(())
}

/// File
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    /// File ID.
    #[serde(rename = "$id")]
    id: Option<String>,
    /// Bucket ID.
    #[serde(rename = "bucketId")]
    bucket_id: String,

    /// File creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    created_at: String,

    /// File update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    updated_at: String,

    /// File permissions. [Learn more about permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions")]
    permissions: Vec<String>,

    /// File name.
    name: String,

    /// File MD5 signature.
    signature: String,

    /// File mime type.
    #[serde(rename = "mimeType")]
    mime_type: String,

    /// File original size in bytes.
    #[serde(rename = "sizeOriginal")]
    size_original: usize,

    /// Total number of chunks available
    #[serde(rename = "chunksTotal")]
    chunks_total: usize,

    /// Total number of chunks uploaded
    #[serde(rename = "chunksUploaded")]
    chunks_uploaded: usize,
}

#[derive(Debug)]
enum AppWriteError {
    //MissingParameter(String),
    //InvalidParameter(String),
    FileSizeError(std::io::Error),
    NetworkError(reqwest::Error),
    InvalidResponse(reqwest::Error),
    AppwriteError(String),
}

impl std::fmt::Display for AppWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
