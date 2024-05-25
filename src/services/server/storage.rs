//! # Storage
//!
//! The Storage service allows you to manage your project files.

use futures_util::Stream;
use reqwest::header;
use serde_json::{json, Value};

use crate::{
    client::Client,
    enumm::HttpMethod,
    enums::{compression::Compression, image_format::ImageFormat, image_gravity::ImageGravity},
    error::Error,
    models::{
        bucket::Bucket, bucket_list::BucketList, file::File, file_list::FileList, UploadType,
    },
    upload_progress::UploadProgress,
    utils::get_content_header_value,
};

pub struct Storage;

impl Storage {
    /// List buckets
    ///
    /// Get a list of all the storage buckets. You can use the query params to
    /// filter your results.
    pub async fn list_buckets(
        client: &Client,
        queries: Option<Vec<String>>,
        search: Option<String>,
    ) -> Result<BucketList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets";

        let mut api_params = serde_json::Map::new();
        if let Some(queries) = queries {
            api_params.insert("queries".to_string(), json!(queries));
        }
        if let Some(search) = search {
            api_params.insert("search".to_string(), json!(search));
        }

        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create bucket
    ///
    /// Create a new storage bucket.
    pub async fn create_bucket(
        client: &Client,
        bucket_id: &str,
        name: &str,
        permissions: Option<Vec<String>>,
        file_security: Option<bool>,
        enabled: Option<bool>,
        maximum_file_size: Option<usize>,
        allowed_file_extensions: Option<Vec<&str>>,
        compression: Option<Compression>,
        encryption: Option<bool>,
        antivirus: Option<bool>,
    ) -> Result<Bucket, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets";

        let mut api_params = serde_json::Map::new();
        api_params.insert("bucketId".to_string(), json!(bucket_id));
        api_params.insert("name".to_string(), json!(name));
        if let Some(permissions) = permissions {
            api_params.insert("permissions".to_string(), json!(permissions));
        }
        if let Some(file_security) = file_security {
            api_params.insert("fileSecurity".to_string(), json!(file_security));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        if let Some(maximum_file_size) = maximum_file_size {
            api_params.insert("maximumFileSize".to_string(), json!(maximum_file_size));
        }
        if let Some(allowed_file_extensions) = allowed_file_extensions {
            api_params.insert(
                "allowedFileExtensions".to_string(),
                json!(allowed_file_extensions),
            );
        }
        if let Some(compression) = compression {
            api_params.insert("compression".to_string(), json!(compression));
        }
        if let Some(encryption) = encryption {
            api_params.insert("encryption".to_string(), json!(encryption));
        }
        if let Some(antivirus) = antivirus {
            api_params.insert("antivirus".to_string(), json!(antivirus));
        }

        let api_params = Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Get bucket
    ///
    /// Get a storage bucket by its unique ID. This endpoint response returns a
    /// JSON object with the storage bucket metadata.
    pub async fn get_bucket(client: &Client, bucket_id: &str) -> Result<Bucket, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}".replace("{bucketId}", bucket_id);

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

