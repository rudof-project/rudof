//! Core resource routing and aggregation.
//!
//! This module provides the main entry points for listing and reading
//! MCP resources. It aggregates resources from all domain-specific
//! modules and handles request routing.

use crate::service::errors::*;
use crate::service::mcp_service::RudofMcpService;
use rmcp::{
    ErrorData as McpError, RoleServer,
    model::{ListResourcesResult, PaginatedRequestParams, ReadResourceRequestParams, ReadResourceResult, Resource},
    service::RequestContext,
};

use crate::service::resources::data_resources_impl::{get_data_resources, handle_data_resource};
use crate::service::resources::shex_resources_impl::{get_shex_resources, handle_shex_resource};

/// Return the list of available resources
pub async fn list_resources(
    _request: Option<PaginatedRequestParams>,
    _ctx: RequestContext<RoleServer>,
) -> Result<ListResourcesResult, McpError> {
    // Collect all resources from different modules
    let mut all_resources: Vec<Resource> = Vec::new();

    all_resources.extend(get_data_resources());
    all_resources.extend(get_shex_resources());

    Ok(ListResourcesResult::with_all_items(all_resources))
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

    if let Some(result) = handle_shex_resource(&uri) {
        return result;
    }

    // Resource not found
    Err(resource_not_found_error(
        "Resource not found",
        "The requested resource does not exist.",
        Some(serde_json::json!({"operation":"read_resource"})),
    ))
}
