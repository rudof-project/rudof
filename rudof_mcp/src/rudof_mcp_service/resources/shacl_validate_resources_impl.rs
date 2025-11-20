use rmcp::{
    ErrorData as McpError,
    model::{Annotated, RawResource, ReadResourceResult, ResourceContents},
};
use serde_json::json;

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
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://formats/validation-result".to_string(),
                name: "Supported Validation SHACL Result Formats".to_string(),
                description: Some(
                    "List of all supported SHACL validation result formats".to_string(),
                ),
                mime_type: Some("application/json".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
    ]
}

pub fn handle_shacl_validate_resource(uri: &str) -> Option<Result<ReadResourceResult, McpError>> {
    match uri {
        "rudof://formats/shacl" => Some(get_shacl_formats(uri)),
        "rudof://formats/validation-result" => Some(get_shacl_validation_result_formats(uri)),
        _ => None,
    }
}

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
