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

    let parsed_schema = InputSpec::from_str(&schema).map_err(|e| {
        internal_error(
            "Invalid schema input",
            e.to_string(),
            Some(json!({"operation":"validate_shex_impl", "phase":"parse_schema"})),
        )
    })?;

    let mut parsed_schema_format = None;
    if let Some(schema_format) = &schema_format {
        parsed_schema_format = Some(ShExFormat::from_str(schema_format).map_err(|e| {
            internal_error(
                "Invalid schema format",
                e.to_string(),
                Some(json!({"operation":"validate_shex_impl", "phase":"parse_schema_format"})),
            )
        })?);
    }

    let parsed_shapemap = InputSpec::from_str(&shapemap).map_err(|e| {
        internal_error(
            "Invalid shapemap input",
            e.to_string(),
            Some(json!({"operation":"validate_shex_impl", "phase":"parse_shapemap"})),
        )
    })?;

    let mut parsed_shapemap_format = None;
    if let Some(shapemap_format) = &shapemap_format {
        parsed_shapemap_format = Some(ShapeMapFormat::from_str(shapemap_format).map_err(|e| {
            internal_error(
                "Invalid shapemap format",
                e.to_string(),
                Some(json!({"operation":"validate_shex_impl", "phase":"parse_shapemap_format"})),
            )
        })?);
    }

    let mut parsed_result_format = None;
    if let Some(result_format) = &result_format {
        parsed_result_format = Some(ResultShExValidationFormat::from_str(result_format).map_err(|e| {
            internal_error(
                "Invalid result format",
                e.to_string(),
                Some(json!({"operation":"validate_shex_impl", "phase":"parse_result_format"})),
            )
        })?);
    }

    let mut parsed_sort_by = None;
    if let Some(sort_by) = &sort_by {
        parsed_sort_by = Some(ShExValidationSortByMode::from_str(sort_by).map_err(|e| {
            internal_error(
                "Invalid sort order",
                e.to_string(),
                Some(json!({"operation":"validate_shex_impl", "phase":"parse_sort_by"})),
            )
        })?);
    }

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
    let result_lines = output_str.lines().count();

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
