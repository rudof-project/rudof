//! Core resource routing and aggregation.
//!
//! This module provides the main entry points for listing and reading
//! MCP resources. It aggregates resources from all domain-specific
//! modules and handles request routing.

use crate::service::errors::*;
use crate::service::mcp_service::RudofMcpService;
use rmcp::{
    ErrorData as McpError, RoleServer,
    model::{
        Annotated, ListResourcesResult, PaginatedRequestParams, RawResource, ReadResourceRequestParams,
        ReadResourceResult,
    },
    service::RequestContext,
};

use crate::service::resources::data_resources_impl::{get_data_resources, handle_data_resource};
use crate::service::resources::node_resources_impl::{get_node_resources, handle_node_resource};
use crate::service::resources::query_resources_impl::{get_query_resources, handle_query_resource};
use crate::service::resources::shacl_validate_resources_impl::{
    get_shacl_validate_resources, handle_shacl_validate_resource,
};
use crate::service::resources::shex_validate_resources_impl::{
    get_shex_validate_resources, handle_shex_validate_resource,
};

/// Return the list of available resources
pub async fn list_resources(
    request: Option<PaginatedRequestParams>,
    _ctx: RequestContext<RoleServer>,
) -> Result<ListResourcesResult, McpError> {
    // Collect all resources from different modules
    let mut all_resources: Vec<Annotated<RawResource>> = Vec::new();

    all_resources.extend(get_data_resources());
    all_resources.extend(get_node_resources());
    all_resources.extend(get_query_resources());
    all_resources.extend(get_shex_validate_resources());
    all_resources.extend(get_shacl_validate_resources());

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
        ..Default::default()
    })
}

/// Read a resource by URI and return the MCP read result
pub async fn read_resource(
    service: &RudofMcpService,
    request: ReadResourceRequestParams,
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

    if let Some(result) = handle_shacl_validate_resource(&uri) {
        return result;
    }

    // Resource not found
    Err(resource_not_found_error(
        "Invalid resource",
        "The requested resource does not exist.",
        Some(serde_json::json!({"operation":"read_resource"})),
    ))
}
