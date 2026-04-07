use crate::service::errors::internal_error;
use rmcp::{ErrorData as McpError, model::CallToolResult, model::Content};
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
        CallToolResult::error(vec![Content::text(text)])
    }
}

impl std::fmt::Display for ToolExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ToolExecutionError {}

/// Parse a required input value and return a recoverable tool error when invalid.
pub fn parse_value_with_hint<T, E, F>(
    value: &str,
    value_name: &str,
    hint: &str,
    parser: F,
) -> ToolResult<T>
where
    F: FnOnce(&str) -> Result<T, E>,
    E: std::fmt::Display,
{
    parser(value).map_err(|e| {
        ToolExecutionError::with_hint(
            format!("Invalid {}: {}", value_name, e),
            hint.to_string(),
        )
    })
}

/// Parse an optional input value and return a recoverable tool error when invalid.
pub fn parse_optional_value_with_hint<T, E, F>(
    value: Option<&str>,
    value_name: &str,
    hint: &str,
    parser: F,
) -> ToolResult<Option<T>>
where
    F: Fn(&str) -> Result<T, E>,
    E: std::fmt::Display,
{
    value
        .map(|raw| parse_value_with_hint(raw, value_name, hint, &parser))
        .transpose()
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

/// Shared metadata for URI format resources.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct FormatEntry {
    pub value: &'static str,
    pub name: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<&'static str>,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub extensions: &'static [&'static str],
    pub description: &'static str,
}

/// Shared metadata for mode/sort option resources.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct OptionEntry {
    pub value: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

/// Shared metadata for query type resources.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct QueryTypeEntry {
    pub name: &'static str,
    pub description: &'static str,
    pub example: &'static str,
}

pub fn format_entries_json(entries: &[FormatEntry], default: &str) -> Value {
    json!({
        "formats": entries,
        "default": default,
    })
}

pub fn option_entries_json(key: &str, entries: &[OptionEntry], default: &str) -> Value {
    json!({
        key: entries,
        "default": default,
    })
}

