//! ShEx resources.
//!
//! ## Resources
//!
//! - `rudof://shex/external-resolvers` - Available external-shape resolver kinds

use rmcp::{
    ErrorData as McpError,
    model::{ReadResourceResult, Resource},
};
use rudof_lib::Rudof;
use serde_json::json;

use crate::service::resources::{json_resource_result, make_resource};

/// Returns the list of ShEx-related resources.
pub fn get_shex_resources() -> Vec<Resource> {
    vec![make_resource(
        "rudof://shex/external-resolvers",
        "Available External-Shape Resolvers",
        "Built-in resolvers for ShEx EXTERNAL shape expressions",
        "application/json",
    )]
}

/// Handles ShEx resource requests by URI.
///
/// Returns `Some(result)` if the URI matches a known resource,
/// or `None` to allow other handlers to process the request.
pub fn handle_shex_resource(uri: &str) -> Option<Result<ReadResourceResult, McpError>> {
    match uri {
        "rudof://shex/external-resolvers" => Some(get_external_resolvers(uri)),
        _ => None,
    }
}

/// Returns the list of built-in external-shape resolver kinds.
///
/// Clients pass entries from `resolvers[].spec_syntax` (substituting `<path>`
/// where applicable) as the `external_resolvers` argument to `validate_shex`.
fn get_external_resolvers(uri: &str) -> Result<ReadResourceResult, McpError> {
    let resolvers: Vec<serde_json::Value> = Rudof::list_external_resolvers()
        .into_iter()
        .map(|info| {
            json!({
                "name": info.name,
                "description": info.description,
                "spec_syntax": info.spec_syntax,
            })
        })
        .collect();
    let body = json!({ "resolvers": resolvers });
    json_resource_result(uri, &body)
}
