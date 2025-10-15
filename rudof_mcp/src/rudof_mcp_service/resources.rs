use rmcp::{
    model::*,
    service::RequestContext,
    ErrorData as McpError,
    RoleServer,
};
use serde_json::json;
use crate::rudof_mcp_service::errors::{self, codes};

// Resource URIs
pub const RDF_FORMATS_URI: &str = "rudof://rdf_formats";

/// Return the list of available resources
pub async fn list_resources(_request: Option<PaginatedRequestParam>, _ctx: RequestContext<RoleServer>) -> Result<ListResourcesResult, McpError> {
    let r1 = RawResource::new(RDF_FORMATS_URI, "RDF Formats").no_annotation();
    Ok(ListResourcesResult { resources: vec![r1], next_cursor: None })
}

/// Read a resource by URI and return the MCP read result
pub async fn read_resource(request: ReadResourceRequestParam, _ctx: RequestContext<RoleServer>) -> Result<ReadResourceResult, McpError> {
    let uri = request.uri;
    match uri.as_str() {
        RDF_FORMATS_URI => {
            let formats = json!({
                "formats": [
                    { "name": "Turtle", "media_type": "text/turtle", "extensions": ["ttl"] },
                    { "name": "RDF/XML", "media_type": "application/rdf+xml", "extensions": ["rdf", "xml"] },
                    { "name": "N-Triples", "media_type": "application/n-triples", "extensions": ["nt"] },
                    { "name": "JSON-LD", "media_type": "application/ld+json", "extensions": ["jsonld", "json"] }
                ],
                "default": "Turtle",
            });

            Ok(ReadResourceResult { contents: vec![ResourceContents::text(formats.to_string(), uri.clone())] })
        }

        _ => Err(errors::resource_not_found(codes::RESOURCE_NOT_FOUND, Some(json!({ "uri": uri })))),
    }
}
