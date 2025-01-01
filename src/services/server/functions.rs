//! # Functions
//!
//! The Functions Service allows you view, create and manage your Cloud
//! Functions.

use std::collections::HashMap;

use futures_util::Stream;
use serde_json::Value;

use crate::{
    app_json_header,
    client::Client,
    enumm::HttpMethod,
    error::Error,
    models::{
        deployment::Deployment, deployment_list::DeploymentList, execution::Execution,
        execution_list::ExecutionList, function::Func, function_list::FunctionList,
        runtime_list::RuntimeList, variable::Variable, variable_list::VariableList, UploadType,
    },
    upload_progress::UploadProgress,
    utils::get_content_header_value,
};

pub struct Functions;

impl Functions {
    /// List functions
    ///
    /// Get a list of all the project"s functions. You can use the query params to
    /// filter your results.
    ///* queries => vec(string)?
    ///* search => string?
    pub async fn list(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<FunctionList, Error> {
        const API_PATH: &str = "/functions";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create function
    ///
    /// Create a new function. You can pass a list of
    /// [permissions](https://appwrite.io/docs/permissions) to allow different
    /// project users or team with access to execute the function using the client
    /// API.
    ///* functionId => string
    ///* name => string
    ///* runtime => Runtime
    ///* execute => vec(string)?
    ///* events => vec(string)?
    ///* schedule => string?
    ///* timeout => number?
    ///* enabled => bool?
    ///* logging => bool?
    ///* entryPoint => string?
    ///* commands => string?
    ///* installationId => string?
    ///* providerRepositoryId => string?
    ///* providerBranch => string?
    ///* providerSilentMode => bool?
    ///* providerRootDirectory => string?
    ///* templateRepository => string?
    ///* templateOwner => string?
    ///* templateRootDirectory => string?
    ///* templateBranch => string?
    pub async fn create(client: &Client, args: HashMap<String, Value>) -> Result<Func, Error> {
        const API_PATH: &str = "/functions";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, API_PATH, api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List runtimes
    ///
    /// Get a list of all runtimes that are currently active on your instance.
    pub async fn list_run_times(client: &Client) -> Result<RuntimeList, Error> {
        const API_PATH: &str = "/functions/runtimes";
        // let api_path = "/avatars/browsers/{code}".replace("{code}", code);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Get function
    ///
    /// Get a function by its unique ID.
    pub async fn get(client: &Client, function_id: &str) -> Result<Func, Error> {
        // const API_PATH: &str = "/functions/runtimes";
        let api_path = "/functions/{functionId}".replace("{functionId}", function_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update function
    ///
    /// Update function by its unique ID.
    ///* functionId => string
    ///* name => string
    ///* runtime => Runtime?
    ///* execute => vec(string)?
    ///* events => vec(string)?
    ///* schedule => string?
    ///* timeout => number?
    ///* enabled => bool?
    ///* logging => bool?
    ///* entryPoint => string?
    ///* commands => string?
    ///* installationId => string?
    ///* providerRepositoryId => string?
    ///* providerBranch => string?
    ///* providerSilentMode => bool?
    ///* providerRootDirectory => string?
    pub async fn update(
        client: &Client,
        function_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Func, Error> {
        // const API_PATH: &str = "/functions/runtimes";
        let api_path = "/functions/{functionId}".replace("{functionId}", function_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PUT, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Delete function
    ///
    /// Delete a function by its unique ID.
    pub async fn delete(client: &Client, function_id: &str) -> Result<(), Error> {
        // const API_PATH: &str = "/functions/runtimes";
        let api_path = "/functions/{functionId}".replace("{functionId}", function_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let __res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(())
    }

    /// List deployments
    ///
    /// Get a list of all the project"s code deployments. You can use the query
    /// params to filter your results.
    ///* queries => vec(string)?
    ///* search => string?
    pub async fn list_deployments(
        client: &Client,
        function_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<DeploymentList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/deployments".replace("{functionId}", function_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
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
    ///* activate => bool
    ///* entrypoint => string?
    ///* commands => string?
    pub async fn create_deployments(
        client: &Client,
        function_id: &str,
        // code: InputFile,
        file_path: &str,
        file_name: String,
        args: HashMap<String, Value>,
    ) -> Result<Deployment, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/deployments".replace("{functionId}", function_id);

        let res: UploadType = client
            .chunk_upload_file(
                file_path,
                api_path.as_str(),
                function_id.to_string(),
                args,
                file_name,
                false,
            )
            .await?;

        match res {
            UploadType::File(_) => Err(Error::WrongUploadType),
            UploadType::Deployment(res) => Ok(res),
        }
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
    ///* activate => bool
    ///* entrypoint => string?
    ///* commands => string?
    pub async fn create_deployments_streamed<'a>(
        client: &'a Client,
        function_id: &'a str,
        // code: InputFile,
        file_path: &'a str,
        file_name: String,
        args: HashMap<String, Value>,
    ) -> impl Stream<Item = Result<(UploadType, UploadProgress), Error>> + 'a {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/deployments".replace("{functionId}", function_id);

        client
            .chunk_upload_file_streamed(
                file_path,
                api_path,
                String::from(function_id),
                args,
                file_name,
                true,
            )
            .await
    }

    /// Get deployment
    ///
    /// Get a code deployment by its unique ID.
    pub async fn get_deployments(
        client: &Client,
        function_id: &str,
        deployment_id: &str,
    ) -> Result<Deployment, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/deployments/{deploymentId}"
            .replace("{functionId}", function_id)
            .replace("deploymentId", deployment_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update function deployment
    ///
    /// Update the function code deployment ID using the unique function ID. Use
    /// this endpoint to switch the code deployment that should be executed by the
    /// execution endpoint.
    pub async fn update_deployments(
        client: &Client,
        function_id: &str,
        deployment_id: &str,
    ) -> Result<Func, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/deployments/{deploymentId}"
            .replace("{functionId}", function_id)
            .replace("deploymentId", deployment_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Delete deployment
    ///
    /// Delete a code deployment by its unique ID.
    pub async fn delete_deployments(
        client: &Client,
        function_id: &str,
        deployment_id: &str,
    ) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/deployments/{deploymentId}"
            .replace("{functionId}", function_id)
            .replace("deploymentId", deployment_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(())
    }

    /// Create build
    ///
    /// Create a new build for an Appwrite Function deployment. This endpoint can
    /// be used to retry a failed build.
    pub async fn create_build(
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Download Deployment
    ///
    /// Get a Deployment's contents by its unique ID. This endpoint supports range
    /// requests for partial or streaming file download.
    pub async fn download_deployment(
        client: &Client,
        function_id: &str,
        deployment_id: &str,
        mut args: HashMap<String, Value>,
    ) -> Result<Vec<u8>, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/deployments/{deploymentId}/download"
            .replace("{functionId}", function_id)
            .replace("deploymentId", deployment_id);

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

    /// List executions
    ///
    /// Get a list of all the current user function execution logs. You can use the
    /// query params to filter your results.
    ///* queries => vec(string)?
    ///* search => string?
    pub async fn list_executions(
        client: &Client,
        function_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<ExecutionList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/executions".replace("{functionId}", function_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create execution
    ///
    /// Trigger a function execution. The returned object will return you the
    /// current execution status. You can ping the `Get Execution` endpoint to get
    /// updates on the current execution status. Once this endpoint is called, your
    /// function execution process will start asynchronously.
    ///* body => string?
    ///* async => bool?
    ///* path => string?
    ///* method => ExecutionMethod?
    ///* headers => Hashmap<String,Value>?
    pub async fn create_executions(
        client: &Client,
        function_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Execution, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/executions".replace("{functionId}", function_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Get execution
    ///
    /// Get a function execution log by its unique ID.
    pub async fn get_executions(
        client: &Client,
        function_id: &str,
        execution_id: &str,
    ) -> Result<Execution, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/executions/{executionId}"
            .replace("{functionId}", function_id)
            .replace("{executionId}", execution_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List variables
    ///
    /// Get a list of all variables of a specific function.
    pub async fn list_variables(client: &Client, function_id: &str) -> Result<VariableList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/variables".replace("{functionId}", function_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create variable
    ///
    /// Create a new function environment variable. These variables can be accessed
    /// in the function at runtime as environment variables.
    ///* key => string
    ///* value => string
    pub async fn create_variables(
        client: &Client,
        function_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Variable, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/variables".replace("{functionId}", function_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Get variable
    ///
    /// Get a variable by its unique ID.
    pub async fn get_variables(
        client: &Client,
        function_id: &str,
        variable_id: &str,
    ) -> Result<Variable, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/variables/{variableId}"
            .replace("{functionId}", function_id)
            .replace("{variableId}", variable_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update variable
    ///
    /// Update variable by its unique ID.
    ///* key => string
    ///* value => string?
    pub async fn update_variables(
        client: &Client,
        function_id: &str,
        variable_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Variable, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/variables/{variableId}"
            .replace("{functionId}", function_id)
            .replace("{variableId}", variable_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PUT, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Delete variable
    ///
    /// Delete a variable by its unique ID.
    pub async fn delete_variables(
        client: &Client,
        function_id: &str,
        variable_id: &str,
    ) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/functions/{functionId}/variables/{variableId}"
            .replace("{functionId}", function_id)
            .replace("{variableId}", variable_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(())
    }
}
