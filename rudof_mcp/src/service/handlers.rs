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
//! - **Pagination**: Cursor-based pagination for listing promprts, resources and tools
//! ```

use crate::service::logging::{LogData, send_log};
use crate::service::mcp_service::RudofMcpService;
use crate::service::pagination::parse_cursor;
use crate::service::{errors::*, resource_templates, resources::*};
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::tool::ToolCallContext,
    model::*,
    service::{NotificationContext, RequestContext},
};
use serde_json::json;

fn task_not_found_error(task_id: &str) -> McpError {
    invalid_params_error(
        "Invalid task id",
        format!("Task not found: {task_id}"),
        Some(json!({"param":"task_id","value":task_id})),
    )
}

fn task_result_not_ready_error(task_id: &str, status: TaskStatus) -> McpError {
    invalid_request_error(
        "Task result is not available yet",
        format!(
            "Task status is {status:?}. Poll tasks/get until status is Completed or Failed."
        ),
        Some(json!({"task_id":task_id, "status":format!("{status:?}")})),
    )
}

fn task_cancel_not_allowed_error(task_id: &str, status: TaskStatus) -> McpError {
    invalid_request_error(
        "Task cannot be cancelled",
        format!("Task {task_id} is already in terminal state {status:?}"),
        Some(json!({"task_id":task_id, "status":format!("{status:?}")})),
    )
}

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
    fn get_info(&self) -> ServerInfo {
        tracing::debug!("Generating ServerInfo");

        let capabilities = ServerCapabilities::builder()
            .enable_logging_with(serde_json::Map::new())
            .enable_prompts_with(PromptsCapability { list_changed: Some(false) })
            .enable_resources_with(ResourcesCapability {
                subscribe: Some(false),
                list_changed: Some(false),
            })
            .enable_tools_with(ToolsCapability { list_changed: Some(false) })
            .enable_completions_with(serde_json::Map::new())
            .build();

        ServerInfo::new(capabilities)
            .with_protocol_version(ProtocolVersion::LATEST)
            .with_server_info(Implementation::from_build_env())
            .with_instructions(
                "This MCP server exposes Rudof tools and prompts. Rudof is a comprehensive
            library that implements Shape Expressions (ShEx), SHACL, DCTAP, and other technologies in the
            RDF ecosystem, enabling schema validation, data transformation, and semantic web
            operations.",
            )
    }

    /// Return a list of available tools using the generated ToolRouter.
    async fn list_tools(
        &self,
        request: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        tracing::debug!("Listing available tools");

        let all_tools = crate::service::tools::annotated_tools();

        // Handle pagination if requested
        let (tools, next_cursor) = if let Some(params) = request {
            let page_size = 20;
            let cursor = parse_cursor(params.cursor, all_tools.len(), "tools/list")?;

            let start = cursor.min(all_tools.len());
            let end = std::cmp::min(start + page_size, all_tools.len());

            let page_tools = all_tools[start..end].to_vec();
            let cursor_value = if end < all_tools.len() {
                Some(end.to_string())
            } else {
                None
            };

            (page_tools, cursor_value)
        } else {
            (all_tools, None)
        };

        Ok(ListToolsResult {
            tools,
            next_cursor,
            ..Default::default()
        })
    }

    /// Return a list of available prompts using the generated PromptRouter.
    async fn list_prompts(
        &self,
        request: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        tracing::debug!("Listing available prompts");
        let all_prompts = self.prompt_router.list_all();

        // Handle pagination if requested
        let (prompts, next_cursor) = if let Some(params) = request {
            let page_size = 20;
            let cursor = parse_cursor(params.cursor, all_prompts.len(), "prompts/list")?;

            let start = cursor.min(all_prompts.len());
            let end = std::cmp::min(start + page_size, all_prompts.len());

            let page_prompts = all_prompts[start..end].to_vec();
            let cursor_value = if end < all_prompts.len() {
                Some(end.to_string())
            } else {
                None
            };

            (page_prompts, cursor_value)
        } else {
            (all_prompts, None)
        };

        Ok(ListPromptsResult {
            prompts,
            next_cursor,
            ..Default::default()
        })
    }

    /// Return a list of available resources
    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        tracing::debug!("Listing available resources");

        // Delegate to resources module
        list_resources(_request, context).await
    }

    /// Read a resource by URI
    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        tracing::debug!(uri = %request.uri, "Reading resource");

        // Delegate read handling to resources module
        read_resource(self, request).await
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
    async fn initialize(
        &self,
        _request: InitializeRequestParams,
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

        tracing::debug!("MCP server initialized successfully");

        Ok(self.get_info())
    }

    /// Handle dynamic log level changes from the client
    /// This updates the MCP logging notification level, controlling which log messages
    /// are sent to the client via MCP notifications.
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
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        tracing::debug!(tool_name = %request.name, "Tool call requested");

        RudofMcpService::with_request_context(context.clone(), async {
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
                    let log_data =
                        LogData::new("Tool executed successfully").with_field("tool_name", tool_name.clone());
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
        })
        .await
    }

    // Construct a PromptContext and delegate to the generated router
    async fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        tracing::debug!(prompt_name = %request.name, "Prompt retrieval requested");

        let ctx = rmcp::handler::server::prompt::PromptContext::new(self, request.name, request.arguments, context);

        let result = self.prompt_router.get_prompt(ctx).await?;
        Ok(result)
    }

    // Handle completion requests for prompt/resource arguments
    async fn complete(
        &self,
        request: CompleteRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CompleteResult, McpError> {
        // Extract the reference information and argument name
        let completions = match &request.r#ref {
            Reference::Prompt(prompt_ref) => {
                self.get_prompt_argument_completions(&prompt_ref.name, &request.argument.name)
            },
            Reference::Resource(resource_ref) => {
                self.get_resource_uri_completions(&resource_ref.uri, &request.argument.name)
            },
        };

        let completion = CompletionInfo::with_pagination(completions, None, false)
            .map_err(|e| McpError::invalid_params(e, None))?;

        Ok(CompleteResult::new(completion))
    }

    // Handle notification when client is initialized
    async fn on_initialized(&self, _context: NotificationContext<RoleServer>) -> () {
        tracing::debug!("Client successfully initialized");
    }

    // Handle cancelled operation notifications
    async fn on_cancelled(
        &self,
        notification: CancelledNotificationParam,
        _context: NotificationContext<RoleServer>,
    ) -> () {
        tracing::debug!(request_id = %notification.request_id, "Operation cancelled by client");
    }

    // Handle progress notifications from client
    async fn on_progress(
        &self,
        notification: ProgressNotificationParam,
        _context: NotificationContext<RoleServer>,
    ) -> () {
        tracing::debug!(
            progress_token = ?notification.progress_token,
            progress = notification.progress,
            total = ?notification.total,
            "Progress update received from client"
        );
    }

    // Handle notification when client's roots list changes
    async fn on_roots_list_changed(&self, _context: NotificationContext<RoleServer>) -> () {
        tracing::debug!("Client's roots list changed");
    }

    // Handle ping requests for health checks
    async fn ping(&self, _context: RequestContext<RoleServer>) -> Result<(), McpError> {
        tracing::debug!("Ping received");
        Ok(())
    }
}
