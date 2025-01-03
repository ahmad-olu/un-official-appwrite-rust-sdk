//! # Storage
//!
//! The Storage service allows you to manage your project files.

use std::collections::HashMap;

use serde_json::Value;

use crate::{
    app_json_header,
    client::{ChunkProgress, Client},
    enumm::HttpMethod,
    error::Error,
    models::{
        bucket::Bucket, bucket_list::BucketList, file::File, file_list::FileList, UploadType,
    },
    utils::get_content_header_value,
};

pub struct Storage;

impl Storage {
    /// List buckets
    ///
    /// Get a list of all the storage buckets. You can use the query params to
    /// filter your results.
    ///* queries => vec(string)?
    ///* search => string?
    pub async fn list_buckets(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<BucketList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path, api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create bucket
    ///
    /// Create a new storage bucket.
    ///* bucketId => string
    ///* name => string
    ///* permissions => vec(string)?
    ///* fileSecurity => bool?
    ///* enabled => bool?
    ///* maximumFileSize => number?
    ///* allowedFileExtensions => vec(string)?
    ///* compression => Compression?
    ///* encryption => bool?
    ///* antivirus => bool?
    pub async fn create_bucket(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<Bucket, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &args, None)
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update bucket
    ///
    /// Update a storage bucket by its unique ID.
    ///* name => string?
    ///* permissions => vec(string)?
    ///* fileSecurity => bool?
    ///* enabled => bool?
    ///* maximumFileSize => number?
    ///* allowedFileExtensions => vec(string)?
    ///* compression => Compression?
    ///* encryption => bool?
    ///* antivirus => bool?
    pub async fn update_bucket(
        client: &Client,
        bucket_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Bucket, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}".replace("{bucketId}", bucket_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PUT, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Delete bucket
    ///
    /// Delete a storage bucket by its unique ID.
    pub async fn delete_bucket(client: &Client, bucket_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}".replace("{bucketId}", bucket_id);

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

    /// List files
    ///
    /// Get a list of all the user files. You can use the query params to filter
    /// your results.
    ///* queries => vec(string)?
    ///* search => string?
    pub async fn list_files(
        client: &Client,
        bucket_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<FileList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}/files".replace("{bucketId}", bucket_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
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
    ///* permissions => vec(string)?
    pub async fn create_files<F>(
        client: &Client,
        bucket_id: String,
        file_id: String,
        file_path: String,
        file_name: String,
        args: HashMap<String, Value>,
        on_progress: F,
    ) -> Result<File, Error>
    where
        F: FnMut(ChunkProgress) + Send + 'static,
    {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}/files".replace("{bucketId}", &bucket_id);

        let res: UploadType = client
            .chunk_upload_file(
                file_path,
                api_path,
                file_id,
                args,
                file_name,
                true,
                on_progress,
            )
            .await?;

        match res {
            UploadType::File(res) => Ok(res),
            UploadType::Deployment(_) => Err(Error::WrongUploadType),
        }
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update file
    ///
    /// Update a file by its unique ID. Only users with write permissions have
    /// access to update this resource.
    ///* permissions => vec(string)?
    ///* name => string?
    pub async fn update_file(
        client: &Client,
        bucket_id: &str,
        file_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<File, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}/files/{fileId}"
            .replace("{bucketId}", bucket_id)
            .replace("{fileId}", file_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PUT, api_path.as_str(), api_headers, &args, None)
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

    /// Get file for download
    ///
    /// Get a file content by its unique ID. The endpoint response return with a
    /// "Content-Disposition: attachment" header that tells the browser to start
    /// downloading the file to user downloads directory.
    pub async fn get_file_download(
        client: &Client,
        bucket_id: &str,
        file_id: &str,
        mut args: HashMap<String, Value>,
    ) -> Result<Vec<u8>, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}/files/{fileId}/download"
            .replace("{bucketId}", bucket_id)
            .replace("{fileId}", file_id);

        args.insert(
            "project".into(),
            get_content_header_value(&client, "project").into(),
        );
        args.insert(
            "key".into(),
            get_content_header_value(&client, "key").into(),
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
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
    ///* width => number?
    ///* height => number?
    ///* gravity => ImageGravity?
    ///* quality => number?
    ///* borderWidth => number?
    ///* borderColor => string?
    ///* borderRadius => number?
    ///* opacity => float?
    ///* background => string?
    ///* output => ImageFormat?
    pub async fn get_file_preview(
        client: &Client,
        bucket_id: &str,
        file_id: &str,
        mut args: HashMap<String, Value>,
    ) -> Result<Vec<u8>, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}/files/{fileId}/preview"
            .replace("{bucketId}", bucket_id)
            .replace("{fileId}", file_id);

        args.insert(
            "project".into(),
            get_content_header_value(&client, "project").into(),
        );
        args.insert(
            "key".into(),
            get_content_header_value(&client, "key").into(),
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
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
        mut args: HashMap<String, Value>,
    ) -> Result<Vec<u8>, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/storage/buckets/{bucketId}/files/{fileId}/view"
            .replace("{bucketId}", bucket_id)
            .replace("{fileId}", file_id);

        args.insert(
            "project".into(),
            get_content_header_value(&client, "project").into(),
        );
        args.insert(
            "key".into(),
            get_content_header_value(&client, "key").into(),
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.bytes().await?.to_vec())
    }
}
