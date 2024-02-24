use std::{fs, path::Path, str::FromStr};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    multipart::{self, Form, Part},
    Response, StatusCode,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    enums::HttpMethod,
    error::{AppWriteError, Error},
    models::file::File,
    upload_progress::UploadProgress,
};

#[derive(Debug, Clone)]
pub struct Client {
    end_point: String,
    end_point_realtime: Option<String>,
    pub header: HeaderMap,
    chunk_size: usize,
    self_signed: bool,
}

impl Client {
    pub fn new() -> Self {
        let mut header = HeaderMap::new();
        Self {
            end_point: String::from("https://cloud.appwrite.io/v1"),
            end_point_realtime: None,
            header,
            chunk_size: 5 * 1024 * 1024,
            self_signed: false,
        }
    }
    pub fn set_self_signed(&mut self, status: bool) -> &mut Self {
        self.self_signed = status;
        self
    }
    fn set_endpoint(&mut self, endpoint: &str) -> &mut Self {
        self.end_point = String::from(endpoint);
        self
    }
    fn add_header(&mut self, key: &str, value: &str) -> Result<&mut Self, Error> {
        self.header
            .insert(HeaderName::from_str(key)?, HeaderValue::from_str(value)?);
        Ok(self)
    }

    fn set_project(&mut self, value: &str) -> Result<&mut Self, Error> {
        self.header
            .insert("X-Appwrite-Project", HeaderValue::from_str(value)?);
        Ok(self)
    }

    fn set_key(&mut self, value: &str) -> Result<&mut Self, Error> {
        self.header
            .insert("x-appwrite-key", HeaderValue::from_str(value)?);
        Ok(self)
    }

    fn set_jwt(&mut self, value: &str) -> Result<&mut Self, Error> {
        self.header
            .insert("x-appwrite-jwt", HeaderValue::from_str(value)?);
        Ok(self)
    }

    fn set_locale(&mut self, value: &str) -> Result<&mut Self, Error> {
        self.header
            .insert("x-appwrite-locale", HeaderValue::from_str(value)?);
        Ok(self)
    }

    pub async fn call<T: Serialize + ?Sized>(
        &self,
        method: HttpMethod,
        path: &str,
        headers: HeaderMap,
        params: &T,
        form: Option<Form>,
    ) -> Result<Response, Error> {
        let res = reqwest::Client::new();
        let res = match method {
            HttpMethod::GET => res.get(format!("{}{}", self.end_point, path)),
            HttpMethod::POST => res.post(format!("{}{}", self.end_point, path)),
            HttpMethod::PUT => res.put(format!("{}{}", self.end_point, path)),
            HttpMethod::DELETE => res.delete(format!("{}{}", self.end_point, path)),
            HttpMethod::PATCH => res.patch(format!("{}{}", self.end_point, path)),
        };
        let res = res
            .query(params)
            .headers(headers)
            .headers(self.header.clone());
        let res = if let Some(form) = form {
            res.multipart(form)
        } else {
            res
        };
        let res = res.send().await?;
        if res.status() >= StatusCode::BAD_REQUEST {
            return Err(Error::AppWriteError(res.json::<AppWriteError>().await?));
        }

        Ok(res)
    }

    pub async fn chunk_upload<T: Serialize + ?Sized>(
        &self,
        file_path: &str,
        storage_path: &str,
        file_id: String,
        params: &T, //TODO: attach params
        on_progress: Option<fn(UploadProgress)>,
    ) -> Result<File, Error> {
        let boundary = Uuid::new_v4();

        let file_name = match Path::new(file_path.clone()).exists() {
            true => match Path::new(file_path.clone()).file_name() {
                Some(val) => format!("{:?}", val),
                None => Err(Error::Unknown)?,
            },
            false => Err(Error::FilePathNotExist(String::from(file_path)))?,
        };

        let file = match fs::read(file_path) {
            Ok(size) => size,
            Err(err) => return Err(Error::Io(err)),
        };

        let file_size = file.len();
        let uri = format!("{}{}", self.end_point, storage_path);

        // File Size Check and Upload
        if file_size <= self.chunk_size {
            // Single-request upload
            let part = Part::bytes(file).file_name(file_name.clone());

            let form = multipart::Form::new()
                .text("fileId", file_id.clone())
                .part("file", part);

            let mut headers = HeaderMap::new();
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str(
                    format!("multipart/form-data; boundary={}", boundary).as_str(),
                )?,
            );
            let params = serde_json::json!({});
            let file = self
                .call(HttpMethod::POST, uri.as_str(), headers, &params, Some(form))
                .await?
                .json::<File>()
                .await?;

            if let Some(ref on_progress) = on_progress {
                on_progress(UploadProgress {
                    id: file.clone().id,
                    progress: 100,
                    size_uploaded: 100,
                    chunks_total: file.chunks_total,
                    chunks_uploaded: file.chunks_uploaded,
                });
            }

            return Ok(file);
        }

        let mut offset: usize = 0;

        let mut first_upload = true; // Track if it's the first upload for x-appwrite-id
        let mut x_appwrite_id: Option<String> = None;

        let mut res: Option<File> = None;

        if file_id != "unique()" {
            let mut headers = HeaderMap::new();
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str(
                    format!("multipart/form-data; boundary={}", boundary).as_str(),
                )?,
            );
            let params = serde_json::json!({});
            let res = self
                .call(
                    HttpMethod::GET,
                    format!("{}{}/{}", self.end_point, storage_path, file_id).as_str(),
                    headers,
                    &params,
                    None,
                )
                .await?
                .json::<File>()
                .await?;
            offset = res.chunks_uploaded * self.chunk_size
        }

        while offset < file_size {
            let end = std::cmp::min(offset + self.chunk_size, file_size);
            let chunk = &file[offset..end];
            let content_range = format!("bytes {}-{}/{}", offset, end - 1, file_size);

            let mut headers = HeaderMap::new();
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str(
                    format!("multipart/form-data; boundary={}", boundary).as_str(),
                )?,
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
                    x_appwrite_id.clone().unwrap().as_str().parse().unwrap(),
                );
            }

            let params = serde_json::json!({
                "permissions": &[] as &[String]
            });
            let response = self
                .call(
                    HttpMethod::POST,
                    uri.as_str(),
                    headers,
                    &params,
                    Some(chunk_form),
                )
                .await?;
            if response.status() != StatusCode::CREATED {
                return Err(Error::AppWriteError(
                    response.json::<AppWriteError>().await?,
                ));
            }
            let file = response.json::<File>().await?;
            if first_upload {
                x_appwrite_id = Some(file.clone().id);
                first_upload = false;
            }

            if let Some(ref on_progress) = on_progress {
                on_progress(UploadProgress {
                    id: file.clone().id,
                    progress: std::cmp::min(offset, self.chunk_size) / self.chunk_size * 100,
                    size_uploaded: std::cmp::min(offset, self.chunk_size),
                    chunks_total: file.chunks_total,
                    chunks_uploaded: file.chunks_uploaded,
                });
            }

            res = Some(file.clone());

            offset += self.chunk_size;
        }
        Ok(res.unwrap())
    }
}
