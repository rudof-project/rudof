use rmcp::ErrorData as McpError;
use serde_json::Value;

// Canonical error codes used by this MCP server implementation.
pub mod codes {
    // GENERAL Errors
    pub const RESOURCE_NOT_FOUND: &str = "resource_not_found";
    pub const SERIALIZE_DATA_ERROR: &str = "serialize_data_error";
    pub const INVALID_FORMAT: &str = "invalid_format";

    // DATA Errors
    pub const RDF_LOAD_ERROR: &str = "rdf_load_error";
    pub const UTF8_CONVERSION_ERROR: &str = "utf8_conversion_error";
    pub const INVALID_BASE_IRI: &str = "invalid_base_iri";
    pub const INVALID_DATA_SPEC: &str = "invalid_data_spec";
    pub const VISUALIZATION_ERROR: &str = "visualization_error";

    // NODE Errors
    pub const INVALID_NODE_SELECTOR: &str = "invalid_node_selector";
    pub const NODE_NOT_FOUND: &str = "node_not_found";
    pub const INVALID_MODE: &str = "invalid_mode";
    pub const RDF_ARC_QUERY_ERROR: &str = "rdf_arc_query_error";

    // QUERY Errors
    pub const INVALID_QUERY_TYPE: &str = "invalid_query_type";
    pub const QUERY_EXECUTION_ERROR: &str = "query_execution_error";
}

// Create an `McpError::resource_not_found` with optional structured data.
pub fn resource_not_found(code_suffix: &'static str, data: Option<Value>) -> McpError {
    McpError::resource_not_found(code_suffix, data)
}

// Create an `McpError::invalid_request` with structured data.
pub fn invalid_request(code: &'static str, data: Option<Value>) -> McpError {
    McpError::invalid_request(code.to_string(), data)
}

// Create an `McpError::internal_error` with structured data.
pub fn internal_error(code: &'static str, data: Option<Value>) -> McpError {
    McpError::internal_error(code, data)
}
