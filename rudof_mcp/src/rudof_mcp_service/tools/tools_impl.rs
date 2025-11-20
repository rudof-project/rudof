use crate::rudof_mcp_service::service::RudofMcpService;
use rmcp::{
    ErrorData as McpError, handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters, model::CallToolResult, tool, tool_router,
};

// Import the public helper functions from the implementation files
use super::data_tools_impl::*;
use super::node_tools_impl::*;
use super::query_tools_impl::*;
use super::shacl_validate_tools_impl::*;
use super::shex_validate_tools_impl::*;
use super::shex_tools_impl::*;

#[tool_router]
impl RudofMcpService {
    #[tool(
        name = "load_rdf_data_from_sources",
        description = "Load RDF data from remote sources (URLs, files, raw text) or SPARQL endpoint into the server's datastore"
    )]
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
    pub async fn node_info(
        &self,
        params: Parameters<NodeInfoRequest>,
    ) -> Result<CallToolResult, McpError> {
        // Delegates the call to the function in node_tols_impl.rs
        node_info_impl(self, params).await
    }

    #[tool(
        name = "execute_sparql_query",
        description = "Execute a SPARQL query (SELECT, CONSTRUCT, ASK, DESCRIBE) against the RDF stored on the server. You can provide either a direct SPARQL query or a natural language description that will be converted to SPARQL using an LLM."
    )]
    pub async fn execute_sparql_query(
        &self,
        params: Parameters<ExecuteSparqlQueryRequest>,
    ) -> Result<CallToolResult, McpError> {
        // Delegates the call to the function in query_tools_impl.rs
        execute_sparql_query_impl(self, params).await
    }

    #[tool(
        name = "validate_shex",
        description = "Validate RDF data against a ShEx schema using the provided inputs"
    )]
    pub async fn validate_shex(
        &self,
        params: Parameters<ValidateShexRequest>,
    ) -> Result<CallToolResult, McpError> {
        // Delegates the call to the function in shex_validate_tools_impl.rs
        validate_shex_impl(self, params).await
    }

    #[tool(
        name = "check_shex",
        description = "Check if a ShEx schema is well-formed"
    )]
    pub async fn check_shex(&self, params: Parameters<CheckShexRequest>) -> Result<CallToolResult, McpError> {
        check_shex_impl(self, params).await
    }

    #[tool(
        name = "shape_info",
        description = "Obtain information about a specific ShEx shape"
    )]
    pub async fn shape_info(&self, params: Parameters<ShapeInfoRequest>) -> Result<CallToolResult, McpError> {
        shape_info_impl(self, params).await
    }

    #[tool(
        name = "convert_shex",
        description = "Convert a ShEx schema between supported formats"
    )]
    pub async fn convert_shex(&self, params: Parameters<ConvertShexRequest>) -> Result<CallToolResult, McpError> {
        convert_shex_impl(self, params).await
    }

    #[tool(
        name = "validate_shacl",
        description = "Validate RDF data against a SHACL schema using the provided inputs"
    )]
    pub async fn validate_shacl(
        &self,
        params: Parameters<ValidateShaclRequest>,
    ) -> Result<CallToolResult, McpError> {
        // Delegates the call to the function in shacl_validate_tools_impl.rs
        validate_shacl_impl(self, params).await
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
                tool.title = Some("Load RDF Data from Sources".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(false)
                        .destructive(false)
                        .idempotent(false)
                        .open_world(true), // Can access external URLs/endpoints
                );
            }
            "export_rdf_data" => {
                tool.title = Some("Export RDF Data".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true)
                        .open_world(false), // Operates on internal data only
                );
            }
            "export_plantuml" => {
                tool.title = Some("Export PlantUML Diagram".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true)
                        .open_world(false),
                );
            }
            "export_image" => {
                tool.title = Some("Export RDF Image Visualization".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true)
                        .open_world(false),
                );
            }
            "node_info" => {
                tool.title = Some("Inspect RDF Node".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true)
                        .open_world(false),
                );
            }
            "execute_sparql_query" => {
                tool.title = Some("Execute SPARQL Query".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true)
                        .open_world(false),
                );
            }
            "validate_shex" => {
                tool.title = Some("Validate RDF with ShEx".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true)
                        .open_world(false),
                );
            }
            "show_shex" => {
                tool.title = Some("Show ShEx Schema".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true)
                        .open_world(false),
                );
            }
            "validate_shacl" => {
                tool.title = Some("Validate RDF with SHACL".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true)
                        .open_world(false),
                );
            }
            "check_shex" => {
                tool.title = Some("Check ShEx Schema Well-Formedness".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true)
                        .open_world(false),
                );
            }
            "shape_info" => {
                tool.title = Some("Show ShEx Shape Info".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true)
                        .open_world(false),
                );
            }
            "convert_shex" => {
                tool.title = Some("Convert ShEx Schema Formats".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true)
                        .open_world(false),
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
        assert!(tools.iter().any(|t| t.name == "export_rdf_data"));
        assert!(tools.iter().any(|t| t.name == "load_rdf_data_from_sources"));
        assert!(tools.iter().any(|t| t.name == "export_plantuml"));
        assert!(tools.iter().any(|t| t.name == "export_image"));
        assert!(tools.iter().any(|t| t.name == "node_info"));
        assert!(tools.iter().any(|t| t.name == "execute_sparql_query"));
        assert!(tools.iter().any(|t| t.name == "validate_shex"));
        assert!(tools.iter().any(|t| t.name == "validate_shacl"));
    }

    #[test]
    fn test_annotated_tools() {
        let tools = annotated_tools();

        assert!(!tools.is_empty());

        // Test load_rdf_data_from_sources
        let load_sources_tool = tools
            .iter()
            .find(|t| t.name == "load_rdf_data_from_sources");
        assert!(load_sources_tool.is_some());
        let tool = load_sources_tool.unwrap();
        assert_eq!(tool.title, Some("Load RDF Data from Sources".to_string()));
        assert!(tool.annotations.is_some());
        let annot = tool.annotations.as_ref().unwrap();
        assert_eq!(annot.read_only_hint, Some(false));
        assert_eq!(annot.destructive_hint, Some(false));
        assert_eq!(annot.idempotent_hint, Some(false));
        assert_eq!(annot.open_world_hint, Some(true));

        // Test export_rdf_data
        let export_tool = tools.iter().find(|t| t.name == "export_rdf_data");
        assert!(export_tool.is_some());
        let tool = export_tool.unwrap();
        assert_eq!(tool.title, Some("Export RDF Data".to_string()));
        assert!(tool.annotations.is_some());
        let annot = tool.annotations.as_ref().unwrap();
        assert_eq!(annot.read_only_hint, Some(true));
        assert_eq!(annot.idempotent_hint, Some(true));
        assert_eq!(annot.open_world_hint, Some(false));

        // Test validate_shex
        let validate_shex_tool = tools.iter().find(|t| t.name == "validate_shex");
        assert!(validate_shex_tool.is_some());
        let tool = validate_shex_tool.unwrap();
        assert_eq!(tool.title, Some("Validate RDF with ShEx".to_string()));
        assert!(tool.annotations.is_some());
        let annot = tool.annotations.as_ref().unwrap();
        assert_eq!(annot.read_only_hint, Some(true));
        assert_eq!(annot.idempotent_hint, Some(true));

        // Test validate_shacl
        let validate_shacl_tool = tools.iter().find(|t| t.name == "validate_shacl");
        assert!(validate_shacl_tool.is_some());
        let tool = validate_shacl_tool.unwrap();
        assert_eq!(tool.title, Some("Validate RDF with SHACL".to_string()));
        assert!(tool.annotations.is_some());
        let annot = tool.annotations.as_ref().unwrap();
        assert_eq!(annot.read_only_hint, Some(true));
        assert_eq!(annot.idempotent_hint, Some(true));
    }
}
