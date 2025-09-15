#![allow(dead_code)]
use std::str::FromStr;
use std::sync::Arc;

use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{
        router::{prompt::PromptRouter, tool::ToolRouter},
        wrapper::Parameters,
    },
    model::*,
    prompt, prompt_handler, prompt_router, schemars,
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use rudof_lib::{RDFFormat, ReaderMode, Rudof, RudofConfig};
use serde_json::json;
use tokio::sync::Mutex;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct StructRequest {
    pub a: i32,
    pub b: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ExamplePromptArgs {
    /// A message to put in the prompt
    pub message: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct LoadRdfDataRequest {
    /// RDF data to load
    pub rdf_data: String,
    /// RDF format, e.g., "turtle", "jsonld"
    pub format: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ExportRdfDataRequest {
    /// RDF format, e.g., "turtle", "jsonld"
    pub format: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct CounterAnalysisArgs {
    /// The target value you're trying to reach
    pub goal: i32,
    /// Preferred strategy: 'fast' or 'careful'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

#[derive(Clone)]
pub struct RudofMcpService {
    rudof: Arc<Mutex<Rudof>>,
    tool_router: ToolRouter<RudofMcpService>,
    prompt_router: PromptRouter<RudofMcpService>,
}

#[tool_router]
impl RudofMcpService {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            rudof: Arc::new(Mutex::new(Rudof::new(&RudofConfig::new()))),
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        }
    }

    /*fn _create_resource_text(&self, uri: &str, name: &str) -> Resource {
        RawResource::new(uri, name.to_string()).no_annotation()
    }*/

    #[tool(description = "Load RDF data from a string")]
    async fn load_rdf_data(
        &self,
        Parameters(LoadRdfDataRequest { rdf_data, format }): Parameters<LoadRdfDataRequest>,
    ) -> Result<CallToolResult, McpError> {
        let mut rudof = self.rudof.lock().await;
        match RDFFormat::from_str(&format) {
            Ok(format) => {
                let _result = rudof
                    .read_data(rdf_data.as_bytes(), &format, None, &ReaderMode::default())
                    .map_err(|e| {
                        McpError::internal_error(
                            "rdf_load_error",
                            Some(json!({ "error": e.to_string() })),
                        )
                    })?;
                // result.await?;
                Ok(CallToolResult::success(vec![Content::text(
                    "RDF data loaded successfully",
                )]))
            }
            Err(e) => {
                tracing::error!("Failed to load RDF data: {}", e);
                return Err(McpError::invalid_request(
                    format!("invalid_format {}", format),
                    Some(json!({ "format": format, "error": e.to_string() })),
                ));
            }
        }
    }

    #[tool(description = "Export RDF data to some format")]
    async fn export_rdf_data(
        &self,
        Parameters(ExportRdfDataRequest { format }): Parameters<ExportRdfDataRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rudof = self.rudof.lock().await;
        match RDFFormat::from_str(&format) {
            Ok(format) => {
                let mut v = Vec::new();
                let _result = rudof.serialize_data(&format, &mut v).map_err(|e| {
                    McpError::internal_error(
                        "serialize_data_error",
                        Some(json!({ "error": e.to_string() })),
                    )
                })?;
                let str = String::from_utf8(v).map_err(|e| {
                    McpError::internal_error(
                        "utf8_conversion_error",
                        Some(json!({ "error": e.to_string() })),
                    )
                })?;
                // result.await?;
                Ok(CallToolResult::success(vec![Content::text(str)]))
            }
            Err(e) => {
                tracing::error!("Failed to serialize RDF data: {}", e);
                return Err(McpError::invalid_request(
                    format!("invalid_format {}", format),
                    Some(json!({ "format": format, "error": e.to_string() })),
                ));
            }
        }
    }
}

#[prompt_router]
impl RudofMcpService {
    /// This is an example prompt that takes one required argument, message
    #[prompt(name = "example_prompt")]
    async fn example_prompt(
        &self,
        Parameters(args): Parameters<ExamplePromptArgs>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<Vec<PromptMessage>, McpError> {
        let prompt = format!(
            "This is an example prompt with your message here: '{}'",
            args.message
        );
        Ok(vec![PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(prompt),
        }])
    }

    /*
    /// Analyze the current value of rudof and suggest next steps
    #[prompt(name = "rudof_analysis")]
    async fn counter_analysis(
        &self,
        Parameters(args): Parameters<CounterAnalysisArgs>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        let strategy = args.strategy.unwrap_or_else(|| "careful".to_string());
        let current_value = *self.counter.lock().await;
        let difference = args.goal - current_value;

        let messages = vec![
            PromptMessage::new_text(
                PromptMessageRole::Assistant,
                "I'll analyze the counter situation and suggest the best approach.",
            ),
            PromptMessage::new_text(
                PromptMessageRole::User,
                format!(
                    "Current counter value: {}\nGoal value: {}\nDifference: {}\nStrategy preference: {}\n\nPlease analyze the situation and suggest the best approach to reach the goal.",
                    current_value, args.goal, difference, strategy
                ),
            ),
        ];

        Ok(GetPromptResult {
            description: Some(format!(
                "Counter analysis for reaching {} from {}",
                args.goal, current_value
            )),
            messages,
        })
    }*/
}

#[tool_handler]
#[prompt_handler]
impl ServerHandler for RudofMcpService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides counter tools and prompts. Tools: increment, decrement, get_value, say_hello, echo, sum. Prompts: example_prompt (takes a message), counter_analysis (analyzes counter state with a goal).".to_string()),
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![
                // self._create_resource_text("str:////Users/to/some/path/", "cwd"),
                // self._create_resource_text("memo://insights", "memo-name"),
            ],
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        match uri.as_str() {
            "rudof://rdf_formats" => Ok(ReadResourceResult {
                contents: vec![ResourceContents::text("RDF Formats", uri)],
            }),
            _ => Err(McpError::resource_not_found(
                "resource_not_found",
                Some(json!({
                    "uri": uri
                })),
            )),
        }
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            next_cursor: None,
            resource_templates: Vec::new(),
        })
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        if let Some(http_request_part) = context.extensions.get::<axum::http::request::Parts>() {
            let initialize_headers = &http_request_part.headers;
            let initialize_uri = &http_request_part.uri;
            tracing::info!(?initialize_headers, %initialize_uri, "initialize from http server");
        }
        Ok(self.get_info())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_prompt_attributes_generated() {
        // Verify that the prompt macros generate the expected attributes
        let example_attr = RudofMcpService::example_prompt_prompt_attr();
        assert_eq!(example_attr.name, "example_prompt");
        assert!(example_attr.description.is_some());
        assert!(example_attr.arguments.is_some());

        let args = example_attr.arguments.unwrap();
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].name, "message");
        assert_eq!(args[0].required, Some(true));
    }

    #[tokio::test]
    async fn test_prompt_router_has_routes() {
        let router = RudofMcpService::prompt_router();
        assert!(router.has_route("example_prompt"));
        assert!(router.has_route("counter_analysis"));

        let prompts = router.list_all();
        assert_eq!(prompts.len(), 2);
    }

    /*
    #[tokio::test]
    async fn test_example_prompt_execution() {
        let counter = Counter::new();
        let context = rmcp::handler::server::prompt::PromptContext::new(
            &counter,
            "example_prompt".to_string(),
            Some({
                let mut map = serde_json::Map::new();
                map.insert(
                    "message".to_string(),
                    serde_json::Value::String("Test message".to_string()),
                );
                map
            }),
            RequestContext {
                meta: Default::default(),
                ct: tokio_util::sync::CancellationToken::new(),
                id: rmcp::model::NumberOrString::String("test-1".to_string()),
                peer: Default::default(),
                extensions: Default::default(),
            },
        );

        let router = Counter::prompt_router();
        let result = router.get_prompt(context).await;
        assert!(result.is_ok());

        let prompt_result = result.unwrap();
        assert_eq!(prompt_result.messages.len(), 1);
        assert_eq!(prompt_result.messages[0].role, PromptMessageRole::User);
    }*/
}
