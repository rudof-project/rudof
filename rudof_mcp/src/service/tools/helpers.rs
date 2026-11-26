use crate::service::errors::internal_error;
use rmcp::{ErrorData as McpError, model::CallToolResult, model::ContentBlock};
use rudof_lib::formats::InputSpec;
use serde::Serialize;
use serde_json::{Value, json};

/// Result type for parsing operations that may produce tool execution errors.
///
/// Tool execution errors (with `isError: true`) are different from protocol errors:
/// - **Tool Execution Errors**: Actionable feedback that LLMs can use to self-correct
///   and retry with adjusted parameters (e.g., invalid format, missing data).
/// - **Protocol Errors**: Issues with the request structure that models are less
///   likely to fix (e.g., unknown tool, malformed request).
pub type ToolResult<T> = Result<T, ToolExecutionError>;

/// Represents a tool execution error that should be returned with `isError: true`.
///
/// This allows LLMs to understand what went wrong and potentially retry
/// with corrected parameters.
#[derive(Debug)]
pub struct ToolExecutionError {
    /// Human-readable error message for the LLM
    pub message: String,
    /// Optional hint for how to fix the error
    pub hint: Option<String>,
}

impl ToolExecutionError {
    /// Create a new tool execution error with a message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            hint: None,
        }
    }

    /// Create a tool execution error with a hint for correction.
    pub fn with_hint(message: impl Into<String>, hint: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            hint: Some(hint.into()),
        }
    }

    /// Convert this error into a `CallToolResult` with `isError: true`.
    ///
    /// This is the MCP-recommended way to report errors that LLMs can
    /// use for self-correction.
    pub fn into_call_tool_result(self) -> CallToolResult {
        let text = if let Some(hint) = self.hint {
            format!("{}\n\nHint: {}", self.message, hint)
        } else {
            self.message
        };
        CallToolResult::error(vec![ContentBlock::text(text)])
    }
}

impl std::fmt::Display for ToolExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ToolExecutionError {}

/// Resolve an `argument` holding content, a URL or a file path into an
/// [`InputSpec`], reporting a recoverable tool error when it is none of those.
///
/// A relative path is resolved by [`InputSpec::from_str`] against the server
/// process's working directory, the one the client started it in.
pub fn resolve_spec(value: &str, argument: &str) -> ToolResult<InputSpec> {
    value.parse().map_err(|e| {
        ToolExecutionError::with_hint(
            format!("Invalid {argument}: {e}"),
            format!("Provide valid {argument} content, a URL, or a local file path"),
        )
    })
}

/// [`resolve_spec`] for an argument that may be omitted.
pub fn resolve_optional_spec(value: Option<&str>, argument: &str) -> ToolResult<Option<InputSpec>> {
    value.map(|value| resolve_spec(value, argument)).transpose()
}

/// Serialize tool response objects into MCP structured content.
pub fn serialize_structured<T: Serialize>(value: &T, operation: &str) -> Result<Value, McpError> {
    serde_json::to_value(value).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({
                "operation": operation,
                "phase": "serialize_response"
            })),
        )
    })
}

/// Default maximum characters included in text previews sent in `content`.
pub const DEFAULT_CONTENT_PREVIEW_CHARS: usize = 1200;

/// Build a Markdown code block preview and append truncation notice when needed.
pub fn code_block_preview(language: &str, text: &str, max_chars: usize) -> String {
    let truncated = text.chars().count() > max_chars;
    let preview: String = text.chars().take(max_chars).collect();
    let display = if truncated { preview.as_str() } else { text };
    let mut block = format!("```{}\n{}\n```", language, display);
    if truncated {
        block.push_str("\n\nPreview truncated. Full output is available in structuredContent.");
    }
    block
}

/// Supported RDF formats as a slice for completions.
pub const RDF_FORMAT_LIST: &[&str] = &["turtle", "ntriples", "rdfxml", "jsonld", "trig", "nquads", "n3"];

/// Detect the SPARQL query type by scanning for the first keyword token.
///
/// Returns `Some("SELECT")`, `Some("CONSTRUCT")`, `Some("ASK")`, `Some("DESCRIBE")`,
/// or `None` when no type keyword is found.
pub fn sparql_query_type(query: &str) -> Option<&'static str> {
    for token in query.split_whitespace() {
        match token.to_uppercase().as_str() {
            "SELECT" => return Some("SELECT"),
            "CONSTRUCT" => return Some("CONSTRUCT"),
            "ASK" => return Some("ASK"),
            "DESCRIBE" => return Some("DESCRIBE"),
            _ => {},
        }
    }
    None
}
