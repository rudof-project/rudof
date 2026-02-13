use crate::rudof_mcp_service::{errors::*, service::RudofMcpService};
use iri_s::IriS;
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::{
    InputSpec, RudofConfig, parse_shape_selector, shex::parse_shex_schema,
    shex::serialize_current_shex_rudof, shex::serialize_shape_current_shex_rudof,
    shex_format::ShExFormat,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use rdf::rdf_impl::ReaderMode;
use std::io::Cursor;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShowShexRequest {
    /// ShEx schema
    pub schema: String,

    /// Optional ShEx schema format
    pub schema_format: Option<String>,

    /// Optional base IRI to resolve relative IRIs in schema
    pub base_schema: Option<String>,

    /// Optional reader mode
    pub reader_mode: Option<String>,

    /// Optional shape selector to show a specific shape
    pub shape: Option<String>,

    /// Optional output schema format
    pub result_schema_format: Option<String>,

    /// Optional include compiled IR information when available
    pub compile: Option<bool>,

    /// Optional include the whole schema in the output
    pub show_schema: Option<bool>,

    /// Optional include timing info
    pub show_time: Option<bool>,

    /// Optional show dependencies
    pub show_dependencies: Option<bool>,

    /// Optional show statistics
    pub show_statistics: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShowShexResponse {
    /// Serialized output (text)
    pub results: String,

    /// Format used for serialization
    pub result_format: String,

    /// Size of results in bytes
    pub result_size_bytes: usize,

    /// Number of lines in result
    pub result_lines: usize,
    /// Elapsed time in seconds (optional)
    pub elapsed_seconds: Option<f64>,
    /// Dependencies were included in the response
    pub dependencies_included: Option<bool>,
    /// Statistics were included in the response
    pub statistics_included: Option<bool>,
}

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
    let parsed_schema_format: ShExFormat = match schema_format {
        Some(s) => ShExFormat::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid schema format",
                e.to_string(),
                Some(json!({"operation":"show_shex_impl","phase":"parse_schema_format"})),
            )
        })?,
        None => ShExFormat::default(),
    };

    let parsed_result_format: ShExFormat = match result_schema_format {
        Some(s) => ShExFormat::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid result schema format",
                e.to_string(),
                Some(json!({"operation":"show_shex_impl","phase":"parse_result_format"})),
            )
        })?,
        None => ShExFormat::default(),
    };

    let parsed_base_schema: Option<IriS> = match base_schema {
        Some(s) => Some(IriS::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid base IRI",
                e.to_string(),
                Some(json!({"operation":"show_shex_impl","phase":"parse_base_schema"})),
            )
        })?),
        None => None,
    };

    let parsed_reader_mode: ReaderMode = match reader_mode {
        Some(s) => ReaderMode::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid reader mode",
                e.to_string(),
                Some(json!({"operation":"show_shex_impl","phase":"parse_reader_mode"})),
            )
        })?,
        None => ReaderMode::Strict,
    };

    let rudof_config = RudofConfig::new().unwrap();

    let begin = Instant::now();

    let mut rudof = service.rudof.lock().await;
    let mut output_buffer = Cursor::new(Vec::new());

    // Parse schema into server's Rudof
    let schema_spec = InputSpec::Str(schema.clone());

    parse_shex_schema(
        &mut rudof,
        &schema_spec,
        &parsed_schema_format,
        &parsed_base_schema,
        &parsed_reader_mode,
        &rudof_config,
    )
    .map_err(|e| {
        internal_error(
            "Parsing ShEx schema failed",
            e.to_string(),
            Some(json!({"operation":"show_shex_impl","phase":"parse_shex"})),
        )
    })?;

    // Serialize requested output into buffer
    if let Some(shape_selector_str) = shape {
        let shape_selector = parse_shape_selector(&shape_selector_str).map_err(|e| {
            invalid_request_error(
                "Invalid shape selector",
                e.to_string(),
                Some(json!({"operation":"show_shex_impl","phase":"parse_shape"})),
            )
        })?;
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
        serialize_current_shex_rudof(
            &rudof,
            &parsed_result_format,
            &formatter,
            &mut output_buffer,
        )
        .map_err(|e| {
            internal_error(
                "Serialization failed",
                e.to_string(),
                Some(json!({"operation":"show_shex_impl","phase":"serialize_schema"})),
            )
        })?;
    }

    // Optionally include IR in logs — for MCP we include it in the summary if requested
    let mut summary = format!(
        "# ShEx Show Results\n\n**Result Format:** {}\n",
        parsed_result_format
    );

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
        dependencies_included: if dependencies_included {
            Some(true)
        } else {
            None
        },
        statistics_included: if statistics_included {
            Some(true)
        } else {
            None
        },
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
    contents.push(Content::text(format!(
        "## Schema\n\n```shex\n{}\n```",
        schema
    )));
    contents.push(Content::text(format!(
        "## Serialized\n\n```
{}\n```",
        output_str
    )));
    if let Some(seconds) = response.elapsed_seconds {
        contents.insert(
            1,
            Content::text(format!("**Elapsed:** {:.03?} sec", seconds)),
        );
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CheckShexRequest {
    pub schema: String,
    pub schema_format: Option<String>,
    pub base_schema: Option<String>,
    pub reader_mode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CheckShexResponse {
    pub valid: bool,
    pub message: Option<String>,
    pub elapsed_seconds: Option<f64>,
}

pub async fn check_shex_impl(
    _service: &RudofMcpService,
    Parameters(CheckShexRequest {
        schema,
        schema_format,
        base_schema,
        reader_mode,
    }): Parameters<CheckShexRequest>,
) -> Result<CallToolResult, McpError> {
    let parsed_schema_format: ShExFormat = match schema_format {
        Some(s) => ShExFormat::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid schema format",
                e.to_string(),
                Some(json!({"operation":"check_shex_impl","phase":"parse_schema_format"})),
            )
        })?,
        None => ShExFormat::default(),
    };

    let parsed_base_schema: Option<IriS> = match base_schema {
        Some(s) => Some(IriS::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid base IRI",
                e.to_string(),
                Some(json!({"operation":"check_shex_impl","phase":"parse_base_schema"})),
            )
        })?),
        None => None,
    };

    let parsed_reader_mode: ReaderMode = match reader_mode {
        Some(s) => ReaderMode::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid reader mode",
                e.to_string(),
                Some(json!({"operation":"check_shex_impl","phase":"parse_reader_mode"})),
            )
        })?,
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
        }
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
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShapeInfoRequest {
    pub schema: String,
    pub schema_format: Option<String>,
    pub base_schema: Option<String>,
    pub reader_mode: Option<String>,
    pub shape: String,
    pub result_schema_format: Option<String>,
    pub compile: Option<bool>,
    pub show_time: Option<bool>,
    pub show_dependencies: Option<bool>,
    pub show_statistics: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShapeInfoResponse {
    pub results: String,
    pub result_format: String,
    pub result_size_bytes: usize,
    pub result_lines: usize,
    pub elapsed_seconds: Option<f64>,
    pub dependencies_included: Option<bool>,
    pub statistics_included: Option<bool>,
}

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
    let parsed_schema_format: ShExFormat = match schema_format {
        Some(s) => ShExFormat::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid schema format",
                e.to_string(),
                Some(json!({"operation":"shape_info_impl","phase":"parse_schema_format"})),
            )
        })?,
        None => ShExFormat::default(),
    };

    let parsed_result_format: ShExFormat = match result_schema_format {
        Some(s) => ShExFormat::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid result schema format",
                e.to_string(),
                Some(json!({"operation":"shape_info_impl","phase":"parse_result_format"})),
            )
        })?,
        None => ShExFormat::default(),
    };

    let parsed_base_schema: Option<IriS> = match base_schema {
        Some(s) => Some(IriS::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid base IRI",
                e.to_string(),
                Some(json!({"operation":"shape_info_impl","phase":"parse_base_schema"})),
            )
        })?),
        None => None,
    };

    let parsed_reader_mode: ReaderMode = match reader_mode {
        Some(s) => ReaderMode::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid reader mode",
                e.to_string(),
                Some(json!({"operation":"shape_info_impl","phase":"parse_reader_mode"})),
            )
        })?,
        None => ReaderMode::Strict,
    };

    let rudof_config = RudofConfig::new().unwrap();
    let begin = Instant::now();

    let mut rudof = service.rudof.lock().await;
    let mut output_buffer = Cursor::new(Vec::new());

    // Parse schema into server's Rudof
    let schema_spec = InputSpec::Str(schema.clone());
    parse_shex_schema(
        &mut rudof,
        &schema_spec,
        &parsed_schema_format,
        &parsed_base_schema,
        &parsed_reader_mode,
        &rudof_config,
    )
    .map_err(|e| {
        internal_error(
            "Parsing ShEx schema failed",
            e.to_string(),
            Some(json!({"operation":"shape_info_impl","phase":"parse_shex"})),
        )
    })?;

    // Parse shape selector and serialize only that shape
    let shape_selector = parse_shape_selector(&shape).map_err(|e| {
        invalid_request_error(
            "Invalid shape selector",
            e.to_string(),
            Some(json!({"operation":"shape_info_impl","phase":"parse_shape"})),
        )
    })?;
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
        dependencies_included: if dependencies_included {
            Some(true)
        } else {
            None
        },
        statistics_included: if statistics_included {
            Some(true)
        } else {
            None
        },
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ConvertShexRequest {
    pub schema: String,
    pub schema_format: Option<String>,
    pub base_schema: Option<String>,
    pub reader_mode: Option<String>,
    pub result_schema_format: Option<String>,
    pub show_time: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ConvertShexResponse {
    pub results: String,
    pub result_format: String,
    pub result_size_bytes: usize,
    pub result_lines: usize,
    pub elapsed_seconds: Option<f64>,
}

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
    let parsed_schema_format: ShExFormat = match schema_format {
        Some(s) => ShExFormat::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid schema format",
                e.to_string(),
                Some(json!({"operation":"convert_shex_impl","phase":"parse_schema_format"})),
            )
        })?,
        None => ShExFormat::default(),
    };

    let parsed_result_format: ShExFormat = match result_schema_format {
        Some(s) => ShExFormat::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid result schema format",
                e.to_string(),
                Some(json!({"operation":"convert_shex_impl","phase":"parse_result_format"})),
            )
        })?,
        None => ShExFormat::default(),
    };

    let parsed_base_schema: Option<IriS> = match base_schema {
        Some(s) => Some(IriS::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid base IRI",
                e.to_string(),
                Some(json!({"operation":"convert_shex_impl","phase":"parse_base_schema"})),
            )
        })?),
        None => None,
    };

    let parsed_reader_mode: ReaderMode = match reader_mode {
        Some(s) => ReaderMode::from_str(&s).map_err(|e| {
            invalid_request_error(
                "Invalid reader mode",
                e.to_string(),
                Some(json!({"operation":"convert_shex_impl","phase":"parse_reader_mode"})),
            )
        })?,
        None => ReaderMode::Strict,
    };

    let rudof_config = RudofConfig::new().unwrap();
    let begin = Instant::now();

    let mut rudof = service.rudof.lock().await;
    let mut output_buffer = Cursor::new(Vec::new());

    // Parse schema into server's Rudof
    let schema_spec = InputSpec::Str(schema.clone());
    parse_shex_schema(
        &mut rudof,
        &schema_spec,
        &parsed_schema_format,
        &parsed_base_schema,
        &parsed_reader_mode,
        &rudof_config,
    )
    .map_err(|e| {
        internal_error(
            "Parsing ShEx schema failed",
            e.to_string(),
            Some(json!({"operation":"convert_shex_impl","phase":"parse_shex"})),
        )
    })?;

    // Serialize requested output into buffer (whole schema)
    let formatter = shex_ast::compact::ShExFormatter::default().without_colors();
    serialize_current_shex_rudof(
        &rudof,
        &parsed_result_format,
        &formatter,
        &mut output_buffer,
    )
    .map_err(|e| {
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

    let mut contents = vec![Content::text(format!(
        "**Result Format:** {}",
        parsed_result_format
    ))];
    contents.push(Content::text(format!(
        "```
{}
```",
        output_str
    )));

    let mut result = CallToolResult::success(contents);
    result.structured_content = Some(structured);
    Ok(result)
}
