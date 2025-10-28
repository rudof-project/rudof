use crate::rudof_mcp_service::service::RudofMcpService;
use rmcp::{
    ErrorData as McpError, handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters, model::CallToolResult, tool, tool_router,
};

// Import the public helper functions from the implementation files
use super::data_tools_impl::*;
use super::node_tools_impl::*;
use super::query_tools_impl::*;

#[tool_router]
impl RudofMcpService {
    #[tool(
        name = "load_rdf_data_from_sources",
        description = "Load RDF data from remote sources (URLs, files, raw text) or SPARQL endpoint into the server's datastore"
    )]
    #[cfg(not(tarpaulin_skip))]
    pub async fn load_rdf_data_from_sources(
        &self,
        params: Parameters<LoadRdfDataFromSourcesRequest>,
    ) -> Result<CallToolResult, McpError> {
        // Delegates the call to the function in data_tools_impl.rs
        load_rdf_data_from_sources_impl(self, params).await
    }

    #[tool(
        name = "export_rdf_data",
        description = "Serialize and return the RDF stored on the server in the requested format"
    )]
    #[cfg(not(tarpaulin_skip))]
    pub async fn export_rdf_data(
        &self,
        params: Parameters<ExportRdfDataRequest>,
    ) -> Result<CallToolResult, McpError> {
        // Delegates the call to the function in data_tools_impl.rs
        export_rdf_data_impl(self, params).await
    }

    #[tool(
        name = "export_plantuml",
        description = "Generate a PlantUML diagram of the RDF stored on the server"
    )]
    #[cfg(not(tarpaulin_skip))]
    pub async fn export_plantuml(
        &self,
        params: Parameters<EmptyRequest>,
    ) -> Result<CallToolResult, McpError> {
        // Delegates the call to the function in data_tools_impl.rs
        export_plantuml_impl(self, params).await
    }

    #[tool(
        name = "export_image",
        description = "Generate an image (SVG or PNG) visualization of the RDF stored on the server"
    )]
    #[cfg(not(tarpaulin_skip))]
    pub async fn export_image(
        &self,
        params: Parameters<ExportImageRequest>,
    ) -> Result<CallToolResult, McpError> {
        // Delegates the call to the function in data_tools_impl.rs
        export_image_impl(self, params).await
    }

    #[tool(
        name = "node_info",
        description = "Show information about a node (outgoing/incoming arcs) from the RDF stored on the server"
    )]
    #[cfg(not(tarpaulin_skip))]
    pub async fn node_info(
        &self,
        params: Parameters<NodeInfoRequest>,
    ) -> Result<CallToolResult, McpError> {
        // Delegates the call to the function in node_tols_impl.rs
        node_info_impl(self, params).await
    }

    #[tool(
        name = "execute_sparql_query",
        description = "Execute a SPARQL query (SELECT, CONSTRUCT, ASK, DESCRIBE) against the RDF stored on the server"
    )]
    #[cfg(not(tarpaulin_skip))]
    pub async fn execute_sparql_query(
        &self,
        params: Parameters<ExecuteSparqlQueryRequest>,
    ) -> Result<CallToolResult, McpError> {
        // Delegates the call to the function in query_tools_impl.rs
        execute_sparql_query_impl(self, params).await
    }
}

// Public wrapper to expose the generated router from the macro
pub fn tool_router_public() -> ToolRouter<RudofMcpService> {
    RudofMcpService::tool_router()
}

// Return the tools list annotated with helpful metadata (titles and annotations)
pub fn annotated_tools() -> Vec<rmcp::model::Tool> {
    let mut tools = tool_router_public().list_all();

    for tool in tools.iter_mut() {
        match tool.name.as_ref() {
            "load_rdf_data_from_sources" => {
                tool.title = Some("Load RDF data from sources".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(false)
                        .destructive(false)
                        .idempotent(false),
                );
            }
            "export_rdf_data" => {
                tool.title = Some("Export RDF data".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true),
                );
            }
            "export_plantuml" => {
                tool.title = Some("Export PlantUML diagram".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true),
                );
            }
            "export_image" => {
                tool.title = Some("Export RDF image visualization".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true),
                );
            }
            "node_info" => {
                tool.title = Some("Inspect RDF Node".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true),
                );
            }
            "execute_sparql_query" => {
                tool.title = Some("Execute SPARQL Query".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true),
                );
            }
            _ => {}
        }
    }

    tools
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_router_public() {
        let router = tool_router_public();
        let tools = router.list_all();

        assert!(!tools.is_empty());
        assert!(tools.iter().any(|t| t.name == "load_rdf_data"));
        assert!(tools.iter().any(|t| t.name == "export_rdf_data"));
        assert!(tools.iter().any(|t| t.name == "load_rdf_data_from_sources"));
        assert!(tools.iter().any(|t| t.name == "export_plantuml"));
        assert!(tools.iter().any(|t| t.name == "export_image"));
        assert!(tools.iter().any(|t| t.name == "node_info"));
    }

    #[test]
    fn test_annotated_tools() {
        let tools = annotated_tools();

        assert!(!tools.is_empty());

        let load_sources_tool = tools
            .iter()
            .find(|t| t.name == "load_rdf_data_from_sources");
        assert!(load_sources_tool.is_some());
        assert_eq!(
            load_sources_tool.unwrap().title,
            Some("Load RDF data from sources".to_string())
        );

        let export_tool = tools.iter().find(|t| t.name == "export_rdf_data");
        assert!(export_tool.is_some());
        assert_eq!(
            export_tool.unwrap().title,
            Some("Export RDF data".to_string())
        );

        let plantuml_tool = tools.iter().find(|t| t.name == "export_plantuml");
        assert!(plantuml_tool.is_some());
        assert_eq!(
            plantuml_tool.unwrap().title,
            Some("Export PlantUML diagram".to_string())
        );

        let image_tool = tools.iter().find(|t| t.name == "export_image");
        assert!(image_tool.is_some());
        assert_eq!(
            image_tool.unwrap().title,
            Some("Export RDF image visualization".to_string())
        );

        let node_tool = tools.iter().find(|t| t.name == "node_info");
        assert!(node_tool.is_some());
        assert_eq!(
            node_tool.unwrap().title,
            Some("Inspect RDF Node".to_string())
        );
    }
}
