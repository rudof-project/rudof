//! MCP Tool Router implementation for Rudof.
//!
//! This module defines the MCP tools exposed by the Rudof MCP server using the
//! `#[tool_router]` and `#[tool]` procedural macros from the `rmcp` crate.
//!
//! # Error Handling
//!
//! Tools follow MCP best practices with two error types:
//! - **Tool Execution Errors** (`isError: true`): For input validation, format errors,
//!   and other issues that LLMs can self-correct
//! - **Protocol Errors**: For internal server errors and unrecoverable issues

use crate::service::service::RudofMcpService;
use rmcp::{
    ErrorData as McpError, handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters, model::CallToolResult, tool, tool_router,
};

// Import the public helper functions from the implementation files
use crate::service::tools::data_tools_impl::*;
use crate::service::tools::node_tools_impl::*;
use crate::service::tools::query_tools_impl::*;
use crate::service::tools::shacl_validate_tools_impl::*;
use crate::service::tools::shex_tools_impl::*;
use crate::service::tools::shex_validate_tools_impl::*;

#[tool_router]
impl RudofMcpService {
    // -------------------------------------------------------------------------
    // Data Management Tools
    // -------------------------------------------------------------------------

    /// Load RDF data into the server's in-memory datastore.
    #[tool(
        name = "load_rdf_data_from_sources",
        description = "Load RDF data from remote sources (URLs, files, raw text) or SPARQL endpoint into the server's datastore"
    )]
    pub async fn load_rdf_data_from_sources(
        &self,
        params: Parameters<LoadRdfDataFromSourcesRequest>,
    ) -> Result<CallToolResult, McpError> {
        load_rdf_data_from_sources_impl(self, params).await
    }

    /// Serialize the current RDF data to a specified format.
    #[tool(
        name = "export_rdf_data",
        description = "Serialize and return the RDF stored on the server in the requested format"
    )]
    pub async fn export_rdf_data(
        &self,
        params: Parameters<ExportRdfDataRequest>,
    ) -> Result<CallToolResult, McpError> {
        export_rdf_data_impl(self, params).await
    }

    /// Generate a PlantUML diagram representing the RDF graph structure.
    #[tool(
        name = "export_plantuml",
        description = "Generate a PlantUML diagram of the RDF stored on the server"
    )]
    pub async fn export_plantuml(
        &self,
        params: Parameters<EmptyRequest>,
    ) -> Result<CallToolResult, McpError> {
        export_plantuml_impl(self, params).await
    }

    /// Generate a visual image of the RDF graph.
    #[tool(
        name = "export_image",
        description = "Generate an image (SVG or PNG) visualization of the RDF stored on the server"
    )]
    pub async fn export_image(
        &self,
        params: Parameters<ExportImageRequest>,
    ) -> Result<CallToolResult, McpError> {
        export_image_impl(self, params).await
    }

    // -------------------------------------------------------------------------
    // Node Inspection Tools
    // -------------------------------------------------------------------------

    /// Retrieve detailed information about an RDF node.
    #[tool(
        name = "node_info",
        description = "Show information about a node (outgoing/incoming arcs) from the RDF stored on the server"
    )]
    pub async fn node_info(
        &self,
        params: Parameters<NodeInfoRequest>,
    ) -> Result<CallToolResult, McpError> {
        node_info_impl(self, params).await
    }

    // -------------------------------------------------------------------------
    // Query Tools
    // -------------------------------------------------------------------------

    /// Execute a SPARQL query against the loaded RDF data.
    #[tool(
        name = "execute_sparql_query",
        description = "Execute a SPARQL query (SELECT, CONSTRUCT, ASK, DESCRIBE) against the RDF stored on the server. You can provide either a direct SPARQL query or a natural language description that will be converted to SPARQL using an LLM."
    )]
    pub async fn execute_sparql_query(
        &self,
        params: Parameters<ExecuteSparqlQueryRequest>,
    ) -> Result<CallToolResult, McpError> {
        execute_sparql_query_impl(self, params).await
    }

    // -------------------------------------------------------------------------
    // ShEx Tools
    // -------------------------------------------------------------------------

    /// Validate RDF data against a ShEx schema.
    #[tool(
        name = "validate_shex",
        description = "Validate the RDF data stored on the server against a ShEx schema"
    )]
    pub async fn validate_shex(
        &self,
        params: Parameters<ValidateShexRequest>,
    ) -> Result<CallToolResult, McpError> {
        validate_shex_impl(self, params).await
    }

    /// Check if a ShEx schema is syntactically valid and well-formed.
    #[tool(
        name = "check_shex",
        description = "Check if a ShEx schema is well-formed"
    )]
    pub async fn check_shex(
        &self,
        params: Parameters<CheckShexRequest>,
    ) -> Result<CallToolResult, McpError> {
        check_shex_impl(self, params).await
    }

    /// Get detailed information about a specific shape in a ShEx schema.
    #[tool(
        name = "shape_info",
        description = "Obtain information about a specific ShEx shape"
    )]
    pub async fn shape_info(
        &self,
        params: Parameters<ShapeInfoRequest>,
    ) -> Result<CallToolResult, McpError> {
        shape_info_impl(self, params).await
    }

    /// Convert a ShEx schema between different serialization formats.
    #[tool(
        name = "convert_shex",
        description = "Convert a ShEx schema between supported formats (shexc, shexj, turtle)"
    )]
    pub async fn convert_shex(
        &self,
        params: Parameters<ConvertShexRequest>,
    ) -> Result<CallToolResult, McpError> {
        convert_shex_impl(self, params).await
    }

    /// Parse and display a ShEx schema with optional analysis features.
    #[tool(
        name = "show_shex",
        description = "Parse a ShEx schema and display it with optional compilation, statistics, and dependency analysis"
    )]
    pub async fn show_shex(
        &self,
        params: Parameters<ShowShexRequest>,
    ) -> Result<CallToolResult, McpError> {
        show_shex_impl(self, params).await
    }

    // -------------------------------------------------------------------------
    // SHACL Tools
    // -------------------------------------------------------------------------

    /// Validate RDF data against a SHACL schema.
    #[tool(
        name = "validate_shacl",
        description = "Validate the RDF data stored on the server against a SHACL schema"
    )]
    pub async fn validate_shacl(
        &self,
        params: Parameters<ValidateShaclRequest>,
    ) -> Result<CallToolResult, McpError> {
        validate_shacl_impl(self, params).await
    }
}

