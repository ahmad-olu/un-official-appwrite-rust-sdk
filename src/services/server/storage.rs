//! # Storage
//!
//! The Storage service allows you to manage your project files.

use futures_util::Stream;

use crate::{
    api_params, app_json_header,
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

        let api_params = api_params!(
            "queries"=> queries,
            "search"=> search,
        );

        let api_headers = app_json_header!();

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

        let api_params = api_params!(
            "bucketId"=> Some(bucket_id),
            "name"=> Some(name),
            "permissions"=> permissions,
            "fileSecurity"=> file_security,
            "enabled"=> enabled,
            "maximumFileSize"=> maximum_file_size,
            "allowedFileExtensions"=> allowed_file_extensions,
            "compression"=> compression,
            "encryption"=> encryption,
            "antivirus"=> antivirus,
        );

        let api_headers = app_json_header!();

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

        let api_params = api_params!();

        let api_headers = app_json_header!();

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

        let api_params = api_params!(
            "name"=> Some(name),
            "permissions"=> permissions,
            "fileSecurity"=> file_security,
            "enabled"=> enabled,
            "maximumFileSize"=> maximum_file_size,
            "allowedFileExtensions"=> allowed_file_extensions,
            "compression"=> compression,
            "encryption"=> encryption,
            "antivirus"=> antivirus,
        );

        let api_headers = app_json_header!();

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

        let api_params = api_params!();

        let api_headers = app_json_header!();

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

        let api_params = api_params!(
            "queries"=> queries,
            "search"=> search,
        );

        let api_headers = app_json_header!();

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

        let api_params = api_params!(
            "permissions"=> permissions,
        );

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

        let api_params = api_params!(
            "permissions"=> permissions,
        );

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

        let api_params = api_params!();

        let api_headers = app_json_header!();

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

        let api_params = api_params!(
            "name"=> name,
            "permissions"=> permissions,
        );

        let api_headers = app_json_header!();

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

        let api_params = api_params!();

        let api_headers = app_json_header!();

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

        let api_params = api_params!(
            "project"=>get_content_header_value(&client, "project"),
            "key"=>get_content_header_value(&client, "key"),
        );

        let api_headers = app_json_header!();

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

        let api_params = api_params!(
            "width"=>width,
            "height"=> height,
            "gravity"=> gravity,
            "quality"=> quality,
            "borderWidth"=> border_width,
            "borderColor"=> border_color,
            "borderRadius"=> border_radius,
            "opacity"=> opacity,
            "rotation"=> rotation,
            "background"=> background,
            "output"=> output,
            "project"=>get_content_header_value(&client, "project"),
            "key"=>get_content_header_value(&client, "key"),
        );

        let api_headers = app_json_header!();

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

        let api_params = api_params!(
            "project"=>get_content_header_value(&client, "project"),
            "key"=>get_content_header_value(&client, "key"),
        );

        let api_headers = app_json_header!();

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
