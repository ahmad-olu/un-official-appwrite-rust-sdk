// use futures_util::{pin_mut, StreamExt};
// use std::sync::{Arc, Mutex};
// use tokio::task;
use unofficial_appwrite::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
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

    Ok(())
}
