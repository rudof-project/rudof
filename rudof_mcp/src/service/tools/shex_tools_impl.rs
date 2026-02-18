use crate::service::{errors::*, mcp_service::RudofMcpService};
use iri_s::IriS;
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::{
    InputSpec, RudofConfig, parse_shape_selector, shex::parse_shex_schema, shex::serialize_current_shex_rudof,
    shex::serialize_shape_current_shex_rudof, shex_format::ShExFormat,
};
use rudof_rdf::rdf_impl::ReaderMode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;
use std::str::FromStr;
use std::time::Instant;

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

    /// Reader mode for parsing.
    /// Supported: strict, lax
    /// Default: strict
    pub reader_mode: Option<String>,

    /// Shape selector to display only a specific shape.
    /// Use IRI or prefixed name (e.g., ":Person" or "http://example.org/Person")
    pub shape: Option<String>,

    /// Output format for the schema.
    /// Supported: shexc, shexj, turtle, ntriples, rdfxml, trig, n3, nquads, json, jsonld, internal, simple
    /// Default: shexc
    pub result_schema_format: Option<String>,

    /// Include compiled intermediate representation analysis
    pub compile: Option<bool>,

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
    /// Processing time in seconds (if show_time was true)
    pub elapsed_seconds: Option<f64>,
    /// Whether dependencies were included
    pub dependencies_included: Option<bool>,
    /// Whether statistics were included
    pub statistics_included: Option<bool>,
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
        reader_mode,
        shape,
        result_schema_format,
        compile,
        show_schema,
        show_time,
        show_dependencies,
        show_statistics,
    }): Parameters<ShowShexRequest>,
) -> Result<CallToolResult, McpError> {
    // Parse schema format - return Tool Execution Error for invalid format
    let parsed_schema_format: ShExFormat = match schema_format.as_deref() {
        Some(s) => match ShExFormat::from_str(s) {
            Ok(fmt) => fmt,
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid schema format '{}': {}", s, e),
                    format!("Supported formats: {}", SHEX_FORMATS),
                )
                .into_call_tool_result());
            },
        },
        None => ShExFormat::default(),
    };

    // Parse result format - return Tool Execution Error for invalid format
    let parsed_result_format: ShExFormat = match result_schema_format.as_deref() {
        Some(s) => match ShExFormat::from_str(s) {
            Ok(fmt) => fmt,
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid result format '{}': {}", s, e),
                    format!("Supported formats: {}", SHEX_FORMATS),
                )
                .into_call_tool_result());
            },
        },
        None => ShExFormat::default(),
    };

    // Parse base IRI - return Tool Execution Error for malformed IRI
    let parsed_base_schema: Option<IriS> = match base_schema.as_deref() {
        Some(s) => match IriS::from_str(s) {
            Ok(iri) => Some(iri),
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid base IRI '{}': {}", s, e),
                    "Provide a valid absolute IRI (e.g., 'http://example.org/base/')",
                )
                .into_call_tool_result());
            },
        },
        None => None,
    };

    // Parse reader mode - return Tool Execution Error for invalid mode
    let parsed_reader_mode: ReaderMode = match reader_mode.as_deref() {
        Some(s) => match ReaderMode::from_str(s) {
            Ok(mode) => mode,
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid reader mode '{}': {}", s, e),
                    format!("Supported modes: {}", READER_MODES),
                )
                .into_call_tool_result());
            },
        },
        None => ReaderMode::Strict,
    };

    let rudof_config = RudofConfig::new().unwrap();

    let begin = Instant::now();

    let mut rudof = service.rudof.lock().await;
    let mut output_buffer = Cursor::new(Vec::new());

    // Parse schema into server's Rudof - return Tool Execution Error for parsing failures
    let schema_spec = InputSpec::Str(schema.clone());

    if let Err(e) = parse_shex_schema(
        &mut rudof,
        &schema_spec,
        &parsed_schema_format,
        &parsed_base_schema,
        &parsed_reader_mode,
        &rudof_config,
    ) {
        return Ok(ToolExecutionError::with_hint(
            format!("ShEx schema parsing failed: {}", e),
            "Check the schema syntax. Common issues: missing prefixes, invalid shape expressions, unbalanced braces.",
        )
        .into_call_tool_result());
    }

    // Serialize requested output into buffer
    if let Some(shape_selector_str) = shape {
        // Parse shape selector - return Tool Execution Error for invalid selector
        let shape_selector = match parse_shape_selector(&shape_selector_str) {
            Ok(sel) => sel,
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid shape selector '{}': {}", shape_selector_str, e),
                    "Use a valid IRI or prefixed name (e.g., ':Person' or 'http://example.org/Person')",
                )
                .into_call_tool_result());
            },
        };
        let formatter = shex_ast::compact::ShExFormatter::default().without_colors();
        serialize_shape_current_shex_rudof(
            &rudof,
            &shape_selector,
            &parsed_result_format,
            &formatter,
            &mut output_buffer,
        )
        .map_err(|e| {
            internal_error(
                "Serialization failed",
                e.to_string(),
                Some(json!({"operation":"show_shex_impl","phase":"serialize_shape"})),
            )
        })?;
    } else if show_schema.unwrap_or(true) {
        let formatter = shex_ast::compact::ShExFormatter::default().without_colors();
        serialize_current_shex_rudof(&rudof, &parsed_result_format, &formatter, &mut output_buffer).map_err(|e| {
            internal_error(
                "Serialization failed",
                e.to_string(),
                Some(json!({"operation":"show_shex_impl","phase":"serialize_schema"})),
            )
        })?;
    }

    // Optionally include IR in logs — for MCP we include it in the summary if requested
    let mut summary = format!("# ShEx Show Results\n\n**Result Format:** {}\n", parsed_result_format);

    let mut dependencies_included = false;
    let mut statistics_included = false;

    if compile.unwrap_or(false) && rudof.get_shex_ir().is_some() {
        summary.push_str("**Compiled IR:** available\n");
        // If requested, include dependencies and statistics in the human-readable contents
        if show_dependencies.unwrap_or(false) {
            dependencies_included = true;
        }
        if show_statistics.unwrap_or(false) {
            statistics_included = true;
        }
    }

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

    let response = ShowShexResponse {
        results: output_str.clone(),
        result_format: parsed_result_format.to_string(),
        result_size_bytes,
        result_lines,
        elapsed_seconds: if show_time.unwrap_or(false) {
            Some(begin.elapsed().as_secs_f64())
        } else {
            None
        },
        dependencies_included: if dependencies_included { Some(true) } else { None },
        statistics_included: if statistics_included { Some(true) } else { None },
    };

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({"operation":"show_shex_impl","phase":"serialize_response"})),
        )
    })?;

    // Optionally append elapsed time to the human-readable summary
    let mut contents = vec![Content::text(summary)];
    contents.push(Content::text(format!("## Schema\n\n```shex\n{}\n```", schema)));
    contents.push(Content::text(format!("## Serialized\n\n```shex\n{}\n```", output_str)));
    if let Some(seconds) = response.elapsed_seconds {
        contents.insert(1, Content::text(format!("**Elapsed:** {:.03?} sec", seconds)));
    }

    // If IR is available and inclusion flags are set, append additional sections
    if (dependencies_included || statistics_included)
        && let Some(shex_ir) = rudof.get_shex_ir()
    {
        if statistics_included {
            // Extends counts
            let mut stats = String::new();
            stats.push_str("## Statistics\n\n");
            stats.push_str(&format!(
                "Local shapes: {}/Total shapes {}\n\n",
                shex_ir.local_shapes_count(),
                shex_ir.total_shapes_count()
            ));
            // Extends counts table
            let extends = shex_ir.count_extends();
            for (k, v) in extends.iter() {
                stats.push_str(&format!("Shapes with {k} extends = {v}\n"));
            }
            contents.push(Content::text(stats));
        }
        if dependencies_included {
            let mut deps = String::new();
            deps.push_str("## Dependencies\n\n");
            for (source, posneg, target) in shex_ir.dependencies() {
                deps.push_str(&format!("{source}-{posneg}->{target}\n"));
            }
            contents.push(Content::text(deps));
        }
    }

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

    /// Reader mode: strict or lax
    pub reader_mode: Option<String>,
}

