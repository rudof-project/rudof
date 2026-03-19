use crate::service::{errors::*, mcp_service::RudofMcpService};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib_refactored::formats::{InputSpec, ShExFormat};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;
use std::str::FromStr;

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

    /// Include the full schema in output (default: true)
    pub show_schema: Option<bool>,

    /// Include processing time information
    pub show_time: Option<bool>,

    /// Include shape dependency analysis
    pub show_dependencies: Option<bool>,

    /// Include schema statistics (shape counts, extends info)
    pub show_statistics: Option<bool>,
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
    /// Number of lines in result
    pub result_lines: usize,
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
        show_schema,
        show_time,
        show_dependencies,
        show_statistics,
    }): Parameters<ShowShexRequest>,
) -> Result<CallToolResult, McpError> {
    let mut rudof = service.rudof.lock().await;

    let parsed_schema = InputSpec::from_str(&schema).map_err(|e| {
        internal_error(
            "ShEx error",
            e.to_string(),
            Some(json!({"operation":"show_shex","phase":"parse_schema"})),
        )
    })?;

    let mut parsed_schema_format = None;
    if let Some(schema_format) = schema_format.as_deref() {
        parsed_schema_format = Some(ShExFormat::from_str(schema_format).map_err(|e| {
            internal_error(
                "ShEx error",
                e.to_string(),
                Some(json!({"operation":"show_shex","phase":"parse_schema_format"})),
            )
        })?);
    }

    let mut parsed_result_format = None;
    if let Some(result_schema_format) = result_schema_format.as_deref() {
        parsed_result_format = Some(ShExFormat::from_str(result_schema_format).map_err(|e| {
            internal_error(
                "ShEx error",
                e.to_string(),
                Some(json!({"operation":"show_shex","phase":"parse_result_schema_format"})),
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
    if let Some(show_schema) = show_schema {
        serialization = serialization.with_show_schema(show_schema);
    }
    if let Some(show_time) = show_time {
        serialization = serialization.with_show_time(show_time);
    }
    if let Some(show_dependencies) = show_dependencies {
        serialization = serialization.with_show_dependencies(show_dependencies);
    }
    if let Some(show_statistics) = show_statistics {
        serialization = serialization.with_show_statistics(show_statistics);
    }
    if let Some(shape_selector) = shape.as_deref() {
        serialization = serialization.with_shape(shape_selector);
    }
    serialization.execute().map_err(|e| {
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
    let result_lines = output_str.lines().count();

    let result_format_str = if let Some(fmt) = parsed_result_format {
        fmt.to_string()
    } else {
        "shexc".to_string()
    };
    let response = ShowShexResponse {
        results: output_str.clone(),
        result_format: result_format_str,
        result_size_bytes,
        result_lines,
    };

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({"operation":"show_shex_impl","phase":"serialize_response"})),
        )
    })?;

    // Optionally append elapsed time to the human-readable summary
    let mut contents = vec![];
    contents.push(Content::text(format!("## Schema Serialized\n\n```shex\n{}\n```", output_str)));

    let mut result = CallToolResult::success(contents);
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

    let parsed_schema = InputSpec::from_str(&schema).map_err(|e| {
        internal_error(
            "ShEx error",
            e.to_string(),
            Some(json!({"operation":"check_shex_impl","phase":"parse_schema"})),
        )
    })?;

    let mut parsed_schema_format = None;
    if let Some(schema_format) = schema_format.as_deref() {
        parsed_schema_format = Some(ShExFormat::from_str(schema_format).map_err(|e| {
            internal_error(
                "ShEx error",
                e.to_string(),
                Some(json!({"operation":"check_shex_impl","phase":"parse_schema_format"})),
            )
        })?);
    }

    let mut output_buffer = Cursor::new(Vec::new());
    let mut checking = rudof.check_shex_schema(&parsed_schema, &mut output_buffer);
    if let Some(base_schema) = base_schema.as_deref() {
        checking = checking.with_base_schema(base_schema);
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

    // Optionally append elapsed time to the human-readable summary
    let mut contents = vec![];
    contents.push(Content::text(format!("## Schema Serialized\n\n```shex\n{}\n```", output_str)));

    let mut result = CallToolResult::success(contents);
    result.structured_content = Some(structured);

    Ok(result)
}