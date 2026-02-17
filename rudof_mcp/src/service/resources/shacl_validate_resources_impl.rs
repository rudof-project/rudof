//! SHACL validation resources.
//!
//! Provides information about SHACL schema formats, validation
//! result formats, and sort options.
//!
//! ## Resources
//!
//! - `rudof://formats/shacl` - Supported SHACL schema formats
//! - `rudof://formats/shacl-validation-result` - SHACL validation result formats
//! - `rudof://formats/shacl-validation-sort-options` - SHACL result sort options

use rmcp::{
    ErrorData as McpError,
    model::{Annotated, RawResource, ReadResourceResult, ResourceContents},
};
use serde_json::json;

/// Returns the list of SHACL validation-related resources.
pub fn get_shacl_validate_resources() -> Vec<Annotated<RawResource>> {
    vec![
        Annotated {
            raw: RawResource {
                uri: "rudof://formats/shacl".to_string(),
                name: "Supported SHACL Formats".to_string(),
                description: Some("List of all supported SHACL schema formats".to_string()),
                mime_type: Some("application/json".to_string()),
                title: None,
                size: None,
                icons: None,
                meta: None,
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://formats/shacl-validation-result".to_string(),
                name: "Supported SHACL Validation Result Formats".to_string(),
                description: Some("List of all supported SHACL validation result formats".to_string()),
                mime_type: Some("application/json".to_string()),
                title: None,
                size: None,
                icons: None,
                meta: None,
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://formats/shacl-validation-sort-options".to_string(),
                name: "SHACL Validation Result Sort Options".to_string(),
                description: Some("Available sort options for SHACL validation results".to_string()),
                mime_type: Some("application/json".to_string()),
                title: None,
                size: None,
                icons: None,
                meta: None,
            },
            annotations: None,
        },
    ]
}

/// Handles SHACL validation resource requests by URI.
///
/// Returns `Some(result)` if the URI matches a known resource,
/// or `None` to allow other handlers to process the request.
pub fn handle_shacl_validate_resource(uri: &str) -> Option<Result<ReadResourceResult, McpError>> {
    match uri {
        "rudof://formats/shacl" => Some(get_shacl_formats(uri)),
        "rudof://formats/shacl-validation-result" => Some(get_shacl_validation_result_formats(uri)),
        "rudof://formats/shacl-validation-sort-options" => Some(get_shacl_validation_sort_options(uri)),
        _ => None,
    }
}

/// Returns the list of supported SHACL schema formats.
fn get_shacl_formats(uri: &str) -> Result<ReadResourceResult, McpError> {
    let formats = json!({
        "formats": [
            {
                "name": "Turtle",
                "value": "turtle",
                "mime_type": "text/turtle",
                "extensions": [".ttl"],
                "description": "Turtle serialization for SHACL shapes (default)"
            },
            {
                "name": "JSON-LD",
                "value": "jsonld",
                "mime_type": "application/ld+json",
                "extensions": [".jsonld"],
                "description": "JSON-LD representation of SHACL shapes"
            },
            {
                "name": "RDF/XML",
                "value": "rdfxml",
                "mime_type": "application/rdf+xml",
                "extensions": [".rdf", ".xml"],
                "description": "RDF/XML serialization"
            },
            {
                "name": "TriG",
                "value": "trig",
                "mime_type": "application/trig",
                "extensions": [".trig"],
                "description": "TriG format for named graphs"
            },
            {
                "name": "N-Quads",
                "value": "nquads",
                "mime_type": "application/n-quads",
                "extensions": [".nq"],
                "description": "N-Quads format"
            },
            {
                "name": "JSON",
                "value": "json",
                "mime_type": "application/json",
                "extensions": [".json"],
                "description": "Plain JSON (when applicable)"
            }
        ],
        "default": "turtle"
    });

    Ok(ReadResourceResult {
        contents: vec![ResourceContents::TextResourceContents {
            uri: uri.to_string(),
            mime_type: Some("application/json".to_string()),
            text: serde_json::to_string_pretty(&formats).unwrap(),
            meta: None,
        }],
    })
}

fn get_shacl_validation_result_formats(uri: &str) -> Result<ReadResourceResult, McpError> {
    let formats = json!({
        "formats": [
            {
                "name": "Details",
                "value": "details",
                "description": "Detailed validation results with full error information (default)"
            },
            {
                "name": "Compact",
                "value": "compact",
                "description": "Compact human-readable validation results"
            },
            {
                "name": "JSON",
                "value": "json",
                "mime_type": "application/json",
                "description": "Structured JSON validation results"
            },
            {
                "name": "Turtle",
                "value": "turtle",
                "mime_type": "text/turtle",
                "extensions": [".ttl"],
                "description": "Validation results in Turtle format"
            },
            {
                "name": "N-Triples",
                "value": "ntriples",
                "mime_type": "application/n-triples",
                "extensions": [".nt"],
                "description": "Validation results in N-Triples format"
            },
            {
                "name": "RDF/XML",
                "value": "rdfxml",
                "mime_type": "application/rdf+xml",
                "extensions": [".rdf", ".xml"],
                "description": "Validation results in RDF/XML format"
            },
            {
                "name": "TriG",
                "value": "trig",
                "mime_type": "application/trig",
                "extensions": [".trig"],
                "description": "Validation results in TriG format"
            },
            {
                "name": "N3",
                "value": "n3",
                "mime_type": "text/n3",
                "extensions": [".n3"],
                "description": "Validation results in Notation3 format"
            },
            {
                "name": "N-Quads",
                "value": "nquads",
                "mime_type": "application/n-quads",
                "extensions": [".nq"],
                "description": "Validation results in N-Quads format"
            }
        ],
        "default": "details"
    });

    Ok(ReadResourceResult {
        contents: vec![ResourceContents::TextResourceContents {
            uri: uri.to_string(),
            mime_type: Some("application/json".to_string()),
            text: serde_json::to_string_pretty(&formats).unwrap(),
            meta: None,
        }],
    })
}

/// Returns the available sort options for SHACL validation results.
fn get_shacl_validation_sort_options(uri: &str) -> Result<ReadResourceResult, McpError> {
    let options = json!({
        "sort_options": [
            {
                "name": "Severity",
                "value": "severity",
                "description": "Sort by violation severity: Violation > Warning > Info (default)"
            },
            {
                "name": "Node",
                "value": "node",
                "description": "Sort by the focus node that was validated"
            },
            {
                "name": "Component",
                "value": "component",
                "description": "Sort by SHACL constraint component type"
            },
            {
                "name": "Value",
                "value": "value",
                "description": "Sort by the value that caused the violation"
            },
            {
                "name": "Path",
                "value": "path",
                "description": "Sort by the property path involved in the violation"
            },
            {
                "name": "Source Shape",
                "value": "sourceshape",
                "description": "Sort by the shape that produced the result"
            },
            {
                "name": "Details",
                "value": "details",
                "description": "Sort by result message/details"
            }
        ],
        "default": "severity"
    });

    Ok(ReadResourceResult {
        contents: vec![ResourceContents::TextResourceContents {
            uri: uri.to_string(),
            mime_type: Some("application/json".to_string()),
            text: serde_json::to_string_pretty(&options).unwrap(),
            meta: None,
        }],
    })
}