pub fn query_type_entries_json(entries: &[QueryTypeEntry]) -> Value {
    json!({
        "query_types": entries,
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

pub const RDF_FORMAT_ENTRIES: &[FormatEntry] = &[
    FormatEntry {
        name: "Turtle",
        value: "turtle",
        mime_type: Some("text/turtle"),
        extensions: &[".ttl"],
        description: "Terse RDF Triple Language - human-readable format",
    },
    FormatEntry {
        name: "N-Triples",
        value: "ntriples",
        mime_type: Some("application/n-triples"),
        extensions: &[".nt"],
        description: "Line-based plain text format for RDF",
    },
    FormatEntry {
        name: "RDF/XML",
        value: "rdfxml",
        mime_type: Some("application/rdf+xml"),
        extensions: &[".rdf", ".xml"],
        description: "XML-based RDF serialization",
    },
    FormatEntry {
        name: "JSON-LD",
        value: "jsonld",
        mime_type: Some("application/ld+json"),
        extensions: &[".jsonld"],
        description: "JSON format with linked data support",
    },
    FormatEntry {
        name: "TriG",
        value: "trig",
        mime_type: Some("application/trig"),
        extensions: &[".trig"],
        description: "Extension of Turtle for named graphs",
    },
    FormatEntry {
        name: "N-Quads",
        value: "nquads",
        mime_type: Some("application/n-quads"),
        extensions: &[".nq"],
        description: "Extension of N-Triples for named graphs",
    },
    FormatEntry {
        name: "N3",
        value: "n3",
        mime_type: Some("text/n3"),
        extensions: &[".n3"],
        description: "Notation3 - superset of Turtle",
    },
];

/// Supported RDF formats as a constant for documentation and hints.
pub const RDF_FORMATS: &str = "turtle, ntriples, rdfxml, jsonld, trig, nquads, n3";

/// Supported ShEx formats as a slice for completions.
pub const SHEX_FORMAT_LIST: &[&str] =
    &["shexc", "shexj", "turtle", "ntriples", "rdfxml", "jsonld", "trig", "n3", "nquads", "internal", "simple", "json"];

pub const SHEX_FORMAT_ENTRIES: &[FormatEntry] = &[
    FormatEntry {
        name: "ShExC",
        value: "shexc",
        mime_type: Some("text/shex"),
        extensions: &[".shex"],
        description: "ShEx Compact Syntax - human-readable format (default)",
    },
    FormatEntry {
        name: "ShExJ",
        value: "shexj",
        mime_type: Some("application/shex+json"),
        extensions: &[".json"],
        description: "ShEx JSON format",
    },
    FormatEntry {
        name: "Internal",
        value: "internal",
        mime_type: None,
        extensions: &[],
        description: "Internal format for ShEx schemas",
    },
    FormatEntry {
        name: "Simple",
        value: "simple",
        mime_type: None,
        extensions: &[],
        description: "Simplified ShEx format",
    },
    FormatEntry {
        name: "JSON",
        value: "json",
        mime_type: Some("application/json"),
        extensions: &[".json"],
        description: "Plain JSON format",
    },
    FormatEntry {
        name: "JSON-LD",
        value: "jsonld",
        mime_type: Some("application/ld+json"),
        extensions: &[".jsonld"],
        description: "JSON-LD format with linked data support",
    },
    FormatEntry {
        name: "Turtle",
        value: "turtle",
        mime_type: Some("text/turtle"),
        extensions: &[".ttl"],
        description: "Terse RDF Triple Language",
    },
    FormatEntry {
        name: "N-Triples",
        value: "ntriples",
        mime_type: Some("application/n-triples"),
        extensions: &[".nt"],
        description: "Line-based plain text format for RDF",
    },
    FormatEntry {
        name: "RDF/XML",
        value: "rdfxml",
        mime_type: Some("application/rdf+xml"),
        extensions: &[".rdf", ".xml"],
        description: "XML-based RDF serialization",
    },
    FormatEntry {
        name: "TriG",
        value: "trig",
        mime_type: Some("application/trig"),
        extensions: &[".trig"],
        description: "Extension of Turtle for named graphs",
    },
    FormatEntry {
        name: "N3",
        value: "n3",
        mime_type: Some("text/n3"),
        extensions: &[".n3"],
        description: "Notation3 - superset of Turtle with additional features",
    },
    FormatEntry {
        name: "N-Quads",
        value: "nquads",
        mime_type: Some("application/n-quads"),
        extensions: &[".nq"],
        description: "Extension of N-Triples for named graphs",
    },
];

/// Supported ShEx formats as a constant for documentation and hints.
pub const SHEX_FORMATS: &str =
    "shexc, shexj, turtle, ntriples, rdfxml, trig, n3, nquads, json, jsonld, internal, simple";

/// Supported SHACL formats as a slice for completions.
pub const SHACL_FORMAT_LIST: &[&str] = &["turtle", "jsonld", "rdfxml", "trig", "nquads", "json"];

pub const SHACL_FORMAT_ENTRIES: &[FormatEntry] = &[
    FormatEntry {
        name: "Turtle",
        value: "turtle",
        mime_type: Some("text/turtle"),
        extensions: &[".ttl"],
        description: "Turtle serialization for SHACL shapes (default)",
    },
    FormatEntry {
        name: "JSON-LD",
        value: "jsonld",
        mime_type: Some("application/ld+json"),
        extensions: &[".jsonld"],
        description: "JSON-LD representation of SHACL shapes",
    },
    FormatEntry {
        name: "RDF/XML",
        value: "rdfxml",
        mime_type: Some("application/rdf+xml"),
        extensions: &[".rdf", ".xml"],
        description: "RDF/XML serialization",
    },
    FormatEntry {
        name: "TriG",
        value: "trig",
        mime_type: Some("application/trig"),
        extensions: &[".trig"],
        description: "TriG format for named graphs",
    },
    FormatEntry {
        name: "N-Quads",
        value: "nquads",
        mime_type: Some("application/n-quads"),
        extensions: &[".nq"],
        description: "N-Quads format",
    },
    FormatEntry {
        name: "JSON",
        value: "json",
        mime_type: Some("application/json"),
        extensions: &[".json"],
        description: "Plain JSON (when applicable)",
    },
];

/// Supported SHACL formats as a constant for documentation and hints.
pub const SHACL_FORMATS: &str = "turtle, rdfxml, jsonld, trig, nquads, json";

/// Supported ShapeMap formats as a constant for documentation and hints.
pub const SHAPEMAP_FORMATS: &str = "compact, json, internal, details, csv";

/// Supported image formats as a constant for documentation and hints.
pub const IMAGE_FORMATS: &str = "svg, png";

/// Supported SPARQL result formats as a constant for documentation and hints.
pub const SPARQL_RESULT_FORMATS: &str = "internal, turtle, ntriples, json-ld, rdf-xml, csv, trig, n3, nquads";

pub const SPARQL_QUERY_RESULT_FORMAT_ENTRIES: &[FormatEntry] = &[
    FormatEntry {
        name: "Internal",
        value: "internal",
        mime_type: None,
        extensions: &[],
        description: "Internal format for query results",
    },
    FormatEntry {
        name: "JSON",
        value: "json",
        mime_type: Some("application/sparql-results+json"),
        extensions: &[".srj", ".json"],
        description: "SPARQL JSON Results format",
    },
    FormatEntry {
        name: "XML",
        value: "xml",
        mime_type: Some("application/sparql-results+xml"),
        extensions: &[".srx", ".xml"],
        description: "SPARQL XML Results format",
    },
    FormatEntry {
        name: "CSV",
        value: "csv",
        mime_type: Some("text/csv"),
        extensions: &[".csv"],
        description: "Comma-separated values format",
    },
    FormatEntry {
        name: "TSV",
        value: "tsv",
        mime_type: Some("text/tab-separated-values"),
        extensions: &[".tsv"],
        description: "Tab-separated values format",
    },
    FormatEntry {
        name: "Turtle",
        value: "turtle",
        mime_type: Some("text/turtle"),
        extensions: &[".ttl"],
        description: "Terse RDF Triple Language (for CONSTRUCT/DESCRIBE)",
    },
    FormatEntry {
        name: "N-Triples",
        value: "ntriples",
        mime_type: Some("application/n-triples"),
        extensions: &[".nt"],
        description: "N-Triples format (for CONSTRUCT/DESCRIBE)",
    },
    FormatEntry {
        name: "RDF/XML",
        value: "rdfxml",
        mime_type: Some("application/rdf+xml"),
        extensions: &[".rdf"],
        description: "RDF/XML format (for CONSTRUCT/DESCRIBE)",
    },
    FormatEntry {
        name: "TriG",
        value: "trig",
        mime_type: Some("application/trig"),
        extensions: &[".trig"],
        description: "Extension of Turtle for named graphs",
    },
];

/// Supported SPARQL query result formats as a slice for completions.
pub const SPARQL_RESULT_FORMAT_LIST: &[&str] =
    &["internal", "json", "xml", "csv", "tsv", "turtle", "ntriples", "rdfxml", "trig"];

/// Supported validation result formats as a slice for completions (shared by ShEx and SHACL).
/// Includes all values from both validators: `csv` is valid for ShEx, `minimal` for SHACL.
pub const RESULT_FORMAT_LIST: &[&str] =
    &["details", "compact", "json", "csv", "minimal", "turtle", "ntriples", "rdfxml", "trig", "n3", "nquads"];

pub const SHEX_VALIDATION_RESULT_FORMAT_ENTRIES: &[FormatEntry] = &[
    FormatEntry {
        name: "Details",
        value: "details",
        mime_type: None,
        extensions: &[],
        description: "Detailed validation results with full error information (default)",
    },
    FormatEntry {
        name: "Compact",
        value: "compact",
        mime_type: None,
        extensions: &[],
        description: "Compact human-readable validation results",
    },
    FormatEntry {
        name: "JSON",
        value: "json",
        mime_type: Some("application/json"),
        extensions: &[],
        description: "Structured JSON validation results",
    },
    FormatEntry {
        name: "CSV",
        value: "csv",
        mime_type: Some("text/csv"),
        extensions: &[".csv"],
        description: "Validation results in CSV format",
    },
    FormatEntry {
        name: "Turtle",
        value: "turtle",
        mime_type: Some("text/turtle"),
        extensions: &[".ttl"],
        description: "Validation results in Turtle format",
    },
    FormatEntry {
        name: "N-Triples",
        value: "ntriples",
        mime_type: Some("application/n-triples"),
        extensions: &[".nt"],
        description: "Validation results in N-Triples format",
    },
    FormatEntry {
        name: "RDF/XML",
        value: "rdfxml",
        mime_type: Some("application/rdf+xml"),
        extensions: &[".rdf", ".xml"],
        description: "Validation results in RDF/XML format",
    },
    FormatEntry {
        name: "TriG",
        value: "trig",
        mime_type: Some("application/trig"),
        extensions: &[".trig"],
        description: "Validation results in TriG format",
    },
    FormatEntry {
        name: "N3",
        value: "n3",
        mime_type: Some("text/n3"),
        extensions: &[".n3"],
        description: "Validation results in Notation3 format",
    },
    FormatEntry {
        name: "N-Quads",
        value: "nquads",
        mime_type: Some("application/n-quads"),
        extensions: &[".nq"],
        description: "Validation results in N-Quads format",
    },
];

pub const SHACL_VALIDATION_RESULT_FORMAT_ENTRIES: &[FormatEntry] = &[
    FormatEntry {
        name: "Details",
        value: "details",
        mime_type: None,
        extensions: &[],
        description: "Detailed validation results with full error information (default)",
    },
    FormatEntry {
        name: "Compact",
        value: "compact",
        mime_type: None,
        extensions: &[],
        description: "Compact human-readable validation results",
    },
    FormatEntry {
        name: "Minimal",
        value: "minimal",
        mime_type: None,
        extensions: &[],
        description: "Minimal validation output",
    },
    FormatEntry {
        name: "JSON",
        value: "json",
        mime_type: Some("application/json"),
        extensions: &[],
        description: "Structured JSON validation results",
    },
    FormatEntry {
        name: "CSV",
        value: "csv",
        mime_type: Some("text/csv"),
        extensions: &[".csv"],
        description: "Validation results in CSV format",
    },
    FormatEntry {
        name: "Turtle",
        value: "turtle",
        mime_type: Some("text/turtle"),
        extensions: &[".ttl"],
        description: "Validation results in Turtle format",
    },
    FormatEntry {
        name: "N-Triples",
        value: "ntriples",
        mime_type: Some("application/n-triples"),
        extensions: &[".nt"],
        description: "Validation results in N-Triples format",
    },
    FormatEntry {
        name: "RDF/XML",
        value: "rdfxml",
        mime_type: Some("application/rdf+xml"),
        extensions: &[".rdf", ".xml"],
        description: "Validation results in RDF/XML format",
    },
    FormatEntry {
        name: "TriG",
        value: "trig",
        mime_type: Some("application/trig"),
        extensions: &[".trig"],
        description: "Validation results in TriG format",
    },
    FormatEntry {
        name: "N3",
        value: "n3",
        mime_type: Some("text/n3"),
        extensions: &[".n3"],
        description: "Validation results in Notation3 format",
    },
    FormatEntry {
        name: "N-Quads",
        value: "nquads",
        mime_type: Some("application/n-quads"),
        extensions: &[".nq"],
        description: "Validation results in N-Quads format",
    },
];

/// Supported ShEx validation result formats as a constant.
pub const SHEX_RESULT_FORMATS: &str = "compact, details, json, csv, turtle, ntriples, rdfxml, trig, n3, nquads";

/// Supported SHACL validation result formats as a constant.
pub const SHACL_RESULT_FORMATS: &str =
    "compact, details, minimal, json, csv, turtle, ntriples, rdfxml, trig, n3, nquads";

/// Supported reader modes as a constant.
pub const READER_MODES_LIST: &[&str] = &["strict", "lax"];

/// Supported node info modes as a slice for completions.
pub const NODE_INFO_MODE_LIST: &[&str] = &["both", "outgoing", "incoming"];

pub const NODE_MODE_ENTRIES: &[OptionEntry] = &[
    OptionEntry {
        name: "Both",
        value: "both",
        description: "Show both incoming and outgoing relationships",
    },
    OptionEntry {
        name: "Incoming",
        value: "incoming",
        description: "Show only relationships pointing to this node",
    },
    OptionEntry {
        name: "Outgoing",
        value: "outgoing",
        description: "Show only relationships originating from this node",
    },
];

/// Supported node info modes as a constant.
pub const NODE_INFO_MODES: &str = "outgoing, incoming, both";

/// Supported ShEx validation result sort modes as a constant.
pub const SHEX_SORT_BY_MODES: &str = "node, shape, status, details";

pub const READER_MODE_ENTRIES: &[OptionEntry] = &[
    OptionEntry {
        name: "Strict",
        value: "strict",
        description: "Stops validation when there is an error (default)",
    },
    OptionEntry {
        name: "Lax",
        value: "lax",
        description: "Emits a warning and continues processing when errors occur",
    },
];

pub const SHEX_VALIDATION_SORT_OPTION_ENTRIES: &[OptionEntry] = &[
    OptionEntry {
        name: "Node",
        value: "node",
        description: "Sort validation results by RDF node (default)",
    },
    OptionEntry {
        name: "Shape",
        value: "shape",
        description: "Sort validation results by ShEx shape",
    },
    OptionEntry {
        name: "Status",
        value: "status",
        description: "Sort validation results by validation status (conformant/non-conformant)",
    },
    OptionEntry {
        name: "Details",
        value: "details",
        description: "Sort validation results by detail level or error information",
    },
];

/// Supported SHACL validation result sort modes as a constant.
pub const SHACL_SORT_BY_MODES: &str = "severity, node, component, value, path, sourceshape, details";

pub const SHACL_VALIDATION_SORT_OPTION_ENTRIES: &[OptionEntry] = &[
    OptionEntry {
        name: "Severity",
        value: "severity",
        description: "Sort by violation severity: Violation > Warning > Info (default)",
    },
    OptionEntry {
        name: "Node",
        value: "node",
        description: "Sort by the focus node that was validated",
    },
    OptionEntry {
        name: "Component",
        value: "component",
        description: "Sort by SHACL constraint component type",
    },
    OptionEntry {
        name: "Value",
        value: "value",
        description: "Sort by the value that caused the violation",
    },
    OptionEntry {
        name: "Path",
        value: "path",
        description: "Sort by the property path involved in the violation",
    },
    OptionEntry {
        name: "Source Shape",
        value: "sourceshape",
        description: "Sort by the shape that produced the result",
    },
    OptionEntry {
        name: "Details",
        value: "details",
        description: "Sort by result message/details",
    },
];

pub const SPARQL_QUERY_TYPE_ENTRIES: &[QueryTypeEntry] = &[
    QueryTypeEntry {
        name: "SELECT",
        description: "Returns a table of variable bindings",
        example: "SELECT ?subject ?predicate ?object WHERE { ?subject ?predicate ?object }",
    },
    QueryTypeEntry {
        name: "CONSTRUCT",
        description: "Returns an RDF graph constructed by substituting variables",
        example: "CONSTRUCT { ?s ?p ?o } WHERE { ?s ?p ?o }",
    },
    QueryTypeEntry {
        name: "ASK",
        description: "Returns a boolean indicating whether a query pattern matches",
        example: "ASK WHERE { ?s ?p ?o }",
    },
    QueryTypeEntry {
        name: "DESCRIBE",
        description: "Returns an RDF graph describing resources",
        example: "DESCRIBE <http://example.org/resource>",
    },
];
