use crate::rudof_mcp_service::{errors::*, service::RudofMcpService};
use iri_s::IriS;
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::{
    InputSpec, RudofConfig, ShaclValidationMode,
    result_shacl_validation_format::{ResultShaclValidationFormat, SortByShaclValidationReport},
    shacl::{add_shacl_schema_rudof, write_validation_report},
    shacl_format::CliShaclFormat,
    shapes_graph_source::ShapesGraphSource,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use rdf::rdf_impl::ReaderMode;
use std::io::Cursor;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShaclRequest {
    /// SHACL shape to validate against.
    pub shape: Option<String>,

    /// Optional SHACL shape format.
    pub shape_format: Option<String>,

    /// Optional Base shape (used to resolve relative IRIs in shape). If not set, falls back to configuration or current working directory.
    pub base_shape: Option<String>,

    /// Optional RDF Reader mode.
    pub reader_mode: Option<String>,

    /// Optional Engine used for validation.
    pub mode: Option<String>,

    /// Optional Ouput result format.
    pub result_format: Option<String>,

    /// Optional Sorting mode for the output result table.
    pub sort_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShaclResponse {
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

pub async fn validate_shacl_impl(
    service: &RudofMcpService,
    Parameters(ValidateShaclRequest {
        shape,
        shape_format,
        base_shape,
        reader_mode,
        mode,
        result_format,
        sort_by,
    }): Parameters<ValidateShaclRequest>,
) -> Result<CallToolResult, McpError> {
    let result_format_str = result_format
        .clone()
        .unwrap_or_else(|| "compact".to_string());
    let sort_by_str = sort_by.clone().unwrap_or_else(|| "node".to_string());

    let shape_spec: Option<InputSpec> = shape.as_ref().map(|s| InputSpec::Str(s.clone()));

    let parsed_shape_format: Option<CliShaclFormat> = match shape_format {
        Some(s) => Some(CliShaclFormat::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid base IRI",
                e.to_string(),
                Some(json!({"operation":"validate_shacl_impl", "phase":"parse_shape_format"})),
            )
        })?),
        None => None,
    };

    let parsed_base_shape: Option<IriS> = match base_shape {
        Some(s) => Some(IriS::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid base IRI",
                e.to_string(),
                Some(json!({"operation":"validate_shacl_impl", "phase":"parse_base_shape"})),
            )
        })?),
        None => None,
    };

    let parsed_reader_mode: ReaderMode = match reader_mode {
        Some(s) => ReaderMode::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid reader mode",
                e.to_string(),
                Some(json!({"operation":"validate_shacl_impl", "phase":"parse_reader_mode"})),
            )
        })?,
        None => ReaderMode::Strict,
    };

    let parsed_mode: ShaclValidationMode = match mode {
        Some(s) => ShaclValidationMode::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid reader mode",
                e.to_string(),
                Some(json!({"operation":"validate_shacl_impl", "phase":"parse_mode"})),
            )
        })?,
        None => ShaclValidationMode::Native,
    };

    let parsed_result_format: ResultShaclValidationFormat = match result_format {
        Some(s) => ResultShaclValidationFormat::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid result format",
                e.to_string(),
                Some(json!({"operation":"validate_shacl_impl", "phase":"parse_result_format"})),
            )
        })?,
        None => ResultShaclValidationFormat::Details,
    };

    let parsed_sort_by: SortByShaclValidationReport = match sort_by {
        Some(s) => SortByShaclValidationReport::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid sort order",
                e.to_string(),
                Some(json!({"operation":"validate_shacl_impl", "phase":"parse_sort_by"})),
            )
        })?,
        None => SortByShaclValidationReport::Severity,
    };

    let rudof_config = RudofConfig::new().unwrap();

    let mut rudof = service.rudof.lock().await;

    let validation_report = if let Some(shape_spec) = shape_spec {
        let shapes_format = parsed_shape_format.unwrap_or_default();
        add_shacl_schema_rudof(
            &mut rudof,
            &shape_spec,
            &shapes_format,
            &parsed_base_shape,
            &parsed_reader_mode,
            &rudof_config,
        )
        .map_err(|e| {
            internal_error(
                "Add SHACL Schema error",
                e.to_string(),
                Some(json!({"operation":"validate_shacl_impl", "phase":"add_shacl_schema"})),
            )
        })?;
        rudof.validate_shacl(&parsed_mode, &ShapesGraphSource::current_schema())
    } else {
        rudof.validate_shacl(&parsed_mode, &ShapesGraphSource::current_data())
    }
    .map_err(|e| {
        internal_error(
            "Validate SHACL error",
            e.to_string(),
            Some(json!({"operation":"validate_shacl_impl", "phase":"validate_shacl"})),
        )
    })?;
    let mut output_buffer = Cursor::new(Vec::new());

    write_validation_report(
        &mut output_buffer,
        &parsed_result_format,
        validation_report,
        &parsed_sort_by,
    )
    .map_err(|e| {
        internal_error(
            "Write validation report error",
            e.to_string(),
            Some(json!({"operation":"validate_shacl_impl", "phase":"write_validation_report"})),
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

    let shape_display = format!(
        "## SHACL Shape\n\n```shacl\n{}\n```",
        shape.clone().unwrap_or_default()
    );

    // Format results based on the format type
    let results_display = match result_format_str.to_lowercase().as_str() {
        "turtle" | "n3" => format!("## Validation Results\n\n```turtle\n{}\n```", output_str),
        "ntriples" | "nquads" => {
            format!("## Validation Results\n\n```ntriples\n{}\n```", output_str)
        }
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
