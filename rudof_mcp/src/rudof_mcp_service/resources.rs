use crate::rudof_mcp_service::errors::{self, error_messages};
use crate::rudof_mcp_service::service::RudofMcpService;
use rmcp::{
    ErrorData as McpError, RoleServer,
    model::{
        Annotated, ListResourcesResult, PaginatedRequestParam, ReadResourceRequestParam,
        ReadResourceResult, RawResource, ResourceContents,
    },
    service::RequestContext,
};
use serde_json::json;

// Return the list of available resources
pub async fn list_resources(
    _request: Option<PaginatedRequestParam>,
    _ctx: RequestContext<RoleServer>,
) -> Result<ListResourcesResult, McpError> {
    let resources = vec![
        Annotated {
            raw: RawResource {
                uri: "rudof://current-data".to_string(),
                name: "Current RDF Data".to_string(),
                description: Some("Access the currently loaded RDF data in the Rudof service".to_string()),
                mime_type: Some("text/turtle".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://validation-result".to_string(),
                name: "Latest Validation Result".to_string(),
                description: Some("View the most recent ShEx validation result".to_string()),
                mime_type: Some("text/plain".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://schema".to_string(),
                name: "Current ShEx Schema".to_string(),
                description: Some("Access the currently loaded ShEx schema".to_string()),
                mime_type: Some("text/shex".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
    ];
    
    Ok(ListResourcesResult {
        resources,
        next_cursor: None,
    })
}

// Read a resource by URI and return the MCP read result
pub async fn read_resource(
    _service: &RudofMcpService,
    request: ReadResourceRequestParam,
) -> Result<ReadResourceResult, McpError> {
    let uri = request.uri;
    
    match uri.as_str() {
        "rudof://current-data" => {
            // Return a simple representation of the RDF data
            // In a real implementation, you'd serialize the actual RDF data
            let text = "RDF data is currently loaded.\nUse the export_rdf_data tool to view the data in various formats.".to_string();
            
            Ok(ReadResourceResult {
                contents: vec![ResourceContents::TextResourceContents {
                    uri: uri.clone(),
                    mime_type: Some("text/plain".to_string()),
                    text,
                    meta: None,
                }],
            })
        }
        "rudof://validation-result" => {
            // Return a placeholder for validation results
            // In a real implementation, you'd store and retrieve the last validation result
            Ok(ReadResourceResult {
                contents: vec![ResourceContents::TextResourceContents {
                    uri: uri.clone(),
                    mime_type: Some("text/plain".to_string()),
                    text: "No validation has been performed yet.\nUse the validate_shex tool to validate RDF data against a ShEx schema.".to_string(),
                    meta: None,
                }],
            })
        }
        "rudof://schema" => {
            // Return a placeholder for the current schema
            // In a real implementation, you'd store and retrieve the last loaded schema
            Ok(ReadResourceResult {
                contents: vec![ResourceContents::TextResourceContents {
                    uri: uri.clone(),
                    mime_type: Some("text/shex".to_string()),
                    text: "No ShEx schema loaded.\nProvide a schema when calling the validate_shex tool.".to_string(),
                    meta: None,
                }],
            })
        }
        _ => {
            Err(errors::resource_not_found(
                error_messages::RESOURCE_NOT_FOUND,
                Some(json!({ "uri": uri })),
            ))
        }
    }
}
