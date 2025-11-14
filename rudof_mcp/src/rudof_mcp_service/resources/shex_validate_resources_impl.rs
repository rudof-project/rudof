use rmcp::{
    ErrorData as McpError,
    model::{Annotated, RawResource, ReadResourceResult, ResourceContents},
};
use serde_json::json;

pub fn get_shex_validate_resources() -> Vec<Annotated<RawResource>> {
    vec![
        Annotated {
            raw: RawResource {
                uri: "rudof://formats/shex".to_string(),
                name: "Supported ShEx Formats".to_string(),
                description: Some("List of all supported ShEx schema formats".to_string()),
                mime_type: Some("application/json".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://formats/validation-result".to_string(),
                name: "Supported Validation Result Formats".to_string(),
                description: Some(
                    "List of all supported ShEx validation result formats".to_string(),
                ),
                mime_type: Some("application/json".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://formats/validation-reader-modes".to_string(),
                name: "Validation Reader Modes".to_string(),
                description: Some("Available reader modes for validation".to_string()),
                mime_type: Some("application/json".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://formats/validation-sort-options".to_string(),
                name: "Validation Result Sort Options".to_string(),
                description: Some("Available sort options for validation results".to_string()),
                mime_type: Some("application/json".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
    ]
}

pub fn handle_shex_validate_resource(uri: &str) -> Option<Result<ReadResourceResult, McpError>> {
    match uri {
        "rudof://formats/shex" => Some(get_shex_formats(uri)),
        "rudof://formats/validation-result" => Some(get_shex_validation_result_formats(uri)),
        "rudof://formats/validation-reader-modes" => Some(get_reader_modes(uri)),
        "rudof://formats/validation-sort-options" => Some(get_validation_sort_options(uri)),
        _ => None,
    }
}

fn get_shex_formats(uri: &str) -> Result<ReadResourceResult, McpError> {
    let formats = json!({
        "formats": [
            {
                "name": "ShExC",
                "value": "shexc",
                "mime_type": "text/shex",
                "extensions": [".shex"],
                "description": "ShEx Compact Syntax - human-readable format (default)"
            },
            {
                "name": "ShExJ",
                "value": "shexj",
                "mime_type": "application/shex+json",
                "extensions": [".json"],
                "description": "ShEx JSON format"
            },
            {
                "name": "Internal",
                "value": "internal",
                "description": "Internal format for ShEx schemas"
            },
            {
                "name": "Simple",
                "value": "simple",
                "description": "Simplified ShEx format"
            },
            {
                "name": "JSON",
                "value": "json",
                "mime_type": "application/json",
                "extensions": [".json"],
                "description": "Plain JSON format"
            },
            {
                "name": "JSON-LD",
                "value": "jsonld",
                "mime_type": "application/ld+json",
                "extensions": [".jsonld"],
                "description": "JSON-LD format with linked data support"
            },
            {
                "name": "Turtle",
                "value": "turtle",
                "mime_type": "text/turtle",
                "extensions": [".ttl"],
                "description": "Terse RDF Triple Language"
            },
            {
                "name": "N-Triples",
                "value": "ntriples",
                "mime_type": "application/n-triples",
                "extensions": [".nt"],
                "description": "Line-based plain text format for RDF"
            },
            {
                "name": "RDF/XML",
                "value": "rdfxml",
                "mime_type": "application/rdf+xml",
                "extensions": [".rdf", ".xml"],
                "description": "XML-based RDF serialization"
            },
            {
                "name": "TriG",
                "value": "trig",
                "mime_type": "application/trig",
                "extensions": [".trig"],
                "description": "Extension of Turtle for named graphs"
            },
            {
                "name": "N3",
                "value": "n3",
                "mime_type": "text/n3",
                "extensions": [".n3"],
                "description": "Notation3 - superset of Turtle with additional features"
            },
            {
                "name": "N-Quads",
                "value": "nquads",
                "mime_type": "application/n-quads",
                "extensions": [".nq"],
                "description": "Extension of N-Triples for named graphs"
            }
        ],
        "default": "shexc"
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

fn get_shex_validation_result_formats(uri: &str) -> Result<ReadResourceResult, McpError> {
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

fn get_reader_modes(uri: &str) -> Result<ReadResourceResult, McpError> {
    let modes = json!({
        "modes": [
            {
                "name": "Strict",
                "value": "strict",
                "description": "Stops validation when there is an error (default)"
            },
            {
                "name": "Lax",
                "value": "lax",
                "description": "Emits a warning and continues processing when errors occur"
            }
        ],
        "default": "strict"
    });

    Ok(ReadResourceResult {
        contents: vec![ResourceContents::TextResourceContents {
            uri: uri.to_string(),
            mime_type: Some("application/json".to_string()),
            text: serde_json::to_string_pretty(&modes).unwrap(),
            meta: None,
        }],
    })
}

fn get_validation_sort_options(uri: &str) -> Result<ReadResourceResult, McpError> {
    let options = json!({
        "sort_options": [
            {
                "name": "Node",
                "value": "node",
                "description": "Sort validation results by RDF node (default)"
            },
            {
                "name": "Shape",
                "value": "shape",
                "description": "Sort validation results by ShEx shape"
            },
            {
                "name": "Status",
                "value": "status",
                "description": "Sort validation results by validation status (conformant/non-conformant)"
            },
            {
                "name": "Details",
                "value": "details",
                "description": "Sort validation results by detail level or error information"
            }
        ],
        "default": "node"
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
