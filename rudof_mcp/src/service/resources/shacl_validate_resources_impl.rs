//! SHACL validation resources.
//!
//! Provides information about SHACL schema formats, validation
//! result formats, and sort options.
//!
//! ## Resources
//!
//! - `rudof://formats/shacl` - Supported SHACL schema formats
//! - `rudof://formats/shacl-validation-result` - SHACL validation result formats
//! - `rudof://formats/shacl-validation-sort-options` - SHACL result sort options

use rmcp::{
    ErrorData as McpError,
    model::{Annotated, RawResource, ReadResourceResult},
};

use crate::service::resources::{json_resource_result, make_resource};
use crate::service::tools::helpers::{
    SHACL_FORMAT_ENTRIES, SHACL_VALIDATION_RESULT_FORMAT_ENTRIES, SHACL_VALIDATION_SORT_OPTION_ENTRIES,
    format_entries_json, option_entries_json,
};

/// Returns the list of SHACL validation-related resources.
pub fn get_shacl_validate_resources() -> Vec<Annotated<RawResource>> {
    vec![
        make_resource(
            "rudof://formats/shacl",
            "Supported SHACL Formats",
            "List of all supported SHACL schema formats",
            "application/json",
        ),
        make_resource(
            "rudof://formats/shacl-validation-result",
            "Supported SHACL Validation Result Formats",
            "List of all supported SHACL validation result formats",
            "application/json",
        ),
        make_resource(
            "rudof://formats/shacl-validation-sort-options",
            "SHACL Validation Result Sort Options",
            "Available sort options for SHACL validation results",
            "application/json",
        ),
    ]
}

/// Handles SHACL validation resource requests by URI.
///
/// Returns `Some(result)` if the URI matches a known resource,
/// or `None` to allow other handlers to process the request.
pub fn handle_shacl_validate_resource(uri: &str) -> Option<Result<ReadResourceResult, McpError>> {
    match uri {
        "rudof://formats/shacl" => Some(get_shacl_formats(uri)),
        "rudof://formats/shacl-validation-result" => Some(get_shacl_validation_result_formats(uri)),
        "rudof://formats/shacl-validation-sort-options" => Some(get_shacl_validation_sort_options(uri)),
        _ => None,
    }
}

/// Returns the list of supported SHACL schema formats.
fn get_shacl_formats(uri: &str) -> Result<ReadResourceResult, McpError> {
    let formats = format_entries_json(SHACL_FORMAT_ENTRIES, "turtle");
    json_resource_result(uri, &formats)
}

fn get_shacl_validation_result_formats(uri: &str) -> Result<ReadResourceResult, McpError> {
    let formats = format_entries_json(SHACL_VALIDATION_RESULT_FORMAT_ENTRIES, "details");
    json_resource_result(uri, &formats)
}

/// Returns the available sort options for SHACL validation results.
fn get_shacl_validation_sort_options(uri: &str) -> Result<ReadResourceResult, McpError> {
    let options = option_entries_json("sort_options", SHACL_VALIDATION_SORT_OPTION_ENTRIES, "severity");
    json_resource_result(uri, &options)
}
