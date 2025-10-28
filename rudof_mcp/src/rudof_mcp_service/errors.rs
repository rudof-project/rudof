use rmcp::ErrorData as McpError;
use serde_json::Value;

// Canonical error codes used by this MCP server implementation.
pub mod error_messages {
    // GENERAL Errors
    pub const RESOURCE_NOT_FOUND: &str = "Resource not found";
    pub const CONVERSION_ERROR: &str = "Conversion error";

    // DATA Errors
    pub const RDF_LOAD_ERROR: &str = "RDF load error";
    pub const INVALID_BASE_IRI: &str = "Invalid base IRI";
    pub const INVALID_DATA_SPEC: &str = "Invalid data spec";
    pub const VISUALIZATION_ERROR: &str = "Visualization error";
    pub const INVALID_DATA_FORMAT: &str = "Invalid data format";
    pub const INVALID_EXPORT_FORMAT: &str = "Invalid export format";
    pub const SERIALIZE_DATA_ERROR: &str = "Data serialization error";

    // NODE Errors
    pub const INVALID_NODE_SELECTOR: &str = "Invalid node selector";
    pub const NODE_NOT_FOUND: &str = "Node not found";
    pub const INVALID_NODE_MODE: &str = "Invalid mode";
    pub const RDF_ARC_QUERY_ERROR: &str = "RDF arc query error";

    // QUERY Errors
    pub const INVALID_QUERY_TYPE: &str = "Invalid query type";
    pub const QUERY_EXECUTION_ERROR: &str = "Query execution error";
    pub const INVALID_QUERY_RESULT_FORMAT: &str = "Invalid query result format";
}

// Create an `McpError::resource_not_found` with optional structured data.
pub fn resource_not_found(error_messages: &'static str, data: Option<Value>) -> McpError {
    McpError::resource_not_found(error_messages, data)
}

// Create an `McpError::invalid_request` with structured data.
pub fn invalid_request(error_messages: &'static str, data: Option<Value>) -> McpError {
    McpError::invalid_request(error_messages, data)
}

// Create an `McpError::internal_error` with structured data.
pub fn internal_error(error_messages: &'static str, data: Option<Value>) -> McpError {
    McpError::internal_error(error_messages, data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_resource_not_found_without_data() {
        let err = resource_not_found(error_messages::RESOURCE_NOT_FOUND, None);
        assert_eq!(err.message, error_messages::RESOURCE_NOT_FOUND);
        assert!(err.data.is_none());
    }

    #[test]
    fn test_resource_not_found_with_data() {
        let data = json!({"id": 123, "reason": "missing"});
        let err = resource_not_found(error_messages::RESOURCE_NOT_FOUND, Some(data.clone()));
        assert_eq!(err.message, error_messages::RESOURCE_NOT_FOUND);
        assert_eq!(err.data, Some(data.clone()));
    }

    #[test]
    fn test_invalid_request_without_data() {
        let err = invalid_request(error_messages::CONVERSION_ERROR, None);
        assert_eq!(err.message, error_messages::CONVERSION_ERROR);
        assert!(err.data.is_none());
    }

    #[test]
    fn test_invalid_request_with_data() {
        let data = json!({"field": "value"});
        let err = invalid_request(error_messages::CONVERSION_ERROR, Some(data.clone()));
        assert_eq!(err.message, error_messages::CONVERSION_ERROR);
        assert_eq!(err.data, Some(data.clone()));
    }

    #[test]
    fn test_internal_error_without_data() {
        let err = internal_error(error_messages::VISUALIZATION_ERROR, None);
        assert_eq!(err.message, error_messages::VISUALIZATION_ERROR);
        assert!(err.data.is_none());
    }

    #[test]
    fn test_internal_error_with_data() {
        let data = json!({"detail": "stacktrace"});
        let err = internal_error(error_messages::VISUALIZATION_ERROR, Some(data.clone()));
        assert_eq!(err.message, error_messages::VISUALIZATION_ERROR);
        assert_eq!(err.data, Some(data.clone()));
    }
}