use rmcp::{
    model::*, 
    service::RequestContext, 
    ErrorData as McpError, 
    ServerHandler, 
    RoleServer
};
use crate::rudof_mcp_service::service::RudofMcpService;
use crate::rudof_mcp_service::resources;

impl ServerHandler for RudofMcpService {
    // Return server metadata including protocol version, capabilities, and implementation info
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server exposes rudof tools and prompts.".to_string()),
        }
    }

    // Return a list of available tools using the generated ToolRouter.
    async fn list_tools(&self, _request: Option<PaginatedRequestParam>, _: RequestContext<RoleServer>) -> Result<ListToolsResult, McpError> {
        let tools = crate::rudof_mcp_service::tools::annotated_tools();
        Ok(ListToolsResult { tools, next_cursor: None })
    }

    // Return a list of available prompts using the generated PromptRouter.
    async fn list_prompts(&self, _request: Option<PaginatedRequestParam>, _: RequestContext<RoleServer>) -> Result<ListPromptsResult, McpError> {
        let prompts = self.prompt_router.list_all();
        Ok(ListPromptsResult { prompts, next_cursor: None })
    }

    // Return a list of available resources
    async fn list_resources(&self, _request: Option<PaginatedRequestParam>, context: RequestContext<RoleServer>) -> Result<ListResourcesResult, McpError> {
        // Delegate to resources module
        resources::list_resources(_request, context).await
    }

    // Read a resource by URI, returning content for known resources or an error if not found
    async fn read_resource(&self, ReadResourceRequestParam { uri }: ReadResourceRequestParam, context: RequestContext<RoleServer>) -> Result<ReadResourceResult, McpError> {
        // Delegate read handling to resources module
        let req = ReadResourceRequestParam { uri };
        resources::read_resource(req, context).await
    }

    // Return an empty list of resource templates
    // (Not implemented)
    async fn list_resource_templates(&self, _request: Option<PaginatedRequestParam>, _: RequestContext<RoleServer>) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult { next_cursor: None, resource_templates: Vec::new() })
    }

    // Handle MCP initialization, logging HTTP context if available, and return server info
    async fn initialize(&self, _request: InitializeRequestParam, context: RequestContext<RoleServer>) -> Result<InitializeResult, McpError> {
        if let Some(http_request_part) = context.extensions.get::<axum::http::request::Parts>() {
            let initialize_headers = &http_request_part.headers;
            let initialize_uri = &http_request_part.uri;
            tracing::info!(?initialize_headers, %initialize_uri, "initialize from http server");
        }
        Ok(self.get_info())
    }

    // Construct a ToolCallContext and delegate to the generated router
    async fn call_tool(&self, request: CallToolRequestParam, context: RequestContext<RoleServer>) -> Result<CallToolResult, McpError> {
        let ctx = rmcp::handler::server::tool::ToolCallContext::new(self, request, context);
        let result = self.tool_router.call(ctx).await?;
        Ok(result)
    }

    // Construct a PromptContext and delegate to the generated router
    async fn get_prompt(&self, request: GetPromptRequestParam, context: RequestContext<RoleServer>) -> Result<GetPromptResult, McpError> {
        let ctx = rmcp::handler::server::prompt::PromptContext::new(self, request.name, request.arguments, context);
        let result = self.prompt_router.get_prompt(ctx).await?;
        Ok(result)
    }
}