/// Response from checking ShEx schema well-formedness.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CheckShexResponse {
    /// Whether the schema is valid and well-formed
    pub valid: bool,
    /// Error message if schema is invalid
    pub message: Option<String>,
    /// Processing time in seconds
    pub elapsed_seconds: Option<f64>,
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
    _service: &RudofMcpService,
    Parameters(CheckShexRequest {
        schema,
        schema_format,
        base_schema,
        reader_mode,
    }): Parameters<CheckShexRequest>,
) -> Result<CallToolResult, McpError> {
    // Parse schema format - return Tool Execution Error for invalid format
    let parsed_schema_format: ShExFormat = match schema_format.as_deref() {
        Some(s) => match ShExFormat::from_str(s) {
            Ok(fmt) => fmt,
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid schema format '{}': {}", s, e),
                    format!("Supported formats: {}", SHEX_FORMATS),
                )
                .into_call_tool_result());
            },
        },
        None => ShExFormat::default(),
    };

    // Parse base IRI - return Tool Execution Error for malformed IRI
    let parsed_base_schema: Option<IriS> = match base_schema.as_deref() {
        Some(s) => match IriS::from_str(s) {
            Ok(iri) => Some(iri),
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid base IRI '{}': {}", s, e),
                    "Provide a valid absolute IRI (e.g., 'http://example.org/base/')",
                )
                .into_call_tool_result());
            },
        },
        None => None,
    };

    // Parse reader mode - return Tool Execution Error for invalid mode
    let parsed_reader_mode: ReaderMode = match reader_mode.as_deref() {
        Some(s) => match ReaderMode::from_str(s) {
            Ok(mode) => mode,
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid reader mode '{}': {}", s, e),
                    format!("Supported modes: {}", READER_MODES),
                )
                .into_call_tool_result());
            },
        },
        None => ReaderMode::Strict,
    };

    let rudof_config = RudofConfig::new().unwrap();

    let begin = Instant::now();

    // Use a temporary Rudof instance to avoid mutating service state during validation
    let mut tmp = rudof_lib::Rudof::new(&rudof_config).unwrap();
    let schema_spec = InputSpec::Str(schema.clone());

    match parse_shex_schema(
        &mut tmp,
        &schema_spec,
        &parsed_schema_format,
        &parsed_base_schema,
        &parsed_reader_mode,
        &rudof_config,
    ) {
        Ok(_) => {
            let elapsed = begin.elapsed().as_secs_f64();
            let response = CheckShexResponse {
                valid: true,
                message: None,
                elapsed_seconds: Some(elapsed),
            };
            let structured = serde_json::to_value(&response).map_err(|e| {
                internal_error(
                    "Serialization error",
                    e.to_string(),
                    Some(json!({"operation":"check_shex_impl","phase":"serialize_response"})),
                )
            })?;
            let contents = vec![Content::text("Schema is valid".to_string())];
            let mut result = CallToolResult::success(contents);
            result.structured_content = Some(structured);
            Ok(result)
        },
        Err(err) => {
            // Return success with structured invalid result rather than an McpError so clients can inspect
            let elapsed = begin.elapsed().as_secs_f64();
            let response = CheckShexResponse {
                valid: false,
                message: Some(err.to_string()),
                elapsed_seconds: Some(elapsed),
            };
            let structured = serde_json::to_value(&response).map_err(|e| {
                internal_error(
                    "Serialization error",
                    e.to_string(),
                    Some(json!({"operation":"check_shex_impl","phase":"serialize_response_error"})),
                )
            })?;
            let contents = vec![Content::text(format!("Schema is NOT valid:\n{}", err))];
            let mut result = CallToolResult::success(contents);
            result.structured_content = Some(structured);
            Ok(result)
        },
    }
}

