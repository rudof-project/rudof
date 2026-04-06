use rmcp::{model::CallToolResult, model::Content};
use std::str::FromStr;

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
        CallToolResult::error(vec![Content::text(text)])
    }
}

impl std::fmt::Display for ToolExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ToolExecutionError {}

/// Parse a required format string into a typed format.
///
/// Returns a `ToolExecutionError` if the format is invalid or missing.
///
/// # Type Parameters
///
/// * `F` - The format type to parse into (must implement `FromStr`)
///
/// # Arguments
///
/// * `format` - Format string to parse
/// * `format_name` - Human-readable name for error messages
/// * `valid_values` - Comma-separated list of valid values for hints
#[allow(dead_code)]
pub fn parse_required_format<F>(format: &str, format_name: &str, valid_values: &str) -> ToolResult<F>
where
    F: FromStr,
    F::Err: std::fmt::Display,
{
    F::from_str(format).map_err(|e| {
        ToolExecutionError::with_hint(
            format!("Invalid {}: {}", format_name, e),
            format!("Supported values: {}", valid_values),
        )
    })
}

/// Supported RDF formats as a constant for documentation and hints.
pub const RDF_FORMATS: &str = "turtle, ntriples, rdfxml, jsonld, trig, nquads, n3";

/// Supported ShEx formats as a constant for documentation and hints.
#[allow(dead_code)]
pub const SHEX_FORMATS: &str =
    "shexc, shexj, turtle, ntriples, rdfxml, trig, n3, nquads, json, jsonld, internal, simple";

/// Supported SHACL formats as a constant for documentation and hints.
#[allow(dead_code)]
pub const SHACL_FORMATS: &str = "turtle, ntriples, rdfxml, jsonld, trig, n3, nquads, internal";

/// Supported ShapeMap formats as a constant for documentation and hints.
#[allow(dead_code)]
pub const SHAPEMAP_FORMATS: &str = "compact, json, internal, details, csv";

/// Supported image formats as a constant for documentation and hints.
pub const IMAGE_FORMATS: &str = "svg, png";

/// Supported SPARQL result formats as a constant for documentation and hints.
pub const SPARQL_RESULT_FORMATS: &str = "internal, turtle, ntriples, json-ld, rdf-xml, csv, trig, n3, nquads";

/// Supported ShEx validation result formats as a constant.
#[allow(dead_code)]
pub const SHEX_RESULT_FORMATS: &str = "compact, details, json, csv, turtle, ntriples, rdfxml, trig, n3, nquads";

/// Supported SHACL validation result formats as a constant.
#[allow(dead_code)]
pub const SHACL_RESULT_FORMATS: &str =
    "compact, details, minimal, json, csv, turtle, ntriples, rdfxml, trig, n3, nquads";

/// Supported reader modes as a constant.
#[allow(dead_code)]
pub const READER_MODES: &str = "strict, lax";

/// Supported node info modes as a constant.
pub const NODE_INFO_MODES: &str = "outgoing, incoming, both";

/// Supported ShEx validation result sort modes as a constant.
#[allow(dead_code)]
pub const SHEX_SORT_BY_MODES: &str = "node, shape, status, details";

/// Supported SHACL validation result sort modes as a constant.
#[allow(dead_code)]
pub const SHACL_SORT_BY_MODES: &str = "severity, node, component, value, path, sourceshape, details";
