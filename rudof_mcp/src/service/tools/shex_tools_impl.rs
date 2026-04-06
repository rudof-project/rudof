use crate::service::{errors::*, mcp_service::RudofMcpService};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::formats::{InputSpec, ShExFormat};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;
use std::str::FromStr;

use super::helpers::*;

/// Request parameters for displaying a ShEx schema.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShowShexRequest {
    /// ShEx schema content as a string
    pub schema: String,

    /// Input schema format.
    /// Supported: shexc, shexj, turtle, ntriples, rdfxml, trig, n3, nquads, json, jsonld, internal, simple
    /// Default: shexc
    pub schema_format: Option<String>,

    /// Base IRI for resolving relative IRIs in the schema
    pub base_schema: Option<String>,

    /// Shape selector to display only a specific shape.
    /// Use IRI or prefixed name (e.g., ":Person" or "http://example.org/Person")
    pub shape: Option<String>,

    /// Output format for the schema.
    /// Supported: shexc, shexj, turtle, ntriples, rdfxml, trig, n3, nquads, json, jsonld, internal, simple
    /// Default: shexc
    pub result_schema_format: Option<String>,
}

/// Response from displaying a ShEx schema.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShowShexResponse {
    /// Serialized schema output
    pub results: String,
    /// Format used for output
    pub result_format: String,
    /// Size of results in bytes
    pub result_size_bytes: usize,
}

/// Parse and display a ShEx schema with optional analysis features.
///
/// # Errors
///
/// Returns a Tool Execution Error when:
/// - Schema format is invalid
/// - Result format is invalid
/// - Base IRI is malformed
/// - Reader mode is invalid
/// - Schema parsing fails
/// - Shape selector is invalid
pub async fn show_shex_impl(
    service: &RudofMcpService,
    Parameters(ShowShexRequest {
        schema,
        schema_format,
        base_schema,
        shape,
        result_schema_format,
    }): Parameters<ShowShexRequest>,
) -> Result<CallToolResult, McpError> {
    let mut rudof = service.rudof.lock().await;

    let shex_format_hint = format!("Supported values: {}", SHEX_FORMATS);

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

    let parsed_result_format = match parse_optional_value_with_hint(
        result_schema_format.as_deref(),
        "result schema format",
        &shex_format_hint,
        ShExFormat::from_str,
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
            "ShEx error",
            e.to_string(),
            Some(json!({"operation":"show_shex","phase":"load_schema"})),
        )
    })?;

    let mut output_buffer = Cursor::new(Vec::new());
    let mut serialization = rudof.serialize_shex_schema(&mut output_buffer);
    if let Some(result_schema_format) = &parsed_result_format {
        serialization = serialization.with_result_shex_format(result_schema_format);
    }
    if let Some(shape_selector) = shape.as_deref() {
        serialization = serialization.with_shape(shape_selector);
    }
    serialization
        .with_show_schema(true)
        .with_show_dependencies(true)
        .with_show_statistics(true)
        .execute().map_err(|e| {
            internal_error(
                "ShEx error",
                e.to_string(),
                Some(json!({"operation":"show_shex","phase":"serialize_schema"})),
            )
        })?;

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"show_shex_impl","phase":"utf8_conversion"})),
        )
    })?;

    let result_size_bytes = output_str.len();

    let result_format_str = if let Some(fmt) = parsed_result_format {
        fmt.to_string()
    } else {
        "shexc".to_string()
    };
    let response = ShowShexResponse {
        results: output_str.clone(),
        result_format: result_format_str.clone(),
        result_size_bytes,
    };

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({"operation":"show_shex_impl","phase":"serialize_response"})),
        )
    })?;

    let summary = format!(
        "ShEx schema serialized.\nResult format: {}\nResult size: {} bytes",
        result_format_str, result_size_bytes
    );

    let preview_language = match response.result_format.to_lowercase().as_str() {
        "json" | "jsonld" | "shexj" => "json",
        "turtle" | "n3" => "turtle",
        "ntriples" | "nquads" => "ntriples",
        "rdfxml" => "xml",
        "trig" => "trig",
        "shexc" => "shex",
        _ => "text",
    };
    let results_preview = code_block_preview(preview_language, &output_str, DEFAULT_CONTENT_PREVIEW_CHARS);

    let mut result = CallToolResult::success(vec![
        Content::text(summary),
        Content::text(format!("## Results Preview\n\n{}", results_preview)),
    ]);
    result.structured_content = Some(structured);

    Ok(result)
}

/// Request parameters for checking ShEx schema well-formedness.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CheckShexRequest {
    /// ShEx schema content to check
    pub schema: String,

    /// Input schema format.
    /// Supported: shexc, shexj, turtle
    pub schema_format: Option<String>,

    /// Base IRI for resolving relative IRIs
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
/// Returns a Tool Execution Error for invalid schema format,
/// base IRI, or reader mode parameters.
pub async fn check_shex_impl(
    service: &RudofMcpService,
    Parameters(CheckShexRequest {
        schema,
        schema_format,
        base_schema,
    }): Parameters<CheckShexRequest>,
) -> Result<CallToolResult, McpError> {
    let rudof = service.rudof.lock().await;

    let shex_format_hint = format!("Supported values: {}", SHEX_FORMATS);

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

    let mut output_buffer = Cursor::new(Vec::new());
    let mut checking = rudof.check_shex_schema(&parsed_schema, &mut output_buffer);
    if let Some(base_schema) = base_schema.as_deref() {
        checking = checking.with_base(base_schema);
    }
    if let Some(schema_format) = &parsed_schema_format {
        checking = checking.with_shex_schema_format(schema_format);
    }
    checking.execute().map_err(|e| {
        internal_error(
            "ShEx error",
            e.to_string(),
            Some(json!({"operation":"check_shex_impl","phase":"check_schema"})),
        )
    })?;

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"show_shex_impl","phase":"utf8_conversion"})),
        )
    })?;

    let response = CheckShexResponse {
        result: output_str.clone(),
    };

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({"operation":"show_shex_impl","phase":"serialize_response"})),
        )
    })?;

    let result_size_chars = output_str.chars().count();
    let summary = format!(
        "ShEx schema checked.\nResult size: {} chars",
        result_size_chars
    );

    let results_preview = code_block_preview("text", &output_str, DEFAULT_CONTENT_PREVIEW_CHARS);

    let mut result = CallToolResult::success(vec![
        Content::text(summary),
        Content::text(format!("## Results Preview\n\n{}", results_preview)),
    ]);
    result.structured_content = Some(structured);

    Ok(result)
}
