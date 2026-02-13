//! MCP `ServerHandler` implementation for Rudof.
//!
//! This module implements the [`rmcp::ServerHandler`] trait for [`RudofMcpService`],
//! handling all MCP protocol requests including:
//!
//! - **Initialization**: Server capabilities and metadata
//! - **Tools**: Listing and executing validation/query tools
//! - **Prompts**: Listing and retrieving prompt templates
//! - **Resources**: Listing, reading, and subscribing to RDF data
//! - **Completions**: Argument suggestions for tools and prompts
//! - **Logging**: Dynamic log level configuration
//! - **Tasks**: Async task management (SEP-1686)
//! ```

use crate::service::logging::{LogData, send_log};
use crate::service::mcp_service::RudofMcpService;
use crate::service::{resource_templates, resources::*};
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::tool::ToolCallContext,
    model::*,
    service::{NotificationContext, RequestContext},
};
use serde_json::json;

impl ServerHandler for RudofMcpService {
    /// Returns server metadata including protocol version, capabilities, and implementation info.
    ///
    /// # Capabilities Advertised
    ///
    /// - **tools**: Available
    /// - **prompts**: Available
    /// - **resources**: Available with `subscribe` and `list_changed` support
    /// - **logging**: Enabled for client-side log filtering
    /// - **completions**: Enabled for argument suggestions
    /// - **tasks**: SEP-1686 async task support
    fn get_info(&self) -> ServerInfo {
        tracing::debug!("Generating ServerInfo");

        let mut logging_meta = serde_json::Map::new();
        logging_meta.insert("enabled".to_string(), json!(true));

        let mut task_cap = serde_json::Map::new();
        task_cap.insert("supported".to_string(), json!(true));

        ServerInfo {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities {
                experimental: None,
                logging: Some(logging_meta),
                prompts: Some(PromptsCapability { list_changed: None }),
                resources: Some(ResourcesCapability {
                    subscribe: Some(true),
                    list_changed: Some(true),
                }),
                tools: Some(ToolsCapability { list_changed: None }),
                completions: Some(serde_json::Map::new()),
                tasks: Some(TasksCapability::server_default()),
            },
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "This MCP server exposes Rudof tools and prompts. Rudof is a comprehensive
            library that implements Shape Expressions (ShEx), SHACL, DCTAP, and other technologies in the
            RDF ecosystem, enabling schema validation, data transformation, and semantic web
            operations."
                    .to_string(),
            ),
        }
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
            let cursor = params.cursor.and_then(|c| c.parse::<usize>().ok()).unwrap_or(0);

            let start = cursor;
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
            let cursor = params.cursor.and_then(|c| c.parse::<usize>().ok()).unwrap_or(0);

            let start = cursor;
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
        ReadResourceRequestParams { uri, .. }: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        tracing::debug!(%uri, "Reading resource");

