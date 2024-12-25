// //! # Functions
// //!
// //! The Functions Service allows you view, create and manage your Cloud
// //! Functions.

// use futures_util::Stream;
// use serde_json::{Map, Value};

// use crate::{
//     api_params, app_json_header,
//     client::Client,
//     enumm::HttpMethod,
//     enums::{execution_method::ExecutionMethod, runtime::Runtime},
//     error::Error,
//     models::{
//         deployment::Deployment, deployment_list::DeploymentList, execution::Execution,
//         execution_list::ExecutionList, function::Func, function_list::FunctionList,
//         runtime_list::RuntimeList, variable::Variable, variable_list::VariableList, UploadType,
//     },
//     upload_progress::UploadProgress,
//     utils::get_content_header_value,
// };

// pub struct Functions;

// impl Functions {
//     /// List functions
//     ///
//     /// Get a list of all the project"s functions. You can use the query params to
//     /// filter your results.
//     pub async fn list(
//         client: &Client,
//         queries: Option<Vec<String>>,
//         search: Option<String>,
//     ) -> Result<FunctionList, Error> {
//         const API_PATH: &str = "/functions";

//         let api_params = api_params!(
//             "queries"=> queries,
//             "search"=>search,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create function
//     ///
//     /// Create a new function. You can pass a list of
//     /// [permissions](https://appwrite.io/docs/permissions) to allow different
//     /// project users or team with access to execute the function using the client
//     /// API.
//     pub async fn create(
//         client: &Client,
//         function_id: &str,
//         name: &str,
//         runtime: Runtime,
//         execute: Option<Vec<&str>>,
//         events: Option<Vec<&str>>,
//         schedule: Option<&str>,
//         timeout: Option<u64>,
//         enabled: Option<bool>,
//         logging: Option<bool>,
//         entry_point: Option<&str>,
//         commands: Option<&str>,
//         installation_id: Option<&str>,
//         provider_repository_id: Option<&str>,
//         provider_branch: Option<&str>,
//         provider_silent_mode: Option<bool>,
//         provider_root_directory: Option<&str>,
//         template_repository: Option<&str>,
//         template_owner: Option<&str>,
//         template_root_directory: Option<&str>,
//         template_branch: Option<&str>,
//     ) -> Result<Func, Error> {
//         const API_PATH: &str = "/functions";

//         let api_params = api_params!(
//             "functionId"=> Some(function_id),
//             "name"=>Some(name),
//             "runtime"=>Some(runtime),
//             "execute"=>execute,
//             "events"=> events,
//             "schedule"=> schedule,
//             "timeout"=> timeout,
//             "enabled"=> enabled,
//             "logging"=> logging,
//             "entryPoint"=> entry_point,
//             "commands"=> commands,
//             "installationId"=> installation_id,
//             "providerRepositoryId"=> provider_repository_id,
//             "providerBranch"=> provider_branch,
//             "providerSilentMode"=> provider_silent_mode,
//             "providerRootDirectory"=> provider_root_directory,
//             "templateRepository"=> template_repository,
//             "templateOwner"=> template_owner,
//             "templateRootDirectory"=> template_root_directory,
//             "templateBranch"=> template_branch,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, API_PATH, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// List runtimes
//     ///
//     /// Get a list of all runtimes that are currently active on your instance.
//     pub async fn list_run_times(client: &Client) -> Result<RuntimeList, Error> {
//         const API_PATH: &str = "/functions/runtimes";
//         // let api_path = "/avatars/browsers/{code}".replace("{code}", code);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get function
//     ///
//     /// Get a function by its unique ID.
//     pub async fn get(client: &Client, function_id: &str) -> Result<Func, Error> {
//         // const API_PATH: &str = "/functions/runtimes";
//         let api_path = "/functions/{functionId}".replace("{functionId}", function_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update function
//     ///
//     /// Update function by its unique ID.
//     pub async fn update(
//         client: &Client,
//         function_id: &str,
//         name: &str,
//         runtime: Option<Runtime>,
//         execute: Option<Vec<&str>>,
//         events: Option<Vec<&str>>,
//         schedule: Option<&str>,
//         timeout: Option<u64>,
//         enabled: Option<bool>,
//         logging: Option<bool>,
//         entry_point: Option<&str>,
//         commands: Option<&str>,
//         installation_id: Option<&str>,
//         provider_repository_id: Option<&str>,
//         provider_branch: Option<&str>,
//         provider_silent_mode: Option<bool>,
//         provider_root_directory: Option<&str>,
//     ) -> Result<Func, Error> {
//         // const API_PATH: &str = "/functions/runtimes";
//         let api_path = "/functions/{functionId}".replace("{functionId}", function_id);

