use base64::{engine::general_purpose, Engine as _};
use rmcp::{
    handler::server::wrapper::Parameters, model::{CallToolResult, Content}, ErrorData as McpError,
};
use rudof_lib::{
    data::{export_rdf_to_image, get_data_rudof, parse_image_format, parse_optional_base_iri},
    data_format::DataFormat, InputSpec, RDFFormat, ReaderMode,
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
    /// Number of sources processed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources_count: Option<usize>,
    /// RDF format used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
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
    /// Size of the serialized data in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<usize>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,
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

    let sources_count = data_specs.len();
    let response = LoadRdfDataFromSourcesResponse {
        message: format!(
            "Successfully loaded RDF data from {} source(s) in {} format",
            sources_count, data_format
        ),
        sources_count: Some(sources_count),
        format: Some(data_format.clone()),
    };
    
    let structured = serde_json::to_value(&response).unwrap();
    let mut result = CallToolResult::success(vec![
        Content::text(response.message.clone()),
    ]);
    result.structured_content = Some(structured);
    
    tracing::info!(
        sources = sources_count,
        format = %data_format,
        "RDF data loaded successfully"
    );
    
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
            
            let size_bytes = v.len();
            let str = String::from_utf8(v).map_err(|e| {
                internal_error(
                    error_messages::CONVERSION_ERROR,
                    Some(json!({ "error": e.to_string() })),
                )
            })?;

            let response = ExportRdfDataResponse {
                data: str.clone(),
                format: format.clone(),
                size_bytes: Some(size_bytes),
            };
            
            let structured = serde_json::to_value(&response).unwrap();
            
            // Return data in a code block for better formatting
            let formatted_data = format!("```{}\n{}\n```", format, str);
            let mut result = CallToolResult::success(vec![
                Content::text(formatted_data),
            ]);
            result.structured_content = Some(structured);
            
            tracing::info!(
                format = %format,
                size_bytes = size_bytes,
                "RDF data exported successfully"
            );
            
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

    let size = str.len();
    let response = ExportPlantUmlResponse {
        plantuml_data: str.clone(),
        size: Some(size),
    };
    
    let structured = serde_json::to_value(&response).unwrap();
    
    // Format as PlantUML code block
    let formatted_data = format!("```plantuml\n{}\n```", str);
    let mut result = CallToolResult::success(vec![
        Content::text(formatted_data),
    ]);
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

    // Return base64 data with explanation
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
                log_level_handle: None,
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
            data_format: "turtle".to_string(),
            base: None,
            endpoint: None,
        });

        let result = load_rdf_data_from_sources_impl(&service, params).await;
        assert!(result.is_ok());
        let call_result = result.unwrap();
        assert!(call_result.structured_content.is_some());
        assert!(call_result.content.iter().any(|c| {
            matches!(c.raw, RawContent::Text(ref s) if s.text.contains("RDF data loaded"))
        }));
    }

    #[tokio::test]
    async fn test_load_rdf_data_from_sources_impl_invalid_format() {
        let service = create_test_service().await;

        let params = Parameters(LoadRdfDataFromSourcesRequest {
            data: vec![SAMPLE_TURTLE.to_string()],
            data_format: "invalidformat".to_string(),
            base: None,
            endpoint: None,
        });

        let result = load_rdf_data_from_sources_impl(&service, params).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("Invalid data format"));
    }

    #[tokio::test]
    async fn test_export_rdf_data_impl_success() {
        let service = create_test_service().await;

        // First load RDF data
        let _ = load_rdf_data_from_sources_impl(
            &service,
            Parameters(LoadRdfDataFromSourcesRequest {
                data: vec![SAMPLE_TURTLE.to_string()],
                data_format: "turtle".to_string(),
                base: None,
                endpoint: None,
            }),
        )
        .await
        .unwrap();

        let params = Parameters(ExportRdfDataRequest {
            format: "turtle".to_string(),
        });

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
                data_format: "turtle".to_string(),
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
