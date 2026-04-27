use base64::{Engine as _, engine::general_purpose};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::formats::{DataFormat, InputSpec, ResultDataFormat};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;

use super::helpers::*;
use crate::service::{errors::*, mcp_service::RudofMcpService};

/// Request parameters for loading RDF data from various sources.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LoadRdfDataFromSourcesRequest {
    /// List of data sources to load. Each source can be:
    /// - A file path (e.g., "/path/to/file.ttl")
    /// - A URL (e.g., "https://example.org/data.ttl")
    /// - Raw RDF content as a string
    pub data: Vec<String>,

    /// RDF serialization format for parsing the data.
    /// Supported: turtle, ntriples, rdfxml, jsonld, trig, nquads, n3
    pub data_format: Option<String>,

    /// Base IRI for resolving relative IRIs in the data.
    /// Example: "http://example.org/base/"
    pub base: Option<String>,

    /// SPARQL endpoint URL or registered endpoint name.
    /// When provided, data will be loaded from this endpoint.
    pub endpoint: Option<String>,
}

/// Response after successfully loading RDF data.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LoadRdfDataFromSourcesResponse {
    /// Number of data sources that were processed
    pub sources_count: usize,
    /// RDF format that was used for parsing
    pub format: String,
    /// Total number of triples currently loaded in the datastore
    pub triple_count: usize,
}

/// Request parameters for exporting RDF data.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExportRdfDataRequest {
    /// Output RDF serialization format.
    /// Supported: turtle, ntriples, rdfxml, jsonld, trig, nquads, n3
    /// Default: turtle
    pub format: Option<String>,
}

/// Response containing exported RDF data.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExportRdfDataResponse {
    /// Serialized RDF data as a string
    pub data: String,
    /// RDF format used for serialization
    pub format: String,
    /// Size of the serialized data in bytes
    pub size_bytes: usize,
}

/// Request parameters for generating an image visualization.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExportImageRequest {
    /// Image output format.
    /// Supported: svg, png
    pub image_format: String,
}

/// Response containing the generated image.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExportImageResponse {
    /// Image format used (svg or png)
    pub image_format: String,
    /// MIME type for the generated image
    pub mime_type: String,
    /// Binary image size in bytes
    pub size_bytes: usize,
}

/// Response containing a PlantUML diagram.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExportPlantUmlResponse {
    /// PlantUML diagram source code
    pub plantuml_data: String,
    /// Size of the diagram in characters
    pub size: usize,
}

/// Empty request for tools that don't require parameters.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmptyRequest {}

