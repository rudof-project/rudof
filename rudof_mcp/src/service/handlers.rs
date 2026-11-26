//! MCP `ServerHandler` implementation for Rudof.
//!
//! This module implements the [`rmcp::ServerHandler`] trait for [`RudofMcpService`],
//! handling all MCP protocol requests including:
//!
//! - **Initialization**: Server capabilities and metadata
//! - **Tools**: Listing and executing tools
//! - **Prompts**: Listing and retrieving prompt templates
//! - **Resources**: Listing and reading resources
//! - **Completions**: Argument suggestions for tools and prompts
//! - **Logging**: Dynamic log level configuration

use crate::service::logging::{LogData, send_log};
use crate::service::mcp_service::RudofMcpService;
use crate::service::{resource_templates, resources::*};
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler, handler::server::tool::ToolCallContext, model::*, prompt_handler,
    service::RequestContext, tool_handler,
};

#[tool_handler(router = self.tool_router)]
#[prompt_handler(router = self.prompt_router)]
impl ServerHandler for RudofMcpService {
    /// Returns server metadata including protocol version, capabilities, and implementation info.
    ///
    /// # Capabilities Advertised
    ///
    /// - **tools**: Available
    /// - **prompts**: Available
    /// - **resources**: Available
    /// - **logging**: Enabled for client-side log filtering
    /// - **completions**: Enabled for argument suggestions
    // `enable_logging_with` is deprecated by SEP-2577; the protocol still
    // supports it and there is no replacement API, so it stays in use.
    #[allow(deprecated)]
    fn get_info(&self) -> ServerConfig {
        tracing::debug!("Generating ServerConfig");

        let mut prompts_capability = PromptsCapability::default();
        prompts_capability.list_changed = Some(false);

        let mut resources_capability = ResourcesCapability::default();
        resources_capability.subscribe = Some(false);
        resources_capability.list_changed = Some(false);

        let mut tools_capability = ToolsCapability::default();
        tools_capability.list_changed = Some(false);

        let capabilities = ServerCapabilities::builder()
            .enable_logging_with(serde_json::Map::new())
            .enable_prompts_with(prompts_capability)
            .enable_resources_with(resources_capability)
            .enable_tools_with(tools_capability)
            .enable_completions_with(serde_json::Map::new())
            .build();

        ServerConfig::new(capabilities)
            .with_protocol_version(ProtocolVersion::LATEST_WITH_INITIALIZE)
            // Not `Implementation::from_build_env()`: that bakes in rmcp's own
            // crate name and version, so the server announced itself as rmcp.
            .with_server_info(Implementation::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")))
            .with_instructions(
                "Rudof MCP server for RDF processing, SPARQL querying and graph validation using ShEx and SHACL.\n\
                \n\
                IMPORTANT — stateful datastore: the server holds a single in-memory RDF graph shared \
                across all tool calls. Call `load_rdf_data_from_sources` before any tool. \
                Supports URLs, local files, and inline text as data sources — each call merges \
                triples into the existing store (cumulative). SPARQL endpoints are also supported \
                as a source via `load_rdf_data_from_sources` but queried live; their results are not merged into the store.\n\
                \n\
                Typical call sequences:\n\
                - Explore data: load → node_info or execute_sparql_query\n\
                - Validate with ShEx or SHACL: load → validate_shex / validate_shacl\n\
                - Query data: load → execute_sparql_query (SELECT, CONSTRUCT and ASK)\n\
                - Visualize graph: load → export_image (SVG/PNG) or export_plantuml\n\
                - Check a schema without data: check_shex or show_shex (no load needed)\n\
                ",
            )
    }

    /// Return a list of available resources
    async fn list_resources(
        &self,
        request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        tracing::debug!("Listing available resources");

        // Delegate to resources module
        list_resources(request, context).await
    }

    /// Read a resource by URI
    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResponse, McpError> {
        tracing::debug!(uri = %request.uri, "Reading resource");

        // Delegate read handling to resources module
        read_resource(self, request).await.map(Into::into)
    }

    /// Return a list of available resource templates
    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        tracing::debug!("Listing available resource templates");

        // Delegate to resource_templates module
        resource_templates::list_resource_templates(_request, context).await
    }

