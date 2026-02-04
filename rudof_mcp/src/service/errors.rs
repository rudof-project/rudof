//! Structured error types for MCP protocol responses.
//!
//! This module provides helper functions to create properly formatted
//! MCP error responses with:
//!
//! - Appropriate JSON-RPC error codes
//! - Human-readable error messages
//! - Structured context data for debugging
//! - Automatic tracing/logging of errors

use rmcp::ErrorData as McpError;
use serde_json::{Map, Value, json};

/// Categories of MCP errors with their corresponding JSON-RPC error codes.
#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    /// The requested resource URI was not found.
    ResourceNotFound,

    /// The request parameters were invalid or malformed.
    InvalidRequest,

    /// An internal server error occurred.
    Internal,
}

/// Create an internal server error response.
///
/// Use this for unexpected failures in Rudof operations, I/O errors,
/// or other server-side issues.
///
/// # Arguments
///
/// * `message` - A brief, user-facing error description
/// * `cause` - The underlying error message or cause
/// * `context` - Optional JSON object with debugging context
pub fn internal_error(
    message: &'static str,
    cause: impl Into<String>,
    context: Option<Value>,
) -> McpError {
    mk_error(ErrorKind::Internal, message, Some(cause.into()), context)
}

/// Create an invalid request error response.
///
/// Use this when client-provided parameters are invalid, such as
/// unknown formats, malformed IRIs, or missing required fields.
///
/// # Arguments
///
/// * `message` - A brief, user-facing error description
/// * `cause` - Specific details about what was invalid
/// * `context` - Optional JSON object with debugging context
pub fn invalid_request_error(
    message: &'static str,
    cause: impl Into<String>,
    context: Option<Value>,
) -> McpError {
    mk_error(
        ErrorKind::InvalidRequest,
        message,
        Some(cause.into()),
        context,
    )
}

/// Create a resource not found error response.
///
/// Use this when a requested resource URI doesn't exist or
/// cannot be resolved.
///
/// # Arguments
///
/// * `message` - A brief, user-facing error description
/// * `cause` - The URI or resource that wasn't found
/// * `context` - Optional JSON object with debugging context
pub fn resource_not_found_error(
    message: &'static str,
    cause: impl Into<String>,
    context: Option<Value>,
) -> McpError {
    mk_error(
        ErrorKind::ResourceNotFound,
        message,
        Some(cause.into()),
        context,
    )
}

/// Internal helper to construct MCP error responses.
///
/// Merges context data, logs the error via tracing, and creates
/// the appropriate `ErrorData` variant.
fn mk_error(
    kind: ErrorKind,
    message: &'static str,
    cause: Option<String>,
    context: Option<Value>,
) -> McpError {
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
