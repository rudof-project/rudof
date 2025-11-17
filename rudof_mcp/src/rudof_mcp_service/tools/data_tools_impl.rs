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
    /// Optional RDF format
    pub data_format: Option<String>,
    /// Optional Base IRI for parsing data
    pub base: Option<String>,
    /// Optional SPARQL endpoint URL or name
    pub endpoint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LoadRdfDataFromSourcesResponse {
    /// Message confirming data load
    pub message: String,
    /// Number of sources processed
    pub sources_count: usize,
    /// RDF format used
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExportRdfDataRequest {
    /// Optional RDF format
    pub format: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExportRdfDataResponse {
    /// Serialized RDF data as a string
    pub data: String,
    /// RDF format used for serialization
    pub format: String,
    /// Size of the serialized data in bytes
    pub size_bytes: usize,
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
    /// Size of the diagram in characters
    pub size: usize,
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
        .map_err(|e| rdf_error("parsing data specs", e.to_string()))?;

    let base_iri =
        parse_optional_base_iri(base).map_err(|e| rdf_error("parsing base IRI", e.to_string()))?;

    let data_format_str = data_format.as_deref().unwrap_or("turtle");
    let parsed_data_format: DataFormat = RDFFormat::from_str(data_format_str)
        .map_err(|e| rdf_error("parsing data format", e.to_string()))?
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
    .map_err(|e| rdf_error("loading RDF data", e.to_string()))?;

    let sources_count = data_specs.len();
    let response = LoadRdfDataFromSourcesResponse {
        message: format!(
            "Successfully loaded RDF data from {} source(s) in {} format",
            sources_count, data_format_str
        ),
        sources_count,
        format: data_format_str.to_string(),
    };

    let structured = serde_json::to_value(&response).unwrap();
    let mut result = CallToolResult::success(vec![Content::text(response.message.clone())]);
    result.structured_content = Some(structured);

    tracing::info!(
        sources = sources_count,
        format = %data_format_str,
        "RDF data loaded successfully"
    );

    // Notify subscribers that all current-data resources have been updated
    service
        .notify_resource_updated("rudof://current-data".to_string())
        .await;
    service
        .notify_resource_updated("rudof://current-data/ntriples".to_string())
        .await;
    service
        .notify_resource_updated("rudof://current-data/rdfxml".to_string())
        .await;
    service
        .notify_resource_updated("rudof://current-data/jsonld".to_string())
        .await;
    service
        .notify_resource_updated("rudof://current-data/trig".to_string())
        .await;
    service
        .notify_resource_updated("rudof://current-data/nquads".to_string())
        .await;
    service
        .notify_resource_updated("rudof://current-data/n3".to_string())
        .await;

    Ok(result)
}

pub async fn export_rdf_data_impl(
    service: &RudofMcpService,
    params: Parameters<ExportRdfDataRequest>,
) -> Result<CallToolResult, McpError> {
    let Parameters(ExportRdfDataRequest { format }) = params;
    let rudof = service.rudof.lock().await;
    let format_str = format.as_deref().unwrap_or("turtle");

    match RDFFormat::from_str(format_str) {
        Ok(parsed_format) => {
            let mut v = Vec::new();
            rudof
                .serialize_data(&parsed_format, &mut v)
                .map_err(|e| rdf_error("serializing data", e.to_string()))?;

            let size_bytes = v.len();
            let str = String::from_utf8(v)
                .map_err(|e| rdf_error("converting to UTF-8", e.to_string()))?;

            let response = ExportRdfDataResponse {
                data: str.clone(),
                format: format_str.to_string(),
                size_bytes,
            };

            let structured = serde_json::to_value(&response).unwrap();

            let formatted_data = format!("```{}\n{}\n```", format_str, str);
            let mut result = CallToolResult::success(vec![Content::text(formatted_data)]);
            result.structured_content = Some(structured);

            tracing::info!(
                format = %format_str,
                size_bytes = size_bytes,
                "RDF data exported successfully"
            );

            Ok(result)
        }
        Err(e) => Err(rdf_error(
            "parsing export format",
            format!("{}: {}", format_str, e),
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

    let size = str.len();
    let response = ExportPlantUmlResponse {
        plantuml_data: str.clone(),
        size,
    };

    let structured = serde_json::to_value(&response).unwrap();

    // Format as PlantUML code block
    let formatted_data = format!("```plantuml\n{}\n```", str);
    let mut result = CallToolResult::success(vec![Content::text(formatted_data)]);
    result.structured_content = Some(structured);

    tracing::info!(size = size, "PlantUML diagram exported successfully");

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

    let size_bytes = v.len();
    let base64_data = general_purpose::STANDARD.encode(&v);

    let response = ExportImageResponse {
        image_data_base64: base64_data.clone(),
        image_format: image_format.clone(),
    };

    let structured = serde_json::to_value(&response).unwrap();

    let mime_type = match image_format.to_lowercase().as_str() {
        "svg" => "image/svg+xml",
        "png" => "image/png",
        _ => "image/png",
    };

    let description = format!(
        "Image generated successfully ({} format, {} bytes encoded as base64)",
        image_format, size_bytes
    );

    let mut result = CallToolResult::success(vec![
        Content::text(description),
        Content::text(format!("\n\nBase64 data:\n{}", base64_data)),
    ]);

    result.structured_content = Some(structured);

    tracing::info!(
        format = %image_format,
        size_bytes = size_bytes,
        mime_type = mime_type,
        "Image exported successfully"
    );

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::model::RawContent;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    const SAMPLE_TURTLE: &str = r#"
        prefix : <http://example.org/>
        prefix xsd: <http://www.w3.org/2001/XMLSchema#>

        :a :name "Alice";
           :birthdate "1990-05-02"^^xsd:date;
           :enrolledIn :cs101.

        :b :name "Bob", "Robert".

        :cs101 :name "Computer Science".
    "#;

    async fn create_test_service() -> RudofMcpService {
        use crate::rudof_mcp_service::service::ServiceConfig;
        use std::collections::HashMap;
        use tokio::sync::RwLock;

        tokio::task::spawn_blocking(|| {
            let rudof_config = rudof_lib::RudofConfig::new().unwrap();
            let rudof = rudof_lib::Rudof::new(&rudof_config).unwrap();
            RudofMcpService {
                rudof: Arc::new(Mutex::new(rudof)),
                tool_router: Default::default(),
                prompt_router: Default::default(),
                config: Arc::new(RwLock::new(ServiceConfig::default())),
                resource_subscriptions: Arc::new(RwLock::new(HashMap::new())),
                current_min_log_level: Arc::new(RwLock::new(None)),
                current_context: Arc::new(RwLock::new(None)),
            }
        })
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn test_load_rdf_data_from_sources_impl_success() {
        let service = create_test_service().await;

        let params = Parameters(LoadRdfDataFromSourcesRequest {
            data: vec![SAMPLE_TURTLE.to_string()],
            data_format: None,
            base: None,
            endpoint: None,
        });

        let result = load_rdf_data_from_sources_impl(&service, params).await;
        assert!(result.is_ok());
        let call_result = result.unwrap();
        assert!(call_result.structured_content.is_some());
        assert!(call_result.content.iter().any(|c| {
            matches!(c.raw, RawContent::Text(ref s) if s.text.contains("Successfully loaded RDF data"))
        }));
    }

    #[tokio::test]
    async fn test_load_rdf_data_from_sources_impl_invalid_format() {
        let service = create_test_service().await;

        let params = Parameters(LoadRdfDataFromSourcesRequest {
            data: vec![SAMPLE_TURTLE.to_string()],
            data_format: Some("invalidformat".to_string()),
            base: None,
            endpoint: None,
        });

        let result = load_rdf_data_from_sources_impl(&service, params).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("RDF load error"));
    }

    #[tokio::test]
    async fn test_export_rdf_data_impl_success() {
        let service = create_test_service().await;

        // First load RDF data
        let _ = load_rdf_data_from_sources_impl(
            &service,
            Parameters(LoadRdfDataFromSourcesRequest {
                data: vec![SAMPLE_TURTLE.to_string()],
                data_format: None,
                base: None,
                endpoint: None,
            }),
        )
        .await
        .unwrap();

        let params = Parameters(ExportRdfDataRequest { format: None });

        let result = export_rdf_data_impl(&service, params).await;
        assert!(result.is_ok());
        let call_result = result.unwrap();
        assert!(call_result.structured_content.is_some());
        assert!(
            call_result
                .content
                .iter()
                .any(|c| { matches!(c.raw, RawContent::Text(ref t) if t.text.contains(":a")) })
        );
    }

    #[tokio::test]
    async fn test_export_plantuml_impl_success() {
        let service = create_test_service().await;

        // Load RDF data first
        let _ = load_rdf_data_from_sources_impl(
            &service,
            Parameters(LoadRdfDataFromSourcesRequest {
                data: vec![SAMPLE_TURTLE.to_string()],
                data_format: None,
                base: None,
                endpoint: None,
            }),
        )
        .await
        .unwrap();

        let params = Parameters(EmptyRequest {});

        let result = export_plantuml_impl(&service, params).await;
        assert!(result.is_ok());
        let call_result = result.unwrap();
        assert!(call_result.structured_content.is_some());
        assert!(
            call_result.content.iter().any(|c| {
                matches!(c.raw, RawContent::Text(ref t) if t.text.contains("@startuml"))
            })
        );
    }

    #[tokio::test]
    async fn test_export_image_impl_success() {
        // (Not implemented)
    }
}
