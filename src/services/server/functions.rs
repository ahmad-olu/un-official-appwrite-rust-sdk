use reqwest::header;
use serde_json::{json, Map, Value};

use crate::{
    client::{ChunksResponse, Client},
    enums::HttpMethod,
    error::Error,
    models::{
        deployment::Deployment, deployment_list::DeploymentList, execution::Execution,
        execution_list::ExecutionList, function::Func, function_list::FunctionList,
        runtime_list::RuntimeList, variable::Variable, variable_list::VariableList,
    },
    upload_progress::UploadProgress,
    utils::get_content_header_value,
};

/// The Functions Service allows you view, create and manage your Cloud
/// Functions.
struct Functions;

impl Functions {
    /// List functions
    ///
    /// Get a list of all the project"s functions. You can use the query params to
    /// filter your results.
    async fn list(
        client: &Client,
        queries: Option<Vec<&str>>,
        search: Option<&str>,
    ) -> Result<FunctionList, Error> {
        const API_PATH: &str = "/functions";
        // let api_path = "/avatars/browsers/{code}".replace("{code}", code);

        let mut api_params = serde_json::Map::new();
        if let Some(queries) = queries {
            api_params.insert("queries".to_string(), json!(queries));
        }
        if let Some(search) = search {
            api_params.insert("search".to_string(), json!(search));
        }

        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create function
    ///
    /// Create a new function. You can pass a list of
    /// [permissions](https://appwrite.io/docs/permissions) to allow different
    /// project users or team with access to execute the function using the client
    /// API.
    async fn create(
        client: &Client,
        function_id: &str,
        name: &str,
        runtime: &str,
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
    ) -> Result<Func, Error> {
        const API_PATH: &str = "/functions";
        // let api_path = "/avatars/browsers/{code}".replace("{code}", code);

        let mut api_params = Map::new();
        api_params.insert("functionId".to_string(), json!(function_id));
        api_params.insert("name".to_string(), json!(name));
        api_params.insert("runtime".to_string(), json!(runtime));
        if let Some(execute) = execute {
            api_params.insert("execute".to_string(), json!(execute));
        }
        if let Some(events) = events {
            api_params.insert("events".to_string(), json!(events));
        }
        if let Some(schedule) = schedule {
            api_params.insert("schedule".to_string(), json!(schedule));
        }
        if let Some(timeout) = timeout {
            api_params.insert("timeout".to_string(), json!(timeout));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        if let Some(logging) = logging {
            api_params.insert("logging".to_string(), json!(logging));
        }
        if let Some(entry_point) = entry_point {
            api_params.insert("entryPoint".to_string(), json!(entry_point));
        }
        if let Some(commands) = commands {
            api_params.insert("commands".to_string(), json!(commands));
        }
        if let Some(installation_id) = installation_id {
            api_params.insert("installationId".to_string(), json!(installation_id));
        }
        if let Some(provider_repository_id) = provider_repository_id {
            api_params.insert(
                "providerRepositoryId".to_string(),
                json!(provider_repository_id),
            );
        }
        if let Some(provider_branch) = provider_branch {
            api_params.insert("providerBranch".to_string(), json!(provider_branch));
        }
        if let Some(provider_silent_mode) = provider_silent_mode {
            api_params.insert(
                "providerSilentMode".to_string(),
                json!(provider_silent_mode),
            );
        }
        if let Some(provider_root_directory) = provider_root_directory {
            api_params.insert(
                "providerRootDirectory".to_string(),
                json!(provider_root_directory),
            );
        }
        if let Some(template_repository) = template_repository {
            api_params.insert("templateRepository".to_string(), json!(template_repository));
        }
        if let Some(template_owner) = template_owner {
            api_params.insert("templateOwner".to_string(), json!(template_owner));
        }
        if let Some(template_root_directory) = template_root_directory {
            api_params.insert(
                "templateRootDirectory".to_string(),
                json!(template_root_directory),
            );
        }
        if let Some(template_branch) = template_branch {
            api_params.insert("templateBranch".to_string(), json!(template_branch));
        }

        let api_params = Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List runtimes
    ///
    /// Get a list of all runtimes that are currently active on your instance.
    async fn list_run_times(client: &Client) -> Result<RuntimeList, Error> {
        const API_PATH: &str = "/functions/runtimes";
        // let api_path = "/avatars/browsers/{code}".replace("{code}", code);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Get function
    ///
    /// Get a function by its unique ID.
    async fn get(client: &Client, function_id: &str) -> Result<Func, Error> {
        // const API_PATH: &str = "/functions/runtimes";
        let api_path = "/functions/{functionId}".replace("{functionId}", function_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update function
    ///
    /// Update function by its unique ID.
    async fn update(
        client: &Client,
        function_id: &str,
        name: &str,
        runtime: Option<&str>,
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
    ) -> Result<Func, Error> {
        // const API_PATH: &str = "/functions/runtimes";
        let api_path = "/functions/{functionId}".replace("{functionId}", function_id);

        let mut api_params = serde_json::Map::new();
        api_params.insert("name".to_string(), json!(name));
        if let Some(runtime) = runtime {
            api_params.insert("runtime".to_string(), json!(runtime));
        }
        if let Some(execute) = execute {
            api_params.insert("execute".to_string(), json!(execute));
        }
        if let Some(events) = events {
            api_params.insert("events".to_string(), json!(events));
        }
        if let Some(schedule) = schedule {
            api_params.insert("schedule".to_string(), json!(schedule));
        }
        if let Some(timeout) = timeout {
            api_params.insert("timeout".to_string(), json!(timeout));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        if let Some(logging) = logging {
            api_params.insert("logging".to_string(), json!(logging));
        }
        if let Some(entry_point) = entry_point {
            api_params.insert("entrypoint".to_string(), json!(entry_point));
        }
        if let Some(commands) = commands {
            api_params.insert("commands".to_string(), json!(commands));
        }
        if let Some(installation_id) = installation_id {
            api_params.insert("installationId".to_string(), json!(installation_id));
        }
        if let Some(provider_repository_id) = provider_repository_id {
            api_params.insert(
                "providerRepositoryId".to_string(),
                json!(provider_repository_id),
            );
        }
        if let Some(provider_branch) = provider_branch {
            api_params.insert("providerBranch".to_string(), json!(provider_branch));
        }
        if let Some(provider_silent_mode) = provider_silent_mode {
            api_params.insert(
                "providerSilentMode".to_string(),
                json!(provider_silent_mode),
            );
        }
        if let Some(provider_root_directory) = provider_root_directory {
            api_params.insert(
                "providerRootDirectory".to_string(),
                json!(provider_root_directory),
            );
        }
        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PUT,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Delete function
    ///
    /// Delete a function by its unique ID.
    async fn delete(client: &Client, function_id: &str) -> Result<(), Error> {
        // const API_PATH: &str = "/functions/runtimes";
        let api_path = "/functions/{functionId}".replace("{functionId}", function_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let __res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(())
    }

    /// List deployments
    ///
    /// Get a list of all the project"s code deployments. You can use the query
    /// params to filter your results.
    async fn list_deployments(
        client: &Client,
        function_id: &str,
        queries: Option<Vec<&str>>,
        search: Option<&str>,
    ) -> Result<DeploymentList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/deployments".replace("{functionId}", function_id);

        let mut api_params = serde_json::Map::new();
        if let Some(queries) = queries {
            api_params.insert("queries".to_string(), json!(queries));
        }
        if let Some(search) = search {
            api_params.insert("search".to_string(), json!(search));
        }

        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create deployment
    ///
    /// Create a new function code deployment. Use this endpoint to upload a new
    /// version of your code function. To execute your newly uploaded code, you"ll
    /// need to update the function"s deployment to use your new deployment UID.
    ///
    /// This endpoint accepts a tar.gz file compressed with your code. Make sure to
    /// include any dependencies your code has within the compressed file. You can
    /// learn more about code packaging in the [Appwrite Cloud Functions
    /// tutorial](https://appwrite.io/docs/functions).
    ///
    /// Use the "command" param to set the entrypoint used to execute your code.
    ///
    async fn create_deployments(
        client: &Client,
        function_id: &str,
        // code: InputFile,
        activate: bool,
        entry_point: Option<&str>,
        commands: Option<&str>,
        on_progress: Option<fn(UploadProgress)>,
    ) -> Result<ChunksResponse<Deployment>, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/deployments".replace("{functionId}", function_id);

        let mut api_params = Map::new();
        api_params.insert("activate".to_string(), json!(activate));
        if let Some(entry_point) = entry_point {
            api_params.insert("entrypoint".to_string(), json!(entry_point));
        }
        if let Some(commands) = commands {
            api_params.insert("commands".to_string(), json!(commands));
        }

        let api_params = serde_json::Value::Object(api_params);

        let res: ChunksResponse<Deployment> = client
            .chunk_upload(
                "file_path",
                api_path.as_str(),
                function_id.to_string(),
                &api_params,
                on_progress,
            )
            .await?;

        Ok(res)
    }

    /// Get deployment
    ///
    /// Get a code deployment by its unique ID.
    async fn get_deployments(
        client: &Client,
        function_id: &str,
        deployment_id: &str,
    ) -> Result<Deployment, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/deployments/{deploymentId}"
            .replace("{functionId}", function_id)
            .replace("deploymentId", deployment_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update function deployment
    ///
    /// Update the function code deployment ID using the unique function ID. Use
    /// this endpoint to switch the code deployment that should be executed by the
    /// execution endpoint.
    async fn update_deployments(
        client: &Client,
        function_id: &str,
        deployment_id: &str,
    ) -> Result<Func, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/deployments/{deploymentId}"
            .replace("{functionId}", function_id)
            .replace("deploymentId", deployment_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Delete deployment
    ///
    /// Delete a code deployment by its unique ID.
    async fn delete_deployments(
        client: &Client,
        function_id: &str,
        deployment_id: &str,
    ) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/deployments/{deploymentId}"
            .replace("{functionId}", function_id)
            .replace("deploymentId", deployment_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(())
    }

    /// Create build
    ///
    /// Create a new build for an Appwrite Function deployment. This endpoint can
    /// be used to retry a failed build.
    async fn create_build(
        client: &Client,
        function_id: &str,
        deployment_id: &str,
        build_id: &str,
    ) -> Result<Value, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/deployments/{deploymentId}/builds/{buildId}"
            .replace("{functionId}", function_id)
            .replace("deploymentId", deployment_id)
            .replace("buildId", build_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Download Deployment
    ///
    /// Get a Deployment's contents by its unique ID. This endpoint supports range
    /// requests for partial or streaming file download.
    async fn download_deployment(
        client: &Client,
        function_id: &str,
        deployment_id: &str,
    ) -> Result<Vec<u8>, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/deployments/{deploymentId}/download"
            .replace("{functionId}", function_id)
            .replace("deploymentId", deployment_id);

        let api_params = serde_json::json!({
            "project": get_content_header_value(&client, "project"),
            "key": get_content_header_value(&client, "key"),
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

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

    /// List executions
    ///
    /// Get a list of all the current user function execution logs. You can use the
    /// query params to filter your results.
    async fn list_executions(
        client: &Client,
        function_id: &str,
        queries: Option<Vec<&str>>,
        search: Option<&str>,
    ) -> Result<ExecutionList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/executions".replace("{functionId}", function_id);

        let mut api_params = serde_json::Map::new();
        if let Some(queries) = queries {
            api_params.insert("queries".to_string(), json!(queries));
        }
        if let Some(search) = search {
            api_params.insert("search".to_string(), json!(search));
        }

        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create execution
    ///
    /// Trigger a function execution. The returned object will return you the
    /// current execution status. You can ping the `Get Execution` endpoint to get
    /// updates on the current execution status. Once this endpoint is called, your
    /// function execution process will start asynchronously.
    async fn create_executions(
        client: &Client,
        function_id: &str,
        body: Option<&str>,
        x_async: Option<bool>,
        path: Option<&str>,
        method: Option<&str>,
        headers: Option<Map<String, Value>>, //should be Map
    ) -> Result<Execution, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/executions".replace("{functionId}", function_id);

        let mut api_params = Map::new();
        if let Some(body) = body {
            api_params.insert("body".to_string(), json!(body));
        }
        if let Some(x_async) = x_async {
            api_params.insert("async".to_string(), json!(x_async));
        }
        if let Some(path) = path {
            api_params.insert("path".to_string(), json!(path));
        }
        if let Some(method) = method {
            api_params.insert("method".to_string(), json!(method));
        }
        if let Some(headers) = headers {
            api_params.insert("headers".to_string(), json!(headers));
        }

        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Get execution
    ///
    /// Get a function execution log by its unique ID.
    async fn get_executions(
        client: &Client,
        function_id: &str,
        execution_id: &str,
    ) -> Result<Execution, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/executions/{executionId}"
            .replace("{functionId}", function_id)
            .replace("{executionId}", execution_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// List variables
    ///
    /// Get a list of all variables of a specific function.
    async fn list_variables(client: &Client, function_id: &str) -> Result<VariableList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/variables".replace("{functionId}", function_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create variable
    ///
    /// Create a new function environment variable. These variables can be accessed
    /// in the function at runtime as environment variables.
    async fn create_variables(
        client: &Client,
        function_id: &str,
        key: &str,
        value: &str,
    ) -> Result<Variable, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/variables".replace("{functionId}", function_id);

        let api_params = serde_json::json!({
            "key":key,
            "value":value,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Get variable
    ///
    /// Get a variable by its unique ID.
    async fn get_variables(
        client: &Client,
        function_id: &str,
        variable_id: &str,
    ) -> Result<Variable, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/variables/{variableId}"
            .replace("{functionId}", function_id)
            .replace("{variableId}", variable_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update variable
    ///
    /// Update variable by its unique ID.
    async fn update_variables(
        client: &Client,
        function_id: &str,
        variable_id: &str,
        key: &str,
        value: Option<&str>,
    ) -> Result<Variable, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/variables/{variableId}"
            .replace("{functionId}", function_id)
            .replace("{variableId}", variable_id);

        let mut api_params = Map::new();
        api_params.insert("key".to_string(), json!(key));
        if let Some(value) = value {
            api_params.insert("value".to_string(), json!(value));
        }

        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PUT,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Delete variable
    ///
    /// Delete a variable by its unique ID.
    async fn delete_variables(
        client: &Client,
        function_id: &str,
        variable_id: &str,
    ) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/variables/{variableId}"
            .replace("{functionId}", function_id)
            .replace("{variableId}", variable_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(())
    }
}
