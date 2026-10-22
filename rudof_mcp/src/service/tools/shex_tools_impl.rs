use crate::service::{errors::*, mcp_service::RudofMcpService};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
};
use rudof_lib::Rudof;
use rudof_lib::formats::ShExFormat;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;

use super::helpers::*;
use super::params::{ResultShExFormatParam, ShExFormatParam};

/// Request parameters for displaying a ShEx schema.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShowShexRequest {
    /// ShEx schema to parse. Accepts inline ShExC/ShExJ text, a URL, or a local file path.
    pub schema: String,

    /// Format of the input schema. Default: shexc.
    pub schema_format: Option<ShExFormatParam>,

    /// Base IRI for resolving relative IRIs in the schema. Example: "http://example.org/"
    pub base_schema: Option<String>,

    /// Display only a specific shape by IRI or prefixed name.
    /// Example: ":Person" or "<http://example.org/Person>". When omitted, all shapes are shown.
    pub shape: Option<String>,

    /// Output format for the serialized schema. Default: shexc.
    pub result_schema_format: Option<ResultShExFormatParam>,

    /// Also report the shape counts and the shapes the schema defines. Default: false.
    pub show_statistics: Option<bool>,

    /// Also report the dependencies between the shapes. Default: false.
    pub show_dependencies: Option<bool>,
}

/// Response from displaying a ShEx schema.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShowShexResponse {
    /// The schema serialized in `result_format`.
    pub schema: String,
    /// The statistics, when `show_statistics` asked for them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<String>,
    /// The shape dependencies, when `show_dependencies` asked for them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<String>,
    /// Format used for output
    pub result_format: ResultShExFormatParam,
    /// Size of the serialized schema in bytes
    pub result_size_bytes: usize,
}

/// Which of the three sections of `serialize_shex_schema` to write.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShexSection {
    Schema,
    Statistics,
    Dependencies,
}

/// Serializes one section of the loaded schema on its own, so each reaches the caller as
/// its own field instead of all three concatenated into one string.
fn serialize_section(
    rudof: &Rudof,
    format: &ShExFormat,
    shape: Option<&str>,
    section: ShexSection,
) -> Result<ToolResult<String>, McpError> {
    let mut buffer = Cursor::new(Vec::new());
    let mut serialization = rudof
        .serialize_shex_schema(&mut buffer)
        .with_result_shex_format(format)
        .with_show_schema(section == ShexSection::Schema)
        .with_show_statistics(section == ShexSection::Statistics)
        .with_show_dependencies(section == ShexSection::Dependencies);
    // The shape selector narrows the schema, and is meaningless for the other two.
    if section == ShexSection::Schema
        && let Some(shape) = shape
    {
        serialization = serialization.with_shape(shape);
    }
    if let Err(e) = serialization.execute() {
        return Ok(Err(ToolExecutionError::with_hint(
            format!("Failed to serialize ShEx schema: {}", e),
            "Try a different result_schema_format or verify the shape selector",
        )));
    }

    let serialized = String::from_utf8(buffer.into_inner()).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"show_shex_impl","phase":"utf8_conversion"})),
        )
    })?;
    Ok(Ok(serialized))
}

