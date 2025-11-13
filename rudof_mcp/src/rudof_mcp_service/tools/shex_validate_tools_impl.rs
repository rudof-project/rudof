use crate::rudof_mcp_service::{errors::*, service::RudofMcpService};
use iri_s::IriS;
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::{
    InputSpec, RudofConfig, result_shex_validation_format::ResultShExValidationFormat,
    shapemap_format::ShapeMapFormat, shex::validate_shex, shex_format::ShExFormat,
    sort_by_result_shape_map::SortByResultShapeMap,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use srdf::ReaderMode;
use std::io::Cursor;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShexRequest {
    /// ShEx schema to validate against.
    pub schema: String,

    /// Optional ShEx Schema format [default: shexc] [possible values: internal, simple, shexc, shexj, json, jsonld, turtle, ntriples, rdfxml, trig, n3, nquads].
    pub schema_format: Option<String>,

    /// Optional Base Schema (used to resolve relative IRIs in Schema). If not set, falls back to configuration or current working directory.
    pub base_schema: Option<String>,

    /// Optional RDF Reader mode [default: strict] [possible values: lax, strict].
    pub reader_mode: Option<String>,

    /// Optional Node selector (IRI or blank node) to validate.
    pub maybe_node: Option<String>,

    /// Optional Shape label to validate the node against (default = START).
    pub maybe_shape: Option<String>,

    /// Optional ShapeMap inline content mapping nodes to shapes.
    pub shapemap: Option<String>,

    /// Optional ShapeMap format [default: compact] [possible values: compact, internal].
    pub shapemap_format: Option<String>,

    /// Optional Ouput result format [default: compact] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads, compact, json].
    pub result_format: Option<String>,

    /// Optional Sorting mode for the output result table [default: node] [possible values: node, shape, status, details].
    pub sort_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShexResponse {
    /// Validation result output.
    pub results: String,
    
    /// Result format used
    pub result_format: String,
    
    /// Sort order applied
    pub sort_by: String,
    
    /// Size of results in bytes
    pub result_size_bytes: usize,
    
    /// Number of lines in result
    pub result_lines: usize,
    
    /// Whether validation passed (if determinable)
    pub validation_status: Option<String>,
}

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
    let has_shapemap = shapemap.is_some();
    let has_node = maybe_node.is_some();
    let has_shape = maybe_shape.is_some();
    
    let shcema_spec = Some(InputSpec::Str(schema.clone()));

    let parsed_schema_format: Option<ShExFormat> = match schema_format {
        Some(s) => Some(ShExFormat::from_str(&s)
            .map_err(|e| shex_error("parsing schema format", e.to_string()))?),
        None => None,
    };

    let parsed_base_schema: Option<IriS> = match base_schema {
        Some(s) => Some(IriS::from_str(&s)
            .map_err(|e| shex_error("parsing base IRI", e.to_string()))?),
        None => None,
    };

    let parsed_reader_mode: ReaderMode = match reader_mode {
        Some(s) => ReaderMode::from_str(&s)
            .map_err(|e| shex_error("parsing reader mode", e.to_string()))?,
        None => ReaderMode::Strict,
    };

    let shapemap_spec: Option<InputSpec> = shapemap.map(|s| InputSpec::Str(s.clone()));

    let parsed_shapemap_format: ShapeMapFormat = match shapemap_format {
        Some(s) => ShapeMapFormat::from_str(&s)
            .map_err(|e| shapemap_error(e.to_string()))?,
        None => ShapeMapFormat::Compact,
    };

    let parsed_result_format: ResultShExValidationFormat = match result_format {
        Some(s) => ResultShExValidationFormat::from_str(&s)
            .map_err(|e| shex_error("parsing result format", e.to_string()))?,
        None => ResultShExValidationFormat::Compact,
    };

    let parsed_sort_by: SortByResultShapeMap = match sort_by {
        Some(s) => SortByResultShapeMap::from_str(&s)
            .map_err(|e| shex_error("parsing sort order", e.to_string()))?,
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
    .map_err(|e| shex_error("validating", e.to_string()))?;

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes)
        .map_err(|e| shex_error("converting results to UTF-8", e.to_string()))?;

    // Calculate metadata
    let result_size_bytes = output_str.len();
    let result_lines = output_str.lines().count();
    
    // Try to determine validation status from output
    let validation_status = if output_str.contains("conformant") || output_str.contains("✓") {
        Some("conformant".to_string())
    } else if output_str.contains("non-conformant") || output_str.contains("✗") {
        Some("non-conformant".to_string())
    } else {
        None
    };

    let response = ValidateShexResponse {
        results: output_str.to_string(),
        result_format: result_format_str.clone(),
        sort_by: sort_by_str.clone(),
        result_size_bytes,
        result_lines,
        validation_status: validation_status.clone(),
    };

    tracing::info!(
        schema_format = ?parsed_schema_format,
        result_format = %result_format_str,
        sort_by = %sort_by_str,
        result_size_bytes,
        result_lines,
        validation_status = ?validation_status,
        has_node,
        has_shape,
        has_shapemap,
        "ShEx validation executed"
    );

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            error_messages::SERIALIZE_DATA_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    // Create a summary 
    let status_emoji = match validation_status.as_deref() {
        Some("conformant") => "✅",
        Some("non-conformant") => "❌",
        _ => "ℹ️",
    };
    
    let mut summary = format!(
        "# ShEx Validation Results\n\n\
        **Status:** {} {}\n\
        **Result Format:** {}\n\
        **Sort By:** {}\n\
        **Result Size:** {} bytes\n\
        **Result Lines:** {}\n",
        status_emoji,
        validation_status.as_deref().unwrap_or("completed"),
        result_format_str,
        sort_by_str,
        result_size_bytes,
        result_lines
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
        "ntriples" | "nquads" => format!("## Validation Results\n\n```ntriples\n{}\n```", output_str),
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

    // Notify subscribers that validation resources have been updated
    service.notify_resource_updated("rudof://validation-result".to_string()).await;
    service.notify_resource_updated("rudof://schema".to_string()).await;

    Ok(result)
}