    /// Handle MCP initialization, logging HTTP context if available, and return server info
    // `LoggingLevel` is deprecated by SEP-2577; see the `get_info` note above.
    #[allow(deprecated)]
    async fn initialize(
        &self,
        request: InitializeRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        tracing::debug!("Initializing MCP server");

        if let Some(http_request_part) = context.extensions.get::<axum::http::request::Parts>() {
            let initialize_headers = &http_request_part.headers;
            let initialize_uri = &http_request_part.uri;
            tracing::debug!(?initialize_headers, %initialize_uri, "initialize from http server");
        }

        // Set default log level to Info when initialized
        {
            let mut min_level = self.current_min_log_level.write().await;
            *min_level = Some(LoggingLevel::Info);
        }

        let result = self.negotiate_initialize(&request)?;

        tracing::debug!(protocol_version = ?result.protocol_version, "MCP server initialized successfully");

        Ok(result)
    }

    /// Handle dynamic log level changes from the client
    /// This updates the MCP logging notification level, controlling which log messages
    /// are sent to the client via MCP notifications.
    // `SetLevelRequestParams` is deprecated by SEP-2577; see the `get_info` note above.
    #[allow(deprecated)]
    async fn set_level(
        &self,
        request: SetLevelRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<(), McpError> {
        tracing::debug!(level = ?request.level, "Log level change requested");

        // Update the MCP minimum log level for notification filtering
        {
            let mut min_level = self.current_min_log_level.write().await;
            *min_level = Some(request.level);
        }

        tracing::debug!(new_level = ?request.level, "MCP notification log level updated");

        Ok(())
    }

    // Construct a ToolCallContext and delegate to the generated router
    // `LoggingLevel` is deprecated by SEP-2577; see the `get_info` note above.
    #[allow(deprecated)]
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, McpError> {
        tracing::debug!(tool_name = %request.name, "Tool call requested");

        // Send debug log for tool invocation (respects log level filtering)
        let tool_name = request.name.clone();
        let log_data = LogData::new("Tool invocation started")
            .with_field("tool_name", tool_name.clone())
            .with_field("has_arguments", request.arguments.is_some());
        send_log(
            LoggingLevel::Debug,
            Some("tools".to_string()),
            log_data,
            self.current_min_log_level.clone(),
            self.log_rate_limiter.clone(),
            &context.peer,
        )
        .await;

        let ctx = ToolCallContext::new(self, request, context.clone());
        let result = self.tool_router.call(ctx).await;

        // Log tool completion (respects log level filtering)
        match &result {
            Ok(_) => {
                let log_data = LogData::new("Tool executed successfully").with_field("tool_name", tool_name.clone());
                send_log(
                    LoggingLevel::Debug,
                    Some("tools".to_string()),
                    log_data,
                    self.current_min_log_level.clone(),
                    self.log_rate_limiter.clone(),
                    &context.peer,
                )
                .await;
            },
            Err(_) => {
                let log_data = LogData::new("Tool execution failed")
                    .with_field("tool_name", tool_name.clone())
                    .with_field("error", "[redacted]");
                send_log(
                    LoggingLevel::Error,
                    Some("tools".to_string()),
                    log_data,
                    self.current_min_log_level.clone(),
                    self.log_rate_limiter.clone(),
                    &context.peer,
                )
                .await;
            },
        }

        result
    }

    // Handle completion requests for prompt/resource arguments
    async fn complete(
        &self,
        request: CompleteRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CompleteResult, McpError> {
        let completions = match &request.r#ref {
            Reference::Prompt(prompt_ref) => {
                self.get_prompt_argument_completions(&prompt_ref.name, &request.argument.name)
            },
            Reference::Resource(resource_ref) => {
                self.get_resource_uri_completions(&resource_ref.uri, &request.argument.name)
            },
            _ => Vec::new(),
        };

        let completion = CompletionInfo::with_all_values(completions).map_err(|e| McpError::invalid_params(e, None))?;

        Ok(CompleteResult::new(completion))
    }
}
