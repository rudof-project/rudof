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

use crate::service::mcp_service::RudofMcpService;
use rmcp::{
    ErrorData as McpError, handler::server::router::tool::ToolRouter, handler::server::wrapper::Parameters,
    model::CallToolResult, tool, tool_router,
};
use schemars::JsonSchema;
use std::sync::{Arc, OnceLock};

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
        description = "Load RDF data from remote sources (URLs, files, raw text) or SPARQL endpoint into the server's datastore",
        annotations(
            title = "Load RDF Data from Sources",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = true,
        )
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
        description = "Serialize and return the RDF stored on the server in the requested format",
        annotations(
            title = "Export RDF Data",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false,
        )
    )]
    pub async fn export_rdf_data(&self, params: Parameters<ExportRdfDataRequest>) -> Result<CallToolResult, McpError> {
        export_rdf_data_impl(self, params).await
    }

    /// Generate a PlantUML diagram representing the RDF graph structure.
    #[tool(
        name = "export_plantuml",
        description = "Generate a PlantUML diagram of the RDF stored on the server",
        annotations(
            title = "Export PlantUML Diagram",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false,
        )
    )]
    pub async fn export_plantuml(&self, params: Parameters<EmptyRequest>) -> Result<CallToolResult, McpError> {
        export_plantuml_impl(self, params).await
    }

    /// Generate a visual image of the RDF graph.
    #[tool(
        name = "export_image",
        description = "Generate an image (SVG or PNG) visualization of the RDF stored on the server",
        annotations(
            title = "Export RDF Image Visualization",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false,
        )
    )]
    pub async fn export_image(&self, params: Parameters<ExportImageRequest>) -> Result<CallToolResult, McpError> {
        export_image_impl(self, params).await
    }

    // -------------------------------------------------------------------------
    // Node Inspection Tools
    // -------------------------------------------------------------------------

    /// Retrieve detailed information about an RDF node.
    #[tool(
        name = "node_info",
        description = "Show information about a node (outgoing/incoming arcs) from the RDF stored on the server",
        annotations(
            title = "Inspect RDF Node",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false,
        )
    )]
    pub async fn node_info(&self, params: Parameters<NodeInfoRequest>) -> Result<CallToolResult, McpError> {
        node_info_impl(self, params).await
    }

    // -------------------------------------------------------------------------
    // Query Tools
    // -------------------------------------------------------------------------

    /// Execute a SPARQL query against the loaded RDF data.
    #[tool(
        name = "execute_sparql_query",
        description = "Execute a SPARQL query (SELECT, CONSTRUCT, ASK, DESCRIBE) against the RDF stored on the server. You can provide either a direct SPARQL query or a natural language description that will be converted to SPARQL using an LLM.",
        annotations(
            title = "Execute SPARQL Query",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false,
        )
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
        description = "Validate the RDF data stored on the server against a ShEx schema",
        annotations(
            title = "Validate RDF with ShEx",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false,
        )
    )]
    pub async fn validate_shex(&self, params: Parameters<ValidateShexRequest>) -> Result<CallToolResult, McpError> {
        validate_shex_impl(self, params).await
    }

    /// Check if a ShEx schema is syntactically valid and well-formed.
    #[tool(
        name = "check_shex",
        description = "Check if a ShEx schema is well-formed",
        annotations(
            title = "Check ShEx Schema Well-Formedness",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false,
        )
    )]
    pub async fn check_shex(&self, params: Parameters<CheckShexRequest>) -> Result<CallToolResult, McpError> {
        check_shex_impl(self, params).await
    }

    /// Parse and display a ShEx schema with optional analysis features.
    #[tool(
        name = "show_shex",
        description = "Parse a ShEx schema and display it with optional compilation, statistics, and dependency analysis",
        annotations(
            title = "Parse and Display ShEx Schema",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false,
        )
    )]
    pub async fn show_shex(&self, params: Parameters<ShowShexRequest>) -> Result<CallToolResult, McpError> {
        show_shex_impl(self, params).await
    }

    /// Validate RDF data against a SHACL schema.
    #[tool(
        name = "validate_shacl",
        description = "Validate the RDF data stored on the server against a SHACL schema",
        annotations(
            title = "Validate RDF with SHACL",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false,
        )
    )]
    pub async fn validate_shacl(&self, params: Parameters<ValidateShaclRequest>) -> Result<CallToolResult, McpError> {
        validate_shacl_impl(self, params).await
    }
}

