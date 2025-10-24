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

use crate::rudof_mcp_service::{errors::*, service::RudofMcpService};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LoadRdfDataFromSourcesRequest {
    /// List of data sources (file paths, URLs, raw text)
    pub data: Vec<String>,
    /// RDF format (e.g. "turtle", "jsonld")
    pub data_format: String,
    /// Base IRI for parsing data
    pub base: Option<String>,
    /// Optional SPARQL endpoint URL or name
    pub endpoint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LoadRdfDataFromSourcesResponse {
    /// Message confirming data load
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExportRdfDataRequest {
    /// RDF format (e.g. "turtle", "jsonld")
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExportRdfDataResponse {
    /// Serialized RDF data as a string
    pub data: String,
    /// Format used for serialization (e.g. "turtle", "jsonld")
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExportImageRequest {
    /// Image format: "SVG" or "PNG"
    pub image_format: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExportImageResponse {
    /// Base64 encoded image data
    pub image_data_base64: String,
    /// Image format: "SVG" or "PNG"
    pub image_format: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExportPlantUmlResponse {
    /// PlantUML diagram data as a string
    pub plantuml_data: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmptyRequest {}

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

    let data_specs: Vec<InputSpec> = data
        .iter()
        .map(|s| InputSpec::from_str(s))
        .collect::<Result<_, _>>()
        .map_err(|e| {
            invalid_request(
                error_messages::INVALID_DATA_SPEC,
                Some(json!({ "error": e.to_string() })),
            )
        })?;

    let base_iri = parse_optional_base_iri(base).map_err(|e| {
        invalid_request(
            error_messages::INVALID_BASE_IRI,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let parsed_data_format: DataFormat = RDFFormat::from_str(&data_format)
        .map_err(|e| {
            invalid_request(
                error_messages::INVALID_DATA_FORMAT,
                Some(json!({ "error": e.to_string() })),
            )
        })?
        .into();

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
            error_messages::RDF_LOAD_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let response = LoadRdfDataFromSourcesResponse {
        message: "RDF data loaded from sources/endpoint successfully".to_string(),
    };
    let structured = serde_json::to_value(&response).unwrap();
    let mut result = CallToolResult::success(vec![Content::text(response.message.clone())]);
    result.structured_content = Some(structured);
    Ok(result)
}

pub async fn export_rdf_data_impl(
    service: &RudofMcpService,
    params: Parameters<ExportRdfDataRequest>,
) -> Result<CallToolResult, McpError> {
    let Parameters(ExportRdfDataRequest { format }) = params;
    let rudof = service.rudof.lock().await;

    match RDFFormat::from_str(&format) {
        Ok(parsed_format) => {
            let mut v = Vec::new();
            rudof.serialize_data(&parsed_format, &mut v).map_err(|e| {
                internal_error(
                    error_messages::SERIALIZE_DATA_ERROR,
                    Some(json!({ "error": e.to_string() })),
                )
            })?;
            let str = String::from_utf8(v).map_err(|e| {
                internal_error(
                    error_messages::CONVERSION_ERROR,
                    Some(json!({ "error": e.to_string() })),
                )
            })?;

            let response = ExportRdfDataResponse {
                data: str.clone(),
                format: format.clone(),
            };
            let structured = serde_json::to_value(&response).unwrap();
            let mut result = CallToolResult::success(vec![Content::text(str)]);
            result.structured_content = Some(structured);
            Ok(result)
        }
        Err(e) => Err(invalid_request(
            error_messages::INVALID_DATA_FORMAT,
            Some(json!({ "format": format, "error": e.to_string() })),
        )),
    }
}

pub async fn export_plantuml_impl(
    service: &RudofMcpService,
    _params: Parameters<EmptyRequest>,
) -> Result<CallToolResult, McpError> {
    let rudof = service.rudof.lock().await;
    let mut v = Vec::new();

    rudof.data2plant_uml(&mut v).map_err(|e| {
        internal_error(
            error_messages::SERIALIZE_DATA_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let str = String::from_utf8(v).map_err(|e| {
        internal_error(
            error_messages::CONVERSION_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let response = ExportPlantUmlResponse {
        plantuml_data: str.clone(),
    };
    let structured = serde_json::to_value(&response).unwrap();
    let mut result = CallToolResult::success(vec![Content::text(str)]);
    result.structured_content = Some(structured);
    Ok(result)
}

pub async fn export_image_impl(
    service: &RudofMcpService,
    params: Parameters<ExportImageRequest>,
) -> Result<CallToolResult, McpError> {
    let Parameters(ExportImageRequest { image_format }) = params;
    let rudof = service.rudof.lock().await;

    let format = parse_image_format(&image_format).map_err(|e| {
        invalid_request(
            error_messages::INVALID_EXPORT_FORMAT,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let v = export_rdf_to_image(&rudof, format).map_err(|e| {
        internal_error(
            error_messages::VISUALIZATION_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let base64_data = general_purpose::STANDARD.encode(&v);

    let response = ExportImageResponse {
        image_data_base64: base64_data.clone(),
        image_format: image_format.clone(),
    };
    let structured = serde_json::to_value(&response).unwrap();

    let mut result = CallToolResult::success(vec![Content::text(base64_data)]);

    result.structured_content = Some(structured);
    Ok(result)
}
