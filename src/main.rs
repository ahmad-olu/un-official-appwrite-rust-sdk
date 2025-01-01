use std::{
    collections::{BTreeMap, HashMap},
    fs, thread,
    time::Duration,
};

use std::sync::mpsc::{self, Sender};

use futures::{pin_mut, FutureExt, Stream, StreamExt};
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    multipart::{self, Part},
    StatusCode,
};
use serde_json::{json, Value};
use tokio::task;
// use futures_util::{pin_mut, StreamExt};
// use std::sync::{Arc, Mutex};
// use tokio::task;
use unofficial_appwrite::{
    client::{Client, ClientBuilder},
    error::{AppWriteError, Error},
    id::ID,
    query::Query as q,
    query_value::Query,
    services::server::storage::Storage,
};
use uuid::Uuid;

async fn upload_small_image(
    file_path: &str,
    bucket_id: &str,
    file_name: String,
) -> Result<(), Error> {
    let api_path = "/storage/buckets/{bucketId}/files".replace("{bucketId}", bucket_id);
    let file_id = ID::unique(7); //`unique()` => pass unique for bigger file
    let params: HashMap<String, Value> = HashMap::new();

    let boundary = Uuid::new_v4();
    let file = match fs::read(file_path) {
        Ok(size) => size,
        Err(err) => return Err(Error::Io(err)),
    };
    let file_size = file.len();

    let uri = api_path;

    let mut headers = HeaderMap::new();
    headers.insert(
        "x-appwrite-Project",
        HeaderValue::from_str("676c2b7b000c834e1fce")?,
    );

    headers.insert("x-appwrite-key", HeaderValue::from_str("standard_5d84014ebaf0de52308eff28946a43062921240c10b81c2fd037ab60b02f0257b7f0a53fe94065170fe7c7d0af2d4136d4cbf32a4055baeada3d27f2e323b70aeda87e97f676207cf10cbb18b7a80f8d1103803617454c89138f217dad701bbe9dc6950bc58853fdb2a0b4b67d2a8b8b6b7b9b2e6d9b94e0a2fcfee794688e2e")?);

    if file_size <= (5 * 1024 * 1024) {
        let part = Part::bytes(file).file_name(file_name);

        let form = multipart::Form::new()
            .text("fileId", file_id.clone())
            .part("file", part);
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str(format!("multipart/form-data; boundary={}", boundary).as_str())?,
        );

        let res = reqwest::Client::new()
            .post(format!("{}{}", "http://127.0.0.1/v1", uri))
            .json(&params)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;
        println!("{:#?}", &res.text().await);
    }
    //     Ok(
    //     "{\"$id\":\"677409feb934023dd953\",\"bucketId\":\"6773f8af000602e81619\",\"$createdAt\":\"2024-12-31T15:13:02.798+00:00\",\"$updatedAt\":\"2024-12-31T15:13:02.798+00:00\",\"$permissions\":[],\"name\":\"felix.mp3\",\"signature\":\"e1a44202e06d06e6831d34c36c124b10\",\"mimeType\":\"audio\\/mpeg\",\"sizeOriginal\":4040584,\"chunksTotal\":1,\"chunksUploaded\":1}",
    // )
    Ok(())
}

