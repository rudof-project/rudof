use crate::service::{errors::*, mcp_service::RudofMcpService};
use iri_s::IriS;
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::{
    InputSpec, RudofConfig, result_shex_validation_format::ResultShExValidationFormat, shapemap_format::ShapeMapFormat,
    shex::validate_shex, shex_format::ShExFormat, sort_by_result_shape_map::SortByResultShapeMap,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use srdf::ReaderMode;
use std::io::Cursor;
use std::str::FromStr;

use super::helpers::*;

/// Request parameters for ShEx validation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShexRequest {
    /// ShEx schema content to validate against
    pub schema: String,

    /// Schema input format.
    /// Supported: shexc, shexj, turtle, ntriples, rdfxml, trig, n3, nquads, json, jsonld, internal, simple
    pub schema_format: Option<String>,

    /// Base IRI for resolving relative IRIs in schema
    pub base_schema: Option<String>,

    /// Reader mode for parsing.
    /// Supported: strict, lax
    pub reader_mode: Option<String>,

    /// Node to validate (IRI, prefixed name, or blank node).
    /// If not provided, uses shapemap to determine nodes.
    pub maybe_node: Option<String>,

    /// Shape to validate the node against.
    /// If not provided, uses START or shapemap.
    pub maybe_shape: Option<String>,

    /// ShapeMap content mapping nodes to shapes.
    /// Example: ":alice@:Person, :bob@:Person"
    pub shapemap: Option<String>,

    /// ShapeMap format.
    /// Supported: compact, json, internal, details, csv
    pub shapemap_format: Option<String>,

    /// Output result format.
    /// Supported: compact, details, json, csv, turtle, ntriples, rdfxml, trig, n3, nquads
    pub result_format: Option<String>,

    /// Sort order for results.
    /// Supported: node, shape, status, details
    pub sort_by: Option<String>,
}

/// Response containing ShEx validation results.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShexResponse {
    /// Validation result output
    pub results: String,
    /// Result format used
    pub result_format: String,
    /// Sort order applied
    pub sort_by: String,
    /// Size of results in bytes
    pub result_size_bytes: usize,
    /// Number of lines in result
    pub result_lines: usize,
}

