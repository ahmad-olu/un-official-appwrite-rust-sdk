use futures_util::{pin_mut, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use unofficial_appwrite::{
    client::ClientBuilder,
    error::Error,
    id::ID,
    query::Query,
    realtime::RealTime,
    services::server::{databases::Databases, storage::Storage},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let client = ClientBuilder::default()
    //     .set_project("65d20d389f2b36778b8b")?
    //     .set_key("ae07b88634eacfb42a2fc4c4a7f278d967d863385d677c054d5b8edddfdd6c98f0669fa0f03d2e8fa9b029024c7b2a7b69726fa1e32a68b6d11df3933a467a9b7160f5c149775e94814ea8f6ff3225ba1854fa069c6f0e130921e3e4f33d2839a54a5f618dfe7f85442458425f6fcbd090d48dd5b830f8881176caec65d0bb20")?
    //     //.set_self_signed(false)?
    //     .build()?;

    //-----------------websocket
    // let stream = RealTime::subscribe(
    //     &client,
    //     vec!["databases.6618eec286a4ef198076.collections.6618ef06d269bf4110d4.documents"],
    // )
    // .await;
    // pin_mut!(stream);
    // while let Some(data) = stream.next().await {
    //     println!("=====>{:?}<=====", data);
    // }

    // let queries = vec![Query::equal(r"$id", json!(vec!["6618ef06d269bf4110d4"]))];

    // let col = Databases::list_collections(
    //     &client,
    //     "6618eec286a4ef198076",
    //     Some(Query::equal(r"$id", json!(vec!["6619010ddab914e6b01e"]))),
    //     Some(queries),
    // )
    // .await?;
    // dbg!(col);

    // let mut id: String;
    // let mut uploaded: usize;
    // let mut total: usize;
    // let mut prog: usize;
    // let create_file_less_than_5_mb = Storage::create_files(
    //     &client,
    //     "65d20d5c8096032a03cd",
    //     ID::unique(),
    //     r"c:\Users\pc\Downloads\Video\New folder (2)\Folder 1\Ultimate Flutter for Cross-Platform App Development (Temidayo Adefioye) (Z-Library).pdf",
    //     String::from("sgs_deadlines_next.pdf"),
    //     None,
    // )
    // .await?;
    // dbg!(create_file_less_than_5_mb);
    // println!("{}==>{}===>{}===>{}", id, uploaded, total, prog);

    // let stream = Storage::create_files_streamed(
    //     &client,
    //     "65d20d5c8096032a03cd",
    //     ID::unique(),
    //     r"c:\Users\pc\Downloads\Video\New folder (2)\Folder 1\Ultimate Flutter for Cross-Platform App Development (Temidayo Adefioye) (Z-Library).pdf",
    //     String::from("sgs_deadlines_next_3.pdf"),
    //     None,
    // )
    // .await;
    // pin_mut!(stream);
    // let mut prog: usize = 0;
    // while let Some(data) = stream.next().await {
    //     let res = data.unwrap();
    //     let file = res.0;
    //     println!("==>{:?}===>{:?}", file, res.1);
    //     prog = res.1.progress;
    // }
    // println!("progress --------------->{prog}");

    //============= Storage =============>

    // let create_buk = Storage::create_bucket(
    //     &client,
    //     ID::unique(),
    //     "My Bucket",
    //     None,
    //     None,
    //     None,
    //     None,
    //     None,
    //     None,
    //     None,
    //     None,
    // )
    // .await?;
    // dbg!(create_buk);

    // let create_file_less_than_5_mb = Storage::create_files(
    //     &client,
    //     "...",
    //     ID::unique(),
    //    r"C:\Users\pc\Downloads\Documents\test\WEB700 Assignment 5.pdf",
    //     None,
    //     None,
    // )
    // .await?;
    // dbg!(create_file_less_than_5_mb);

    // let create_file_greater_than_5_mb = Storage::create_files(
    //     &client,
    //     "...",
    //     ID::unique(),
    //     r"C:\Users\pc\Downloads\Documents\test\y2mate.com - Lovefool  Vintage Jazz Cardigans Cover ft Haley Reinhart_v720P.mp4",
    //     None,
    //     Some(|prog| {
    //         println!("{}:{}:{}", prog.id, prog.progress, prog.size_uploaded);
    //     }),
    // )
    // .await?;
    // dbg!(create_file_greater_than_5_mb);

    // let get_file =
    //     Storage::get_file(&client, "...", "...").await?;
    // dbg!(get_file);

    // let get_file_download =
    //     Storage::get_file_download(&client, "...", "...").await?;
    // fs::write(
    //     r"C:\Users\pc\Downloads\Documents\test\second\new.mp4",
    //     get_file_download,
    // )
    // .expect("didn't work");
    //============= Database ============>

    // let list_db = Databases::list(&client, None, None).await?;
    // dbg!(list_db);

    // let create_db = Databases::create(&client, ID::unique(), "test_db", None).await?;
    // dbg!(create_db);

    // let create_collection = Databases::create_collection(
    //     &client,
    //     "...",
    //     ID::unique(),
    //     "test_collection_1",
    //     Some(vec![
    //         Permission::read("any").as_str(),
    //         Permission::create("user:22222346").as_str(),
    //     ]),
    //     None,
    //     None,
    // )
    // .await?;
    // dbg!(create_collection);

    // let queries: Query = Query;
    // let queries = vec![queries.equal("name", vec![json!("test_collection_3")])];
    // let list_collections =
    //     Databases::list_collections(&client, "...", None, Some(queries)).await?;
    // dbg!(list_collections);

    // let att = Databases::create_boolean_attribute(
    //     &client,
    //     "...",
    //     "...",
    //     "isAdmin",
    //     true,
    //     None,
    //     None,
    // )
    // .await?;
    // dbg!(att);
    // let name = Databases::create_string_attribute(
    //     &client,
    //     "...",
    //     "...",
    //     "title",
    //     255,
    //     true,
    //     None,
    //     None,
    //     None,
    // )
    // .await?;
    // dbg!(name);

    // let _del_att = Databases::delete_attribute(
    //     &client,
    //     "...",
    //     "...",
    //     "isAdmin",
    // )
    // .await?;

    // let list_doc = Databases::list_documents(
    //     &client,
    //     "...",
    //     "...",
    //     None,
    // )
    // .await?;
    // dbg!(list_doc);

    // let a = vec![Query::equal("title", json!("next1"))];

    // let mut data = Map::new();
    // data.insert(String::from("isAdmin"), json!(false));
    // data.insert(String::from("title"), json!("next1"));
    // let create_doc = Databases::create_documents(
    //     &client,
    //     "...",
    //     "...",
    //     ID::unique(),
    //     data,
    //     None,
    // )
    // .await?;
    // dbg!(create_doc);

    // let queries = vec![Query::search("name", "a")];
    // println!("{queries:?}");
    // let get_doc = Databases::list_documents(
    //     &client,
    //     "...",
    //     "...",
    //     Some(queries),
    // )
    // .await?;
    // dbg!(get_doc);

    // let relationship = Databases::create_relationship_attribute(
    //     &client,
    //     "...",
    //     "...",
    //     "...",
    //     RelationshipType::OneToOne,
    //     None,
    //     Some("test_col_2"),
    //     None,
    //     None,
    // )
    // .await?;
    // dbg!(relationship);

    //============= User ================>

    // let create_user = Users::create(
    //     &client,
    //     ID::unique(),
    //     Some("olu3@gmail.com"),
    //     None,
    //     Some("password"),
    //     Some("olu3"),
    // )
    // .await?;
    // println!("{:?}", create_user);

    // let user_list = Users::list(&client, None, None).await?;
    // println!("{:#?}", user_list.users);

    // let user_changed_password =
    //     Users::update_password(&client, "...", "newPassword").await?;
    // println!("{:#?}", user_changed_password);

    // let user_changed_email =
    //     Users::update_email(&client, "...", "new_olu@gmail.com").await?;
    // println!("{:#?}", user_changed_email);

    // let user_prefs = Users::get_prefs(&client, "...").await?;
    // println!("{:#?}", user_prefs);

    // let mut prefs = Map::new();
    // prefs.insert("isAdmin".to_string(), json!(true));
    // let user_update_prefs = Users::update_prefs(&client, "...", prefs).await?;
    // println!("{:#?}", user_update_prefs);

    //-------------------------------------------
    // fn prog(p: OnProgress) {
    //     println!("{:?}", p)
    // }
    // let file = create_file(Some(|p| println!("prog>>-------->{:?}", p))).await?;
    // println!("{:#?}", file);

    //realtime

    // let (ws_stream, _response)= connect_async("wss://cloud.appwrite.io/v1/realtime?project=65d20d389f2b36778b8b&channels[]=documents&channels[]=databases.65d6ca3f97569a4c7ea8.collections").await.expect("Can't connect");

    // // //socket.send(Message::Text("Hello, Test".into())).unwrap();

    // let (mut _write, mut read) = ws_stream.split();

    // if let Some(message) = read.next().await {
    //     let msg = message.expect("Failed to read the message");
    //     let msg = match msg {
    //         Message::Text(s) => s,
    //         _ => {
    //             panic!()
    //         }
    //     };
    //     let parsed: Value = serde_json::from_str(&msg).expect("Unable to parse json");
    //     println!("{:#?}", parsed);
    // }

    // let a = Path::new(r"C:\Users\pc\Pictures\Imagine\6676892.jpg")
    //     .file_name()
    //     .unwrap();
    // println!("{}", format!("{:?}", a));

    Ok(())
}
