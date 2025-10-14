use std::str::FromStr;
use serde_json::json;

use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*, 
    tool, 
    tool_router, 
    ErrorData as McpError,
};

use crate::rudof_mcp_service::types::*;
use rudof_lib::{RDFFormat, ReaderMode};

#[tool_router]
impl super::RudofMcpService {
    #[tool(description = "Load RDF data from a string")]
    pub async fn load_rdf_data(
        &self,
        Parameters(LoadRdfDataRequest { rdf_data, format }): Parameters<LoadRdfDataRequest>,
    ) -> Result<CallToolResult, McpError> {
        let mut rudof = self.rudof.lock().await;
        match RDFFormat::from_str(&format) {
            Ok(format) => {
                let _result = rudof
                    .read_data(rdf_data.as_bytes(), &format, None, &ReaderMode::default())
                    .map_err(|e| {
                        McpError::internal_error(
                            "rdf_load_error",
                            Some(json!({ "error": e.to_string() })),
                        )
                    })?;
                Ok(CallToolResult::success(vec![Content::text("RDF data loaded successfully")] ))
            }
            Err(e) => {
                tracing::error!("Failed to load RDF data: {}", e);
                return Err(McpError::invalid_request(
                    format!("invalid_format {}", format),
                    Some(json!({ "format": format, "error": e.to_string() })),
                ));
            }
        }
    }

    #[tool(description = "Export RDF data to some format")]
    pub async fn export_rdf_data(
        &self,
        Parameters(ExportRdfDataRequest { format }): Parameters<ExportRdfDataRequest>,
    ) -> Result<CallToolResult, McpError> {
        let rudof = self.rudof.lock().await;
        match RDFFormat::from_str(&format) {
            Ok(format) => {
                let mut v = Vec::new();
                let _result = rudof.serialize_data(&format, &mut v).map_err(|e| {
                    McpError::internal_error(
                        "serialize_data_error",
                        Some(json!({ "error": e.to_string() })),
                    )
                })?;
                let str = String::from_utf8(v).map_err(|e| {
                    McpError::internal_error(
                        "utf8_conversion_error",
                        Some(json!({ "error": e.to_string() })),
                    )
                })?;
                Ok(CallToolResult::success(vec![Content::text(str)]))
            }
            Err(e) => {
                tracing::error!("Failed to serialize RDF data: {}", e);
                return Err(McpError::invalid_request(
                    format!("invalid_format {}", format),
                    Some(json!({ "format": format, "error": e.to_string() })),
                ));
            }
        }
    }
}

/// Public wrapper to expose the generated router from the macro
pub fn tool_router_public() -> ToolRouter<super::RudofMcpService> {
    super::RudofMcpService::tool_router()
}
