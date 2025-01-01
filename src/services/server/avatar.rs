//! # Avatars
//!
//! The Avatars service aims to help you complete everyday tasks related to
//! your app image, icons, and avatars.

use std::collections::HashMap;

use crate::{
    app_json_header, client::Client, enumm::HttpMethod, enums::flag::Flag, error::Error,
    utils::get_content_header_value,
};
use serde_json::{json, Value};

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
    ///* width => number?
    ///* height => number?
    ///* quality => number?
    pub async fn get_browser(
        client: &Client,
        code: &str,
        mut args: HashMap<String, Value>,
    ) -> Result<Vec<u8>, Error> {
        let api_path = "/avatars/browsers/{code}".replace("{code}", code);
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
    ///* width => number?
    ///* height => number?
    ///* quality => number?
    pub async fn get_credit_card(
        client: &Client,
        code: &str,
        mut args: HashMap<String, Value>,
    ) -> Result<Vec<u8>, Error> {
        let api_path = "/avatars/credit-cards/{code}".replace("{code}", code);

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

    /// Get favicon
    ///
    /// Use this endpoint to fetch the favorite icon (AKA favicon) of any remote
    /// website URL.
    ///
    ///* url => string
    pub async fn get_fav_icon(
        client: &Client,
        mut args: HashMap<String, Value>,
    ) -> Result<Vec<u8>, Error> {
        const API_PATH: &str = "/avatars/favicon";

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
            .call(HttpMethod::GET, API_PATH, api_headers, &args, None)
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
    ///* width => number?
    ///* height => number?
    ///* quality => number?
    pub async fn get_flag(
        client: &Client,
        code: Flag,
        mut args: HashMap<String, Value>,
    ) -> Result<Vec<u8>, Error> {
        let api_path = format!("/avatars/flags/{}", json!(code));

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
    ///* width => number?
    ///* height => number?
    ///* url => string
    pub async fn get_image(
        client: &Client,
        mut args: HashMap<String, Value>,
    ) -> Result<Vec<u8>, Error> {
        const API_PATH: &str = "/avatars/image";

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
            .call(HttpMethod::GET, API_PATH, api_headers, &args, None)
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
    ///* name => string?
    ///* background => string?
    ///* width => number?
    ///* height => number?
    pub async fn get_initials(
        client: &Client,
        mut args: HashMap<String, Value>,
    ) -> Result<Vec<u8>, Error> {
        const API_PATH: &str = "/avatars/initials";

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
            .call(HttpMethod::GET, API_PATH, api_headers, &args, None)
            .await?;

        Ok(res.bytes().await?.to_vec())
    }

    /// Get QR code
    ///
    /// Converts a given plain text to a QR code image. You can use the query
    /// parameters to change the size and style of the resulting image.
    ///
    ///* text => string
    ///* size => number?
    ///* margin => number?
    ///* download => bool?
    pub async fn get_qr(
        client: &Client,
        mut args: HashMap<String, Value>,
    ) -> Result<Vec<u8>, Error> {
        const API_PATH: &str = "/avatars/qr";

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
            .call(HttpMethod::GET, API_PATH, api_headers, &args, None)
            .await?;

        Ok(res.bytes().await?.to_vec())
    }
}