/// Public wrapper to expose the generated router from the macro
pub fn tool_router_public() -> ToolRouter<RudofMcpService> {
    RudofMcpService::tool_router()
}

/// Return the tools list enriched with output schema and task execution metadata.
///
/// Behavioral annotations (title/read_only/destructive/idempotent/open_world)
/// are declared inline in each `#[tool]` attribute.
///
/// # Returns
///
/// A vector of `Tool` definitions with annotations for all registered tools.
fn output_schema_for<T: JsonSchema + 'static>(tool_name: &str) -> Arc<rmcp::model::JsonObject> {
    rmcp::handler::server::tool::schema_for_output::<T>().unwrap_or_else(|e| {
        tracing::error!(
            tool_name = %tool_name,
            error = %e,
            "Invalid tool output schema; falling back to empty schema"
        );
        Arc::new(rmcp::model::JsonObject::default())
    })
}

fn apply_tool_metadata(
    tool: &mut rmcp::model::Tool,
    task_support: rmcp::model::TaskSupport,
    output_schema: Arc<rmcp::model::JsonObject>,
) {
    tool.execution = Some(rmcp::model::ToolExecution::from_raw(Some(task_support)));
    tool.output_schema = Some(output_schema);
}

fn build_annotated_tools() -> Vec<rmcp::model::Tool> {
    let mut tools = tool_router_public().list_all();

    for tool in tools.iter_mut() {
        let (output_schema, task_support) = match tool.name.as_ref() {
            "load_rdf_data_from_sources" => (
                output_schema_for::<LoadRdfDataFromSourcesResponse>("load_rdf_data_from_sources"),
                rmcp::model::TaskSupport::Forbidden,
            ),
            "export_rdf_data" => (
                output_schema_for::<ExportRdfDataResponse>("export_rdf_data"),
                rmcp::model::TaskSupport::Forbidden,
            ),
            "export_plantuml" => (
                output_schema_for::<ExportPlantUmlResponse>("export_plantuml"),
                rmcp::model::TaskSupport::Forbidden,
            ),
            "export_image" => (
                output_schema_for::<ExportImageResponse>("export_image"),
                rmcp::model::TaskSupport::Forbidden,
            ),
            "node_info" => (
                output_schema_for::<NodeInfoResponse>("node_info"),
                rmcp::model::TaskSupport::Forbidden,
            ),
            "execute_sparql_query" => (
                output_schema_for::<QueryExecutionResponse>("execute_sparql_query"),
                rmcp::model::TaskSupport::Forbidden,
            ),
            "show_shex" => (
                output_schema_for::<ShowShexResponse>("show_shex"),
                rmcp::model::TaskSupport::Forbidden,
            ),
            "check_shex" => (
                output_schema_for::<CheckShexResponse>("check_shex"),
                rmcp::model::TaskSupport::Forbidden,
            ),
            "validate_shex" => (
                output_schema_for::<ValidateShexResponse>("validate_shex"),
                rmcp::model::TaskSupport::Forbidden,
            ),
            "validate_shacl" => (
                output_schema_for::<ValidateShaclResponse>("validate_shacl"),
                rmcp::model::TaskSupport::Forbidden,
            ),
            _ => {
                tracing::warn!(tool_name = %tool.name, "Tool missing output schema");
                continue;
            },
        };

        apply_tool_metadata(tool, task_support, output_schema);
    }

    tools
}

/// Return the cached annotated tools list.
///
/// Output schemas and task support metadata are static — computed once on first
/// call via [`OnceLock`] and reused for every subsequent `tools/list` request.
pub fn annotated_tools() -> &'static [rmcp::model::Tool] {
    static TOOLS: OnceLock<Vec<rmcp::model::Tool>> = OnceLock::new();
    TOOLS.get_or_init(build_annotated_tools)
}