/// Public wrapper to expose the generated router from the macro
pub fn tool_router_public() -> ToolRouter<RudofMcpService> {
    RudofMcpService::tool_router()
}

/// Return the tools list annotated with helpful metadata
/// Return the tools list annotated with helpful metadata.
///
/// This function adds MCP tool annotations to each tool, providing clients
/// with information about tool behavior:
///
/// - `read_only`: Whether the tool only reads data without modifying state
/// - `destructive`: Whether the tool may cause irreversible changes
/// - `idempotent`: Whether calling the tool multiple times has the same effect
/// - `open_world`: Whether the tool may interact with external resources
///
/// # Returns
///
/// A vector of `Tool` definitions with annotations for all registered tools.
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
                        .open_world(true),
                );
            }
            "export_rdf_data" => {
                tool.title = Some("Export RDF Data".to_string());
                tool.annotations = Some(
                    rmcp::model::ToolAnnotations::new()
                        .read_only(true)
                        .destructive(false)
                        .idempotent(true)
                        .open_world(false),
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
            "show_shex" => {
                tool.title = Some("Parse and Display ShEx Schema".to_string());
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
            _ => {
                // Log warning for unhandled tools to catch missing annotations
                tracing::warn!(tool_name = %tool.name, "Tool missing annotations");
            }
        }
    }

    tools
}