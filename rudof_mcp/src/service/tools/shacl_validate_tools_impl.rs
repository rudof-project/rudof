use crate::service::{errors::*, mcp_service::RudofMcpService};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::formats::{
    InputSpec, ResultShaclValidationFormat, ShaclFormat, ShaclValidationMode, ShaclValidationSortByMode,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;
use std::str::FromStr;

use super::helpers::*;

/// Request parameters for SHACL validation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShaclRequest {
    /// SHACL shapes content. If not provided, uses shapes from current data.
    pub shapes: Option<String>,

    /// SHACL shapes format.
    /// Supported: turtle, ntriples, rdfxml, jsonld, trig, n3, nquads, internal
    pub shapes_format: Option<String>,

    /// Base IRI for resolving relative IRIs in shapes
    pub base: Option<String>,

    /// Validation engine mode.
    /// Supported: native, sparql
    pub mode: Option<String>,

    /// Output result format.
    /// Supported: compact, details, minimal, json, csv, turtle, ntriples, rdfxml, trig, n3, nquads
    pub result_format: Option<String>,

    /// Sort order for results.
    /// Supported: severity, node, component, value, path, sourceshape, details
    pub sort_by: Option<String>,
}

/// Response containing SHACL validation results.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShaclResponse {
    /// Validation result output
    pub results: String,
    /// Result format used
    pub result_format: String,
    /// Sort order applied
    pub sort_by: String,
    /// Size of results in bytes
    pub result_size_bytes: usize,
}

/// Validate RDF data against SHACL shapes.
///
/// # Errors
///
/// Returns a Tool Execution Error when:
/// - Shape format is invalid
/// - Base IRI is malformed
/// - Reader mode is invalid
/// - Validation mode is invalid
/// - Result format is invalid
/// - Sort order is invalid
///
/// Returns a Protocol Error for internal validation failures.
pub async fn validate_shacl_impl(
    service: &RudofMcpService,
    Parameters(ValidateShaclRequest {
        shapes,
        shapes_format,
        base,
        mode,
        result_format,
        sort_by,
    }): Parameters<ValidateShaclRequest>,
) -> Result<CallToolResult, McpError> {
    let mut rudof = service.rudof.lock().await;

    let shape_format_hint = format!("Supported values: {}", SHACL_FORMATS);
    let mode_hint = "Supported values: native, sparql";
    let result_format_hint = format!("Supported values: {}", SHACL_RESULT_FORMATS);
    let sort_by_hint = format!("Supported values: {}", SHACL_SORT_BY_MODES);

    let parsed_shapes = match parse_optional_value_with_hint(
        shapes.as_deref(),
        "shapes",
        "Provide valid SHACL shapes content, URL, or file path",
        InputSpec::from_str,
    ) {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_shapes_format = match parse_optional_value_with_hint(
        shapes_format.as_deref(),
        "shapes format",
        &shape_format_hint,
        ShaclFormat::from_str,
    ) {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_mode = match parse_optional_value_with_hint(
        mode.as_deref(),
        "validation mode",
        mode_hint,
        ShaclValidationMode::from_str,
    ) {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_result_format = match parse_optional_value_with_hint(
        result_format.as_deref(),
        "result format",
        &result_format_hint,
        ResultShaclValidationFormat::from_str,
    ) {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_sort_by = match parse_optional_value_with_hint(
        sort_by.as_deref(),
        "sort_by",
        &sort_by_hint,
        ShaclValidationSortByMode::from_str,
    ) {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    // Guard: JSON result format is not yet implemented for SHACL validation.
    if matches!(parsed_result_format, Some(ResultShaclValidationFormat::Json)) {
        return Ok(unsupported_format_error(
            "SHACL validation result",
            "json",
            SHACL_RESULT_FORMATS,
        )
        .into_call_tool_result());
    }

    let mut loading_shacl_schema = rudof.load_shacl_shapes();
    if let Some(shape) = &parsed_shapes {
        loading_shacl_schema = loading_shacl_schema.with_shacl_schema(shape)
    }
    if let Some(shape_format) = &parsed_shapes_format {
        loading_shacl_schema = loading_shacl_schema.with_shacl_schema_format(shape_format);
    }
    if let Some(base_shape) = base.as_deref() {
        loading_shacl_schema = loading_shacl_schema.with_base(base_shape);
    }
    if let Err(e) = loading_shacl_schema.execute() {
        return Ok(ToolExecutionError::with_hint(
            format!("Failed to load SHACL shapes: {}", e),
            "Check the shapes content and shapes_format parameter",
        )
        .into_call_tool_result());
    }

    let mut validation = rudof.validate_shacl();
    if let Some(mode) = &parsed_mode {
        validation = validation.with_shacl_validation_mode(mode);
    }
    if let Err(e) = validation.execute() {
        return Ok(ToolExecutionError::with_hint(
            format!("SHACL validation failed: {}", e),
            "Ensure the RDF data is loaded and the SHACL shapes are correct",
        )
        .into_call_tool_result());
    }

    let mut output_buffer = Cursor::new(Vec::new());
    let mut serialization = rudof.serialize_shacl_validation_results(&mut output_buffer);
    if let Some(result_format) = &parsed_result_format {
        serialization = serialization.with_result_shacl_validation_format(result_format);
    }
    if let Some(sort_by) = &parsed_sort_by {
        serialization = serialization.with_shacl_validation_sort_order_mode(sort_by);
    }
    serialization.execute().map_err(|e| {
        internal_error(
            "Shacl validation error",
            e.to_string(),
            Some(json!({"operation":"validate_shacl","phase":"serialize_validation_results"})),
        )
    })?;

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"validate_shacl_impl", "phase":"utf8_conversion"})),
        )
    })?;

    // Calculate metadata
    let result_size_bytes = output_str.len();

    let result_format_str = if let Some(format) = &parsed_result_format {
        format.to_string()
    } else {
        "details".to_string()
    };
    let sort_by_str = if let Some(sort_by) = &parsed_sort_by {
        sort_by.to_string()
    } else {
        "severity".to_string()
    };
    let response = ValidateShaclResponse {
        results: output_str.clone(),
        result_format: result_format_str.clone(),
        sort_by: sort_by_str.clone(),
        result_size_bytes,
    };

    let structured = serialize_structured(&response, "validate_shacl_impl")?;

    let summary = format!(
        "# SHACL Validation Results\n\n\
        **Result Format:** {}\n\
        **Sort By:** {}\n\
        **Result Size:** {} bytes\n",
        result_format_str, sort_by_str, result_size_bytes
    );

    let mut result = CallToolResult::success(vec![
        Content::text(summary),
        Content::text(format!("## Validation Results\n\n{}", output_str)),
    ]);
    result.structured_content = Some(structured);

    Ok(result)
}
