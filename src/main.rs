use unofficial_appwrite::{client::ClientBuilder, error::Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = ClientBuilder::default()
        .set_project("...")?
        .set_key("...")?
        //.set_self_signed(false)?
        .build()?;

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