/// Request parameters for getting information about a specific shape.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShapeInfoRequest {
    /// ShEx schema content
    pub schema: String,

    /// Schema format.
    /// Supported: shexc, shexj, turtle
    pub schema_format: Option<String>,

    /// Base IRI for resolving relative IRIs
    pub base_schema: Option<String>,

    /// Reader mode for parsing.
    /// Supported: strict, lax
    pub reader_mode: Option<String>,

    /// Shape to retrieve information about (IRI or prefixed name)
    pub shape: String,

    /// Output format for the result.
    /// Supported: shexc, shexj, turtle
    pub result_schema_format: Option<String>,

    /// Whether to compile the schema to IR
    pub compile: Option<bool>,

    /// Whether to include elapsed time in response
    pub show_time: Option<bool>,

    /// Whether to include dependencies in response
    pub show_dependencies: Option<bool>,

    /// Whether to include statistics in response
    pub show_statistics: Option<bool>,
}

/// Response containing shape information.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShapeInfoResponse {
    /// Serialized shape information
    pub results: String,
    /// Format used for results
    pub result_format: String,
    /// Size of results in bytes
    pub result_size_bytes: usize,
    /// Number of lines in result
    pub result_lines: usize,
    /// Elapsed time in seconds (if requested)
    pub elapsed_seconds: Option<f64>,
    /// Whether dependencies were included
    pub dependencies_included: Option<bool>,
    /// Whether statistics were included
    pub statistics_included: Option<bool>,
}

