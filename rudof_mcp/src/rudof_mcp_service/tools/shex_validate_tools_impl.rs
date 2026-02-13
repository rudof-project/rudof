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
use rdf::rdf_impl::ReaderMode;
use std::io::Cursor;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShexRequest {
    /// ShEx schema to validate against.
    pub schema: String,

    /// Optional ShEx Schema format
    pub schema_format: Option<String>,

    /// Optional Base Schema (used to resolve relative IRIs in Schema). If not set, falls back to configuration or current working directory.
    pub base_schema: Option<String>,

    /// Optional RDF Reader mode.
    pub reader_mode: Option<String>,

    /// Optional Node selector (IRI or blank node) to validate.
    pub maybe_node: Option<String>,

    /// Optional Shape label to validate the node against (default = START).
    pub maybe_shape: Option<String>,

    /// Optional ShapeMap inline content mapping nodes to shapes.
    pub shapemap: Option<String>,

    /// Optional ShapeMap format.
    pub shapemap_format: Option<String>,

    /// Optional Ouput result format.
    pub result_format: Option<String>,

    /// Optional Sorting mode for the output result table.
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
    let result_format_str = result_format
        .clone()
        .unwrap_or_else(|| "compact".to_string());
    let sort_by_str = sort_by.clone().unwrap_or_else(|| "node".to_string());

    let shcema_spec = Some(InputSpec::Str(schema.clone()));

    let parsed_schema_format: Option<ShExFormat> = match schema_format {
        Some(s) => Some(ShExFormat::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid schema format",
                e.to_string(),
                Some(json!({"operation":"validate_shex_impl", "phase":"parse_schema_format"})),
            )
        })?),
        None => None,
    };

    let parsed_base_schema: Option<IriS> = match base_schema {
        Some(s) => Some(IriS::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid base IRI",
                e.to_string(),
                Some(json!({"operation":"validate_shex_impl", "phase":"parse_base_schema"})),
            )
        })?),
        None => None,
    };

    let parsed_reader_mode: ReaderMode = match reader_mode {
        Some(s) => ReaderMode::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid reader mode",
                e.to_string(),
                Some(json!({"operation":"validate_shex_impl", "phase":"parse_reader_mode"})),
            )
        })?,
        None => ReaderMode::Strict,
    };

    let shapemap_spec: Option<InputSpec> = shapemap.map(|s| InputSpec::Str(s.clone()));

    let parsed_shapemap_format: ShapeMapFormat = match shapemap_format {
        Some(s) => ShapeMapFormat::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid shapemap format",
                e.to_string(),
                Some(json!({"operation":"validate_shex_impl", "phase":"parse_shapemap_format"})),
            )
        })?,
        None => ShapeMapFormat::Compact,
    };

    let parsed_result_format: ResultShExValidationFormat = match result_format {
        Some(s) => ResultShExValidationFormat::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid result format",
                e.to_string(),
                Some(json!({"operation":"validate_shex_impl", "phase":"parse_result_format"})),
            )
        })?,
        None => ResultShExValidationFormat::Compact,
    };

    let parsed_sort_by: SortByResultShapeMap = match sort_by {
        Some(s) => SortByResultShapeMap::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid sort order",
                e.to_string(),
                Some(json!({"operation":"validate_shex_impl", "phase":"parse_sort_by"})),
            )
        })?,
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
        }
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
