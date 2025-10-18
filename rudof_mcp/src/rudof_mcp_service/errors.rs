use rmcp::ErrorData as McpError;
use serde_json::Value;

// Canonical error codes used by this MCP server implementation.
pub mod codes {
    pub const RESOURCE_NOT_FOUND: &str = "resource_not_found";
    pub const INVALID_FORMAT: &str = "invalid_format";
    pub const INTERNAL_ERROR: &str = "internal_error";
    pub const RDF_LOAD_ERROR: &str = "rdf_load_error";
    pub const SERIALIZE_DATA_ERROR: &str = "serialize_data_error";
    pub const UTF8_CONVERSION_ERROR: &str = "utf8_conversion_error";
    pub const INVALID_NODE_SELECTOR: &str = "invalid_node_selector";
    pub const UNKNOWN_PREFIX: &str = "unknown_prefix";
    pub const INVALID_PREDICATE: &str = "invalid_predicate";
    pub const RDF_ARC_QUERY_ERROR: &str = "rdf_arc_query_error";
    pub const INVALID_MODE: &str = "invalid_mode";
    pub const NODE_NOT_FOUND: &str = "node_not_found";
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