    /// Update bucket
    ///
    /// Update a storage bucket by its unique ID.
    pub async fn update_bucket(
        client: &Client,
        bucket_id: &str,
        name: &str,
        permissions: Option<Vec<String>>,
        file_security: Option<bool>,
        enabled: Option<bool>,
        maximum_file_size: Option<usize>,
        allowed_file_extensions: Option<Vec<&str>>,
        compression: Option<Compression>,
        encryption: Option<bool>,
        antivirus: Option<bool>,
    ) -> Result<Bucket, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}".replace("{bucketId}", bucket_id);

        let mut api_params = serde_json::Map::new();
        api_params.insert("name".to_string(), json!(name));
        if let Some(permissions) = permissions {
            api_params.insert("permissions".to_string(), json!(permissions));
        }
        if let Some(file_security) = file_security {
            api_params.insert("fileSecurity".to_string(), json!(file_security));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        if let Some(maximum_file_size) = maximum_file_size {
            api_params.insert("maximumFileSize".to_string(), json!(maximum_file_size));
        }
        if let Some(allowed_file_extensions) = allowed_file_extensions {
            api_params.insert(
                "allowedFileExtensions".to_string(),
                json!(allowed_file_extensions),
            );
        }
        if let Some(compression) = compression {
            api_params.insert("compression".to_string(), json!(compression));
        }
        if let Some(encryption) = encryption {
            api_params.insert("encryption".to_string(), json!(encryption));
        }
        if let Some(antivirus) = antivirus {
            api_params.insert("antivirus".to_string(), json!(antivirus));
        }

        let api_params = Value::Object(api_params);

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

    /// Delete bucket
    ///
    /// Delete a storage bucket by its unique ID.
    pub async fn delete_bucket(client: &Client, bucket_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}".replace("{bucketId}", bucket_id);

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

    /// List files
    ///
    /// Get a list of all the user files. You can use the query params to filter
    /// your results.
    pub async fn list_files(
        client: &Client,
        bucket_id: &str,
        queries: Option<Vec<String>>,
        search: Option<String>,
    ) -> Result<FileList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}/files".replace("{bucketId}", bucket_id);

        let mut api_params = serde_json::Map::new();
        if let Some(queries) = queries {
            api_params.insert("queries".to_string(), json!(queries));
        }
        if let Some(search) = search {
            api_params.insert("search".to_string(), json!(search));
        }

        let api_params = serde_json::Value::Object(api_params);

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

    /// Create file
    ///
    /// Create a new file. Before using this route, you should create a new bucket
    /// resource using either a [server
    /// integration](https://appwrite.io/docs/server/storage#storageCreateBucket)
    /// API or directly from your Appwrite console.
    ///
    /// Larger files should be uploaded using multiple requests with the
    /// [content-range](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Range)
    /// header to send a partial request with a maximum supported chunk of `5MB`.
    /// The `content-range` header values should always be in bytes.
    ///
    /// When the first request is sent, the server will return the **File** object,
    /// and the subsequent part request must include the file"s **id** in
    /// `x-appwrite-id` header to allow the server to know that the partial upload
    /// is for the existing file and not for a new one.
    ///
    /// If you"re creating a new file using one of the Appwrite SDKs, all the
    /// chunking logic will be managed by the SDK internally.
    ///
    pub async fn create_files(
        client: &Client,
        bucket_id: &str,
        file_id: &str,
        file_path: &str,
        file_name: String,
        permissions: Option<Vec<String>>,
    ) -> Result<File, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}/files".replace("{bucketId}", bucket_id);

        let mut api_params = serde_json::Map::new();
        if let Some(permissions) = permissions {
            api_params.insert("permissions".to_string(), json!(permissions));
        }

        let api_params = serde_json::Value::Object(api_params);

        let res: UploadType = client
            .chunk_upload_file(
                file_path,
                api_path.as_str(),
                String::from(file_id),
                &api_params,
                file_name,
                true,
            )
            .await?;

        match res {
            UploadType::File(res) => Ok(res),
            UploadType::Deployment(_) => Err(Error::WrongUploadType),
        }
    }

    /// Create file Streamed
    ///
    /// Create a new file. Before using this route, you should create a new bucket
    /// resource using either a [server
    /// integration](https://appwrite.io/docs/server/storage#storageCreateBucket)
    /// API or directly from your Appwrite console.
    ///
    /// Larger files should be uploaded using multiple requests with the
    /// [content-range](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Range)
    /// header to send a partial request with a maximum supported chunk of `5MB`.
    /// The `content-range` header values should always be in bytes.
    ///
    /// When the first request is sent, the server will return the **File** object,
    /// and the subsequent part request must include the file"s **id** in
    /// `x-appwrite-id` header to allow the server to know that the partial upload
    /// is for the existing file and not for a new one.
    ///
    /// If you"re creating a new file using one of the Appwrite SDKs, all the
    /// chunking logic will be managed by the SDK internally.
    ///
    pub async fn create_files_streamed<'a>(
        client: &'a Client,
        bucket_id: &'a str,
        file_id: &'a str,
        file_path: &'a str,
        file_name: String,
        permissions: Option<Vec<String>>,
    ) -> impl Stream<Item = Result<(UploadType, UploadProgress), Error>> + 'a {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}/files".replace("{bucketId}", bucket_id);

        let mut api_params = serde_json::Map::new();
        if let Some(permissions) = permissions {
            api_params.insert("permissions".to_string(), json!(permissions));
        }

        let api_params = serde_json::Value::Object(api_params);

        client
            .chunk_upload_file_streamed(
                file_path,
                api_path,
                String::from(file_id),
                api_params,
                file_name,
                true,
            )
            .await
    }

    /// Get file
    ///
    /// Get a file by its unique ID. This endpoint response returns a JSON object
    /// with the file metadata.
    pub async fn get_file(client: &Client, bucket_id: &str, file_id: &str) -> Result<File, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}/files/{fileId}"
            .replace("{bucketId}", bucket_id)
            .replace("{fileId}", file_id);

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

    /// Update file
    ///
    /// Update a file by its unique ID. Only users with write permissions have
    /// access to update this resource.
    pub async fn update_file(
        client: &Client,
        bucket_id: &str,
        file_id: &str,
        name: Option<&str>,
        permissions: Option<Vec<String>>,
    ) -> Result<File, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}/files/{fileId}"
            .replace("{bucketId}", bucket_id)
            .replace("{fileId}", file_id);

        let mut api_params = serde_json::Map::new();
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }
        if let Some(permissions) = permissions {
            api_params.insert("permissions".to_string(), json!(permissions));
        }

        let api_params = serde_json::Value::Object(api_params);

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

    /// Delete File
    ///
    /// Delete a file by its unique ID. Only users with write permissions have
    /// access to delete this resource.
    pub async fn delete_file(client: &Client, bucket_id: &str, file_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}/files/{fileId}"
            .replace("{bucketId}", bucket_id)
            .replace("{fileId}", file_id);

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

    /// Get file for download
    ///
    /// Get a file content by its unique ID. The endpoint response return with a
    /// "Content-Disposition: attachment" header that tells the browser to start
    /// downloading the file to user downloads directory.
    pub async fn get_file_download(
        client: &Client,
        bucket_id: &str,
        file_id: &str,
    ) -> Result<Vec<u8>, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}/files/{fileId}/download"
            .replace("{bucketId}", bucket_id)
            .replace("{fileId}", file_id);

        let api_params = serde_json::json!({
            "project":get_content_header_value(&client, "project"),
            "key":get_content_header_value(&client, "key"),
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

        Ok(res.bytes().await?.to_vec())
    }

    /// Get file preview
    ///
    /// Get a file preview image. Currently, this method supports preview for image
    /// files (jpg, png, and gif), other supported formats, like pdf, docs, slides,
    /// and spreadsheets, will return the file icon image. You can also pass query
    /// string arguments for cutting and resizing your preview image. Preview is
    /// supported only for image files smaller than 10MB.
    pub async fn get_file_preview(
        client: &Client,
        bucket_id: &str,
        file_id: &str,
        width: Option<usize>,
        height: Option<usize>,
        gravity: Option<ImageGravity>,
        quality: Option<usize>,
        border_width: Option<usize>,
        border_color: Option<&str>,
        border_radius: Option<usize>,
        opacity: Option<f32>,
        rotation: Option<usize>,
        background: Option<&str>,
        output: Option<ImageFormat>,
    ) -> Result<Vec<u8>, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}/files/{fileId}/preview"
            .replace("{bucketId}", bucket_id)
            .replace("{fileId}", file_id);

        let mut api_params = serde_json::Map::new();

        if let Some(width) = width {
            api_params.insert("width".to_string(), json!(width));
        }
        if let Some(height) = height {
            api_params.insert("height".to_string(), json!(height));
        }
        if let Some(gravity) = gravity {
            api_params.insert("gravity".to_string(), json!(gravity));
        }
        if let Some(quality) = quality {
            api_params.insert("quality".to_string(), json!(quality));
        }
        if let Some(border_width) = border_width {
            api_params.insert("borderWidth".to_string(), json!(border_width));
        }
        if let Some(border_color) = border_color {
            api_params.insert("borderColor".to_string(), json!(border_color));
        }
        if let Some(border_radius) = border_radius {
            api_params.insert("borderRadius".to_string(), json!(border_radius));
        }
        if let Some(opacity) = opacity {
            api_params.insert("opacity".to_string(), json!(opacity));
        }
        if let Some(rotation) = rotation {
            api_params.insert("rotation".to_string(), json!(rotation));
        }
        if let Some(background) = background {
            api_params.insert("background".to_string(), json!(background));
        }
        if let Some(output) = output {
            api_params.insert("output".to_string(), json!(output));
        }

        api_params.insert(
            "project".to_string(),
            json!(get_content_header_value(&client, "project")),
        );
        api_params.insert(
            "key".to_string(),
            json!(get_content_header_value(&client, "key")),
        );
        let api_params = serde_json::Value::Object(api_params);

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

        Ok(res.bytes().await?.to_vec())
    }

    /// Get file for view
    ///
    /// Get a file content by its unique ID. This endpoint is similar to the
    /// download method but returns with no  'Content-Disposition: attachment'
    /// header.
    pub async fn get_file_view(
        client: &Client,
        bucket_id: &str,
        file_id: &str,
    ) -> Result<Vec<u8>, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}/files/{fileId}/view"
            .replace("{bucketId}", bucket_id)
            .replace("{fileId}", file_id);

        let api_params = serde_json::json!({
            "project":get_content_header_value(&client, "project"),
            "key":get_content_header_value(&client, "key"),
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

        Ok(res.bytes().await?.to_vec())
    }
}