async fn upload_large_image(
    file_path: &str,
    bucket_id: &str,
    file_name: String,
) -> Result<(), Error> {
    let chunk_size = 5 * 1024 * 1024;
    let mut headers_main = HeaderMap::new();
    headers_main.insert(
        "x-appwrite-Project",
        HeaderValue::from_str("676c2b7b000c834e1fce")?,
    );

    headers_main.insert("x-appwrite-key", HeaderValue::from_str("standard_5d84014ebaf0de52308eff28946a43062921240c10b81c2fd037ab60b02f0257b7f0a53fe94065170fe7c7d0af2d4136d4cbf32a4055baeada3d27f2e323b70aeda87e97f676207cf10cbb18b7a80f8d1103803617454c89138f217dad701bbe9dc6950bc58853fdb2a0b4b67d2a8b8b6b7b9b2e6d9b94e0a2fcfee794688e2e")?);
    let api_path = "/storage/buckets/{bucketId}/files".replace("{bucketId}", bucket_id);
    let file_id = ID::unique(7);
    let params: HashMap<String, Value> = HashMap::new();

    let boundary = Uuid::new_v4();
    let file = match fs::read(file_path) {
        Ok(size) => size,
        Err(err) => return Err(Error::Io(err)),
    };
    let file_size = file.len();

    let uri = api_path;

    if file_size <= chunk_size {
        let part = Part::bytes(file.clone()).file_name(file_name.clone());

        let form = multipart::Form::new()
            .text("fileId", file_id.clone())
            .part("file", part);
        headers_main.insert(
            CONTENT_TYPE,
            HeaderValue::from_str(format!("multipart/form-data; boundary={}", boundary).as_str())?,
        );

        let res = reqwest::Client::new()
            .post(format!("{}{}", "http://127.0.0.1/v1", uri))
            .json(&params)
            .headers(headers_main.clone())
            .multipart(form)
            .send()
            .await?;
        println!("{:#?}", &res.text().await);
    }

    let mut offset: usize = 0;

    let mut first_upload = true; // Track if it's the first upload for x-appwrite-id
    let mut x_appwrite_id: Option<String> = None;

    if file_id != "unique()" {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str(format!("multipart/form-data; boundary={}", boundary).as_str())?,
        );
        let params: HashMap<String, Value> = HashMap::new();

        let res = reqwest::Client::new()
            .get(format!("{}{}/{}", "http://127.0.0.1/v1", uri, file_id))
            .json(&params)
            .headers(headers)
            .headers(headers_main.clone())
            .send()
            .await?;
        let a: Value = serde_json::from_str(&res.text().await.unwrap()).expect("invalid json");
        if let Some(chunks_uploaded) = a.get("chunksUploaded").and_then(|v| v.as_i64()) {
            println!("chunksUploaded: {}", chunks_uploaded);
            offset = (chunks_uploaded as usize) * chunk_size
        }
    }
    while offset < file_size {
        let end = std::cmp::min(offset + chunk_size, file_size);
        let chunk = &file[offset..end];
        let content_range = format!("bytes {}-{}/{}", offset, end - 1, file_size);

        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str(format!("multipart/form-data; boundary={}", boundary).as_str())?,
        );
        headers.insert(
            "Content-Range",
            HeaderValue::from_str(content_range.as_str())?,
        );
        let mut chunk_form = multipart::Form::new();
        chunk_form = chunk_form.text("fileId", file_id.clone()).part(
            "file",
            Part::bytes(chunk.to_vec()).file_name(file_name.clone()),
        );
        if !first_upload {
            headers.insert(
                "x-appwrite-id",
                x_appwrite_id
                    .clone()
                    .ok_or(Error::Custom("Failed to clone appwrite id".to_string()))?
                    .as_str()
                    .parse()
                    .map_err(|_a| {
                        Error::Custom(
                            "Unable to parse string slice into value header [invalid header value]"
                                .to_string(),
                        )
                    })?,
            );
        }

        let response = reqwest::Client::new()
            .post(format!("{}{}", "http://127.0.0.1/v1", uri))
            .json(&params)
            .headers(headers)
            .headers(headers_main.clone())
            .multipart(chunk_form)
            .send()
            .await?;

        if response.status() != StatusCode::CREATED {
            let err = response.json::<AppWriteError>().await?;
            return Err(Error::AppWriteError {
                message: err.message,
                code: err.code,
                response: err.response,
                error_type: err.error_type,
            });
        }
        let file: Value =
            serde_json::from_str(&response.text().await.unwrap()).expect("invalid json");
        if let Some(id) = file.get("$id").and_then(|v| v.as_str()) {
            x_appwrite_id = Some(id.to_string());
            first_upload = false;
        }
        println!("{:#?}", file);

        offset += chunk_size;
    }
    //     Object {
    //     "$createdAt": String("2024-12-31T16:26:28.501+00:00"),
    //     "$id": String("67741b3467bdbYvWzv5s"),
    //     "$permissions": Array [],
    //     "$updatedAt": String("2024-12-31T16:26:28.580+00:00"),
    //     "bucketId": String("6773f8af000602e81619"),
    //     "chunksTotal": Number(3),
    //     "chunksUploaded": Number(2),
    //     "mimeType": String(""),
    //     "name": String("Q.mp4"),
    //     "signature": String(""),
    //     "sizeOriginal": Number(14486511),
    // }
    // Object {
    //     "$createdAt": String("2024-12-31T16:26:28.501+00:00"),
    //     "$id": String("67741b3467bdbYvWzv5s"),
    //     "$permissions": Array [],
    //     "$updatedAt": String("2024-12-31T16:26:28.720+00:00"),
    //     "bucketId": String("6773f8af000602e81619"),
    //     "chunksTotal": Number(3),
    //     "chunksUploaded": Number(3),
    //     "mimeType": String("video/mp4"),
    //     "name": String("Q.mp4"),
    //     "signature": String("19fe37e26a21419aa269ced0a4b19075"),
    //     "sizeOriginal": Number(14486511),
    // }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct ChunkProgress {
    pub chunks_uploaded: u64,
    pub chunks_total: u64,
    pub size_uploaded: usize,
    pub progress: f64,
    pub id: String,
}

