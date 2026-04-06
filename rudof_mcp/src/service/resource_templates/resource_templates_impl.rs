use rmcp::{ErrorData as McpError, RoleServer, model::*, service::RequestContext};

/// Return the list of available resource templates.
///
/// Templates document URI patterns that clients can use to construct resource URIs
/// without knowing all valid values in advance.
pub async fn list_resource_templates(
    _request: Option<PaginatedRequestParams>,
    _ctx: RequestContext<RoleServer>,
) -> Result<ListResourceTemplatesResult, McpError> {
    let templates: Vec<ResourceTemplate> = vec![
        Annotated::new(
            RawResourceTemplate {
                uri_template: "rudof://current-data/{format}".to_string(),
                name: "RDF Data in Specific Format".to_string(),
                title: None,
                description: Some(
                    "Access the currently loaded RDF data serialized in a specific format. \
                     {format} must be one of: turtle, ntriples, rdfxml, jsonld, trig, nquads, n3."
                        .to_string(),
                ),
                mime_type: None,
                icons: None,
            },
            None,
        ),
    ];

    Ok(ListResourceTemplatesResult {
        resource_templates: templates,
        next_cursor: None,
        ..Default::default()
    })
}