/// Validate RDF data against a ShEx schema.
///
/// # Errors
///
/// Returns a Tool Execution Error when:
/// - Schema format is invalid
/// - Base IRI is malformed
/// - Reader mode is invalid
/// - ShapeMap format is invalid
/// - Result format is invalid
/// - Sort order is invalid
///
/// Returns a Protocol Error for internal validation failures.
pub async fn validate_shex_impl(
    service: &RudofMcpService,
    Parameters(ValidateShexRequest {
        schema,
        schema_format,
        base_schema,
        reader_mode,
        maybe_node,
        maybe_shape,
        shapemap,
        shapemap_format,
        result_format,
        sort_by,
    }): Parameters<ValidateShexRequest>,
) -> Result<CallToolResult, McpError> {
    let result_format_str = result_format.clone().unwrap_or_else(|| "compact".to_string());
    let sort_by_str = sort_by.clone().unwrap_or_else(|| "node".to_string());

    let shcema_spec = Some(InputSpec::Str(schema.clone()));

    // Parse schema format - return Tool Execution Error for invalid format
    let parsed_schema_format: Option<ShExFormat> = match schema_format.as_deref() {
        Some(s) => match ShExFormat::from_str(s) {
            Ok(fmt) => Some(fmt),
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid schema format '{}': {}", s, e),
                    format!("Supported formats: {}", SHEX_FORMATS),
                )
                .into_call_tool_result());
            },
        },
        None => None,
    };

    // Parse base IRI - return Tool Execution Error for malformed IRI
    let parsed_base_schema: Option<IriS> = match base_schema.as_deref() {
        Some(s) => match IriS::from_str(s) {
            Ok(iri) => Some(iri),
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid base IRI '{}': {}", s, e),
                    "Provide a valid absolute IRI (e.g., 'http://example.org/base/')",
                )
                .into_call_tool_result());
            },
        },
        None => None,
    };

    // Parse reader mode - return Tool Execution Error for invalid mode
    let parsed_reader_mode: ReaderMode = match reader_mode.as_deref() {
        Some(s) => match ReaderMode::from_str(s) {
            Ok(mode) => mode,
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid reader mode '{}': {}", s, e),
                    format!("Supported modes: {}", READER_MODES),
                )
                .into_call_tool_result());
            },
        },
        None => ReaderMode::Strict,
    };

    let shapemap_spec: Option<InputSpec> = shapemap.map(|s| InputSpec::Str(s.clone()));

    // Parse shapemap format - return Tool Execution Error for invalid format
    let parsed_shapemap_format: ShapeMapFormat = match shapemap_format.as_deref() {
        Some(s) => match ShapeMapFormat::from_str(s) {
            Ok(fmt) => fmt,
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid shapemap format '{}': {}", s, e),
                    format!("Supported formats: {}", SHAPEMAP_FORMATS),
                )
                .into_call_tool_result());
            },
        },
        None => ShapeMapFormat::Compact,
    };

    // Parse result format - return Tool Execution Error for invalid format
    let parsed_result_format: ResultShExValidationFormat = match result_format.as_deref() {
        Some(s) => match ResultShExValidationFormat::from_str(s) {
            Ok(fmt) => fmt,
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid result format '{}': {}", s, e),
                    format!("Supported formats: {}", SHEX_RESULT_FORMATS),
                )
                .into_call_tool_result());
            },
        },
        None => ResultShExValidationFormat::Compact,
    };

    // Parse sort order - return Tool Execution Error for invalid order
    let parsed_sort_by: SortByResultShapeMap = match sort_by.as_deref() {
        Some(s) => match SortByResultShapeMap::from_str(s) {
            Ok(sort) => sort,
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid sort order '{}': {}", s, e),
                    format!("Supported values: {}", SHEX_SORT_BY_MODES),
                )
                .into_call_tool_result());
            },
        },
        None => SortByResultShapeMap::Node,
    };

    let rudof_config = RudofConfig::new().unwrap();

    let mut rudof = service.rudof.lock().await;
    let mut output_buffer = Cursor::new(Vec::new());

    validate_shex(
        &mut rudof,
        &shcema_spec,
        &parsed_schema_format,
        &parsed_base_schema,
        &parsed_reader_mode,
        &maybe_node,
        &maybe_shape,
        &shapemap_spec,
        &parsed_shapemap_format,
        &parsed_result_format,
        &parsed_sort_by,
        &rudof_config,
        &mut output_buffer,
    )
    .map_err(|e| {
        internal_error(
            "Validation failed",
            e.to_string(),
            Some(json!({"operation":"validate_shex_impl", "phase":"validate_shex"})),
        )
    })?;

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"validate_shex_impl", "phase":"utf8_conversion"})),
        )
    })?;
    // Calculate metadata
    let result_size_bytes = output_str.len();
    let result_lines = output_str.lines().count();

    let response = ValidateShexResponse {
        results: output_str.to_string(),
        result_format: result_format_str.clone(),
        sort_by: sort_by_str.clone(),
        result_size_bytes,
        result_lines,
    };

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({"operation":"validate_shex_impl", "phase":"serialize_response"})),
        )
    })?;

    let mut summary = format!(
        "# ShEx Validation Results\n\n\
        **Result Format:** {}\n\
        **Sort By:** {}\n\
        **Result Size:** {} bytes\n\
        **Result Lines:** {}\n",
        result_format_str, sort_by_str, result_size_bytes, result_lines
    );

    // Add validation parameters if provided
    if let Some(node) = &maybe_node {
        summary.push_str(&format!("**Node:** {}\n", node));
    }
    if let Some(shape) = &maybe_shape {
        summary.push_str(&format!("**Shape:** {}\n", shape));
    }

    let schema_display = format!("## ShEx Schema\n\n```shex\n{}\n```", schema);

    // Format results based on the format type
    let results_display = match result_format_str.to_lowercase().as_str() {
        "turtle" | "n3" => format!("## Validation Results\n\n```turtle\n{}\n```", output_str),
        "ntriples" | "nquads" => {
            format!("## Validation Results\n\n```ntriples\n{}\n```", output_str)
        },
        "rdfxml" => format!("## Validation Results\n\n```xml\n{}\n```", output_str),
        "trig" => format!("## Validation Results\n\n```trig\n{}\n```", output_str),
        "json" | "jsonld" => format!("## Validation Results\n\n```json\n{}\n```", output_str),
        _ => format!("## Validation Results\n\n```\n{}\n```", output_str),
    };

    let mut result = CallToolResult::success(vec![
        Content::text(summary),
        Content::text(schema_display),
        Content::text(results_display),
    ]);
    result.structured_content = Some(structured);

    Ok(result)
}
