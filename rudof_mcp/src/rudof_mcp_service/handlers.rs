use crate::rudof_mcp_service::service::RudofMcpService;
use crate::rudof_mcp_service::{resource_templates, resources::*};
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler, 
    model::*, 
    service::{RequestContext, NotificationContext}
};
use serde_json::json;
use std::collections::BTreeMap;

impl ServerHandler for RudofMcpService {
    /// Return server metadata including protocol version, capabilities, and implementation info
    fn get_info(&self) -> ServerInfo {
        // Create experimental capabilities for Rudof-specific features
        let mut experimental = BTreeMap::new();
        
        let mut shex_val_meta = serde_json::Map::new();
        shex_val_meta.insert("version".to_string(), json!("1.0"));
        shex_val_meta.insert("formats".to_string(), json!(["shexc", "shexj", "turtle", "ntriples", "rdfxml", "trig", "n3", "nquads"]));
        shex_val_meta.insert("result_formats".to_string(), json!(["compact", "turtle", "json"]));
        experimental.insert("rudof.shex_validation".to_string(), shex_val_meta);
        
        let mut rdf_viz_meta = serde_json::Map::new();
        rdf_viz_meta.insert("version".to_string(), json!("1.0"));
        rdf_viz_meta.insert("formats".to_string(), json!(["plantuml", "svg", "png"]));
        experimental.insert("rudof.rdf_visualization".to_string(), rdf_viz_meta);
        
        let mut sparql_meta = serde_json::Map::new();
        sparql_meta.insert("version".to_string(), json!("1.0"));
        sparql_meta.insert("query_types".to_string(), json!(["SELECT", "CONSTRUCT", "ASK", "DESCRIBE"]));
        experimental.insert("rudof.sparql".to_string(), sparql_meta);
        
        let mut logging_meta = serde_json::Map::new();
        logging_meta.insert("enabled".to_string(), json!(true));
        
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_06_18,
            capabilities: ServerCapabilities {
                experimental: Some(experimental),
                logging: Some(logging_meta), 
                prompts: Some(PromptsCapability {
                    list_changed: Some(true),
                }),
                resources: Some(ResourcesCapability {
                    subscribe: Some(true),
                    list_changed: Some(true),
                }),
                tools: Some(ToolsCapability {
                    list_changed: Some(true),
                }),
                completions: Some(serde_json::Map::new()),
            },
            server_info: Implementation::from_build_env(),
            instructions: Some("This MCP server exposes Rudof tools and prompts. Rudof is a comprehensive library that implements Shape Expressions (ShEx), 
            SHACL, DCTAP, and other technologies in the RDF ecosystem, enabling schema validation, 
            data transformation, and semantic web operations.".to_string()),
        }
    }