/// Parse and display a ShEx schema with optional analysis features.
///
/// # Errors
///
/// Returns a Tool Execution Error when:
/// - The schema cannot be resolved or parsed
/// - The shape selector is invalid
pub async fn show_shex_impl(
    service: &RudofMcpService,
    Parameters(ShowShexRequest {
        schema,
        schema_format,
        base_schema,
        shape,
        result_schema_format,
        show_statistics,
        show_dependencies,
    }): Parameters<ShowShexRequest>,
) -> Result<CallToolResult, McpError> {
    let mut rudof = service.rudof.lock().await;
    let session_dir = service.session_dir.read().await.clone();

    let parsed_schema = match resolve_spec(&session_dir, &schema, "schema") {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let schema_format = schema_format.unwrap_or(ShExFormatParam::ShExC);
    let parsed_schema_format: ShExFormat = schema_format.into();
    let result_schema_format = result_schema_format.unwrap_or(ResultShExFormatParam::ShExC);
    let parsed_result_format: ShExFormat = result_schema_format.into();

    let mut shex_schema_loading = rudof
        .load_shex_schema(&parsed_schema)
        .with_shex_schema_format(&parsed_schema_format);
    if let Some(base_schema) = base_schema.as_deref() {
        shex_schema_loading = shex_schema_loading.with_base(base_schema);
    }
    if let Err(e) = shex_schema_loading.execute() {
        return Ok(ToolExecutionError::with_hint(
            format!("Failed to parse ShEx schema: {}", e),
            "Check the schema content and schema_format parameter",
        )
        .into_call_tool_result());
    }

    let schema_output = match serialize_section(&rudof, &parsed_result_format, shape.as_deref(), ShexSection::Schema)? {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let statistics = match show_statistics.unwrap_or(false) {
        true => match serialize_section(&rudof, &parsed_result_format, None, ShexSection::Statistics)? {
            Ok(value) => Some(value),
            Err(e) => return Ok(e.into_call_tool_result()),
        },
        false => None,
    };
    let dependencies = match show_dependencies.unwrap_or(false) {
        true => match serialize_section(&rudof, &parsed_result_format, None, ShexSection::Dependencies)? {
            Ok(value) => Some(value),
            Err(e) => return Ok(e.into_call_tool_result()),
        },
        false => None,
    };

    let result_size_bytes = schema_output.len();

    let response = ShowShexResponse {
        schema: schema_output.clone(),
        statistics,
        dependencies,
        result_format: result_schema_format,
        result_size_bytes,
    };

    let structured = serialize_structured(&response, "show_shex_impl")?;

    let summary = format!(
        "ShEx schema serialized.\nResult format: {}\nSchema size: {} bytes",
        result_schema_format, result_size_bytes
    );

    let preview_language = match result_schema_format.as_str() {
        "json" | "jsonld" | "shexj" => "json",
        "turtle" | "n3" => "turtle",
        "ntriples" | "nquads" => "ntriples",
        "rdfxml" => "xml",
        "trig" => "trig",
        "shexc" => "shex",
        _ => "text",
    };
    let results_preview = code_block_preview(preview_language, &schema_output, DEFAULT_CONTENT_PREVIEW_CHARS);

    let mut result = CallToolResult::success(vec![
        ContentBlock::text(summary),
        ContentBlock::text(format!("## Results Preview\n\n{}", results_preview)),
    ]);
    result.structured_content = Some(structured);

    Ok(result)
}

/// Request parameters for checking ShEx schema well-formedness.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CheckShexRequest {
    /// ShEx schema to check. Accepts inline ShExC/ShExJ text, a URL, or a local file path.
    pub schema: String,

    /// Format of the input schema. Default: shexc.
    pub schema_format: Option<ShExFormatParam>,

    /// Base IRI for resolving relative IRIs in the schema. Example: "http://example.org/"
    pub base_schema: Option<String>,
}

/// Response from checking ShEx schema well-formedness.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CheckShexResponse {
    /// Error message if schema is invalid
    pub result: String,
}

/// Check if a ShEx schema is syntactically valid and well-formed.
///
/// This tool validates the schema syntax without performing any
/// RDF validation. Returns structured information about validity.
///
/// # Errors
///
/// Returns a Tool Execution Error when the schema is not well-formed.
pub async fn check_shex_impl(
    service: &RudofMcpService,
    Parameters(CheckShexRequest {
        schema,
        schema_format,
        base_schema,
    }): Parameters<CheckShexRequest>,
) -> Result<CallToolResult, McpError> {
    let rudof = service.rudof.lock().await;
    let session_dir = service.session_dir.read().await.clone();

    let parsed_schema = match resolve_spec(&session_dir, &schema, "schema") {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_schema_format: ShExFormat = schema_format.unwrap_or(ShExFormatParam::ShExC).into();

    let mut output_buffer = Cursor::new(Vec::new());
    let mut checking = rudof
        .check_shex_schema(&parsed_schema, &mut output_buffer)
        .with_shex_schema_format(&parsed_schema_format);
    if let Some(base_schema) = base_schema.as_deref() {
        checking = checking.with_base(base_schema);
    }
    if let Err(e) = checking.execute() {
        return Ok(ToolExecutionError::with_hint(
            format!("ShEx schema is not well-formed: {}", e),
            "Review the schema syntax and correct the reported errors",
        )
        .into_call_tool_result());
    }

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"check_shex_impl","phase":"utf8_conversion"})),
        )
    })?;

    let response = CheckShexResponse {
        result: output_str.clone(),
    };

    let structured = serialize_structured(&response, "check_shex_impl")?;

    let result_size_chars = output_str.chars().count();
    let summary = format!("ShEx schema checked.\nResult size: {} chars", result_size_chars);

    let results_preview = code_block_preview("text", &output_str, DEFAULT_CONTENT_PREVIEW_CHARS);

    let mut result = CallToolResult::success(vec![
        ContentBlock::text(summary),
        ContentBlock::text(format!("## Results Preview\n\n{}", results_preview)),
    ]);
    result.structured_content = Some(structured);

    Ok(result)
}
