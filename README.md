# Unofficial Appwrite Rust SDK

-----

This SDK is compatible with Appwrite server version 1.5.x.
This is the Rust SDK for integrating with Appwrite from your Rust server-side code

-----

Appwrite is an open-source backend as a service server that abstract and simplify complex and repetitive development tasks behind a very simple to use REST API. Appwrite aims to help you develop your apps faster and in a more secure way. Use the Rust to integrate your app with the Appwrite server to easily start interacting with all of Appwrite backend APIs and tools. For full API documentation and tutorials go to <https://appwrite.io/docs>

## Getting Started

-----

### Initialize & Make API Request

-----
Once you add the dependencies, its extremely easy to get started with the SDK; All you need to do is import the package in your code, set your Appwrite credentials, and start making API calls. Below is a simple example:

``` rust

use unofficial_appwrite::client::ClientBuilder;
use unofficial_appwrite::error::Error;
use unofficial_appwrite::services::server::users::Users;
use unofficial_appwrite::id::ID;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let  client = ClientBuilder::default()
    .set_endpoint("http://[HOSTNAME_OR_IP]/v1") // Make sure your endpoint is accessible
    .set_project("5ff...")? // Your project ID
    .set_key("cd868c7af8bdc893b4...93b7535db89")?
    //.set_self_signed(false)? // Use only on dev mode with a self-signed SSL cert
    .build()?;

    //create new user
 let create = Users::create(
            &client,
            maplit::hashmap! {
                "userId".into() => ID::unique(7).into(),
                "email".into()=> "fakeEmail@Email.com".into(),
                "password".into()=> "VeryVerySecurePassword@123456789".into(),
                "name".into()=> "fakeEmail".into()
            },
        )
        .await?;
        assert_eq!(create.email, "fakeemail@email.com");
}
```

### Other Examples

- check out the test in the sdk for examples : `https://github.com/ahmad-olu/un-official-appwrite-rust-sdk`

-----

#### Storage

```rust
use unofficial_appwrite::client::ClientBuilder;
use unofficial_appwrite::error::Error;
use unofficial_appwrite::id::ID;
use unofficial_appwrite::services::server::storage::Storage;
use futures_util::{pin_mut, StreamExt};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Error> {

     let  client = ClientBuilder::default()
    .set_endpoint("http://[HOSTNAME_OR_IP]/v1") // Make sure your endpoint is accessible
    .set_project("5ff3379a01d25")? // Your project ID
    .set_key("cd868c7af8bdc893b4...93b7535db89")?
    //.set_self_signed(false)? // Use only on dev mode with a self-signed SSL cert
    .build()?;

    // create a file without getting upload progress
    let id = ID::unique_old().into();
    let create_file = Storage::create_files(
        &client,
        "6773f8af000602e81619".to_string(),
        id,
        file_path.to_string(),
        file_name.to_string(),
        HashMap::<String, Value>::new(),
        |progress| {
            println!(
                "Uploaded: {}/{} ({}%), ID: {}",
                progress.size_uploaded,
                (progress.chunks_total as usize) * progress.size_uploaded, // Approximate total size
                (progress.progress * 100.0).round(),
                progress.id,
            );
        },
    )
    .await?;
    dbg!(create_file);
}

```

#### Realtime

```rust
use futures_util::{pin_mut, StreamExt};
use unofficial_appwrite::{client::ClientBuilder, error::Error, realtime::RealTime};

#[tokio::main]
async fn main() -> Result<(), Error> {

     let  client = ClientBuilder::default()
    .set_endpoint("http://[HOSTNAME_OR_IP]/v1") // Make sure your endpoint is accessible
    .set_project("5ff3...")? // Your project ID
    .set_key("cd868c7af8bdc893b4...93b7535db89")?
    //.set_self_signed(false)? // Use only on dev mode with a self-signed SSL cert
    .build()?;

    // subscribe to realtime
 let stream = RealTime::subscribe(
        &client,
        vec!["databases.6618eec286a4ef198076.collections.6618ef06d269bf4110d4.documents"],
    )
    .await;
    pin_mut!(stream);
    while let Some(data) = stream.next().await {
        println!("=====>{:?}<=====", data);
    }
}
```

