//! SPARQL query resources.
//!
//! Provides information about supported SPARQL query types
//! and result formats.
//!
//! ## Resources
//!
//! - `rudof://formats/query-types` - Supported SPARQL query types
//! - `rudof://formats/query-results` - Supported query result formats

use rmcp::{
    ErrorData as McpError,
    model::{Annotated, RawResource, ReadResourceResult},
};

use crate::service::resources::{json_resource_result, make_resource};
use crate::service::tools::helpers::{
    SPARQL_QUERY_RESULT_FORMAT_ENTRIES, SPARQL_QUERY_TYPE_ENTRIES, format_entries_json, query_type_entries_json,
};

/// Returns the list of SPARQL query-related resources.
pub fn get_query_resources() -> Vec<Annotated<RawResource>> {
    vec![
        make_resource(
            "rudof://formats/query-types",
            "Supported SPARQL Query Types",
            "List of supported SPARQL query types",
            "application/json",
        ),
        make_resource(
            "rudof://formats/query-results",
            "Supported Query Result Formats",
            "List of all supported SPARQL query result formats",
            "application/json",
        ),
    ]
}

pub fn handle_query_resource(uri: &str) -> Option<Result<ReadResourceResult, McpError>> {
    match uri {
        "rudof://formats/query-types" => Some(get_query_types(uri)),
        "rudof://formats/query-results" => Some(get_query_result_formats(uri)),
        _ => None,
    }
}

fn get_query_types(uri: &str) -> Result<ReadResourceResult, McpError> {
    let types = query_type_entries_json(SPARQL_QUERY_TYPE_ENTRIES);
    json_resource_result(uri, &types)
}

fn get_query_result_formats(uri: &str) -> Result<ReadResourceResult, McpError> {
    let formats = format_entries_json(SPARQL_QUERY_RESULT_FORMAT_ENTRIES, "internal");
    json_resource_result(uri, &formats)
}
