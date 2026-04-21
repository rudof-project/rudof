//! ShEx validation resources.
//!
//! Provides information about ShEx schema formats, validation
//! result formats, reader modes, and sort options.
//!
//! ## Resources
//!
//! - `rudof://formats/shex` - Supported ShEx schema formats
//! - `rudof://formats/shex-validation-result` - ShEx validation result formats
//! - `rudof://formats/validation-reader-modes` - Reader modes (strict/lax)
//! - `rudof://formats/shex-validation-sort-options` - Result sort options

use rmcp::{
    ErrorData as McpError,
    model::{Annotated, RawResource, ReadResourceResult},
};

use crate::service::resources::{json_resource_result, make_resource};
use crate::service::tools::helpers::{
    READER_MODE_ENTRIES, SHEX_FORMAT_ENTRIES, SHEX_VALIDATION_RESULT_FORMAT_ENTRIES,
    SHEX_VALIDATION_SORT_OPTION_ENTRIES, format_entries_json, option_entries_json,
};

/// Returns the list of ShEx validation-related resources.
pub fn get_shex_validate_resources() -> Vec<Annotated<RawResource>> {
    vec![
        make_resource(
            "rudof://formats/shex",
            "Supported ShEx Formats",
            "List of all supported ShEx schema formats",
            "application/json",
        ),
        make_resource(
            "rudof://formats/shex-validation-result",
            "Supported ShEx Validation Result Formats",
            "List of all supported ShEx validation result formats",
            "application/json",
        ),
        make_resource(
            "rudof://formats/validation-reader-modes",
            "Validation Reader Modes",
            "Available reader modes for validation",
            "application/json",
        ),
        make_resource(
            "rudof://formats/shex-validation-sort-options",
            "ShEx Validation Result Sort Options",
            "Available sort options for ShEx validation results",
            "application/json",
        ),
    ]
}

/// Handles ShEx validation resource requests by URI.
///
/// Returns `Some(result)` if the URI matches a known resource,
/// or `None` to allow other handlers to process the request.
pub fn handle_shex_validate_resource(uri: &str) -> Option<Result<ReadResourceResult, McpError>> {
    match uri {
        "rudof://formats/shex" => Some(get_shex_formats(uri)),
        "rudof://formats/shex-validation-result" => Some(get_shex_validation_result_formats(uri)),
        "rudof://formats/validation-reader-modes" => Some(get_reader_modes(uri)),
        "rudof://formats/shex-validation-sort-options" => Some(get_shex_validation_sort_options(uri)),
        _ => None,
    }
}

/// Returns the list of supported ShEx schema formats.
fn get_shex_formats(uri: &str) -> Result<ReadResourceResult, McpError> {
    let formats = format_entries_json(SHEX_FORMAT_ENTRIES, "shexc");
    json_resource_result(uri, &formats)
}

fn get_shex_validation_result_formats(uri: &str) -> Result<ReadResourceResult, McpError> {
    let formats = format_entries_json(SHEX_VALIDATION_RESULT_FORMAT_ENTRIES, "details");
    json_resource_result(uri, &formats)
}

fn get_reader_modes(uri: &str) -> Result<ReadResourceResult, McpError> {
    let modes = option_entries_json("modes", READER_MODE_ENTRIES, "strict");
    json_resource_result(uri, &modes)
}

/// Returns the available sort options for ShEx validation results.
fn get_shex_validation_sort_options(uri: &str) -> Result<ReadResourceResult, McpError> {
    let options = option_entries_json("sort_options", SHEX_VALIDATION_SORT_OPTION_ENTRIES, "node");
    json_resource_result(uri, &options)
}
