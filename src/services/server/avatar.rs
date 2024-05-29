//! # Avatars
//!
//! The Avatars service aims to help you complete everyday tasks related to
//! your app image, icons, and avatars.

use crate::{
    api_params, app_json_header, client::Client, enumm::HttpMethod, enums::flag::Flag,
    error::Error, utils::get_content_header_value,
};
use serde_json::json;

pub struct Avatars;

impl Avatars {
    /// Get browser icon
    ///
    /// You can use this endpoint to show different browser icons to your users.
    /// The code argument receives the browser code as it appears in your user [GET
    /// /account/sessions](https://appwrite.io/docs/references/cloud/client-web/account#getSessions)
    /// endpoint. Use width, height and quality arguments to change the output
    /// settings.
    ///
    /// When one dimension is specified and the other is 0, the image is scaled
    /// with preserved aspect ratio. If both dimensions are 0, the API provides an
    /// image at source quality. If dimensions are not specified, the default size
    /// of image returned is 100x100px.
    pub async fn get_browser(
        client: &Client,
        code: &str,
        width: Option<u64>,
        height: Option<u64>,
        quality: Option<u64>,
    ) -> Result<Vec<u8>, Error> {
        let api_path = "/avatars/browsers/{code}".replace("{code}", code);

        let api_params = api_params!(
            "width"=>width,
            "height"=>height,
            "quality"=>quality,
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

    /// Get credit card icon
    ///
    /// The credit card endpoint will return you the icon of the credit card
    /// provider you need. Use width, height and quality arguments to change the
    /// output settings.
    ///
    /// When one dimension is specified and the other is 0, the image is scaled
    /// with preserved aspect ratio. If both dimensions are 0, the API provides an
    /// image at source quality. If dimensions are not specified, the default size
    /// of image returned is 100x100px.
    ///
    pub async fn get_credit_card(
        client: &Client,
        code: &str,
        width: Option<u64>,
        height: Option<u64>,
        quality: Option<u64>,
    ) -> Result<Vec<u8>, Error> {
        let api_path = "/avatars/credit-cards/{code}".replace("{code}", code);

        let api_params = api_params!(
            "width"=>width,
            "height"=>height,
            "quality"=>quality,
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

    /// Get favicon
    ///
    /// Use this endpoint to fetch the favorite icon (AKA favicon) of any remote
    /// website URL.
    ///
    pub async fn get_fav_icon(client: &Client, url: &str) -> Result<Vec<u8>, Error> {
        const API_PATH: &str = "/avatars/favicon";

        let api_params = api_params!(
            "url"=>Some(url),
            "project"=>get_content_header_value(&client, "project"),
            "key"=>get_content_header_value(&client, "key"),
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.bytes().await?.to_vec())
    }

    /// Get country flag
    ///
    /// You can use this endpoint to show different country flags icons to your
    /// users. The code argument receives the 2 letter country code. Use width,
    /// height and quality arguments to change the output settings. Country codes
    /// follow the [ISO 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1) standard.
    ///
    /// When one dimension is specified and the other is 0, the image is scaled
    /// with preserved aspect ratio. If both dimensions are 0, the API provides an
    /// image at source quality. If dimensions are not specified, the default size
    /// of image returned is 100x100px.
    ///
    pub async fn get_flag(
        client: &Client,
        code: Flag,
        width: Option<u64>,
        height: Option<u64>,
        quality: Option<u64>,
    ) -> Result<Vec<u8>, Error> {
        let api_path = format!("/avatars/flags/{}", json!(code));

        let api_params = api_params!(
            "width"=>width,
            "height"=>height,
            "quality"=>quality,
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

    /// Get image from URL
    ///
    /// Use this endpoint to fetch a remote image URL and crop it to any image size
    /// you want. This endpoint is very useful if you need to crop and display
    /// remote images in your app or in case you want to make sure a 3rd party
    /// image is properly served using a TLS protocol.
    ///
    /// When one dimension is specified and the other is 0, the image is scaled
    /// with preserved aspect ratio. If both dimensions are 0, the API provides an
    /// image at source quality. If dimensions are not specified, the default size
    /// of image returned is 400x400px.
    ///
    pub async fn get_image(
        client: &Client,
        url: &str,
        width: Option<u64>,
        height: Option<u64>,
    ) -> Result<Vec<u8>, Error> {
        const API_PATH: &str = "/avatars/image";

        let api_params = api_params!(
            "width"=>width,
            "height"=>height,
            "url"=>Some(url),
            "project"=>get_content_header_value(&client, "project"),
            "key"=>get_content_header_value(&client, "key"),
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.bytes().await?.to_vec())
    }

    /// Get user initials
    ///
    /// Use this endpoint to show your user initials avatar icon on your website or
    /// app. By default, this route will try to print your logged-in user name or
    /// email initials. You can also overwrite the user name if you pass the 'name'
    /// parameter. If no name is given and no user is logged, an empty avatar will
    /// be returned.
    ///
    /// You can use the color and background params to change the avatar colors. By
    /// default, a random theme will be selected. The random theme will persist for
    /// the user's initials when reloading the same theme will always return for
    /// the same initials.
    ///
    /// When one dimension is specified and the other is 0, the image is scaled
    /// with preserved aspect ratio. If both dimensions are 0, the API provides an
    /// image at source quality. If dimensions are not specified, the default size
    /// of image returned is 100x100px.
    ///
    pub async fn get_initials(
        client: &Client,
        name: Option<&str>,
        width: Option<u64>,
        height: Option<u64>,
        background: Option<&str>,
    ) -> Result<Vec<u8>, Error> {
        const API_PATH: &str = "/avatars/initials";

        let api_params = api_params!(
            "name"=>name,
            "background"=> background,
            "width"=>width,
            "height"=>height,
            "project"=>get_content_header_value(&client, "project"),
            "key"=>get_content_header_value(&client, "key"),
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.bytes().await?.to_vec())
    }

    /// Get QR code
    ///
    /// Converts a given plain text to a QR code image. You can use the query
    /// parameters to change the size and style of the resulting image.
    ///
    pub async fn get_qr(
        client: &Client,
        text: &str,
        size: Option<u64>,
        margin: Option<u64>,
        download: Option<bool>,
    ) -> Result<Vec<u8>, Error> {
        const API_PATH: &str = "/avatars/qr";

        let api_params = api_params!(
            "text"=>Some(text),
            "size"=>size,
            "margin"=>margin,
            "download"=> download,
            "project"=>get_content_header_value(&client, "project"),
            "key"=>get_content_header_value(&client, "key"),
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.bytes().await?.to_vec())
    }
}
