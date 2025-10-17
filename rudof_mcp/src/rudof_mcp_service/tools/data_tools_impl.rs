use std::str::FromStr;
use serde_json::json;
use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    ErrorData as McpError,
};
use rudof_lib::{RDFFormat, ReaderMode};
use crate::rudof_mcp_service::{types::*, service::RudofMcpService, errors::*};

/// Internal implementation for load_rdf_data
pub async fn load_rdf_data_impl(service: &RudofMcpService, params: Parameters<LoadRdfDataRequest>) -> Result<CallToolResult, McpError> {
    let Parameters(LoadRdfDataRequest { rdf_data, format }) = params;
    let mut rudof = service.rudof.lock().await;
    
    match RDFFormat::from_str(&format) {
        Ok(parsed_format) => {
            rudof.read_data(rdf_data.as_bytes(), &parsed_format, None, &ReaderMode::default()).map_err(|e| {
                internal_error(codes::RDF_LOAD_ERROR, Some(json!({ "error": e.to_string() })))
            })?;
            let response = LoadRdfDataResponse { message: "RDF data loaded successfully".to_string() };
            let structured = serde_json::to_value(&response).map_err(|e| {
                internal_error(codes::RDF_LOAD_ERROR, Some(json!({ "error": e.to_string() })))
            })?;
            let mut result = CallToolResult::success(vec![Content::text(response.message.clone())]);
            result.structured_content = Some(structured);
            Ok(result)
        }
        Err(e) => Err(invalid_request(codes::INVALID_FORMAT, Some(json!({ "format": format, "error": e.to_string() })))),
    }
}

/// Internal implementation for export_rdf_data
pub async fn export_rdf_data_impl(service: &RudofMcpService, params: Parameters<ExportRdfDataRequest>) -> Result<CallToolResult, McpError> {
    let Parameters(ExportRdfDataRequest { format }) = params;
    let rudof = service.rudof.lock().await;
    
    match RDFFormat::from_str(&format) {
        Ok(parsed_format) => {
            let mut v = Vec::new();
            rudof.serialize_data(&parsed_format, &mut v).map_err(|e| {
                internal_error(codes::SERIALIZE_DATA_ERROR, Some(json!({ "error": e.to_string() })))
            })?;
            let str = String::from_utf8(v).map_err(|e| {
                internal_error(codes::UTF8_CONVERSION_ERROR, Some(json!({ "error": e.to_string() })))
            })?;
            let response = ExportRdfDataResponse { data: str.clone(), format: format.clone() };
            let structured = serde_json::to_value(&response).map_err(|e| {
                internal_error(codes::SERIALIZE_DATA_ERROR, Some(json!({ "error": e.to_string() })))
            })?;
            let mut result = CallToolResult::success(vec![Content::text(str)]);
            result.structured_content = Some(structured);
            Ok(result)
        }
        Err(e) => Err(invalid_request(codes::INVALID_FORMAT, Some(json!({ "format": format, "error": e.to_string() })))),
    }
}