async fn upload_large_callback_image<F>(
    file_path: &str,
    bucket_id: &str,
    file_name: String,
    mut on_progress: F,
) -> Result<(), Error>
where
    F: FnMut(ChunkProgress) + Send + 'static,
{
    let chunk_size = 5 * 1024 * 1024;
    let mut headers_main = HeaderMap::new();
    headers_main.insert(
        "x-appwrite-Project",
        HeaderValue::from_str("676c2b7b000c834e1fce")?,
    );

    headers_main.insert("x-appwrite-key", HeaderValue::from_str("standard_5d84014ebaf0de52308eff28946a43062921240c10b81c2fd037ab60b02f0257b7f0a53fe94065170fe7c7d0af2d4136d4cbf32a4055baeada3d27f2e323b70aeda87e97f676207cf10cbb18b7a80f8d1103803617454c89138f217dad701bbe9dc6950bc58853fdb2a0b4b67d2a8b8b6b7b9b2e6d9b94e0a2fcfee794688e2e")?);
    let api_path = "/storage/buckets/{bucketId}/files".replace("{bucketId}", bucket_id);
    let file_id = ID::unique(7);
    let params: HashMap<String, Value> = HashMap::new();

    let boundary = Uuid::new_v4();
    let file = match fs::read(file_path) {
        Ok(size) => size,
        Err(err) => return Err(Error::Io(err)),
    };
    let file_size = file.len();

    let uri = api_path;

    if file_size <= chunk_size {
        let part = Part::bytes(file.clone()).file_name(file_name.clone());

        let form = multipart::Form::new()
            .text("fileId", file_id.clone())
            .part("file", part);
        headers_main.insert(
            CONTENT_TYPE,
            HeaderValue::from_str(format!("multipart/form-data; boundary={}", boundary).as_str())?,
        );

        let res = reqwest::Client::new()
            .post(format!("{}{}", "http://127.0.0.1/v1", uri))
            .json(&params)
            .headers(headers_main.clone())
            .multipart(form)
            .send()
            .await?;
        println!("{:#?}", &res.text().await);
    }

    let mut offset: usize = 0;

    let mut first_upload = true; // Track if it's the first upload for x-appwrite-id
    let mut x_appwrite_id: Option<String> = None;

    if file_id != "unique()" {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str(format!("multipart/form-data; boundary={}", boundary).as_str())?,
        );
        let params: HashMap<String, Value> = HashMap::new();

        let res = reqwest::Client::new()
            .get(format!("{}{}/{}", "http://127.0.0.1/v1", uri, file_id))
            .json(&params)
            .headers(headers)
            .headers(headers_main.clone())
            .send()
            .await?;
        let a: Value = serde_json::from_str(&res.text().await.unwrap()).expect("invalid json");
        if let Some(chunks_uploaded) = a.get("chunksUploaded").and_then(|v| v.as_i64()) {
            println!("chunksUploaded: {}", chunks_uploaded);
            offset = (chunks_uploaded as usize) * chunk_size
        }
    }
    while offset < file_size {
        let end = std::cmp::min(offset + chunk_size, file_size);
        let chunk = &file[offset..end];
        let content_range = format!("bytes {}-{}/{}", offset, end - 1, file_size);

        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str(format!("multipart/form-data; boundary={}", boundary).as_str())?,
        );
        headers.insert(
            "Content-Range",
            HeaderValue::from_str(content_range.as_str())?,
        );
        let mut chunk_form = multipart::Form::new();
        chunk_form = chunk_form.text("fileId", file_id.clone()).part(
            "file",
            Part::bytes(chunk.to_vec()).file_name(file_name.clone()),
        );
        if !first_upload {
            headers.insert(
                "x-appwrite-id",
                x_appwrite_id
                    .clone()
                    .ok_or(Error::Custom("Failed to clone appwrite id".to_string()))?
                    .as_str()
                    .parse()
                    .map_err(|_a| {
                        Error::Custom(
                            "Unable to parse string slice into value header [invalid header value]"
                                .to_string(),
                        )
                    })?,
            );
        }

        let response = reqwest::Client::new()
            .post(format!("{}{}", "http://127.0.0.1/v1", uri))
            .json(&params)
            .headers(headers)
            .headers(headers_main.clone())
            .multipart(chunk_form)
            .send()
            .await?;

        if response.status() != StatusCode::CREATED {
            let err = response.json::<AppWriteError>().await?;
            return Err(Error::AppWriteError {
                message: err.message,
                code: err.code,
                response: err.response,
                error_type: err.error_type,
            });
        }
        let file: Value =
            serde_json::from_str(&response.text().await.unwrap()).expect("invalid json");
        if let Some(id) = file.get("$id").and_then(|v| v.as_str()) {
            x_appwrite_id = Some(id.to_string());
            first_upload = false;
        }
        offset += chunk_size;
        let size_uploaded = std::cmp::min(offset, file_size);
        let progress = size_uploaded as f64 / file_size as f64;
        let chunks_total = file.get("chunksTotal").and_then(|v| v.as_u64()).unwrap();
        let chunks_uploaded = file.get("chunksUploaded").and_then(|v| v.as_u64()).unwrap();
        let id = file.get("$id").and_then(|v| v.as_str()).unwrap();

        on_progress(ChunkProgress {
            chunks_uploaded,
            chunks_total,
            size_uploaded,
            progress,
            id: id.to_string(),
        });
    }
    Ok(())

    // !-> how to use
    // let progress_callback = |progress: ChunkProgress| {
    //     println!(
    //         "Uploaded: {}/{} ({}%), ID: {}",
    //         progress.size_uploaded,
    //         (progress.chunks_total as usize) * progress.size_uploaded, // Approximate total size
    //         (progress.progress * 100.0).round(),
    //         progress.id,
    //     );
    // };
    // upload_large_callback_image(
    //     file_path,
    //     "6773f8af000602e81619",
    //     file_name.to_string(),
    //     progress_callback,
    // )
    // .await?;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let file_path = "Q.mp4";
    let file_name = "Q.mp4";

    let client = ClientBuilder::default()
            .set_endpoint("http://127.0.0.1/v1")?
            .set_project("676c2b7b000c834e1fce")?
            .set_key("standard_5d84014ebaf0de52308eff28946a43062921240c10b81c2fd037ab60b02f0257b7f0a53fe94065170fe7c7d0af2d4136d4cbf32a4055baeada3d27f2e323b70aeda87e97f676207cf10cbb18b7a80f8d1103803617454c89138f217dad701bbe9dc6950bc58853fdb2a0b4b67d2a8b8b6b7b9b2e6d9b94e0a2fcfee794688e2e")?
            //.set_self_signed(false)?
            .build()?;

    let id = ID::unique(7);
    let a = Storage::create_files(
        &client,
        "6773f8af000602e81619",
        &id,
        file_path,
        file_name.to_string(),
        HashMap::<String, Value>::new(),
    )
    .await?;
    dbg!(a);

    // pin_mut!(a);
    // while let Some(data) = a.next().await {
    //     let res = data.unwrap();
    //     let file = res.0;
    //     println!("==>{:?}===>{:?}", file, res.1);
    // }

    // upload_small_image(file_path, "6773f8af000602e81619", file_name.to_string()).await?;

    // let mut a: BTreeMap<String, Value> = BTreeMap::new();

    // a.insert("email".into(), "fakeEmail@Email.com".into());
    // a.insert("password".into(), "VeryVerySecurePassword@123456789".into());
    // a.insert("userId".into(), "unique()".into());
    // dbg!(a);

    // let client = ClientBuilder::default()
    //     .set_project("65d20d....")?
    //     .set_key("ae07b88634eacfb42a2fc4c4a7f278d96....")?
    //     //.set_self_signed(false)?
    //     .build()?;

    // let prog = Arc::new(Mutex::new(0));

    // // Clone the Arc for the async task
    // let prog_clone = Arc::clone(&prog);

    // task::spawn(async move {
    //     let stream = Storage::create_files_streamed(
    //         &client,
    //         "65d20d5c8096032a03cd",
    //         ID::unique(),
    //         r"c:\Users\pc\Downloads\Video\New folder (2)\Folder 1\NGINX_101_SecretHeart.pdf",
    //         String::from("NGINX_101_SecretHeart.pdf"),
    //         None,
    //     )
    //     .await;
    //     pin_mut!(stream);

    //     while let Some(data) = stream.next().await {
    //         match data {
    //             Ok((_file, progress)) => {
    //                 // let res = data.unwrap();
    //                 // let file = res.0;
    //                 // println!("==>{:?}===>{:?}", file, res.1);

    //                 // Update the shared state
    //                 let mut prog = prog_clone.lock().unwrap();
    //                 *prog = progress.progress;
    //             }
    //             Err(e) => {
    //                 eprintln!("Stream error: {:?}", e);
    //                 break;
    //             }
    //         }
    //     }
    // });
    // let prog = prog.lock().unwrap();
    // println!("Progress: {}", *prog);

    // let a = app_json_header!();
    // println!("{:?}", a);

    Ok(())
}

// cd /home/r2/Documents/playground/docker/appwrite && \
// docker compose up -d --remove-orphans
