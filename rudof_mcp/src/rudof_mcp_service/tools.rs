use std::str::FromStr;
use serde_json::json;

use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Content},
    tool,
    tool_router,
    ErrorData as McpError,
};
use crate::rudof_mcp_service::errors::{self, codes};

use crate::rudof_mcp_service::types::*;
use rudof_lib::{RDFFormat, ReaderMode};

#[tool_router]
impl super::RudofMcpService {
    #[tool(name = "load_rdf_data", description = "Load RDF data from a string into the server's datastore")]
    pub async fn load_rdf_data(&self, Parameters(LoadRdfDataRequest { rdf_data, format }): Parameters<LoadRdfDataRequest>) -> Result<CallToolResult, McpError> {
        let mut rudof = self.rudof.lock().await;
        match RDFFormat::from_str(&format) {
            Ok(parsed_format) => {
                let _result = rudof
                    .read_data(rdf_data.as_bytes(), &parsed_format, None, &ReaderMode::default())
                    .map_err(|e| {
                        errors::internal_error(
                                codes::RDF_LOAD_ERROR,
                                Some(json!({ "error": e.to_string() })),
                            )
                    })?;
                let response = crate::rudof_mcp_service::types::LoadRdfDataResponse { message: "RDF data loaded successfully".to_string() };
                let structured = serde_json::to_value(&response).map_err(|e| {
                    errors::internal_error(
                        codes::RDF_LOAD_ERROR,
                        Some(json!({ "error": e.to_string() })),
                    )
                })?;
                let mut result = CallToolResult::success(vec![Content::text(response.message.clone())]);
                result.structured_content = Some(structured);
                Ok(result)
            }
            Err(e) => {
                tracing::error!("Failed to load RDF data: {}", e);
                return Err(errors::invalid_request(
                    codes::INVALID_FORMAT,
                    Some(json!({ "format": format, "error": e.to_string() })),
                ));
            }
        }
    }

    #[tool(name = "export_rdf_data", description = "Serialize and return the current RDF datastore in the requested format")]
    pub async fn export_rdf_data(&self, Parameters(ExportRdfDataRequest { format }): Parameters<ExportRdfDataRequest>) -> Result<CallToolResult, McpError> {
        let rudof = self.rudof.lock().await;
        match RDFFormat::from_str(&format) {
            Ok(parsed_format) => {
                let mut v = Vec::new();
                let _result = rudof.serialize_data(&parsed_format, &mut v).map_err(|e| {
                    errors::internal_error(
                        codes::SERIALIZE_DATA_ERROR,
                        Some(json!({ "error": e.to_string() })),
                    )
                })?;
                let str = String::from_utf8(v).map_err(|e| {
                    errors::internal_error(
                        codes::UTF8_CONVERSION_ERROR,
                        Some(json!({ "error": e.to_string() })),
                    )
                })?;
                let response = crate::rudof_mcp_service::types::ExportRdfDataResponse { data: str.clone(), format: format.clone() };
                let structured = serde_json::to_value(&response).map_err(|e| {
                    errors::internal_error(
                        codes::SERIALIZE_DATA_ERROR,
                        Some(json!({ "error": e.to_string() })),
                    )
                })?;
                let mut result = CallToolResult::success(vec![Content::text(str)]);
                result.structured_content = Some(structured);
                Ok(result)
            }
            Err(e) => {
                tracing::error!("Failed to serialize RDF data: {}", e);
                return Err(errors::invalid_request(
                    codes::INVALID_FORMAT,
                    Some(json!({ "format": format, "error": e.to_string() })),
                ));
            }
        }
    }
}

// Public wrapper to expose the generated router from the macro
pub fn tool_router_public() -> ToolRouter<super::RudofMcpService> {
    super::RudofMcpService::tool_router()
}

// Return the tools list annotated with helpful metadata (titles and annotations)
pub fn annotated_tools() -> Vec<rmcp::model::Tool> {
    let mut tools = tool_router_public().list_all();

    for tool in tools.iter_mut() {
        match tool.name.as_ref() {
            "load_rdf_data" => {
                tool.title = Some("Load RDF data".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                    .read_only(false)
                    .destructive(false)
                    .idempotent(false)
                );
            }
            "export_rdf_data" => {
                tool.title = Some("Export RDF data".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                    .read_only(true)
                    .destructive(false)
                    .idempotent(true)
                );
            }
            _ => {}
        }
    }

    tools
}
