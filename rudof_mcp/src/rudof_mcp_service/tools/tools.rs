use rmcp::{
    handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::CallToolResult,
    tool, tool_router, ErrorData as McpError,
};
use crate::rudof_mcp_service::{
    service::RudofMcpService,
    types::*,
};

// Import the public helper functions from the implementation files
use super::data_tools_impl::*;
use super::node_tools_impl::*;

#[tool_router] 
impl RudofMcpService {
    #[tool(name = "load_rdf_data", description = "Load RDF data from a string into the server's datastore")]
    pub async fn load_rdf_data(&self, params: Parameters<LoadRdfDataRequest>) -> Result<CallToolResult, McpError> {
        // Delegates the call to the function in data_tools_impl.rs
        load_rdf_data_impl(self, params).await
    }

    #[tool(name = "export_rdf_data", description = "Serialize and return the current RDF datastore in the requested format")]
    pub async fn export_rdf_data(&self, params: Parameters<ExportRdfDataRequest>) -> Result<CallToolResult, McpError> {
        // Delegates the call to the function in data_tools_impl.rs
        export_rdf_data_impl(self, params).await
    }

    #[tool(name = "node_info", description = "Show information about a node (outgoing/incoming arcs) from the server RDF datastore")]
    pub async fn node_info(&self, params: Parameters<NodeInfoRequest>) -> Result<CallToolResult, McpError> {
        // Delegates the call to the function in node_tols_impl.rs
        node_info_impl(self, params).await
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
            "node_info" => {
                tool.title = Some("Inspect RDF Node".to_string());
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
        
        let request = LoadRdfDataRequest {
            rdf_data: SAMPLE_TURTLE.to_string(),
            format: "turtle".to_string(),
        };

        let result = service.load_rdf_data(Parameters(request)).await;
        
        assert!(result.is_ok());
        let call_result = result.unwrap();
        assert!(call_result.is_error.is_none() || !call_result.is_error.unwrap());
        assert!(!call_result.content.is_empty());
    }

    #[tokio::test]
    async fn test_load_rdf_data_invalid_format() {
        let service = create_test_service();
        
        let request = LoadRdfDataRequest {
            rdf_data: SAMPLE_TURTLE.to_string(),
            format: "invalid_format".to_string(),
        };

        let result = service.load_rdf_data(Parameters(request)).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_rdf_data_invalid_syntax() {
        let service = create_test_service();
        
        let request = LoadRdfDataRequest {
            rdf_data: "this is not valid RDF".to_string(),
            format: "turtle".to_string(),
        };

        let result = service.load_rdf_data(Parameters(request)).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_export_rdf_data_success() {
        let service = create_test_service();
        
        // First load some data
        let load_req = LoadRdfDataRequest {
            rdf_data: SAMPLE_TURTLE.to_string(),
            format: "turtle".to_string(),
        };
        let _ = service.load_rdf_data(Parameters(load_req)).await.unwrap();

        // Then export it
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
    async fn test_node_info_success() {
        let service = create_test_service();
        
        // Load test data
        let load_req = LoadRdfDataRequest {
            rdf_data: SAMPLE_TURTLE.to_string(),
            format: "turtle".to_string(),
        };
        let _ = service.load_rdf_data(Parameters(load_req)).await.unwrap();

        // Query node info
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
        
        // Load test data
        let load_req = LoadRdfDataRequest {
            rdf_data: SAMPLE_TURTLE.to_string(),
            format: "turtle".to_string(),
        };
        let _ = service.load_rdf_data(Parameters(load_req)).await.unwrap();

        // Query non-existent node
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
        
        let load_req = LoadRdfDataRequest {
            rdf_data: SAMPLE_TURTLE.to_string(),
            format: "turtle".to_string(),
        };
        let _ = service.load_rdf_data(Parameters(load_req)).await.unwrap();

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
        
        let load_req = LoadRdfDataRequest {
            rdf_data: SAMPLE_TURTLE.to_string(),
            format: "turtle".to_string(),
        };
        let _ = service.load_rdf_data(Parameters(load_req)).await.unwrap();

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
        
        let load_req = LoadRdfDataRequest {
            rdf_data: SAMPLE_TURTLE.to_string(),
            format: "turtle".to_string(),
        };
        let _ = service.load_rdf_data(Parameters(load_req)).await.unwrap();

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
        
        let load_req = LoadRdfDataRequest {
            rdf_data: SAMPLE_TURTLE.to_string(),
            format: "turtle".to_string(),
        };
        let _ = service.load_rdf_data(Parameters(load_req)).await.unwrap();

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
        assert!(tools.iter().any(|t| t.name == "node_info"));
    }

    #[test]
    fn test_annotated_tools() {
        let tools = annotated_tools();
        
        assert!(!tools.is_empty());
        
        // Check load_rdf_data annotations
        let load_tool = tools.iter().find(|t| t.name == "load_rdf_data");
        assert!(load_tool.is_some());
        let load_tool = load_tool.unwrap();
        assert_eq!(load_tool.title, Some("Load RDF data".to_string()));
        assert!(load_tool.annotations.is_some());
        
        // Check export_rdf_data annotations
        let export_tool = tools.iter().find(|t| t.name == "export_rdf_data");
        assert!(export_tool.is_some());
        let export_tool = export_tool.unwrap();
        assert_eq!(export_tool.title, Some("Export RDF data".to_string()));
        
        // Check node_info annotations
        let node_tool = tools.iter().find(|t| t.name == "node_info");
        assert!(node_tool.is_some());
        let node_tool = node_tool.unwrap();
        assert_eq!(node_tool.title, Some("Inspect RDF Node".to_string()));
    }

    #[tokio::test]
    async fn test_load_and_export_roundtrip() {
        let service = create_test_service();
        
        // Load data
        let load_req = LoadRdfDataRequest {
            rdf_data: SAMPLE_TURTLE.to_string(),
            format: "turtle".to_string(),
        };
        let _ = service.load_rdf_data(Parameters(load_req)).await.unwrap();

        // Export in same format
        let export_req = ExportRdfDataRequest {
            format: "turtle".to_string(),
        };
        let result = service.export_rdf_data(Parameters(export_req)).await;
        
        assert!(result.is_ok());
        let call_result = result.unwrap();
        
        // Verify content is not empty
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
        
        // Load data in turtle
        let load_req = LoadRdfDataRequest {
            rdf_data: SAMPLE_TURTLE.to_string(),
            format: "turtle".to_string(),
        };
        let _ = service.load_rdf_data(Parameters(load_req)).await.unwrap();

        // Try exporting in different formats
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