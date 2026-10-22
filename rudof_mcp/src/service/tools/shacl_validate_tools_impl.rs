use super::helpers::*;
use super::params::{ResultShaclValidationFormatParam, ShaclFormatParam, ShaclModeParam, ShaclSortByParam};
use crate::service::{errors::*, mcp_service::RudofMcpService};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
};
use rudof_lib::formats::{ResultShaclValidationFormat, ShaclFormat, ShaclValidationMode, ShaclValidationSortByMode};
use rudof_lib::types::{ShaclSeverity, ShaclValidationResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;

/// Request parameters for SHACL validation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShaclRequest {
    /// SHACL shapes graph. Accepts inline text, a URL, or a local file path.
    /// When omitted, shapes embedded in the loaded RDF data are used.
    pub shapes: Option<String>,

    /// Format of the shapes graph. Default: turtle.
    pub shapes_format: Option<ShaclFormatParam>,

    /// Base IRI for resolving relative IRIs in the shapes. Example: "http://example.org/"
    pub base: Option<String>,

    /// SHACL validation engine. Default: native.
    pub mode: Option<ShaclModeParam>,

    /// Output format for the validation report. Default: details.
    pub result_format: Option<ResultShaclValidationFormatParam>,

    /// Sort order for results. Default: severity.
    pub sort_by: Option<ShaclSortByParam>,
}

/// Response containing SHACL validation results.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateShaclResponse {
    /// Whether the data conforms to the shapes.
    pub conforms: bool,
    /// How many results carry each severity, in the order the severities first appear.
    pub counts_by_severity: Vec<SeverityCount>,
    /// One entry per validation result, each carrying the nested `sh:detail` results
    /// that caused it. Empty when the validator was configured not to retain them.
    pub results: Vec<ShaclValidationResult>,
    /// The report as rendered in `result_format`.
    pub report: String,
    /// Result format used
    pub result_format: ResultShaclValidationFormatParam,
    /// Sort order applied
    pub sort_by: ShaclSortByParam,
    /// Size of the rendered report in bytes
    pub result_size_bytes: usize,
}

/// How many validation results carry one severity.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SeverityCount {
    /// The severity counted.
    pub severity: ShaclSeverity,
    /// How many results carry it, nested `sh:detail` results excluded.
    pub count: usize,
}

/// Counts the results of each severity, in the order the severities first appear.
fn counts_by_severity(results: &[ShaclValidationResult]) -> Vec<SeverityCount> {
    let mut counts: Vec<SeverityCount> = Vec::new();
    for result in results {
        match counts.iter_mut().find(|count| &count.severity == result.severity()) {
            Some(count) => count.count += 1,
            None => counts.push(SeverityCount {
                severity: result.severity().clone(),
                count: 1,
            }),
        }
    }
    counts
}

/// Validate RDF data against SHACL shapes.
///
/// # Errors
///
/// Returns a Tool Execution Error when:
/// - The shapes cannot be resolved or parsed
/// - Validation fails to run
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
    let session_dir = service.session_dir.read().await.clone();

    let parsed_shapes = match resolve_optional_spec(&session_dir, shapes.as_deref(), "shapes") {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_shapes_format: ShaclFormat = shapes_format.unwrap_or(ShaclFormatParam::Turtle).into();
    let parsed_mode: ShaclValidationMode = mode.unwrap_or(ShaclModeParam::Native).into();
    let result_format = result_format.unwrap_or(ResultShaclValidationFormatParam::Details);
    let parsed_result_format: ResultShaclValidationFormat = result_format.into();
    let sort_by = sort_by.unwrap_or(ShaclSortByParam::Severity);
    let parsed_sort_by: ShaclValidationSortByMode = sort_by.into();

    let mut loading_shacl_schema = rudof
        .load_shacl_shapes()
        .with_shacl_schema_format(&parsed_shapes_format);
    if let Some(shape) = &parsed_shapes {
        loading_shacl_schema = loading_shacl_schema.with_shacl_schema(shape)
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

    let validation = rudof.validate_shacl().with_shacl_validation_mode(&parsed_mode);
    if let Err(e) = validation.execute() {
        return Ok(ToolExecutionError::with_hint(
            format!("SHACL validation failed: {}", e),
            "Ensure the RDF data is loaded and the SHACL shapes are correct",
        )
        .into_call_tool_result());
    }

    let report = rudof.shacl_validation_results().ok_or_else(|| {
        internal_error(
            "Shacl validation error",
            "Validation reported success but left no report behind",
            Some(json!({"operation":"validate_shacl","phase":"read_validation_report"})),
        )
    })?;
    let conforms = report.conforms();
    let results = report.results().clone();
    let counts_by_severity = counts_by_severity(&results);

    let mut output_buffer = Cursor::new(Vec::new());
    let serialization = rudof
        .serialize_shacl_validation_results(&mut output_buffer)
        .with_result_shacl_validation_format(&parsed_result_format)
        .with_shacl_validation_sort_order_mode(&parsed_sort_by);
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

    let response = ValidateShaclResponse {
        conforms,
        counts_by_severity,
        results,
        report: output_str.clone(),
        result_format,
        sort_by,
        result_size_bytes,
    };

    let structured = serialize_structured(&response, "validate_shacl_impl")?;

    let severities = response
        .counts_by_severity
        .iter()
        .map(|count| format!("{}: {}", count.severity, count.count))
        .collect::<Vec<_>>()
        .join(", ");
    let summary = format!(
        "# SHACL Validation Results\n\n\
        **Conforms:** {}\n\
        **Results:** {}{}\n\
        **Result Format:** {}\n\
        **Sort By:** {}\n\
        **Result Size:** {} bytes\n",
        response.conforms,
        response.results.len(),
        if severities.is_empty() {
            String::new()
        } else {
            format!(" ({severities})")
        },
        result_format,
        sort_by,
        result_size_bytes
    );

    let mut result = CallToolResult::success(vec![
        ContentBlock::text(summary),
        ContentBlock::text(format!("## Validation Results\n\n{}", output_str)),
    ]);
    result.structured_content = Some(structured);

    Ok(result)
}
