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
    InvalidParams,

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
pub fn internal_error(message: &'static str, cause: impl Into<String>, context: Option<Value>) -> McpError {
    mk_error(ErrorKind::Internal, message, Some(cause.into()), context)
}

/// Create an invalid params error response.
///
/// Use this when a specific request parameter value is invalid,
/// malformed, or unsupported.
pub fn invalid_params_error(message: &'static str, cause: impl Into<String>, context: Option<Value>) -> McpError {
    mk_error(ErrorKind::InvalidParams, message, Some(cause.into()), context)
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
pub fn resource_not_found_error(message: &'static str, cause: impl Into<String>, context: Option<Value>) -> McpError {
    mk_error(ErrorKind::ResourceNotFound, message, Some(cause.into()), context)
}

/// Internal helper to construct MCP error responses.
///
/// Merges context data, logs the error via tracing, and creates
/// the appropriate `ErrorData` variant.
fn mk_error(kind: ErrorKind, message: &'static str, cause: Option<String>, context: Option<Value>) -> McpError {
    let mut map = Map::new();
    if let Some(ctx) = context {
        match ctx {
            Value::Object(o) => {
                for (k, v) in o.into_iter() {
                    map.insert(k, v);
                }
            },
            other => {
                map.insert("context".to_string(), other);
            },
        }
    }

    if let Some(c) = cause {
        map.insert("cause".to_string(), json!(c));
    }

    match kind {
        ErrorKind::Internal => {
            tracing::error!(?message, ?map, "MCP internal error occurred");
        },
        ErrorKind::InvalidParams => {
            tracing::warn!(?message, ?map, "MCP client request error occurred");
        },
        ErrorKind::ResourceNotFound => {
            tracing::warn!(?message, ?map, "MCP resource not found");
        },
    }

    let value = Some(Value::Object(map));

    match kind {
        ErrorKind::ResourceNotFound => McpError::resource_not_found(message, value),
        ErrorKind::InvalidParams => McpError::invalid_params(message, value),
        ErrorKind::Internal => McpError::internal_error(message, value),
    }
}
