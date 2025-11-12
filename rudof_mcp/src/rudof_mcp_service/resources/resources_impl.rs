use crate::rudof_mcp_service::errors::{self, error_messages};
use crate::rudof_mcp_service::service::RudofMcpService;
use rmcp::{
    ErrorData as McpError, RoleServer,
    model::{
        Annotated, ListResourcesResult, PaginatedRequestParam, ReadResourceRequestParam,
        ReadResourceResult, RawResource,
    },
    service::RequestContext,
};
use serde_json::json;

use super::data_resources_impl::{get_data_resources, handle_data_resource};
use super::node_resources_impl::{get_node_resources, handle_node_resource};
use super::query_resources_impl::{get_query_resources, handle_query_resource};
use super::shex_validate_resources_impl::{get_shex_validate_resources, handle_shex_validate_resource};

/// Return the list of available resources
pub async fn list_resources(
    request: Option<PaginatedRequestParam>,
    _ctx: RequestContext<RoleServer>,
) -> Result<ListResourcesResult, McpError> {
    // Collect all resources from different modules
    let mut all_resources: Vec<Annotated<RawResource>> = Vec::new();
    
    all_resources.extend(get_data_resources());
    all_resources.extend(get_node_resources());
    all_resources.extend(get_query_resources());
    all_resources.extend(get_shex_validate_resources());
    
    // Handle pagination if requested
    let (resources, next_cursor) = if let Some(params) = request {
        let page_size = 20;
        let cursor = params.cursor.and_then(|c| c.parse::<usize>().ok()).unwrap_or(0);
        
        let start = cursor;
        let end = std::cmp::min(start + page_size, all_resources.len());
        
        let page_resources = all_resources[start..end].to_vec();
        let cursor_value = if end < all_resources.len() {
            Some(end.to_string())
        } else {
            None
        };
        
        (page_resources, cursor_value)
    } else {
        (all_resources, None)
    };
    
    Ok(ListResourcesResult {
        resources,
        next_cursor,
    })
}

/// Read a resource by URI and return the MCP read result
pub async fn read_resource(
    service: &RudofMcpService,
    request: ReadResourceRequestParam,
) -> Result<ReadResourceResult, McpError> {
    let uri = request.uri;
    
    // Try handling the resource from different modules
    if let Some(result) = handle_data_resource(service, &uri).await {
        return result;
    }

    if let Some(result) = handle_node_resource(&uri) {
        return result;
    }

    if let Some(result) = handle_query_resource(&uri) {
        return result;
    }

    if let Some(result) = handle_shex_validate_resource(&uri) {
        return result;
    }
     
    // Resource not found
    Err(errors::resource_not_found(
        error_messages::RESOURCE_NOT_FOUND,
        Some(json!({ "uri": uri })),
    ))
}
