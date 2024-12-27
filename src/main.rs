use std::collections::BTreeMap;

use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{json, Value};
// use futures_util::{pin_mut, StreamExt};
// use std::sync::{Arc, Mutex};
// use tokio::task;
use unofficial_appwrite::{error::Error, id::ID, query::Query as q, query_value::Query};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let a = ace(16.into());

    let a: &[String] = &[];

    let a = Query::new("equal", "man".into(), vec![12].into());

    println!("{:#?}", a);
    println!(
        "{:#?}",
        Query::new("equal", "man".into(), vec!["run"].into()).to_string()
    );

    println!(
        "{:#?}",
        Query::new("equal", "man".into(), vec![14].into()).to_string()
    );

    println!("{:#?}", q::equal("man".into(), vec!["run"].into()));

    // let _params = json!({
    //     "userId":ID::unique(7),
    //     "email": "fakeEmail@Email.com",
    //     "password": "VeryVerySecurePassword@123456789",
    //     "name":"ola",
    // });

    // let mut a: BTreeMap<String, Value> = BTreeMap::new();

    // a.insert("email".into(), "fakeEmail@Email.com".into());
    // a.insert("password".into(), "VeryVerySecurePassword@123456789".into());
    // a.insert("userId".into(), ID::unique(7).into());

    // let mut headers = HeaderMap::new();
    // headers.insert(
    //     "x-appwrite-Project",
    //     HeaderValue::from_str("676c2b7b000c834e1fce")?,
    // );

    // headers.insert("x-appwrite-key", HeaderValue::from_str("standard_5d84014ebaf0de52308eff28946a43062921240c10b81c2fd037ab60b02f0257b7f0a53fe94065170fe7c7d0af2d4136d4cbf32a4055baeada3d27f2e323b70aeda87e97f676207cf10cbb18b7a80f8d1103803617454c89138f217dad701bbe9dc6950bc58853fdb2a0b4b67d2a8b8b6b7b9b2e6d9b94e0a2fcfee794688e2e")?);
    // let res = reqwest::Client::new()
    //     .post(format!("{}{}", "http://127.0.0.1/v1", "/users"))
    //     .json(&a)
    //     .headers(headers)
    //     .send()
    //     .await?;

    // dbg!(res);

    // let mut a: BTreeMap<String, Value> = BTreeMap::new();

    // a.insert("email".into(), "fakeEmail@Email.com".into());
    // a.insert("password".into(), "VeryVerySecurePassword@123456789".into());
    // a.insert("userId".into(), "unique()".into());
    // dbg!(a);

    // let client = ClientBuilder::default()
    //     .set_project("65d20d....")?
    //     .set_key("ae07b88634eacfb42a2fc4c4a7f278d96....")?
    //     //.set_self_signed(false)?
    //     .build()?;

    // let prog = Arc::new(Mutex::new(0));

    // // Clone the Arc for the async task
    // let prog_clone = Arc::clone(&prog);

    // task::spawn(async move {
    //     let stream = Storage::create_files_streamed(
    //         &client,
    //         "65d20d5c8096032a03cd",
    //         ID::unique(),
    //         r"c:\Users\pc\Downloads\Video\New folder (2)\Folder 1\NGINX_101_SecretHeart.pdf",
    //         String::from("NGINX_101_SecretHeart.pdf"),
    //         None,
    //     )
    //     .await;
    //     pin_mut!(stream);

    //     while let Some(data) = stream.next().await {
    //         match data {
    //             Ok((_file, progress)) => {
    //                 // let res = data.unwrap();
    //                 // let file = res.0;
    //                 // println!("==>{:?}===>{:?}", file, res.1);

    //                 // Update the shared state
    //                 let mut prog = prog_clone.lock().unwrap();
    //                 *prog = progress.progress;
    //             }
    //             Err(e) => {
    //                 eprintln!("Stream error: {:?}", e);
    //                 break;
    //             }
    //         }
    //     }
    // });
    // let prog = prog.lock().unwrap();
    // println!("Progress: {}", *prog);

    // let a = app_json_header!();
    // println!("{:?}", a);

    Ok(())
}

// ~/Documents/playground/docker/appwrite $ cd /home/r2/Documents/playground/docker/appwrite && \
// docker compose up -d --remove-orphans
