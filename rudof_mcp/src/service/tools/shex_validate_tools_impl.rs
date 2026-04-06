use crate::service::{errors::*, mcp_service::RudofMcpService};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::formats::{InputSpec, ResultShExValidationFormat, ShExFormat, ShExValidationSortByMode, ShapeMapFormat};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
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

    /// Base IRI for resolving relative IRIs in data/nodes (new)
    pub base_data: Option<String>,

    /// Node to validate (IRI, prefixed name, or blank node).
    /// If not provided, uses shapemap to determine nodes.
    pub maybe_node: Option<String>,

    /// Shape to validate the node against.
    /// If not provided, uses START or shapemap.
    pub maybe_shape: Option<String>,

    /// ShapeMap content mapping nodes to shapes.
    /// Example: ":alice@:Person, :bob@:Person"
    pub shapemap: String,

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
        base_data,
        maybe_node,
        maybe_shape,
        shapemap,
        shapemap_format,
        result_format,
        sort_by,
    }): Parameters<ValidateShexRequest>,
) -> Result<CallToolResult, McpError> {
    let mut rudof = service.rudof.lock().await;

    let shex_format_hint = format!("Supported values: {}", SHEX_FORMATS);
    let shapemap_format_hint = format!("Supported values: {}", SHAPEMAP_FORMATS);
    let result_format_hint = format!("Supported values: {}", SHEX_RESULT_FORMATS);
    let sort_by_hint = format!("Supported values: {}", SHEX_SORT_BY_MODES);

    let parsed_schema = match parse_value_with_hint(
        &schema,
        "schema",
        "Provide valid schema content, URL, or file path",
        InputSpec::from_str,
    ) {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_schema_format = match parse_optional_value_with_hint(
        schema_format.as_deref(),
        "schema format",
        &shex_format_hint,
        ShExFormat::from_str,
    ) {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_shapemap = match parse_value_with_hint(
        &shapemap,
        "shapemap",
        "Provide a valid ShapeMap value, URL, or file path",
        InputSpec::from_str,
    ) {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_shapemap_format = match parse_optional_value_with_hint(
        shapemap_format.as_deref(),
        "shapemap format",
        &shapemap_format_hint,
        ShapeMapFormat::from_str,
    ) {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_result_format = match parse_optional_value_with_hint(
        result_format.as_deref(),
        "result format",
        &result_format_hint,
        ResultShExValidationFormat::from_str,
    ) {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_sort_by = match parse_optional_value_with_hint(
        sort_by.as_deref(),
        "sort_by",
        &sort_by_hint,
        ShExValidationSortByMode::from_str,
    ) {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let mut shex_schema_loading = rudof.load_shex_schema(&parsed_schema);
    if let Some(base_schema) = base_schema.as_deref() {
        shex_schema_loading = shex_schema_loading.with_base(base_schema);
    }
    if let Some(schema_format) = &parsed_schema_format {
        shex_schema_loading = shex_schema_loading.with_shex_schema_format(schema_format);
    }
    shex_schema_loading.execute().map_err(|e| {
        internal_error(
            "Failed to load ShEx schema",
            e.to_string(),
            Some(json!({"operation":"validate_shex_impl", "phase":"load_schema"})),
        )
    })?;

    let mut shapemap_loading = rudof.load_shapemap(&parsed_shapemap);
    if let Some(shapemap_format) = &parsed_shapemap_format {
        shapemap_loading = shapemap_loading.with_shapemap_format(shapemap_format);
    }
    if let Some(base_data) = base_data.as_deref() {
        shapemap_loading = shapemap_loading.with_base_nodes(base_data);
    }
    if let Some(base_schema) = base_schema.as_deref() {
        shapemap_loading = shapemap_loading.with_base_shapes(base_schema);
    }
    shapemap_loading.execute().map_err(|e| {
        internal_error(
            "Failed to load ShapeMap",
            e.to_string(),
            Some(json!({"operation":"validate_shex_impl", "phase":"load_shapemap"})),
        )
    })?;

    rudof.validate_shex().execute().map_err(|e| {
        internal_error(
            "ShEx validation failed",
            e.to_string(),
            Some(json!({"operation":"validate_shex_impl", "phase":"validate_shex"})),
        )
    })?;

    let mut output_buffer = Cursor::new(Vec::new());
    let mut serialization = rudof.serialize_shex_validation_results(&mut output_buffer);
    if let Some(sort_by) = &parsed_sort_by {
        serialization = serialization.with_shex_validation_sort_order_mode(sort_by);
    }
    if let Some(result_format) = &parsed_result_format {
        serialization = serialization.with_result_shex_validation_format(result_format);
    }
    serialization.execute().map_err(|e| {
        internal_error(
            "Failed to serialize validation results",
            e.to_string(),
            Some(json!({"operation":"validate_shex_impl", "phase":"serialize_results"})),
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

    let result_format_str = if let Some(result_format) = &parsed_result_format {
        result_format.to_string()
    } else {
        "details".to_string()
    };
    let sort_by_str = if let Some(sort_by) = &parsed_sort_by {
        sort_by.to_string()
    } else {
        "node".to_string()
    };
    let response = ValidateShexResponse {
        results: output_str.to_string(),
        result_format: result_format_str.clone(),
        sort_by: sort_by_str.clone(),
        result_size_bytes,
    };

    let structured = serialize_structured(&response, "validate_shex_impl")?;

    let mut summary = format!(
        "ShEx validation completed.\nResult format: {}\nSort by: {}\nResult size: {} bytes",
        result_format_str, sort_by_str, result_size_bytes,
    );

    // Add validation parameters if provided
    if let Some(node) = &maybe_node {
        summary.push_str(&format!("\nNode: {}", node));
    }
    if let Some(shape) = &maybe_shape {
        summary.push_str(&format!("\nShape: {}", shape));
    }

    let schema_preview = code_block_preview("shex", &schema, 600);

    let results_language = match result_format_str.to_lowercase().as_str() {
        "csv" => "csv",
        "json" | "jsonld" => "json",
        "turtle" | "n3" => "turtle",
        "ntriples" | "nquads" => "ntriples",
        "rdfxml" => "xml",
        "trig" => "trig",
        _ => "text",
    };
    let results_preview = code_block_preview(results_language, &output_str, DEFAULT_CONTENT_PREVIEW_CHARS);

    let mut result = CallToolResult::success(vec![
        Content::text(summary),
        Content::text(format!("## Schema Preview\n\n{}", schema_preview)),
        Content::text(format!("## Results Preview\n\n{}", results_preview)),
    ]);
    result.structured_content = Some(structured);

    Ok(result)
}
