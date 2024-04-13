use std::{fs, path::Path, str::FromStr};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    multipart::{self, Form, Part},
    Response, StatusCode,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    enumm::HttpMethod,
    error::{AppWriteError, Error},
    models::{deployment::Deployment, file::File},
    upload_progress::UploadProgress,
};

#[derive(Debug, Clone)]
pub struct Client {
    end_point: String,
    pub end_point_realtime: Option<String>, //todo set this
    pub header: HeaderMap,
    chunk_size: usize,
    self_signed: bool,
}

#[derive(Clone)]
pub struct ClientBuilder {
    end_point: Option<String>,
    pub end_point_realtime: Option<String>, //todo set this
    pub header: HeaderMap,
    chunk_size: Option<usize>,
    self_signed: Option<bool>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            end_point: Some(String::from("https://cloud.appwrite.io/v1")),
            end_point_realtime: None,
            header: HeaderMap::new(),
            chunk_size: Some(5 * 1024 * 1024),
            self_signed: Some(false),
        }
    }
}
impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            end_point: None,
            ..Default::default()
        }
    }
    pub fn set_self_signed(&mut self, status: bool) -> Result<&mut Self, Error> {
        self.self_signed = Some(status);
        Ok(self)
    }
    pub fn set_endpoint(&mut self, endpoint: &str) -> Result<&mut Self, Error> {
        self.end_point = Some(String::from(endpoint));
        if self.end_point_realtime.as_ref().is_none() {
            self.end_point_realtime = self.end_point.clone().and_then(|val| {
                Some(
                    val.replace("https://", "wss://")
                        .replace("http://", "ws://"),
                )
            });
        }
        Ok(self)
    }
    pub fn add_header(&mut self, key: &str, value: &str) -> Result<&mut Self, Error> {
        self.header
            .insert(HeaderName::from_str(key)?, HeaderValue::from_str(value)?);
        Ok(self)
    }

    pub fn set_project(&mut self, value: &str) -> Result<&mut Self, Error> {
        self.header
            .insert("x-appwrite-Project", HeaderValue::from_str(value)?);
        Ok(self)
    }

    pub fn set_key(&mut self, value: &str) -> Result<&mut Self, Error> {
        self.header
            .insert("x-appwrite-key", HeaderValue::from_str(value)?);
        Ok(self)
    }

    pub fn set_jwt(&mut self, value: &str) -> Result<&mut Self, Error> {
        self.header
            .insert("x-appwrite-jwt", HeaderValue::from_str(value)?);
        Ok(self)
    }

    pub fn set_locale(&mut self, value: &str) -> Result<&mut Self, Error> {
        self.header
            .insert("x-appwrite-locale", HeaderValue::from_str(value)?);
        Ok(self)
    }

    pub fn build(&self) -> Result<Client, Error> {
        let Some(endpoint) = self.end_point.as_ref() else {
            return Err(Error::Unknown);
        };
        Ok(Client {
            end_point: endpoint.to_string(),
            end_point_realtime: self.end_point_realtime.clone(),
            header: self.header.clone(),
            chunk_size: self.chunk_size.clone().unwrap_or_else(|| 5 * 1024 * 1024),
            self_signed: self.self_signed.clone().unwrap_or_else(|| false),
        })
    }
}

impl Client {
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
            .json(&params)
            .headers(headers)
            .headers(self.header.clone());

        let res = if let Some(form) = form {
            res.multipart(form)
        } else {
            res
        };
        let res = res.send().await?;
        // if res.status() >= StatusCode::BAD_REQUEST {
        //     return Err(Error::AppWriteError(res.json::<AppWriteError>().await?));
        // }
        match res.status().as_u16() {
            200..=299 => Ok(res),
            400..=599 => Err(Error::AppWriteError(res.json::<AppWriteError>().await?)),
            _ => {
                //TODO: Error Message: Unexpected status code:{}, response.statuscode
                Err(Error::Unknown)
            }
        }
    }

    // pub async fn web_auth(url: &str) {
    //     let request = reqwest::Client::new().get(url);
    //     request

    // }
    pub async fn chunk_upload_file<T: Serialize + ?Sized>(
        &self,
        file_path: &str,
        api_path: &str,
        file_id: String,
        params: &T,
        on_progress: Option<fn(UploadProgress)>,
    ) -> Result<File, Error> {
        let boundary = Uuid::new_v4();

        let file_name = match Path::new(file_path).exists() {
            true => match Path::new(file_path).file_name() {
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
        let uri = api_path;

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

            let file = self
                .call(HttpMethod::POST, uri, headers, &params, Some(form))
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
                    format!("{}{}/{}", self.end_point, api_path, file_id).as_str(),
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

            // let params = serde_json::json!({
            //     "permissions": &[] as &[String]
            // });
            let response = self
                .call(HttpMethod::POST, uri, headers, &params, Some(chunk_form))
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

    pub async fn chunk_upload_deployment<T: Serialize + ?Sized>(
        &self,
        file_path: &str,
        api_path: &str,
        file_id: String,
        params: &T,
        on_progress: Option<fn(UploadProgress)>,
    ) -> Result<Deployment, Error> {
        let boundary = Uuid::new_v4();

        let file_name = match Path::new(file_path).exists() {
            true => match Path::new(file_path).file_name() {
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
        let uri = api_path;

        // Deployment Size Check and Upload
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

            let file = self
                .call(HttpMethod::POST, uri, headers, &params, Some(form))
                .await?
                .json::<Deployment>()
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

        let mut res: Option<Deployment> = None;

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
                    format!("{}{}/{}", self.end_point, api_path, file_id).as_str(),
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

            // let params = serde_json::json!({
            //     "permissions": &[] as &[String]
            // });
            let response = self
                .call(HttpMethod::POST, uri, headers, &params, Some(chunk_form))
                .await?;
            if response.status() != StatusCode::CREATED {
                return Err(Error::AppWriteError(
                    response.json::<AppWriteError>().await?,
                ));
            }
            let file = response.json::<Deployment>().await?;
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
