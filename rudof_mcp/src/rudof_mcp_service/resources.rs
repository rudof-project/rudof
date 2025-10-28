#![cfg(not(tarpaulin_skip))]

use crate::rudof_mcp_service::errors::{self, error_messages};
use crate::rudof_mcp_service::service::RudofMcpService;
use rmcp::{
    ErrorData as McpError, RoleServer,
    model::{
        ListResourcesResult, PaginatedRequestParam, ReadResourceRequestParam, ReadResourceResult,
    },
    service::RequestContext,
};
use serde_json::json;

// Return the list of available resources
// (Not implemented)
pub async fn list_resources(
    _request: Option<PaginatedRequestParam>,
    _ctx: RequestContext<RoleServer>,
) -> Result<ListResourcesResult, McpError> {
    Ok(ListResourcesResult {
        resources: vec![],
        next_cursor: None,
    })
}

// Read a resource by URI and return the MCP read result
// (Not implemented)
pub async fn read_resource(
    _service: &RudofMcpService,
    request: ReadResourceRequestParam,
) -> Result<ReadResourceResult, McpError> {
    let uri = request.uri;
    match uri.as_str() {
        _ => Err(errors::resource_not_found(
            error_messages::RESOURCE_NOT_FOUND,
            Some(json!({ "uri": uri })),
        )),
    }
}