/// Get information about a specific shape in a ShEx schema.
///
/// # Errors
///
/// Returns a Tool Execution Error when:
/// - Schema format is invalid
/// - Result format is invalid
/// - Base IRI is malformed
/// - Reader mode is invalid
/// - Shape selector is invalid
///
/// Returns a Protocol Error for internal parsing/serialization failures.
pub async fn shape_info_impl(
    service: &RudofMcpService,
    Parameters(ShapeInfoRequest {
        schema,
        schema_format,
        base_schema,
        reader_mode,
        shape,
        result_schema_format,
        compile,
        show_time,
        show_dependencies,
        show_statistics,
    }): Parameters<ShapeInfoRequest>,
) -> Result<CallToolResult, McpError> {
    let parsed_schema_format: ShExFormat =
        match parse_optional_format::<ShExFormat>(schema_format.as_deref(), "schema format", SHEX_FORMATS) {
            Ok(fmt) => fmt,
            Err(e) => return Ok(e.into_call_tool_result()),
        };

    let parsed_result_format: ShExFormat = match parse_optional_format::<ShExFormat>(
        result_schema_format.as_deref(),
        "result schema format",
        SHEX_FORMATS,
    ) {
        Ok(fmt) => fmt,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_base_schema: Option<IriS> = match parse_optional_iri(base_schema.as_deref(), "base IRI") {
        Ok(iri) => iri,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_reader_mode: ReaderMode = match parse_optional_reader_mode(reader_mode.as_deref()) {
        Ok(mode) => mode,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let rudof_config = RudofConfig::new().unwrap();
    let begin = Instant::now();

    let mut rudof = service.rudof.lock().await;
    let mut output_buffer = Cursor::new(Vec::new());

    // Parse schema into server's Rudof - return Tool Execution Error for user's syntax errors
    let schema_spec = InputSpec::Str(schema.clone());
    if let Err(e) = parse_shex_schema(
        &mut rudof,
        &schema_spec,
        &parsed_schema_format,
        &parsed_base_schema,
        &parsed_reader_mode,
        &rudof_config,
    ) {
        return Ok(ToolExecutionError::with_hint(
            format!("ShEx schema parsing failed: {}", e),
            "Check the schema syntax. Common issues: missing prefixes, invalid shape expressions, unbalanced braces.",
        )
        .into_call_tool_result());
    }

    // Parse shape selector and serialize only that shape
    let shape_selector = match parse_shape_selector(&shape) {
        Ok(sel) => sel,
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Invalid shape selector '{}': {}", shape, e),
                "Use a valid IRI or prefixed name (e.g., ':Person' or 'http://example.org/Person')",
            )
            .into_call_tool_result());
        },
    };
    let formatter = shex_ast::compact::ShExFormatter::default().without_colors();
    serialize_shape_current_shex_rudof(
        &rudof,
        &shape_selector,
        &parsed_result_format,
        &formatter,
        &mut output_buffer,
    )
    .map_err(|e| {
        internal_error(
            "Serialization failed",
            e.to_string(),
            Some(json!({"operation":"shape_info_impl","phase":"serialize_shape"})),
        )
    })?;

    let mut dependencies_included = false;
    let mut statistics_included = false;
    if compile.unwrap_or(false) && rudof.get_shex_ir().is_some() {
        if show_dependencies.unwrap_or(false) {
            dependencies_included = true;
        }
        if show_statistics.unwrap_or(false) {
            statistics_included = true;
        }
    }

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"shape_info_impl","phase":"utf8_conversion"})),
        )
    })?;

    let result_size_bytes = output_str.len();
    let result_lines = output_str.lines().count();

    let response = ShapeInfoResponse {
        results: output_str.clone(),
        result_format: parsed_result_format.to_string(),
        result_size_bytes,
        result_lines,
        elapsed_seconds: if show_time.unwrap_or(false) {
            Some(begin.elapsed().as_secs_f64())
        } else {
            None
        },
        dependencies_included: if dependencies_included { Some(true) } else { None },
        statistics_included: if statistics_included { Some(true) } else { None },
    };

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({"operation":"shape_info_impl","phase":"serialize_response"})),
        )
    })?;

    let mut contents = vec![Content::text(format!("## Shape {}", shape))];
    contents.push(Content::text(format!(
        "```
        {}
        ```",
        response.results
    )));

    if let Some(shex_ir) = rudof.get_shex_ir() {
        if statistics_included {
            let mut stats = String::new();
            stats.push_str("## Statistics\n\n");
            stats.push_str(&format!(
                "Local shapes: {}/Total shapes {}\n\n",
                shex_ir.local_shapes_count(),
                shex_ir.total_shapes_count()
            ));
            let extends = shex_ir.count_extends();
            for (k, v) in extends.iter() {
                stats.push_str(&format!("Shapes with {k} extends = {v}\n"));
            }
            contents.push(Content::text(stats));
        }
        if dependencies_included {
            let mut deps = String::new();
            deps.push_str("## Dependencies\n\n");
            for (source, posneg, target) in shex_ir.dependencies() {
                deps.push_str(&format!("{source}-{posneg}->{target}\n"));
            }
            contents.push(Content::text(deps));
        }
    }

    let mut result = CallToolResult::success(contents);
    result.structured_content = Some(structured);
    Ok(result)
}

/// Request parameters for converting ShEx schemas between formats.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ConvertShexRequest {
    /// ShEx schema content to convert
    pub schema: String,

    /// Input schema format.
    /// Supported: shexc, shexj, turtle
    pub schema_format: Option<String>,

    /// Base IRI for resolving relative IRIs
    pub base_schema: Option<String>,

    /// Reader mode for parsing.
    /// Supported: strict, lax
    pub reader_mode: Option<String>,

    /// Output format for the converted schema.
    /// Supported: shexc, shexj, turtle
    pub result_schema_format: Option<String>,

    /// Whether to include elapsed time in response
    pub show_time: Option<bool>,
}

/// Response containing the converted ShEx schema.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ConvertShexResponse {
    /// Converted schema content
    pub results: String,
    /// Format of the result
    pub result_format: String,
    /// Size of results in bytes
    pub result_size_bytes: usize,
    /// Number of lines in result
    pub result_lines: usize,
    /// Elapsed time in seconds (if requested)
    pub elapsed_seconds: Option<f64>,
}

/// Convert a ShEx schema between different formats.
///
/// # Errors
///
/// Returns a Tool Execution Error when:
/// - Schema format is invalid
/// - Result format is invalid
/// - Base IRI is malformed
/// - Reader mode is invalid
/// - Schema has syntax errors
///
/// Returns a Protocol Error for internal serialization failures.
pub async fn convert_shex_impl(
    service: &RudofMcpService,
    Parameters(ConvertShexRequest {
        schema,
        schema_format,
        base_schema,
        reader_mode,
        result_schema_format,
        show_time,
    }): Parameters<ConvertShexRequest>,
) -> Result<CallToolResult, McpError> {
    let parsed_schema_format: ShExFormat =
        match parse_optional_format::<ShExFormat>(schema_format.as_deref(), "schema format", SHEX_FORMATS) {
            Ok(fmt) => fmt,
            Err(e) => return Ok(e.into_call_tool_result()),
        };

    let parsed_result_format: ShExFormat = match parse_optional_format::<ShExFormat>(
        result_schema_format.as_deref(),
        "result schema format",
        SHEX_FORMATS,
    ) {
        Ok(fmt) => fmt,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_base_schema: Option<IriS> = match parse_optional_iri(base_schema.as_deref(), "base IRI") {
        Ok(iri) => iri,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let parsed_reader_mode: ReaderMode = match parse_optional_reader_mode(reader_mode.as_deref()) {
        Ok(mode) => mode,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

    let rudof_config = RudofConfig::new().unwrap();
    let begin = Instant::now();

    let mut rudof = service.rudof.lock().await;
    let mut output_buffer = Cursor::new(Vec::new());

    // Parse schema into server's Rudof - return Tool Execution Error for user's syntax errors
    let schema_spec = InputSpec::Str(schema.clone());
    if let Err(e) = parse_shex_schema(
        &mut rudof,
        &schema_spec,
        &parsed_schema_format,
        &parsed_base_schema,
        &parsed_reader_mode,
        &rudof_config,
    ) {
        return Ok(ToolExecutionError::with_hint(
            format!("ShEx schema parsing failed: {}", e),
            "Check the schema syntax. Common issues: missing prefixes, invalid shape expressions, unbalanced braces.",
        )
        .into_call_tool_result());
    }

    // Serialize requested output into buffer (whole schema)
    let formatter = shex_ast::compact::ShExFormatter::default().without_colors();
    serialize_current_shex_rudof(&rudof, &parsed_result_format, &formatter, &mut output_buffer).map_err(|e| {
        internal_error(
            "Serialization failed",
            e.to_string(),
            Some(json!({"operation":"convert_shex_impl","phase":"serialize_schema"})),
        )
    })?;

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"convert_shex_impl","phase":"utf8_conversion"})),
        )
    })?;

    let result_size_bytes = output_str.len();
    let result_lines = output_str.lines().count();

    let response = ConvertShexResponse {
        results: output_str.clone(),
        result_format: parsed_result_format.to_string(),
        result_size_bytes,
        result_lines,
        elapsed_seconds: if show_time.unwrap_or(false) {
            Some(begin.elapsed().as_secs_f64())
        } else {
            None
        },
    };

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({"operation":"convert_shex_impl","phase":"serialize_response"})),
        )
    })?;

    let mut contents = vec![Content::text(format!("**Result Format:** {}", parsed_result_format))];
    contents.push(Content::text(format!(
        "```shex
        {}
        ```",
        output_str
    )));

    let mut result = CallToolResult::success(contents);
    result.structured_content = Some(structured);
    Ok(result)
}
