use rmcp::ErrorData as McpError;
use serde_json::{json, Map, Value};

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    ResourceNotFound,
    InvalidRequest,
    Internal,
}

pub fn internal_error(message: &'static str, cause: impl Into<String>, context: Option<Value>) -> McpError {
    mk_error(ErrorKind::Internal, message, Some(cause.into()), context)
}

pub fn invalid_request_error(message: &'static str, cause: impl Into<String>, context: Option<Value>) -> McpError {
    mk_error(ErrorKind::InvalidRequest, message, Some(cause.into()), context)
}

pub fn resource_not_found_error(message: &'static str, cause: impl Into<String>, context: Option<Value>) -> McpError {
    mk_error(ErrorKind::ResourceNotFound, message, Some(cause.into()), context)
}

fn mk_error(kind: ErrorKind, message: &'static str, cause: Option<String>, context: Option<Value>) -> McpError {
    let mut map = Map::new();
    if let Some(ctx) = context {
        match ctx {
            Value::Object(o) => {
                for (k, v) in o.into_iter() {
                    map.insert(k, v);
                }
            }
            other => {
                map.insert("context".to_string(), other);
            }
        }
    }

    if let Some(c) = cause {
        map.insert("cause".to_string(), json!(c));
    }

    tracing::error!(?message, ?map, "MCP error occurred");

    let value = Some(Value::Object(map));

    match kind {
        ErrorKind::ResourceNotFound => McpError::resource_not_found(message, value),
        ErrorKind::InvalidRequest => McpError::invalid_request(message, value),
        ErrorKind::Internal => McpError::internal_error(message, value),
    }
}