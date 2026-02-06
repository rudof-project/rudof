use base64::{Engine as _, engine::general_purpose};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::{
    InputSpec, RDFFormat, ReaderMode,
    data::{export_rdf_to_image, get_data_rudof, parse_image_format, parse_optional_base_iri},
    data_format::DataFormat,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;

use super::helpers::*;
use crate::service::{errors::*, service::RudofMcpService};

/// Request parameters for loading RDF data from various sources.
///
/// Supports loading from:
/// - Local file paths (e.g., `/path/to/data.ttl`)
/// - URLs (e.g., `https://example.org/data.ttl`)
/// - Raw RDF content as a string
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
    /// Human-readable message confirming the data load
    pub message: String,
    /// Number of data sources that were processed
    pub sources_count: usize,
    /// RDF format that was used for parsing
    pub format: String,
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
    /// Base64 encoded image data
    pub image_data_base64: String,
    /// Image format used (svg or png)
    pub image_format: String,
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
    let config = rudof.config().clone();

    // Parse data specifications - return Tool Execution Error for invalid input
    let data_specs: Vec<InputSpec> = match data
        .iter()
        .map(|s| InputSpec::from_str(s))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(specs) => specs,
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Invalid data specification: {}", e),
                "Provide valid file paths, URLs, or raw RDF content",
            )
            .into_call_tool_result());
        }
    };

    // Parse base IRI - return Tool Execution Error for malformed IRI
    let base_iri = match parse_optional_base_iri(base) {
        Ok(iri) => iri,
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Invalid base IRI: {}", e),
                "Provide a valid absolute IRI (e.g., 'http://example.org/base/')",
            )
            .into_call_tool_result());
        }
    };

    // Parse data format - return Tool Execution Error for unsupported format
    let data_format_str = data_format.as_deref().unwrap_or("turtle");
    let parsed_data_format: DataFormat = match RDFFormat::from_str(data_format_str) {
        Ok(fmt) => fmt.into(),
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Invalid data format '{}': {}", data_format_str, e),
                format!("Supported formats: {}", RDF_FORMATS),
            )
            .into_call_tool_result());
        }
    };

    get_data_rudof(
        &mut rudof,
        &data_specs,
        &parsed_data_format,
        &base_iri,
        &endpoint,
        &ReaderMode::default(),
        &config,
        false,
    )
    .map_err(|e| {
        internal_error(
            "RDF load error",
            e.to_string(),
            Some(json!({"operation":"load_rdf_data_from_sources_impl","phase":"get_data_rudof"})),
        )
    })?;

    let sources_count = data_specs.len();
    let response = LoadRdfDataFromSourcesResponse {
        message: format!(
            "Successfully loaded RDF data from {} source(s) in {} format",
            sources_count, data_format_str
        ),
        sources_count,
        format: data_format_str.to_string(),
    };

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({"operation":"load_rdf_data_from_sources_impl", "phase":"serialize_response"})),
        )
    })?;
    let mut result = CallToolResult::success(vec![Content::text(response.message.clone())]);
    result.structured_content = Some(structured);

    // Release the lock before async operations (notifications and persistence)
    drop(rudof);

    // Notify subscribers that all current-data resources have been updated
    const DATA_RESOURCE_URIS: &[&str] = &[
        "rudof://current-data",
        "rudof://current-data/ntriples",
        "rudof://current-data/rdfxml",
        "rudof://current-data/jsonld",
        "rudof://current-data/trig",
        "rudof://current-data/nquads",
        "rudof://current-data/n3",
    ];

    for uri in DATA_RESOURCE_URIS {
        service.notify_resource_updated((*uri).to_string()).await;
    }

    // Persist state for Docker ephemeral container support
    if let Err(e) = service.persist_state().await {
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
    let rudof = service.rudof.lock().await;
    let format_str = format.as_deref().unwrap_or("turtle");

    // Parse format - return Tool Execution Error for unsupported format
    let parsed_format = match RDFFormat::from_str(format_str) {
        Ok(fmt) => fmt,
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Invalid export format '{}': {}", format_str, e),
                format!("Supported formats: {}", RDF_FORMATS),
            )
            .into_call_tool_result());
        }
    };

    let mut v = Vec::new();
    rudof.serialize_data(&parsed_format, &mut v).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({"operation":"export_rdf_data_impl", "phase":"serialize_data"})),
        )
    })?;

    let size_bytes = v.len();
    let str = String::from_utf8(v).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"export_rdf_data_impl", "phase":"utf8_conversion"})),
        )
    })?;
    let response = ExportRdfDataResponse {
        data: str.clone(),
        format: format_str.to_string(),
        size_bytes,
    };

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({"operation":"export_rdf_data_impl", "phase":"serialize_response"})),
        )
    })?;

    let formatted_data = format!("```{}\n{}\n```", format_str, str);
    let mut result = CallToolResult::success(vec![Content::text(formatted_data)]);
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
    let rudof = service.rudof.lock().await;
    let mut v = Vec::new();

    rudof.data2plant_uml(&mut v).map_err(|e| {
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

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({"operation":"export_plantuml_impl", "phase":"serialize_response"})),
        )
    })?;

    // Format as PlantUML code block
    let formatted_data = format!("```plantuml\n{}\n```", str);
    let mut result = CallToolResult::success(vec![Content::text(formatted_data)]);
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
    let rudof = service.rudof.lock().await;

    // Parse image format - return Tool Execution Error for unsupported format
    let format = match parse_image_format(&image_format) {
        Ok(fmt) => fmt,
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Invalid image format '{}': {}", image_format, e),
                format!("Supported formats: {}", IMAGE_FORMATS),
            )
            .into_call_tool_result());
        }
    };

    let v = export_rdf_to_image(&rudof, format).map_err(|e| {
        internal_error(
            "Export rdf to image error",
            e.to_string(),
            Some(json!({"operation":"export_image_impl", "phase":"export_rdf_to_image"})),
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
        image_data_base64: base64_data.clone(),
        image_format: image_format.clone(),
    };

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({"operation":"export_image_impl", "phase":"serialize_response"})),
        )
    })?;

    let description = format!(
        "Image generated successfully ({} format, {} bytes)",
        image_format, size_bytes
    );

    let mut result = CallToolResult::success(vec![
        Content::text(description),
        Content::image(base64_data, mime_type),
    ]);

    result.structured_content = Some(structured);

    Ok(result)
}