/// Load RDF data from various sources into the server's datastore.
///
/// This function supports loading from:
/// - Local file paths
/// - HTTP/HTTPS URLs
/// - Raw RDF content as strings
///
/// # Errors
///
/// Returns a Tool Execution Error (for LLM self-correction) when:
/// - Data format is invalid or unsupported
/// - Base IRI is malformed
/// - Data source specification is invalid
///
/// Returns a Protocol Error when:
/// - Internal server errors occur during data loading
pub async fn load_rdf_data_from_sources_impl(
    service: &RudofMcpService,
    params: Parameters<LoadRdfDataFromSourcesRequest>,
) -> Result<CallToolResult, McpError> {
    let Parameters(LoadRdfDataFromSourcesRequest {
        data,
        data_format,
        base,
        endpoint,
    }) = params;
    let mut rudof = service.rudof.lock().await;

    // Parse data specifications - return Tool Execution Error for invalid input
    let data_specs: Vec<InputSpec> = match data
        .iter()
        .map(|s| InputSpec::parse_from_str(s, false))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(specs) => specs,
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Invalid data specification: {}", e),
                "Provide valid file paths, URLs, or raw RDF content",
            )
            .into_call_tool_result());
        },
    };

    // Parse data format - return Tool Execution Error for unsupported format
    let data_format_str = data_format.as_deref().unwrap_or("turtle");
    let parsed_data_format: DataFormat = match DataFormat::from_str(data_format_str) {
        Ok(fmt) => fmt,
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Invalid data format '{}': {}", data_format_str, e),
                format!("Supported formats: {}", RDF_FORMATS),
            )
            .into_call_tool_result());
        },
    };

    let mut data_loading = rudof
        .load_data()
        .with_data(&data_specs)
        .with_data_format(&parsed_data_format);
    if let Some(endpoint) = endpoint.as_deref() {
        data_loading = data_loading.with_endpoint(endpoint);
    }
    if let Some(base) = base.as_deref() {
        data_loading = data_loading.with_base(base);
    }
    data_loading.execute().map_err(|e| {
        internal_error(
            "RDF load error",
            e.to_string(),
            Some(json!({"operation":"load_rdf_data_from_sources_impl","phase":"get_data_rudof"})),
        )
    })?;

    // Serialize to N-Triples once — reused for triple counting AND state persistence.
    let mut ntriples_buffer = Vec::new();
    rudof
        .serialize_data(&mut ntriples_buffer)
        .with_result_data_format(&ResultDataFormat::NTriples)
        .execute()
        .map_err(|e| {
            internal_error(
                "RDF count error",
                e.to_string(),
                Some(json!({"operation":"load_rdf_data_from_sources_impl","phase":"serialize_for_count"})),
            )
        })?;

    let ntriples = String::from_utf8(ntriples_buffer).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"load_rdf_data_from_sources_impl","phase":"utf8_count_conversion"})),
        )
    })?;
    let triple_count = RudofMcpService::count_triples_in_ntriples(&ntriples);

    let sources_count = data_specs.len();
    let response = LoadRdfDataFromSourcesResponse {
        sources_count,
        format: data_format_str.to_string(),
        triple_count,
    };

    let structured = serialize_structured(&response, "load_rdf_data_from_sources_impl")?;

    let summary = format!(
        "Successfully loaded RDF data from {} source(s) in {} format. Total triples: {}",
        sources_count, data_format_str, triple_count
    );

    let mut result = CallToolResult::success(vec![Content::text(summary)]);
    result.structured_content = Some(structured);

    // Explicitly release the Mutex lock before the async persist_state_with() call.
    // persist_state_with() re-acquires the lock internally; holding it here would deadlock.
    drop(rudof);

    // Persist state for Docker ephemeral container support, reusing the N-Triples
    // string we already serialized above to avoid a redundant serialization pass.
    if let Err(e) = service.persist_state_with(Some(ntriples)).await {
        tracing::warn!("Failed to persist state after loading RDF data: {}", e);
    }

    Ok(result)
}

/// Export the current RDF data in the specified format.
///
/// # Errors
///
/// Returns a Tool Execution Error when the requested format is unsupported.
/// Returns a Protocol Error for internal serialization failures.
pub async fn export_rdf_data_impl(
    service: &RudofMcpService,
    params: Parameters<ExportRdfDataRequest>,
) -> Result<CallToolResult, McpError> {
    let Parameters(ExportRdfDataRequest { format }) = params;
    let mut rudof = service.rudof.lock().await;
    let format_str = format.as_deref().unwrap_or("turtle");

    // Parse format - return Tool Execution Error for unsupported format
    let parsed_format = match ResultDataFormat::from_str(format_str) {
        Ok(fmt) => fmt,
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Invalid export format '{}': {}", format_str, e),
                format!("Supported formats: {}", RDF_FORMATS),
            )
            .into_call_tool_result());
        },
    };

    let mut v = Vec::new();
    rudof
        .serialize_data(&mut v)
        .with_result_data_format(&parsed_format)
        .execute()
        .map_err(|e| {
            internal_error(
                "Serialization error",
                e.to_string(),
                Some(json!({"operation":"export_rdf_data_impl", "phase":"serialize_data"})),
            )
        })?;

    let size_bytes = v.len();
    let serialized = String::from_utf8(v).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"export_rdf_data_impl", "phase":"utf8_conversion"})),
        )
    })?;
    let response = ExportRdfDataResponse {
        data: serialized.clone(),
        format: format_str.to_string(),
        size_bytes,
    };

    let structured = serialize_structured(&response, "export_rdf_data_impl")?;

    let preview = code_block_preview(format_str, &serialized, DEFAULT_CONTENT_PREVIEW_CHARS);
    let summary = format!(
        "RDF export completed.\nFormat: {}\nSize: {} bytes",
        format_str, size_bytes
    );
    let mut result = CallToolResult::success(vec![
        Content::text(summary),
        Content::text(format!("## Data Preview\n\n{}", preview)),
    ]);
    result.structured_content = Some(structured);

    Ok(result)
}