#### Utilities

##### Queries

```rust
    unofficial_appwrite::query::Query::equal("title".into(), vec!["bamboo", "ace"].into());
    unofficial_appwrite::query::Query::less_than("score".into(), 10.into());
    unofficial_appwrite::query::Query::or(
                 vec![
                     unofficial_appwrite::query::Query::less_than("size".into(), 5.into()),
                     unofficial_appwrite::query::Query::greater_than("size".into(), 10.into())
                 ]
                 .into()
             );
```

##### Id

```rust
    use unofficial_appwrite::id::ID;
    ID::unique(7).into();
    //or
    ID::unique_old().into();
```

##### Permission

```rust
    use unofficial_appwrite::permission::Permission;
    Permission::read("any"),
    Permission::create("user:22222346"),
```

##### Role

```rust
    use unofficial_appwrite::role::Role;
    Role::any()
```

NOTE: for other examples. check out the official docs or sdk of official sdk as a guide to using this sdk.

### Learn more

-----
You can use the following resources to learn more and get help

- 🚀 Getting Started Tutorial - <https://appwrite.io/docs/getting-started-for-server>
- 📜 Appwrite Docs - <https://appwrite.io/docs>
- 💬 Discord Community - <https://appwrite.io/discord>

### What next

-----

- Clean up some of the excess code

### contributing

-----

## API Changes in `unofficial-apperite_rust-sdk`

### Why the Changes?

The latest version of the `unofficial-apperite_rust-sdk` introduces significant API updates to address two primary goals:

1. Ease of Use
The new API is designed to be simpler and cleaner. Previously, function signatures in Rust could become overly verbose due to the lack of optional function arguments, unlike other languages such as Dart, JavaScript, or Python, which provide more flexible parameter handling. For example:

```rust
pub async fn create(
    client: &Client,
    function_id: &str,
    name: &str,
    runtime: Runtime,
    execute: Option<Vec<&str>>,
    events: Option<Vec<&str>>,
    schedule: Option<&str>,
    timeout: Option<u64>,
    enabled: Option<bool>,
    logging: Option<bool>,
    entry_point: Option<&str>,
    commands: Option<&str>,
    installation_id: Option<&str>,
    provider_repository_id: Option<&str>,
    provider_branch: Option<&str>,
    provider_silent_mode: Option<bool>,
    provider_root_directory: Option<&str>,
    template_repository: Option<&str>,
    template_owner: Option<&str>,
    template_root_directory: Option<&str>,
    template_branch: Option<&str>,
) -> Result<Func, Error> { ... }
```

Now, instead of this "polluted" approach, the new API allows you to pass required and optional parameters as a HashMap:

```rust
pub async fn create(client: &Client, args: HashMap<String, Value>) -> Result<Func, Error> { ... }
```

2. Improved Developer Workflow
As the sole developer maintaining this SDK without direct collaboration with the Appwrite team, keeping up with API changes is challenging. To stay updated, I often have to analyze other SDKs like Dart. This new structure simplifies updates by making the function signatures more flexible and future-proof.

### How to Use the New API

To pass parameters, use a `HashMap` with the required keys and values. Required parameters are documented as `* parameterName => type`, and optional ones are indicated by a `?`, like `* timeout => number?`. Examples of parameter types include:

- `string` for text values
- `number` for integers
- `floats` for float
- `bool` for boolean values
- `vec(string)` or `list(string)` for vectors
Here’s an example usage:

```rust
maplit::hashmap! {
    "userId".into() => ID::unique(7).into(),
    "email".into() => "fakeEmailAcc@Email.com".into(),
    "password".into() => "VerySecurePassword123!".into(),
}
```

### On the Subject of IDs

The new API introduces improved ID handling, but backward compatibility is maintained:

- Use `ID::unique(7).into()` for the updated API.
- Use `ID::unique_old().into()` where required, such as with Storage::create_files(...), until the upload file functionality is refactored.
The goal is to ensure a smoother development experience while maintaining backward compatibility during the transition.