    /// Return a list of available tools using the generated ToolRouter.
    async fn list_tools(
        &self,
        request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let all_tools = crate::rudof_mcp_service::tools::annotated_tools();
        
        // Handle pagination if requested
        let (tools, next_cursor) = if let Some(params) = request {
            let page_size = 20; // Default page size
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
        })
    }

    /// Return a list of available prompts using the generated PromptRouter.
    async fn list_prompts(
        &self,
        request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        let all_prompts = self.prompt_router.list_all();
        
        // Handle pagination if requested
        let (prompts, next_cursor) = if let Some(params) = request {
            let page_size = 20; // Default page size
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
        })
    }

    /// Return a list of available resources
    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        // Delegate to resources module
        list_resources(_request, context).await
    }

    /// Read a resource by URI
    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        // Delegate read handling to resources module
        let req = ReadResourceRequestParam { uri };
        read_resource(self, req).await
    }

    /// Return a list of available resource templates
    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        // Delegate to resource_templates module
        resource_templates::list_resource_templates(_request, context).await
    }

    /// Handle MCP initialization, logging HTTP context if available, and return server info
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

    /// Handle dynamic log level changes from the client
    async fn set_level(
        &self,
        request: SetLevelRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<(), McpError> {
        tracing::info!(level = ?request.level, "Log level change requested");
        
        // Check if we have a log level handle for dynamic updates
        if let Some(handle) = &self.log_level_handle {
            // Convert MCP LoggingLevel to tracing LevelFilter
            let level_str = match request.level {
                LoggingLevel::Debug => "debug",
                LoggingLevel::Info => "info",
                LoggingLevel::Notice => "info",  
                LoggingLevel::Warning => "warn",
                LoggingLevel::Error => "error",
                LoggingLevel::Critical => "error",  
                LoggingLevel::Alert => "error",     
                LoggingLevel::Emergency => "error", 
            };
            
            // Create new filter with the requested level
            match tracing_subscriber::EnvFilter::try_new(level_str) {
                Ok(new_filter) => {
                    // Reload the filter
                    match handle.write().await.reload(new_filter) {
                        Ok(()) => {
                            tracing::info!(
                                new_level = %level_str,
                                "Log level successfully changed"
                            );
                            Ok(())
                        }
                        Err(e) => {
                            tracing::error!(error = ?e, "Failed to reload log filter");
                            Err(rmcp::ErrorData::internal_error(
                                format!("Failed to reload log filter: {}", e),
                                None,
                            ))
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(error = ?e, level = %level_str, "Invalid log level format");
                    Err(rmcp::ErrorData::invalid_params(
                        format!("Invalid log level: {}", e),
                        None,
                    ))
                }
            }
        } else {
            // No reload handle available - log warning but acknowledge request
            tracing::warn!(
                "Dynamic log level changes not available - no reload handle configured. \
                 Service must be initialized with with_log_handle() to support this feature."
            );
            Ok(())
        }
    }

    // Construct a ToolCallContext and delegate to the generated router
    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        // Store the context so tools can access it for notifications
        {
            let mut ctx_guard = self.current_context.write().await;
            *ctx_guard = Some(context.clone());
        }
        
        let ctx = rmcp::handler::server::tool::ToolCallContext::new(self, request, context);
        let result = self.tool_router.call(ctx).await?;
        
        // Clear the context after the tool call
        {
            let mut ctx_guard = self.current_context.write().await;
            *ctx_guard = None;
        }
        
        Ok(result)
    }

    // Construct a PromptContext and delegate to the generated router
    async fn get_prompt(
        &self,
        request: GetPromptRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        let ctx = rmcp::handler::server::prompt::PromptContext::new(
            self,
            request.name,
            request.arguments,
            context,
        );
        let result = self.prompt_router.get_prompt(ctx).await?;
        Ok(result)
    }

    // Handle completion requests for tool/prompt arguments
    async fn complete(
        &self,
        request: CompleteRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<CompleteResult, McpError> {
        tracing::debug!("Completion requested: {:?}", request);

        // Extract the reference information and argument name
        let completions = match &request.r#ref {
            Reference::Prompt(prompt_ref) => {
                self.get_prompt_argument_completions(&prompt_ref.name, &request.argument.name)
            }
            Reference::Resource(resource_ref) => {
                self.get_resource_uri_completions(&resource_ref.uri, &request.argument.name)
            }
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
        request: SubscribeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<(), McpError> {
        let uri = request.uri;
        // Generate a simple subscriber ID using timestamp
        let subscriber_id = format!("sub_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos());
        
        tracing::info!(%uri, %subscriber_id, "Client subscribed to resource");
        
        // Store the subscription
        self.subscribe_resource(uri.clone(), subscriber_id).await;
        
        Ok(())
    }

    // Handle resource unsubscription requests
    async fn unsubscribe(
        &self,
        request: UnsubscribeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<(), McpError> {
        let uri = request.uri;
        
        tracing::info!(%uri, "Client unsubscribed from resource");
        
        // Note: Without tracking who subscribed, we clear all subscriptions for this URI
        // In production, you'd want to track the specific subscriber ID
        let subscribers = self.get_resource_subscribers(&uri).await;
        for subscriber_id in subscribers {
            self.unsubscribe_resource(&uri, &subscriber_id).await;
        }
        
        Ok(())
    }

    // Handle notification when client is initialized
    async fn on_initialized(
        &self,
        _context: NotificationContext<RoleServer>,
    ) -> () {
        tracing::info!("Client successfully initialized");
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
    async fn on_roots_list_changed(
        &self,
        _context: NotificationContext<RoleServer>,
    ) -> () {
        tracing::info!("Client's roots list changed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rudof_mcp_service::service::ServiceConfig;
    use std::{collections::HashMap, sync::Arc};
    use tokio::sync::{Mutex, RwLock};

    /// Initialize the RudofMcpService in a blocking-safe context
    async fn create_test_service() -> RudofMcpService {
        tokio::task::spawn_blocking(|| {
            let rudof_config = rudof_lib::RudofConfig::new().unwrap();
            let rudof = rudof_lib::Rudof::new(&rudof_config).unwrap();
            RudofMcpService {
                rudof: Arc::new(Mutex::new(rudof)),
                tool_router: Default::default(),
                prompt_router: Default::default(),
                config: Arc::new(RwLock::new(ServiceConfig::default())),
                resource_subscriptions: Arc::new(RwLock::new(HashMap::new())),
                log_level_handle: None,
                current_context: Arc::new(RwLock::new(None)),
            }
        })
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn test_get_info_returns_expected_metadata() {
        let service = create_test_service().await;
        let info = service.get_info();

        assert_eq!(info.protocol_version, ProtocolVersion::V_2025_06_18);
        
        // Verify all capabilities are enabled
        let caps = info.capabilities;
        assert!(caps.tools.is_some());
        assert_eq!(caps.tools.as_ref().unwrap().list_changed, Some(true));
        
        assert!(caps.prompts.is_some());
        assert_eq!(caps.prompts.as_ref().unwrap().list_changed, Some(true));
        
        assert!(caps.resources.is_some());
        assert_eq!(caps.resources.as_ref().unwrap().subscribe, Some(true));
        assert_eq!(caps.resources.as_ref().unwrap().list_changed, Some(true));
        
        assert!(!info.server_info.name.is_empty());
        assert!(info.instructions.is_some());
    }
}
