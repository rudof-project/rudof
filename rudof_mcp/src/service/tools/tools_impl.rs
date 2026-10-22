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
    ErrorData as McpError,
    handler::server::router::tool::ToolRouter,
    handler::server::{tool::schema_for_output, wrapper::Parameters},
    model::CallToolResult,
    tool, tool_router,
};

// Import the public helper functions from the implementation files
use crate::service::tools::data_tools_impl::*;
use crate::service::tools::node_tools_impl::*;
use crate::service::tools::prefix_tools_impl::*;
use crate::service::tools::query_tools_impl::*;
use crate::service::tools::session_tools_impl::*;
use crate::service::tools::shacl_validate_tools_impl::*;
use crate::service::tools::shex_tools_impl::*;
use crate::service::tools::shex_validate_tools_impl::*;
use crate::service::tools::version_tools_impl::*;

#[tool_router]
impl RudofMcpService {
    // -------------------------------------------------------------------------
    // Data Management Tools
    // -------------------------------------------------------------------------

    /// Load RDF data into the server's in-memory datastore.
    #[tool(
        name = "load_rdf_data_from_sources",
        output_schema = schema_for_output::<LoadRdfDataFromSourcesResponse>(),
        description = "Load RDF triples into the server's in-memory datastore from URLs, local file paths, or inline RDF text. Each call is cumulative — triples are merged into existing data. `data` is optional — omit or pass [] when only using `endpoint`. To query a live SPARQL endpoint: set endpoint URL and pass data:[]. Call this before validate_shex, validate_shacl, execute_sparql_query, node_info, or export tools.",
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
        output_schema = schema_for_output::<ExportRdfDataResponse>(),
        description = "Serialize the server's in-memory RDF graph and return it as text. Use to inspect or export loaded data. Default format: turtle.",
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
        output_schema = schema_for_output::<ExportPlantUmlResponse>(),
        description = "Generate a PlantUML class diagram of the RDF graph structure. Shows subjects, predicates, and objects as a visual graph. Requires data to be loaded first.",
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
        output_schema = schema_for_output::<ExportImageResponse>(),
        description = "Render the server's RDF graph as an SVG or PNG image. Use for visual inspection of graph topology. Requires data to be loaded first.",
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
        output_schema = schema_for_output::<NodeInfoResponse>(),
        description = "Inspect a specific RDF node's neighborhood: outgoing arcs (node as subject) and incoming arcs (node as object). Use to explore what properties a resource has and what other resources point to it. Requires data to be loaded first.",
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
        output_schema = schema_for_output::<QueryExecutionResponse>(),
        description = "Execute a SPARQL query against the server's in-memory RDF graph. Supports SELECT, CONSTRUCT, and ASK (DESCRIBE not yet implemented). Provide a direct SPARQL string in `query` OR a natural language description in `query_natural_language` — not both. No `endpoint` parameter here — to query a live SPARQL endpoint, first call load_rdf_data_from_sources with {endpoint: 'https://...', data: []}. Requires data to be loaded first.",
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
        output_schema = schema_for_output::<ValidateShexResponse>(),
        description = "Validate the loaded RDF data against a ShEx schema. Requires a node–shape mapping: supply `shapemap` (e.g. ':alice@:Person') or `maybe_node` with an optional `maybe_shape` to auto-generate one. Requires data to be loaded first.",
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
        output_schema = schema_for_output::<CheckShexResponse>(),
        description = "Parse and verify that a ShEx schema is syntactically valid and well-formed without running any RDF validation. Use to catch schema errors before attempting validation. Does not require data to be loaded.",
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
        output_schema = schema_for_output::<ShowShexResponse>(),
        description = "Parse a ShEx schema and display it in the requested output format, with shape statistics and dependency analysis. Use to inspect, convert, or debug a schema. Does not require data to be loaded.",
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
        output_schema = schema_for_output::<ValidateShaclResponse>(),
        description = "Validate the loaded RDF data against a SHACL shapes graph. Returns a standard SHACL validation report. If `shapes` is omitted, shapes embedded in the loaded data are used. Requires data to be loaded first.",
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

    // -------------------------------------------------------------------------
    // Session Management Tools
    // -------------------------------------------------------------------------

    /// Reset session state (RDF data, loaded schemas/shapes, results, ...).
    #[tool(
        name = "reset_session_state",
        output_schema = schema_for_output::<ResetSessionStateResponse>(),
        description = "Clear session state loaded so far in this MCP session. With no `targets` (or [\"all\"]), clears everything (RDF data, ShEx/SHACL/pgschema/DCTap schemas, shapemap, query/validation results, ...) — the same as starting a fresh session. With one or more target names, clears only that state, leaving the rest untouched. Valid targets: data, shex, shex-validation, shacl, shacl-validation, pgschema, pgschema-validation, shapemap, dctap, service, query, sparql, typemap, rdf-config.",
        annotations(
            title = "Reset Session State",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
            open_world_hint = false,
        )
    )]
    pub async fn reset_session_state(
        &self,
        params: Parameters<ResetSessionStateRequest>,
    ) -> Result<CallToolResult, McpError> {
        reset_session_state_impl(self, params).await
    }

    /// Get or change this session's virtual working directory.
    #[tool(
        name = "change_directory",
        output_schema = schema_for_output::<ChangeDirectoryResponse>(),
        description = "Get or change this MCP session's virtual working directory, which relative local file paths passed to other tools (e.g. `load_rdf_data_from_sources`, `show_shex`, `validate_shacl`) are resolved against. Omit `path` to just report the current session directory. Each session has its own independent working directory — this never changes the server process's actual directory or affects other sessions.",
        annotations(
            title = "Get/Change Session Working Directory",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false,
        )
    )]
    pub async fn change_directory(
        &self,
        params: Parameters<ChangeDirectoryRequest>,
    ) -> Result<CallToolResult, McpError> {
        change_directory_impl(self, params).await
    }

    /// Report the rudof version this MCP server is running.
    #[tool(
        name = "get_rudof_version",
        output_schema = schema_for_output::<GetVersionResponse>(),
        description = "Report the rudof version this MCP server is running (the same version reported by `rudof --version`). Takes no parameters.",
        annotations(
            title = "Get Rudof Version",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false,
        )
    )]
    pub async fn get_rudof_version(&self, params: Parameters<EmptyRequest>) -> Result<CallToolResult, McpError> {
        get_version_impl(self, params).await
    }

    /// Show, or manage, the session's default prefix declarations.
    #[tool(
        name = "manage_prefixes",
        output_schema = schema_for_output::<ManagePrefixesResponse>(),
        description = "Show or manage the session's default prefix declarations -- the prefixes assumed and prepended by default to RDF data, SPARQL queries, ShEx schemas and SHACL shapes, independently of whatever prefixes a loaded resource already declares. Mirrors the `prefixes` command in the rudof shell. `action` defaults to \"list\" (no other fields needed). \"add\" requires `alias` and `iri`. \"remove\" requires `alias`. \"rename\" and \"copy\" require `alias` (the existing one) and `new_alias`. Always returns the full, current list of default prefixes after the action.",
        annotations(
            title = "Show/Manage Default Prefixes",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false,
        )
    )]
    pub async fn manage_prefixes(&self, params: Parameters<ManagePrefixesRequest>) -> Result<CallToolResult, McpError> {
        manage_prefixes_impl(self, params).await
    }
}

/// Public wrapper to expose the generated router from the macro
pub fn tool_router_public() -> ToolRouter<RudofMcpService> {
    RudofMcpService::tool_router()
}