/// Generate a PlantUML diagram of the current RDF graph.
///
/// The diagram shows the structure of the RDF data including
/// subjects, predicates, and objects.
pub async fn export_plantuml_impl(
    service: &RudofMcpService,
    _params: Parameters<EmptyRequest>,
) -> Result<CallToolResult, McpError> {
    let mut rudof = service.rudof.lock().await;
    let mut v = Vec::new();

    rudof
        .serialize_data(&mut v)
        .with_result_data_format(&ResultDataFormat::PlantUML)
        .execute()
        .map_err(|e| {
            internal_error(
                "Serialization error",
                e.to_string(),
                Some(json!({"operation":"export_plantuml_impl", "phase":"serialize_data"})),
            )
        })?;

    let str = String::from_utf8(v).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"export_plantuml_impl", "phase":"utf8_conversion"})),
        )
    })?;
    let size = str.len();
    let response = ExportPlantUmlResponse {
        plantuml_data: str.clone(),
        size,
    };

    let structured = serialize_structured(&response, "export_plantuml_impl")?;

    let preview = code_block_preview("plantuml", &str, DEFAULT_CONTENT_PREVIEW_CHARS);
    let summary = format!("PlantUML export completed. Size: {} chars", size);
    let mut result = CallToolResult::success(vec![
        Content::text(summary),
        Content::text(format!("## Diagram Preview\n\n{}", preview)),
    ]);
    result.structured_content = Some(structured);

    Ok(result)
}

/// Generate a visual image of the RDF graph.
///
/// # Errors
///
/// Returns a Tool Execution Error for unsupported image formats.
/// Returns a Protocol Error for internal rendering failures.
pub async fn export_image_impl(
    service: &RudofMcpService,
    params: Parameters<ExportImageRequest>,
) -> Result<CallToolResult, McpError> {
    let Parameters(ExportImageRequest { image_format }) = params;
    let mut rudof = service.rudof.lock().await;

    // Parse image format - return Tool Execution Error for unsupported format
    let format = match ResultDataFormat::from_str(&image_format) {
        Ok(fmt) => fmt,
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Invalid image format '{}': {}", image_format, e),
                format!("Supported formats: {}", IMAGE_FORMATS),
            )
            .into_call_tool_result());
        },
    };

    let mut v = Vec::new();

    rudof
        .serialize_data(&mut v)
        .with_result_data_format(&format)
        .execute()
        .map_err(|e| {
            internal_error(
                "Serialization error",
                e.to_string(),
                Some(json!({"operation":"export_image_impl", "phase":"serialize_data"})),
            )
        })?;

    let size_bytes = v.len();
    let base64_data = general_purpose::STANDARD.encode(&v);

    // Determine MIME type based on image format
    let mime_type = match image_format.to_lowercase().as_str() {
        "svg" => "image/svg+xml",
        "png" => "image/png",
        _ => "application/octet-stream",
    };

    let response = ExportImageResponse {
        image_format: image_format.clone(),
        mime_type: mime_type.to_string(),
        size_bytes,
    };

    let structured = serialize_structured(&response, "export_image_impl")?;

    let summary = format!(
        "Image generated successfully ({} format, {} bytes)",
        image_format, size_bytes
    );

    let mut result = CallToolResult::success(vec![Content::text(summary), Content::image(base64_data, mime_type)]);
    result.structured_content = Some(structured);

    Ok(result)
}