//         let api_params = api_params!(
//             "name"=>Some(name),
//             "runtime"=>runtime,
//             "execute"=>execute,
//             "events"=> events,
//             "schedule"=> schedule,
//             "timeout"=> timeout,
//             "enabled"=> enabled,
//             "logging"=> logging,
//             "entryPoint"=> entry_point,
//             "commands"=> commands,
//             "installationId"=> installation_id,
//             "providerRepositoryId"=> provider_repository_id,
//             "providerBranch"=> provider_branch,
//             "providerSilentMode"=> provider_silent_mode,
//             "providerRootDirectory"=> provider_root_directory,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PUT,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Delete function
//     ///
//     /// Delete a function by its unique ID.
//     pub async fn delete(client: &Client, function_id: &str) -> Result<(), Error> {
//         // const API_PATH: &str = "/functions/runtimes";
//         let api_path = "/functions/{functionId}".replace("{functionId}", function_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let __res = client
//             .call(
//                 HttpMethod::DELETE,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(())
//     }

//     /// List deployments
//     ///
//     /// Get a list of all the project"s code deployments. You can use the query
//     /// params to filter your results.
//     pub async fn list_deployments(
//         client: &Client,
//         function_id: &str,
//         queries: Option<Vec<String>>,
//         search: Option<String>,
//     ) -> Result<DeploymentList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/deployments".replace("{functionId}", function_id);

//         let api_params = api_params!(
//             "queries"=> queries,
//             "search"=>search,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create deployment
//     ///
//     /// Create a new function code deployment. Use this endpoint to upload a new
//     /// version of your code function. To execute your newly uploaded code, you"ll
//     /// need to update the function"s deployment to use your new deployment UID.
//     ///
//     /// This endpoint accepts a tar.gz file compressed with your code. Make sure to
//     /// include any dependencies your code has within the compressed file. You can
//     /// learn more about code packaging in the [Appwrite Cloud Functions
//     /// tutorial](https://appwrite.io/docs/functions).
//     ///
//     /// Use the "command" param to set the entrypoint used to execute your code.
//     ///
//     pub async fn create_deployments(
//         client: &Client,
//         function_id: &str,
//         // code: InputFile,
//         file_path: &str,
//         file_name: String,
//         activate: bool,
//         entrypoint: Option<&str>,
//         commands: Option<&str>,
//     ) -> Result<Deployment, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/deployments".replace("{functionId}", function_id);

//         let api_params = api_params!(
//             "activate"=> Some(activate),
//             "entrypoint"=>entrypoint,
//             "commands"=> commands,
//         );

//         let res: UploadType = client
//             .chunk_upload_file(
//                 file_path,
//                 api_path.as_str(),
//                 function_id.to_string(),
//                 &api_params,
//                 file_name,
//                 false,
//             )
//             .await?;

//         match res {
//             UploadType::File(_) => Err(Error::WrongUploadType),
//             UploadType::Deployment(res) => Ok(res),
//         }
//     }

//     /// Create deployment
//     ///
//     /// Create a new function code deployment. Use this endpoint to upload a new
//     /// version of your code function. To execute your newly uploaded code, you"ll
//     /// need to update the function"s deployment to use your new deployment UID.
//     ///
//     /// This endpoint accepts a tar.gz file compressed with your code. Make sure to
//     /// include any dependencies your code has within the compressed file. You can
//     /// learn more about code packaging in the [Appwrite Cloud Functions
//     /// tutorial](https://appwrite.io/docs/functions).
//     ///
//     /// Use the "command" param to set the entrypoint used to execute your code.
//     ///
//     pub async fn create_deployments_streamed<'a>(
//         client: &'a Client,
//         function_id: &'a str,
//         // code: InputFile,
//         file_path: &'a str,
//         file_name: String,
//         activate: bool,
//         entrypoint: Option<&'a str>,
//         commands: Option<&'a str>,
//     ) -> impl Stream<Item = Result<(UploadType, UploadProgress), Error>> + 'a {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/deployments".replace("{functionId}", function_id);

//         let api_params = api_params!(
//             "activate"=> Some(activate),
//             "entrypoint"=>entrypoint,
//             "commands"=> commands,
//         );

//         client
//             .chunk_upload_file_streamed(
//                 file_path,
//                 api_path,
//                 String::from(function_id),
//                 api_params,
//                 file_name,
//                 true,
//             )
//             .await
//     }

//     /// Get deployment
//     ///
//     /// Get a code deployment by its unique ID.
//     pub async fn get_deployments(
//         client: &Client,
//         function_id: &str,
//         deployment_id: &str,
//     ) -> Result<Deployment, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/deployments/{deploymentId}"
//             .replace("{functionId}", function_id)
//             .replace("deploymentId", deployment_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update function deployment
//     ///
//     /// Update the function code deployment ID using the unique function ID. Use
//     /// this endpoint to switch the code deployment that should be executed by the
//     /// execution endpoint.
//     pub async fn update_deployments(
//         client: &Client,
//         function_id: &str,
//         deployment_id: &str,
//     ) -> Result<Func, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/deployments/{deploymentId}"
//             .replace("{functionId}", function_id)
//             .replace("deploymentId", deployment_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Delete deployment
//     ///
//     /// Delete a code deployment by its unique ID.
//     pub async fn delete_deployments(
//         client: &Client,
//         function_id: &str,
//         deployment_id: &str,
//     ) -> Result<(), Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/deployments/{deploymentId}"
//             .replace("{functionId}", function_id)
//             .replace("deploymentId", deployment_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let _res = client
//             .call(
//                 HttpMethod::DELETE,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(())
//     }

