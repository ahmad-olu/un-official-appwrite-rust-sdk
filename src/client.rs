use std::{collections::BTreeMap, fs, str::FromStr};

use async_fn_stream::try_fn_stream;
use futures_util::Stream;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    multipart::{self, Form, Part},
    Response, StatusCode,
};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

use crate::{
    enumm::HttpMethod,
    error::{AppWriteError, Error},
    models::{deployment::Deployment, file::File, UploadType},
    upload_progress::UploadProgress,
    value::Value as val,
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
    pub async fn call(
        &self,
        method: HttpMethod,
        path: &str,
        headers: HeaderMap,
        params: BTreeMap<String, val>,
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
            .for_each(|val| match val.0.contains("queries") {
                true => {
                    let v = val.1.as_array().unwrap();
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
                        format!("{}{}={}", start_character, val.0, val.1.as_str().unwrap())
                            .as_str(),
                    )
                }
            });
        Ok(params_chain)
    }

    pub async fn chunk_upload_file<T: Serialize + ?Sized>(
        &self,
        file_path: &str,
        api_path: &str,
        file_id: String,
        params: BTreeMap<String, val>,
        file_name: String,
        //on_progress: Option<fn(UploadProgress)>,
        is_file: bool,
    ) -> Result<UploadType, Error> {
        let boundary = Uuid::new_v4();

        let file = match fs::read(file_path) {
            Ok(size) => size,
            Err(err) => return Err(Error::Io(err)),
        };

        let file_size = file.len();
        let uri = api_path;

        // File Size Check and Upload
        if file_size <= self.chunk_size {
            // Single-request upload
            let part = Part::bytes(file).file_name(file_name);

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

            match is_file {
                true => {
                    let file = self
                        .call(HttpMethod::POST, uri, headers, params.clone(), Some(form))
                        .await?
                        .json::<File>()
                        .await?;

                    // if let Some(ref on_progress) = on_progress {
                    //     on_progress(UploadProgress {
                    //         id: file.clone().id,
                    //         progress: 100,
                    //         size_uploaded: 100,
                    //         chunks_total: file.chunks_total,
                    //         chunks_uploaded: file.chunks_uploaded,
                    //     })
                    // };
                    return Ok(UploadType::File(file));
                }
                false => {
                    let deployment = self
                        .call(HttpMethod::POST, uri, headers, params.clone(), Some(form))
                        .await?
                        .json::<Deployment>()
                        .await?;

                    // if let Some(ref on_progress) = on_progress {
                    //     on_progress(UploadProgress {
                    //         id: deployment.clone().id,
                    //         progress: 100,
                    //         size_uploaded: 100,
                    //         chunks_total: deployment.chunks_total,
                    //         chunks_uploaded: deployment.chunks_uploaded,
                    //     })
                    // };
                    return Ok(UploadType::Deployment(deployment));
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
            let params: BTreeMap<String, val> = BTreeMap::new();
            match is_file {
                true => {
                    let res = self
                        .call(
                            HttpMethod::GET,
                            format!("{}{}/{}", self.end_point, api_path, file_id).as_str(),
                            headers,
                            params.clone(),
                            None,
                        )
                        .await?
                        .json::<File>()
                        .await?;
                    offset = res.chunks_uploaded * self.chunk_size
                }
                false => {
                    let res = self
                        .call(
                            HttpMethod::GET,
                            format!("{}{}/{}", self.end_point, api_path, file_id).as_str(),
                            headers,
                            params.clone(),
                            None,
                        )
                        .await?
                        .json::<Deployment>()
                        .await?;
                    offset = res.chunks_uploaded * self.chunk_size
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
                                "Unable to parse string slice into value header [invalid header value]".to_string(),
                            )
                        })?,
                );
            }

            // let params = serde_json::json!({
            //     "permissions": &[] as &[String]
            // });
            let response = self
                .call(
                    HttpMethod::POST,
                    uri,
                    headers,
                    params.clone(),
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
                    }

                    // if let Some(ref on_progress) = on_progress {
                    //     on_progress(UploadProgress {
                    //         id: file.clone().id,
                    //         progress: std::cmp::min(offset, self.chunk_size) / self.chunk_size
                    //             * 100,
                    //         size_uploaded: std::cmp::min(offset, self.chunk_size),
                    //         chunks_total: file.chunks_total,
                    //         chunks_uploaded: file.chunks_uploaded,
                    //     });
                    // }

                    res = Some(UploadType::File(file.clone()));
                }
                false => {
                    let deployment = response.json::<Deployment>().await?;
                    if first_upload {
                        x_appwrite_id = Some(deployment.clone().id);
                        first_upload = false;
                    }

                    // if let Some(ref on_progress) = on_progress {
                    //     on_progress(UploadProgress {
                    //         id: deployment.clone().id,
                    //         progress: std::cmp::min(offset, self.chunk_size) / self.chunk_size
                    //             * 100,
                    //         size_uploaded: std::cmp::min(offset, self.chunk_size),
                    //         chunks_total: deployment.chunks_total,
                    //         chunks_uploaded: deployment.chunks_uploaded,
                    //     });
                    // }

                    res = Some(UploadType::Deployment(deployment.clone()));
                }
            }

            offset += self.chunk_size;
        }
        Ok(res.ok_or(Error::Custom("No Upload Type".to_string()))?)
    }

    pub async fn chunk_upload_file_streamed<'a>(
        &'a self,
        file_path: &'a str,
        api_path: String,
        file_id: String,
        params: BTreeMap<String, val>,
        file_name: String,
        is_file: bool,
    ) -> impl Stream<Item = Result<(UploadType, UploadProgress), Error>> + 'a {
        try_fn_stream(|emitter| async move {
            let boundary = Uuid::new_v4();

            let file = match fs::read(file_path) {
                Ok(size) => size,
                Err(err) => return Err(Error::Io(err)),
            };

            let file_size = file.len();
            let uri = api_path.clone();
            // File Size Check and Upload
            if file_size <= self.chunk_size {
                // Single-request upload
                let part = Part::bytes(file).file_name(file_name);

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

                match is_file {
                    true => {
                        let file = self
                            .call(
                                HttpMethod::POST,
                                uri.as_str(),
                                headers,
                                params.clone(),
                                Some(form),
                            )
                            .await?
                            .json::<File>()
                            .await?;

                        emitter
                            .emit((
                                UploadType::File(file.clone()),
                                UploadProgress {
                                    id: file.clone().id,
                                    progress: 100,
                                    size_uploaded: 100,
                                    chunks_total: file.chunks_total,
                                    chunks_uploaded: file.chunks_uploaded,
                                },
                            ))
                            .await;

                        return Ok(());
                    }
                    false => {
                        let deployment = self
                            .call(
                                HttpMethod::POST,
                                uri.as_str(),
                                headers,
                                params.clone(),
                                Some(form),
                            )
                            .await?
                            .json::<Deployment>()
                            .await?;

                        emitter
                            .emit((
                                UploadType::Deployment(deployment.clone()),
                                UploadProgress {
                                    id: deployment.clone().id,
                                    progress: 100,
                                    size_uploaded: 100,
                                    chunks_total: deployment.chunks_total,
                                    chunks_uploaded: deployment.chunks_uploaded,
                                },
                            ))
                            .await;

                        return Ok(());
                    }
                }
            }
            let mut offset: usize = 0;

            let mut first_upload = true; // Track if it's the first upload for x-appwrite-id
            let mut x_appwrite_id: Option<String> = None;

            // let mut res: Option<UploadType> = None;

            if file_id != "unique()" {
                let mut headers = HeaderMap::new();
                headers.insert(
                    CONTENT_TYPE,
                    HeaderValue::from_str(
                        format!("multipart/form-data; boundary={}", boundary).as_str(),
                    )?,
                );
                let params: BTreeMap<String, val> = BTreeMap::new();
                match is_file {
                    true => {
                        let res = self
                            .call(
                                HttpMethod::GET,
                                format!("{}{}/{}", self.end_point, api_path, file_id).as_str(),
                                headers,
                                params.clone(),
                                None,
                            )
                            .await?
                            .json::<File>()
                            .await?;
                        offset = res.chunks_uploaded * self.chunk_size
                    }
                    false => {
                        let res = self
                            .call(
                                HttpMethod::GET,
                                format!("{}{}/{}", self.end_point, api_path, file_id).as_str(),
                                headers,
                                params.clone(),
                                None,
                            )
                            .await?
                            .json::<Deployment>()
                            .await?;
                        offset = res.chunks_uploaded * self.chunk_size
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
                                    "Unable to parse string slice into value header [invalid header value]".to_string(),
                                )
                            })?,
                    );
                }

                // let params = serde_json::json!({
                //     "permissions": &[] as &[String]
                // });
                let response = self
                    .call(
                        HttpMethod::POST,
                        uri.as_str(),
                        headers,
                        params.clone(),
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
                        }

                        emitter
                            .emit((
                                UploadType::File(file.clone()),
                                UploadProgress {
                                    id: file.clone().id,
                                    progress: std::cmp::min(offset, self.chunk_size)
                                        / self.chunk_size
                                        * 100,
                                    size_uploaded: std::cmp::min(offset, self.chunk_size),
                                    chunks_total: file.chunks_total,
                                    chunks_uploaded: file.chunks_uploaded,
                                },
                            ))
                            .await;
                    }
                    false => {
                        let deployment = response.json::<Deployment>().await?;
                        if first_upload {
                            x_appwrite_id = Some(deployment.clone().id);
                            first_upload = false;
                        }

                        emitter
                            .emit((
                                UploadType::Deployment(deployment.clone()),
                                UploadProgress {
                                    id: deployment.clone().id,
                                    progress: std::cmp::min(offset, self.chunk_size)
                                        / self.chunk_size
                                        * 100,
                                    size_uploaded: std::cmp::min(offset, self.chunk_size),
                                    chunks_total: deployment.chunks_total,
                                    chunks_uploaded: deployment.chunks_uploaded,
                                },
                            ))
                            .await;
                    }
                }

                offset += self.chunk_size;
            }

            Ok(())
        })
    }
}
