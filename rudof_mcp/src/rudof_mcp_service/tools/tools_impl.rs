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
        description = "Execute a SPARQL query (SELECT, CONSTRUCT, ASK, DESCRIBE) against the RDF stored on the server"
    )]
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

    // Helper to create a test service instance
    fn create_test_service() -> RudofMcpService {
        RudofMcpService::new()
    }

    // Sample RDF data for testing (Turtle format)
    const SAMPLE_TURTLE: &str = r#"
        @prefix ex: <http://example.org/> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        
        ex:alice rdf:type ex:Person ;
                 ex:name "Alice" ;
                 ex:age 30 ;
                 ex:knows ex:bob .
        
        ex:bob rdf:type ex:Person ;
               ex:name "Bob" .
    "#;

    #[tokio::test]
    async fn test_load_rdf_data_success() {
        let service = create_test_service();

        let request = LoadRdfDataFromSourcesRequest {
            data: vec![SAMPLE_TURTLE.to_string()],
            data_format: "turtle".to_string(),
            base: None,
            endpoint: None,
        };

        let result = service.load_rdf_data_from_sources(Parameters(request)).await;

        assert!(result.is_ok());
        let call_result = result.unwrap();
        assert!(call_result.is_error.is_none() || !call_result.is_error.unwrap());
        assert!(!call_result.content.is_empty());
    }

    #[tokio::test]
    async fn test_load_rdf_data_invalid_format() {
        let service = create_test_service();

        let request = LoadRdfDataFromSourcesRequest {
            data: vec![SAMPLE_TURTLE.to_string()],
            data_format: "invalid_format".to_string(),
            base: None,
            endpoint: None,
        };

        let result = service.load_rdf_data_from_sources(Parameters(request)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_rdf_data_invalid_syntax() {
        let service = create_test_service();

        let request = LoadRdfDataFromSourcesRequest {
            data: vec!["no_valid_RDF".to_string()],
            data_format: "turtle".to_string(),
            base: None,
            endpoint: None,
        };
        let result = service.load_rdf_data_from_sources(Parameters(request)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_export_rdf_data_success() {
        let service = create_test_service();

        let request = LoadRdfDataFromSourcesRequest {
            data: vec![SAMPLE_TURTLE.to_string()],
            data_format: "turtle".to_string(),
            base: None,
            endpoint: None,
        };
        let _ = service.load_rdf_data_from_sources(Parameters(request)).await.unwrap();

        let export_req = ExportRdfDataRequest {
            format: "turtle".to_string(),
        };

        let result = service.export_rdf_data(Parameters(export_req)).await;

        assert!(result.is_ok());
        let call_result = result.unwrap();
        assert!(!call_result.content.is_empty());

        assert!(call_result.structured_content.is_some());
    }

    #[tokio::test]
    async fn test_export_rdf_data_invalid_format() {
        let service = create_test_service();

        let request = ExportRdfDataRequest {
            format: "invalid_format".to_string(),
        };

        let result = service.export_rdf_data(Parameters(request)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_rdf_data_from_sources_invalid_format() {
        let service = create_test_service();

        let request = LoadRdfDataFromSourcesRequest {
            data: vec!["file://some/path".to_string()],
            data_format: "invalid_format".to_string(),
            base: None,
            endpoint: None,
        };

        let result = service
            .load_rdf_data_from_sources(Parameters(request))
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_rdf_data_from_sources_empty_sources() {
        let service = create_test_service();

        let request = LoadRdfDataFromSourcesRequest {
            data: vec![],
            data_format: "turtle".to_string(),
            base: None,
            endpoint: None,
        };

        let result = service
            .load_rdf_data_from_sources(Parameters(request))
            .await;

        assert!(
            result.is_err(),
            "Expected an error for empty sources, got success: {:?}",
            result
        );
        let err = result.unwrap_err();
        assert_eq!(
            err.message, "rdf_load_error",
            "Expected 'rdf_load_error', got: {:?}",
            err.message
        );
    }

    #[tokio::test]
    async fn test_export_plantuml_success() {
        let service = create_test_service();

        let request = LoadRdfDataFromSourcesRequest {
            data: vec![SAMPLE_TURTLE.to_string()],
            data_format: "turtle".to_string(),
            base: None,
            endpoint: None,
        };
        let _ = service.load_rdf_data_from_sources(Parameters(request)).await.unwrap();

        let result = service.export_plantuml(Parameters(EmptyRequest {})).await;

        assert!(result.is_ok());
        let call_result = result.unwrap();
        assert!(call_result.structured_content.is_some());

        if let Some(text) = call_result.content[0].as_text() {
            assert!(text.text.contains("@startuml"));
            assert!(text.text.contains("Alice"));
        } else {
            panic!("Expected Text content");
        }
    }

    #[tokio::test]
    async fn test_export_image_invalid_format() {
        let service = create_test_service();

        let request = ExportImageRequest {
            image_format: "JPG".to_string(),
        };
        let result = service.export_image(Parameters(request)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_node_info_success() {
        let service = create_test_service();

        let request = LoadRdfDataFromSourcesRequest {
            data: vec![SAMPLE_TURTLE.to_string()],
            data_format: "turtle".to_string(),
            base: None,
            endpoint: None,
        };
        let _ = service.load_rdf_data_from_sources(Parameters(request)).await.unwrap();

        let node_req = NodeInfoRequest {
            node: "<http://example.org/alice>".to_string(),
            predicates: None,
            mode: Some("both".to_string()),
        };

        let result = service.node_info(Parameters(node_req)).await;

        assert!(result.is_ok());
        let call_result = result.unwrap();
        assert!(!call_result.content.is_empty());
        assert!(call_result.structured_content.is_some());
    }

    #[tokio::test]
    async fn test_node_info_not_found() {
        let service = create_test_service();

        let request = LoadRdfDataFromSourcesRequest {
            data: vec![SAMPLE_TURTLE.to_string()],
            data_format: "turtle".to_string(),
            base: None,
            endpoint: None,
        };
        let _ = service.load_rdf_data_from_sources(Parameters(request)).await.unwrap();

        let node_req = NodeInfoRequest {
            node: "http://example.org/nonexistent".to_string(),
            predicates: None,
            mode: Some("both".to_string()),
        };

        let result = service.node_info(Parameters(node_req)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_node_info_outgoing_only() {
        let service = create_test_service();

        let request = LoadRdfDataFromSourcesRequest {
            data: vec![SAMPLE_TURTLE.to_string()],
            data_format: "turtle".to_string(),
            base: None,
            endpoint: None,
        };
        let _ = service.load_rdf_data_from_sources(Parameters(request)).await.unwrap();

        let node_req = NodeInfoRequest {
            node: "<http://example.org/alice>".to_string(),
            predicates: None,
            mode: Some("outgoing".to_string()),
        };

        let result = service.node_info(Parameters(node_req)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_node_info_incoming_only() {
        let service = create_test_service();

        let request = LoadRdfDataFromSourcesRequest {
            data: vec![SAMPLE_TURTLE.to_string()],
            data_format: "turtle".to_string(),
            base: None,
            endpoint: None,
        };
        let _ = service.load_rdf_data_from_sources(Parameters(request)).await.unwrap();

        let node_req = NodeInfoRequest {
            node: "ex:bob".to_string(),
            predicates: None,
            mode: Some("incoming".to_string()),
        };

        let result = service.node_info(Parameters(node_req)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_node_info_invalid_mode() {
        let service = create_test_service();

        let request = LoadRdfDataFromSourcesRequest {
            data: vec![SAMPLE_TURTLE.to_string()],
            data_format: "turtle".to_string(),
            base: None,
            endpoint: None,
        };
        let _ = service.load_rdf_data_from_sources(Parameters(request)).await.unwrap();

        let node_req = NodeInfoRequest {
            node: "http://example.org/alice".to_string(),
            predicates: None,
            mode: Some("invalid_mode".to_string()),
        };

        let result = service.node_info(Parameters(node_req)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_node_info_with_predicates() {
        let service = create_test_service();

        let request = LoadRdfDataFromSourcesRequest {
            data: vec![SAMPLE_TURTLE.to_string()],
            data_format: "turtle".to_string(),
            base: None,
            endpoint: None,
        };
        let _ = service.load_rdf_data_from_sources(Parameters(request)).await.unwrap();

        let node_req = NodeInfoRequest {
            node: "<http://example.org/alice>".to_string(),
            predicates: Some(vec!["<http://example.org/name>".to_string()]),
            mode: Some("both".to_string()),
        };

        let result = service.node_info(Parameters(node_req)).await;

        assert!(result.is_ok());
    }

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

        let load_tool = tools.iter().find(|t| t.name == "load_rdf_data");
        assert!(load_tool.is_some());
        assert_eq!(load_tool.unwrap().title, Some("Load RDF data".to_string()));

        let export_tool = tools.iter().find(|t| t.name == "export_rdf_data");
        assert!(export_tool.is_some());
        assert_eq!(
            export_tool.unwrap().title,
            Some("Export RDF data".to_string())
        );

        let load_sources_tool = tools
            .iter()
            .find(|t| t.name == "load_rdf_data_from_sources");
        assert!(load_sources_tool.is_some());
        assert_eq!(
            load_sources_tool.unwrap().title,
            Some("Load RDF data from sources".to_string())
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

    #[tokio::test]
    async fn test_load_and_export_roundtrip() {
        let service = create_test_service();

        let request = LoadRdfDataFromSourcesRequest {
            data: vec![SAMPLE_TURTLE.to_string()],
            data_format: "turtle".to_string(),
            base: None,
            endpoint: None,
        };
        let _ = service.load_rdf_data_from_sources(Parameters(request)).await.unwrap();

        let export_req = ExportRdfDataRequest {
            format: "turtle".to_string(),
        };
        let result = service.export_rdf_data(Parameters(export_req)).await;

        assert!(result.is_ok());
        let call_result = result.unwrap();

        if let Some(text) = call_result.content[0].as_text() {
            assert!(!text.text.is_empty());
            assert!(text.text.contains("alice") || text.text.contains("Alice"));
        } else {
            panic!("Expected Text content");
        }
    }

    #[tokio::test]
    async fn test_multiple_format_export() {
        let service = create_test_service();

        let request = LoadRdfDataFromSourcesRequest {
            data: vec![SAMPLE_TURTLE.to_string()],
            data_format: "turtle".to_string(),
            base: None,
            endpoint: None,
        };
        let _ = service.load_rdf_data_from_sources(Parameters(request)).await.unwrap();

        let formats = vec!["turtle", "ntriples", "rdf/xml", "json"];

        for fmt in formats {
            let export_req = ExportRdfDataRequest {
                format: fmt.to_string(),
            };
            let result = service.export_rdf_data(Parameters(export_req)).await;
            assert!(result.is_ok(), "Failed to export in format: {}", fmt);
        }
    }
}