        // Delegate read handling to resources module
        let req = ReadResourceRequestParams { uri, meta: None };
        read_resource(self, req).await
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
        // Store the context so tools can access it for notifications
        {
            let mut ctx_guard = self.current_context.write().await;
            *ctx_guard = Some(context.clone());
        }

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
                    &context.peer,
                )
                .await;
            },
            Err(e) => {
                let log_data = LogData::new("Tool execution failed")
                    .with_field("tool_name", tool_name.clone())
                    .with_field("error", e.message.clone());
                send_log(
                    LoggingLevel::Error,
                    Some("tools".to_string()),
                    log_data,
                    self.current_min_log_level.clone(),
                    &context.peer,
                )
                .await;
            },
        }

        // Clear the context after the tool call
        {
            let mut ctx_guard = self.current_context.write().await;
            *ctx_guard = None;
        }

        result
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

    // Handle completion requests for tool/prompt arguments
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

        Ok(CompleteResult {
            completion: CompletionInfo {
                values: completions,
                total: None,
                has_more: Some(false),
            },
        })
    }

    // Handle resource subscription requests
    async fn subscribe(
        &self,
        request: SubscribeRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<(), McpError> {
        let uri = request.uri;
        // Generate a simple subscriber ID using timestamp
        let subscriber_id = format!(
            "sub_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );

        // Store the subscription
        self.subscribe_resource(uri.clone(), subscriber_id).await;

        Ok(())
    }

    // Handle resource unsubscription requests
    async fn unsubscribe(
        &self,
        request: UnsubscribeRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<(), McpError> {
        let uri = request.uri;

        let subscribers = self.get_resource_subscribers(&uri).await;
        for subscriber_id in subscribers {
            self.unsubscribe_resource(&uri, &subscriber_id).await;
        }

        Ok(())
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

    /// Enqueue a task for async execution (SEP-1686).
    ///
    /// This method creates a task entry and spawns a background worker to execute
    /// the tool asynchronously. The client can poll for status using `get_task_info`
    /// and retrieve results with `get_task_result` once completed.
    ///
    /// # Flow
    /// 1. Task is enqueued with `Working` status
    /// 2. Background worker spawned via `tokio::spawn`
    /// 3. Worker executes the tool via `tool_router`
    /// 4. On success: `task_store.complete()` is called
    /// 5. On failure: `task_store.fail()` is called
    /// 6. Client polls until terminal state (Completed/Failed/Cancelled)
    async fn enqueue_task(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CreateTaskResult, McpError> {
        tracing::debug!(tool_name = %request.name, "Enqueuing task for async execution");

        // 1. Enqueue the task to get a task_id
        let result = self.task_store.enqueue().await;
        let task_id = result.task.task_id.clone();

        tracing::debug!(
            task_id = %task_id,
            status = ?result.task.status,
            "Task enqueued, spawning background worker"
        );

        // 2. Spawn background worker to execute the tool
        let service = self.clone();
        let tool_request = request.clone();

        tokio::spawn(async move {
            tracing::debug!(task_id = %task_id, tool_name = %tool_request.name, "Background worker started");

            // Update status to indicate execution has started
            service
                .task_store
                .update_status(
                    &task_id,
                    rmcp::model::TaskStatus::Working,
                    Some(format!("Executing tool: {}", tool_request.name)),
                )
                .await;

            // Build the tool call context and execute
            // Note: We use the cloned context; the cancellation token is shared
            let ctx = rmcp::handler::server::tool::ToolCallContext::new(&service, tool_request.clone(), context);

            match service.tool_router.call(ctx).await {
                Ok(tool_result) => {
                    tracing::debug!(task_id = %task_id, "Task completed successfully");
                    service.task_store.complete(&task_id, tool_result).await;
                },
                Err(e) => {
                    tracing::error!(task_id = %task_id, error = ?e, "Task failed");
                    service.task_store.fail(&task_id, e.message.to_string()).await;
                },
            }
        });

        // 3. Return immediately with task info (client will poll for results)
        Ok(result)
    }

    // List all tasks with optional pagination (SEP-1686)
    async fn list_tasks(
        &self,
        request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListTasksResult, McpError> {
        tracing::debug!("Listing tasks");
        Ok(self.task_store.list(request).await)
    }

    // Get information about a specific task (SEP-1686)
    async fn get_task_info(
        &self,
        request: GetTaskInfoParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<GetTaskInfoResult, McpError> {
        tracing::debug!(task_id = %request.task_id, "Getting task info");

        self.task_store
            .get_info(request.clone())
            .await
            .ok_or_else(|| McpError::resource_not_found(format!("Task not found: {}", request.task_id), None))
    }

    // Get the result of a completed task (SEP-1686)
    async fn get_task_result(
        &self,
        request: GetTaskResultParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<TaskResult, McpError> {
        tracing::debug!(task_id = %request.task_id, "Getting task result");

        // First check if task exists and its status
        let info = self
            .task_store
            .get_info(GetTaskInfoParams {
                task_id: request.task_id.clone(),
                meta: None,
            })
            .await;

        match info {
            None => Err(McpError::resource_not_found(
                format!("Task not found: {}", request.task_id),
                None,
            )),
            Some(task_info) => {
                let task = task_info.task.ok_or_else(|| {
                    McpError::resource_not_found(format!("Task not found: {}", request.task_id), None)
                })?;
                match task.status {
                    TaskStatus::Working | TaskStatus::InputRequired => {
                        Err(McpError::invalid_request("Task is still in progress", None))
                    },
                    TaskStatus::Cancelled => Err(McpError::invalid_request("Task was cancelled", None)),
                    TaskStatus::Completed | TaskStatus::Failed => self
                        .task_store
                        .get_result(request.clone())
                        .await
                        .ok_or_else(|| McpError::internal_error("Task result not available", None)),
                }
            },
        }
    }

    // Cancel a running task (SEP-1686)
    async fn cancel_task(
        &self,
        request: CancelTaskParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<(), McpError> {
        tracing::debug!(task_id = %request.task_id, "Cancelling task");

        self.task_store
            .cancel(request.clone())
            .await
            .map(|_| ())
            .ok_or_else(|| McpError::invalid_request(format!("Cannot cancel task: {}", request.task_id), None))
    }
}
