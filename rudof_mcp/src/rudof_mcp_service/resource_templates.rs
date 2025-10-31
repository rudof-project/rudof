use rmcp::{ErrorData as McpError, RoleServer, model::*, service::RequestContext};

// Return the list of available resource templates
// (Not implemented)
pub async fn list_resource_templates(
    _request: Option<PaginatedRequestParam>,
    _ctx: RequestContext<RoleServer>,
) -> Result<ListResourceTemplatesResult, McpError> {
    Ok(ListResourceTemplatesResult {
        resource_templates: Vec::new(),
        next_cursor: None,
    })
}
