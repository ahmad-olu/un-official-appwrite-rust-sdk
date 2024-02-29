use crate::client::Client;
use futures_util::{future, pin_mut, StreamExt};
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

struct RealTime;

impl RealTime {
    async fn subscribe(client: &Client, channels: Vec<&str>, project_id: &str) {
        if channels.len() < 1 {
            return;
        }
        // if client.end_point_realtime.is_none() {
        //     return;
        // }
        let mut url_search_params = String::new();
        url_search_params.push_str(format!("/realtime?project={}", project_id).as_str());
        channels.iter().for_each(|channel| {
            url_search_params.push_str(format!("&channels[]={}", channel).as_str())
        });
        let url = format!(
            "{}{}",
            client.end_point_realtime.clone().unwrap(),
            url_search_params
        );
        let (ws_stream, _response) = connect_async(url).await.expect("Can't connect");

        let (mut _write, mut read) = ws_stream.split();

        if let Some(message) = read.next().await {
            let msg = message.expect("Failed to read the message");
            let msg = match msg {
                Message::Text(s) => s,
                _ => {
                    panic!()
                }
            };
            let parsed: Value = serde_json::from_str(&msg).expect("Unable to parse json");
            println!("{:#?}", parsed);
        }

        // let a = ws_stream
        //     .close(None)
        //     .await
        //     .expect("unable to close this service");
        todo!()
    }
}
