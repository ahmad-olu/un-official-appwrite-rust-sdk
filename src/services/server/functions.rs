use reqwest::header;
use serde_json::{Map, Value};

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

        let api_params = serde_json::json!({
            "queries":queries,
            "search":search,
        });

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

        let api_params = serde_json::json!({
            "functionId": function_id,
        "name": name,
        "runtime": runtime,
        "execute": execute,
        "events": events,
        "schedule": schedule,
        "timeout": timeout,
        "enabled": enabled,
        "logging": logging,
        "entrypoint": entry_point,
        "commands": commands,
        "installationId": installation_id,
        "providerRepositoryId": provider_repository_id,
        "providerBranch": provider_branch,
        "providerSilentMode": provider_silent_mode,
        "providerRootDirectory": provider_root_directory,
        "templateRepository": template_repository,
        "templateOwner": template_owner,
        "templateRootDirectory": template_root_directory,
        "templateBranch": template_branch,
        });

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

        let api_params = serde_json::json!({
            "name": name,
            "runtime": runtime,
            "execute": execute,
            "events": events,
            "schedule": schedule,
            "timeout": timeout,
            "enabled": enabled,
            "logging": logging,
            "entrypoint": entry_point,
            "commands": commands,
            "installationId": installation_id,
            "providerRepositoryId": provider_repository_id,
            "providerBranch": provider_branch,
            "providerSilentMode": provider_silent_mode,
            "providerRootDirectory": provider_root_directory,
        });

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

        let api_params = serde_json::json!({
            "queries":queries,
            "search":search,
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

        let api_params = serde_json::json!({
            "entrypoint": entry_point,
            "commands": commands,
          //  "code": code.,
            "activate": activate,
        });

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

        let api_params = serde_json::json!({
            "queries":queries,
            "search":search,
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
        headers: Option<Value>, //should be Map
    ) -> Result<Execution, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/executions".replace("{functionId}", function_id);

        let api_params = serde_json::json!({
            "body":body,
            "async":x_async,
            "path": path,
            "method":method,
            "headers": headers,
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

        let api_params = serde_json::json!({
            "key":key,
            "value":value,
        });

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
