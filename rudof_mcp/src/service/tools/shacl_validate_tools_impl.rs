use crate::service::{errors::*, mcp_service::RudofMcpService};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib_refactored::{
    formats::{InputSpec, ResultShaclValidationFormat, ShaclFormat, ShaclValidationMode, ShaclValidationSortByMode},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;
use std::str::FromStr;

/// Request parameters for SHACL validation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShaclRequest {
    /// SHACL shapes content. If not provided, uses shapes from current data.
    pub shape: Option<String>,

    /// SHACL shapes format.
    /// Supported: turtle, ntriples, rdfxml, jsonld, trig, n3, nquads, internal
    pub shape_format: Option<String>,

    /// Base IRI for resolving relative IRIs in shapes
    pub base_shape: Option<String>,

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
    /// Number of lines in result
    pub result_lines: usize,
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
        shape,
        shape_format,
        base_shape,
        mode,
        result_format,
        sort_by,
    }): Parameters<ValidateShaclRequest>,
) -> Result<CallToolResult, McpError> {
    let mut rudof = service.rudof.lock().await;

    let mut parsed_shapes = None;
    if let Some(shape) = shape.as_deref() {
        parsed_shapes = Some(InputSpec::from_str(shape).map_err(|e| {
            internal_error(
                "Shacl validation error",
                e.to_string(),
                Some(json!({"operation":"validate_shacl","phase":"parse_shapes"})),
            )
        })?);
    }

    let mut parsed_shapes_format = None;
    if let Some(shape_format) = shape_format.as_deref() {
        parsed_shapes_format = Some(ShaclFormat::from_str(shape_format).map_err(|e| {
            internal_error(
                "Shacl validation error",
                e.to_string(),
                Some(json!({"operation":"validate_shacl","phase":"parse_shape_format"})),
            )
        })?);
    }

    let mut parsed_mode = None;
    if let Some(mode) = mode.as_deref() {
        parsed_mode = Some(ShaclValidationMode::from_str(mode).map_err(|e| {
            internal_error(
                "Shacl validation error",
                e.to_string(),
                Some(json!({"operation":"validate_shacl","phase":"parse_validation_mode"})),
            )
        })?);
    }

    let mut parsed_result_format = None;
    if let Some(result_format) = result_format.as_deref() {
        parsed_result_format = Some(ResultShaclValidationFormat::from_str(result_format).map_err(|e| {
            internal_error(
                "Shacl validation error",
                e.to_string(),
                Some(json!({"operation":"validate_shacl","phase":"parse_result_format"})),
            )
        })?);
    }

    let mut parsed_sort_by = None;
    if let Some(sort_by) = sort_by.as_deref() {
        parsed_sort_by = Some(ShaclValidationSortByMode::from_str(sort_by).map_err(|e| {
            internal_error(
                "Shacl validation error",
                e.to_string(),
                Some(json!({"operation":"validate_shacl","phase":"parse_sort_by"})),
            )
        })?);
    }

    let mut loading_shacl_schema = rudof.load_shacl_shapes();
    if let Some(shape) = &parsed_shapes {
        loading_shacl_schema = loading_shacl_schema.with_shacl_schema(shape)
    }
    if let Some(shape_format) = &parsed_shapes_format {
        loading_shacl_schema = loading_shacl_schema.with_shacl_schema_format(shape_format);
    }
    if let Some(base_shape) = base_shape.as_deref() {
        loading_shacl_schema = loading_shacl_schema.with_base(base_shape);
    }
    loading_shacl_schema.execute().map_err(|e| {
        internal_error(
            "Shacl validation error",
            e.to_string(),
            Some(json!({"operation":"validate_shacl","phase":"load_shacl_schema"})),
        )
    })?;

    let mut validation = rudof.validate_shacl();
    if let Some(mode) = &parsed_mode {
        validation = validation.with_shacl_validation_mode(mode);
    }
    validation.execute().map_err(|e| {
        internal_error(
            "Shacl validation error",
            e.to_string(),
            Some(json!({"operation":"validate_shacl","phase":"execute_validation"})),
        )
    })?;

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
    let result_lines = output_str.lines().count();

    let result_format_str = if let Some(format) = &parsed_result_format {
        format.to_string()
    } else {
         "turtle".to_string()
    };
    let sort_by_str = if let Some(sort_by) = &parsed_sort_by {
        sort_by.to_string()
    } else {
        "severity".to_string()
    };
    let response = ValidateShaclResponse {
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
            Some(json!({"operation":"validate_shacl_impl", "phase":"serialize_response"})),
        )
    })?;

    let summary = format!(
        "# SHACL Validation Results\n\n\
        **Result Format:** {}\n\
        **Sort By:** {}\n\
        **Result Size:** {} bytes\n\
        **Result Lines:** {}\n",
        result_format_str, sort_by_str, result_size_bytes, result_lines
    );

    let shape_display = format!("## SHACL Shape\n\n```shacl\n{}\n```", shape.clone().unwrap_or_default());

    // Format results based on the format type
    let results_display = match result_format_str.to_lowercase().as_str() {
        "turtle" | "n3" => format!("## Validation Results\n\n```turtle\n{}\n```", output_str),
        "ntriples" | "nquads" => {
            format!("## Validation Results\n\n```ntriples\n{}\n```", output_str)
        },
        "rdfxml" => format!("## Validation Results\n\n```xml\n{}\n```", output_str),
        "trig" => format!("## Validation Results\n\n```trig\n{}\n```", output_str),
        "json" => format!("## Validation Results\n\n```json\n{}\n```", output_str),
        _ => format!("## Validation Results\n\n```\n{}\n```", output_str),
    };

    let mut result = CallToolResult::success(vec![
        Content::text(summary),
        Content::text(shape_display),
        Content::text(results_display),
    ]);
    result.structured_content = Some(structured);

    Ok(result)
}