//     /// Create build
//     ///
//     /// Create a new build for an Appwrite Function deployment. This endpoint can
//     /// be used to retry a failed build.
//     pub async fn create_build(
//         client: &Client,
//         function_id: &str,
//         deployment_id: &str,
//         build_id: &str,
//     ) -> Result<Value, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/deployments/{deploymentId}/builds/{buildId}"
//             .replace("{functionId}", function_id)
//             .replace("deploymentId", deployment_id)
//             .replace("buildId", build_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Download Deployment
//     ///
//     /// Get a Deployment's contents by its unique ID. This endpoint supports range
//     /// requests for partial or streaming file download.
//     pub async fn download_deployment(
//         client: &Client,
//         function_id: &str,
//         deployment_id: &str,
//     ) -> Result<Vec<u8>, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/deployments/{deploymentId}/download"
//             .replace("{functionId}", function_id)
//             .replace("deploymentId", deployment_id);

//         let api_params = api_params!(
//             "project"=> get_content_header_value(&client, "project"),
//             "key"=> get_content_header_value(&client, "key"),
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.bytes().await?.to_vec())
//     }

//     /// List executions
//     ///
//     /// Get a list of all the current user function execution logs. You can use the
//     /// query params to filter your results.
//     pub async fn list_executions(
//         client: &Client,
//         function_id: &str,
//         queries: Option<Vec<String>>,
//         search: Option<String>,
//     ) -> Result<ExecutionList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/executions".replace("{functionId}", function_id);

//         let api_params = api_params!(
//             "queries"=>queries,
//             "search"=> search,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create execution
//     ///
//     /// Trigger a function execution. The returned object will return you the
//     /// current execution status. You can ping the `Get Execution` endpoint to get
//     /// updates on the current execution status. Once this endpoint is called, your
//     /// function execution process will start asynchronously.
//     pub async fn create_executions(
//         client: &Client,
//         function_id: &str,
//         body: Option<&str>,
//         x_async: Option<bool>,
//         path: Option<&str>,
//         method: Option<ExecutionMethod>,
//         headers: Option<Map<String, Value>>, //should be Map
//     ) -> Result<Execution, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/executions".replace("{functionId}", function_id);

//         let api_params = api_params!(
//             "body"=> body,
//             "async"=>x_async,
//             "path"=> path,
//             "method"=> method,
//             "headers"=> headers,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get execution
//     ///
//     /// Get a function execution log by its unique ID.
//     pub async fn get_executions(
//         client: &Client,
//         function_id: &str,
//         execution_id: &str,
//     ) -> Result<Execution, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/executions/{executionId}"
//             .replace("{functionId}", function_id)
//             .replace("{executionId}", execution_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// List variables
//     ///
//     /// Get a list of all variables of a specific function.
//     pub async fn list_variables(client: &Client, function_id: &str) -> Result<VariableList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/variables".replace("{functionId}", function_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create variable
//     ///
//     /// Create a new function environment variable. These variables can be accessed
//     /// in the function at runtime as environment variables.
//     pub async fn create_variables(
//         client: &Client,
//         function_id: &str,
//         key: &str,
//         value: &str,
//     ) -> Result<Variable, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/variables".replace("{functionId}", function_id);

//         let api_params = api_params!(
//             "key"=> Some(key),
//             "value"=>Some(value),
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get variable
//     ///
//     /// Get a variable by its unique ID.
//     pub async fn get_variables(
//         client: &Client,
//         function_id: &str,
//         variable_id: &str,
//     ) -> Result<Variable, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/variables/{variableId}"
//             .replace("{functionId}", function_id)
//             .replace("{variableId}", variable_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update variable
//     ///
//     /// Update variable by its unique ID.
//     pub async fn update_variables(
//         client: &Client,
//         function_id: &str,
//         variable_id: &str,
//         key: &str,
//         value: Option<&str>,
//     ) -> Result<Variable, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/variables/{variableId}"
//             .replace("{functionId}", function_id)
//             .replace("{variableId}", variable_id);

//         let api_params = api_params!(
//             "key"=> Some(key),
//             "value"=>value,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PUT,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Delete variable
//     ///
//     /// Delete a variable by its unique ID.
//     pub async fn delete_variables(
//         client: &Client,
//         function_id: &str,
//         variable_id: &str,
//     ) -> Result<(), Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/functions/{functionId}/variables/{variableId}"
//             .replace("{functionId}", function_id)
//             .replace("{variableId}", variable_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let _res = client
//             .call(
//                 HttpMethod::DELETE,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(())
//     }
// }
