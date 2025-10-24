use crate::rudof_mcp_service::errors::{self, error_messages};
use rmcp::{
    ErrorData as McpError, RoleServer,
    model::{
        ListResourcesResult, PaginatedRequestParam, RawResource, ReadResourceRequestParam,
        ReadResourceResult, Resource as AnnotatedResource, ResourceContents,
    },
    service::RequestContext,
};
use serde_json::json;
use std::str::FromStr;

use crate::rudof_mcp_service::service::RudofMcpService;
use rudof_lib::RDFFormat;

// Return the list of available resources
pub async fn list_resources(
    _request: Option<PaginatedRequestParam>,
    _ctx: RequestContext<RoleServer>,
) -> Result<ListResourcesResult, McpError> {
    let current_rdf_graph_resource = AnnotatedResource {
        raw: RawResource {
            uri: "rdf://graph".to_string(),
            name: "Current RDF Graph".to_string(),
            description: Some("The RDF dataset currently loaded into memory.".to_string()),
            mime_type: Some("text/turtle".to_string()),
            title: Some("Current RDF Graph".to_string()),
            size: None,
            icons: None,
        },
        annotations: None,
    };

    Ok(ListResourcesResult {
        resources: vec![current_rdf_graph_resource],
        next_cursor: None,
    })
}

// Read a resource by URI and return the MCP read result
pub async fn read_resource(
    service: &RudofMcpService,
    request: ReadResourceRequestParam,
) -> Result<ReadResourceResult, McpError> {
    let uri = request.uri;
    match uri.as_str() {
        "rdf://graph" => get_current_rdf_graph_resource(service, &uri).await,

        _ => Err(errors::resource_not_found(
            error_messages::RESOURCE_NOT_FOUND,
            Some(json!({ "uri": uri })),
        )),
    }
}

async fn get_current_rdf_graph_resource(
    service: &RudofMcpService,
    uri: &String,
) -> Result<ReadResourceResult, McpError> {
    let rudof = service.rudof.lock().await;

    let format = RDFFormat::from_str("turtle").map_err(|e| {
        errors::internal_error(
            error_messages::CONVERSION_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;
    let mut buf = Vec::new();
    rudof.serialize_data(&format, &mut buf).map_err(|e| {
        errors::internal_error(
            error_messages::SERIALIZE_DATA_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let graph_text = String::from_utf8(buf).map_err(|e| {
        errors::internal_error(
            error_messages::CONVERSION_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    Ok(ReadResourceResult {
        contents: vec![ResourceContents::text(graph_text, uri)],
    })
}
