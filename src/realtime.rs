use crate::{client::Client, error::Error};
use async_fn_stream::try_fn_stream;
use futures_util::{Stream, StreamExt};
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub struct RealTime;

impl RealTime {
    pub async fn subscribe<'a>(
        client: &'a Client,
        channels: Vec<&'a str>,
        // project_id: &'a str,
    ) -> impl Stream<Item = Result<Value, Error>> + 'a {
        try_fn_stream(|emitter| async move {
            if channels.len() < 1 {
                return Ok(());
            }
            let mut url_search_params = String::new();
            let project_id = client
                .header
                .get("x-appwrite-Project")
                .ok_or(Error::Custom(
                    "Failed to get `x-appwrite-Project`. try configuring it".to_string(),
                ))?
                .to_str()
                .map_err(|_e| {
                    Error::Custom("could not convert result from header to string".to_string())
                })?;
            url_search_params.push_str(format!("/realtime?project={}", project_id,).as_str());
            channels.iter().for_each(|channel| {
                url_search_params.push_str(format!("&channels[]={}", channel).as_str())
            });
            let url = format!(
                "{}{}",
                client.end_point_realtime.clone().ok_or(Error::Custom(
                    "Unable to clone real time endpoint from client".to_string(),
                ))?,
                url_search_params
            );
            println!("{}", url);
            let (ws_stream, _response) = connect_async(url).await.expect("Can't connect");

            let (mut _write, mut read) = ws_stream.split();

            while let Some(message) = read.next().await {
                let msg = match message {
                    Ok(Message::Text(s)) => s,
                    Ok(_) => continue, // Skip non-text messages
                    Err(err) => {
                        eprintln!("Failed to read the message: {}", err);
                        break; // Break the loop on error
                    }
                };
                let parsed: Value = match serde_json::from_str(&msg) {
                    Ok(val) => val,
                    Err(err) => {
                        println!("Unable to parse JSON: {}", err);
                        break; // Break the loop on error
                    }
                };
                emitter.emit(parsed).await
            }
            Ok(())
        })

        // let a = ws_stream
        //     .close(None)
        //     .await
        //     .expect("unable to close this service");
        // todo!()
    }
}
