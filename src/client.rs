use std::{collections::HashMap, fs, str::FromStr};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    multipart::{self, Form, Part},
    Response, StatusCode,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    enumm::HttpMethod,
    error::{AppWriteError, Error},
    models::{deployment::Deployment, file::File, UploadType},
};

#[derive(Debug, Clone)]
pub struct ChunkProgress {
    pub chunks_uploaded: u64,
    pub chunks_total: u64,
    pub size_uploaded: usize,
    pub progress: f64,
    pub id: String,
}
impl ChunkProgress {
    pub fn new() -> Self {
        Self {
            id: String::from(""),
            chunks_total: 0,
            chunks_uploaded: 0,
            progress: 0.0,
            size_uploaded: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    end_point: String,
    pub end_point_realtime: Option<String>, //todo set this
    pub header: HeaderMap,
    chunk_size: usize,
    _self_signed: bool,
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
            end_point_realtime: Some(String::from("wss://cloud.appwrite.io/v1")),
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
            self.end_point_realtime = self.end_point.clone().and_then(|value| {
                Some(
                    value
                        .replace("https://", "wss://")
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
            _self_signed: self.self_signed.clone().unwrap_or_else(|| false),
        })
    }
}

impl Client {
    pub async fn call(
        &self,
        method: HttpMethod,
        path: &str,
        headers: HeaderMap,
        params: &HashMap<String, Value>,
        form: Option<Form>,
    ) -> Result<Response, Error> {
        let res = reqwest::Client::new();
        let res = match method {
            HttpMethod::GET => {
                let param = Self::_flatten_params_for_get(&json!(params))?;
                res.get(format!("{}{}{}", self.end_point, path, param))
            }
            HttpMethod::POST => res
                .post(format!("{}{}", self.end_point, path))
                .json(&params),
            HttpMethod::PUT => res.put(format!("{}{}", self.end_point, path)).json(&params),
            HttpMethod::DELETE => res
                .delete(format!("{}{}", self.end_point, path))
                .json(&params),
            HttpMethod::PATCH => res
                .patch(format!("{}{}", self.end_point, path))
                .json(&params),
        };
        let res = res.headers(headers).headers(self.header.clone());

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
            400..=599 => {
                let err = res.json::<AppWriteError>().await?;
                Err(Error::AppWriteError {
                    message: err.message,
                    code: err.code,
                    response: err.response,
                    error_type: err.error_type,
                })
            }
            _ => {
                //TODO: Error Message: Unexpected status code:{}, response.statuscode
                Err(Error::Unknown)
            }
        }
    }

    fn _flatten_params_for_get(api_params: &serde_json::Value) -> Result<String, Error> {
        let mut params_chain = String::new();
        let mut i = 0;

        api_params
            .as_object()
            .ok_or(Error::Custom(
                "Unable to convert value because it's not an object".to_string(),
            ))?
            .iter()
            .for_each(|value| match value.0.contains("queries") {
                true => {
                    let v = value.1.as_array().unwrap();
                    v.iter().for_each(|query| {
                        let start_string = match i == 0 {
                            true => "?",
                            false => "&",
                        };
                        params_chain.push_str(
                            format!("{}queries[{}]={}", start_string, i, query.as_str().unwrap())
                                .as_str(),
                        );
                        i += 1;
                    });
                }
                false => {
                    let start_character = match i != 0 {
                        true => "&",
                        false => "?",
                    };
                    params_chain.push_str(
                        format!(
                            "{}{}={}",
                            start_character,
                            value.0,
                            value.1.as_str().unwrap()
                        )
                        .as_str(),
                    )
                }
            });
        Ok(params_chain)
    }

    pub async fn chunk_upload_file<F>(
        &self,
        file_path: String,
        api_path: String,
        file_id: String,
        params: HashMap<String, Value>,
        file_name: String,
        is_file: bool,
        mut on_progress: F,
    ) -> Result<UploadType, Error>
    where
        F: FnMut(ChunkProgress) + Send + 'static,
    {
        let boundary = Uuid::new_v4();
        let file = match fs::read(file_path) {
            Ok(size) => size,
            Err(err) => return Err(Error::Io(err)),
        };
        let file_size = file.len();

        //let uri = api_path;

        if file_size <= self.chunk_size {
            let part = Part::bytes(file.clone()).file_name(file_name.clone());

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

            let res = self
                .call(HttpMethod::POST, &api_path, headers, &params, Some(form))
                .await?;
            match is_file {
                true => {
                    return Ok(UploadType::File(res.json::<File>().await?));
                }
                false => {
                    return Ok(UploadType::Deployment(res.json::<Deployment>().await?));
                }
            }
        }

        let mut offset: usize = 0;

        let mut first_upload = true; // Track if it's the first upload for x-appwrite-id
        let mut x_appwrite_id: Option<String> = None;
        let mut res: Option<UploadType> = None;

        if file_id != "unique()" {
            let mut headers = HeaderMap::new();
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str(
                    format!("multipart/form-data; boundary={}", boundary).as_str(),
                )?,
            );

            let res = self
                .call(
                    HttpMethod::GET,
                    format!("{}{}/{}", self.end_point, api_path, file_id).as_str(),
                    headers,
                    &params,
                    None,
                )
                .await?;
            match is_file {
                true => {
                    offset = res.json::<File>().await?.chunks_uploaded;
                }
                false => {
                    offset = res.json::<Deployment>().await?.chunks_uploaded;
                }
            }
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

            let response = self
                .call(
                    HttpMethod::POST,
                    &api_path,
                    headers,
                    &params,
                    Some(chunk_form),
                )
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

            match is_file {
                true => {
                    let file = response.json::<File>().await?;
                    if first_upload {
                        x_appwrite_id = Some(file.clone().id);
                        first_upload = false;
                    };
                    let size_uploaded = std::cmp::min(offset, file_size);
                    let progress = size_uploaded as f64 / file_size as f64;
                    let chunks_uploaded = file.chunks_uploaded as u64;
                    let chunks_total = file.chunks_total as u64;

                    on_progress(ChunkProgress {
                        chunks_uploaded,
                        chunks_total,
                        size_uploaded,
                        progress,
                        id: file.clone().id,
                    });
                    res = Some(UploadType::File(file));
                }
                false => {
                    let deployment = response.json::<Deployment>().await?;
                    if first_upload {
                        x_appwrite_id = Some(deployment.clone().id);
                        first_upload = false;
                    };
                    let size_uploaded = std::cmp::min(offset, file_size);
                    let progress = size_uploaded as f64 / file_size as f64;
                    let chunks_uploaded = deployment.chunks_uploaded as u64;
                    let chunks_total = deployment.chunks_total as u64;

                    on_progress(ChunkProgress {
                        chunks_uploaded,
                        chunks_total,
                        size_uploaded,
                        progress,
                        id: deployment.clone().id,
                    });
                    res = Some(UploadType::Deployment(deployment));
                }
            }

            offset += self.chunk_size;
        }
        Ok(res.ok_or(Error::Custom("No Upload Type".to_string()))?)

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
}
