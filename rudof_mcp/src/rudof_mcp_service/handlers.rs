use crate::rudof_mcp_service::service::RudofMcpService;
use crate::rudof_mcp_service::{resource_templates, resources};
use rmcp::{ErrorData as McpError, RoleServer, ServerHandler, model::*, service::RequestContext};

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
    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let tools = crate::rudof_mcp_service::tools::annotated_tools();
        Ok(ListToolsResult {
            tools,
            next_cursor: None,
        })
    }

    // Return a list of available prompts using the generated PromptRouter.
    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        let prompts = self.prompt_router.list_all();
        Ok(ListPromptsResult {
            prompts,
            next_cursor: None,
        })
    }

    // Return a list of available resources
    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        // Delegate to resources module
        resources::list_resources(_request, context).await
    }

    // Read a resource by URI, returning content for known resources or an error if not found
    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        // Delegate read handling to resources module
        let req = ReadResourceRequestParam { uri };
        resources::read_resource(self, req).await
    }

    // Return a list of available resource templates
    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        // Delegate to resource_templates module
        resource_templates::list_resource_templates(_request, context).await
    }

    // Handle MCP initialization, logging HTTP context if available, and return server info
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

    // Construct a ToolCallContext and delegate to the generated router
    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let ctx = rmcp::handler::server::tool::ToolCallContext::new(self, request, context);
        let result = self.tool_router.call(ctx).await?;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    /// Initialize the RudofMcpService in a blocking-safe context
    async fn create_test_service() -> RudofMcpService {
        tokio::task::spawn_blocking(|| {
            let rudof_config = rudof_lib::RudofConfig::new().unwrap();
            let rudof = rudof_lib::Rudof::new(&rudof_config).unwrap();
            RudofMcpService {
                rudof: Arc::new(Mutex::new(rudof)),
                tool_router: Default::default(),
                prompt_router: Default::default(),
            }
        })
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn test_get_info_returns_expected_metadata() {
        let service = create_test_service().await;
        let info = service.get_info();

        assert_eq!(info.protocol_version, ProtocolVersion::V_2024_11_05);
        assert!(info.capabilities.tools.is_some());
        assert!(info.capabilities.prompts.is_some());
        assert!(info.capabilities.resources.is_some());
        assert!(info.server_info.name.len() > 0);
        assert!(info.instructions.is_some());
    }
}
