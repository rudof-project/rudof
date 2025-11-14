use crate::rudof_mcp_service::errors::rdf_error;
use crate::rudof_mcp_service::service::RudofMcpService;
use rmcp::{
    ErrorData as McpError,
    model::{Annotated, RawResource, ReadResourceResult, ResourceContents},
};
use rudof_lib::RDFFormat;
use serde_json::json;
use std::str::FromStr;

pub fn get_data_resources() -> Vec<Annotated<RawResource>> {
    vec![
        Annotated {
            raw: RawResource {
                uri: "rudof://current-data".to_string(),
                name: "Current RDF Data (Turtle)".to_string(),
                description: Some("Currently loaded RDF data in Turtle format".to_string()),
                mime_type: Some("text/turtle".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://current-data/ntriples".to_string(),
                name: "Current RDF Data (N-Triples)".to_string(),
                description: Some("Currently loaded RDF data in N-Triples format".to_string()),
                mime_type: Some("application/n-triples".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://current-data/rdfxml".to_string(),
                name: "Current RDF Data (RDF/XML)".to_string(),
                description: Some("Currently loaded RDF data in RDF/XML format".to_string()),
                mime_type: Some("application/rdf+xml".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://current-data/jsonld".to_string(),
                name: "Current RDF Data (JSON-LD)".to_string(),
                description: Some("Currently loaded RDF data in JSON-LD format".to_string()),
                mime_type: Some("application/ld+json".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://current-data/trig".to_string(),
                name: "Current RDF Data (TriG)".to_string(),
                description: Some("Currently loaded RDF data in TriG format".to_string()),
                mime_type: Some("application/trig".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://current-data/nquads".to_string(),
                name: "Current RDF Data (N-Quads)".to_string(),
                description: Some("Currently loaded RDF data in N-Quads format".to_string()),
                mime_type: Some("application/n-quads".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://current-data/n3".to_string(),
                name: "Current RDF Data (N3)".to_string(),
                description: Some("Currently loaded RDF data in Notation3 format".to_string()),
                mime_type: Some("text/n3".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://formats/rdf".to_string(),
                name: "Supported RDF Formats".to_string(),
                description: Some(
                    "List of all supported RDF data formats for import/export".to_string(),
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

pub async fn handle_data_resource(
    service: &RudofMcpService,
    uri: &str,
) -> Option<Result<ReadResourceResult, McpError>> {
    match uri {
        "rudof://current-data" => Some(export_rdf_data(service, uri, "turtle").await),
        "rudof://current-data/ntriples" => Some(export_rdf_data(service, uri, "ntriples").await),
        "rudof://current-data/rdfxml" => Some(export_rdf_data(service, uri, "rdfxml").await),
        "rudof://current-data/jsonld" => Some(export_rdf_data(service, uri, "jsonld").await),
        "rudof://current-data/trig" => Some(export_rdf_data(service, uri, "trig").await),
        "rudof://current-data/nquads" => Some(export_rdf_data(service, uri, "nquads").await),
        "rudof://current-data/n3" => Some(export_rdf_data(service, uri, "n3").await),
        "rudof://formats/rdf" => Some(get_rdf_formats(uri)),
        _ => None,
    }
}

/// Export RDF data from the service in a specific format
pub async fn export_rdf_data(
    service: &RudofMcpService,
    uri: &str,
    format_str: &str,
) -> Result<ReadResourceResult, McpError> {
    let rudof = service.rudof.lock().await;

    let rdf_format =
        RDFFormat::from_str(format_str).map_err(|e| rdf_error("parsing format", e.to_string()))?;

    let mut buffer = Vec::new();
    rudof
        .serialize_data(&rdf_format, &mut buffer)
        .map_err(|e| rdf_error("serializing data", e.to_string()))?;

    let text =
        String::from_utf8(buffer).map_err(|e| rdf_error("converting to UTF-8", e.to_string()))?;

    let mime_type = match format_str {
        "turtle" => "text/turtle",
        "ntriples" => "application/n-triples",
        "rdfxml" => "application/rdf+xml",
        "jsonld" => "application/ld+json",
        "trig" => "application/trig",
        "nquads" => "application/n-quads",
        "n3" => "text/n3",
        _ => "text/plain",
    };

    Ok(ReadResourceResult {
        contents: vec![ResourceContents::TextResourceContents {
            uri: uri.to_string(),
            mime_type: Some(mime_type.to_string()),
            text,
            meta: None,
        }],
    })
}

fn get_rdf_formats(uri: &str) -> Result<ReadResourceResult, McpError> {
    let formats = json!({
        "formats": [
            {
                "name": "Turtle",
                "value": "turtle",
                "mime_type": "text/turtle",
                "extensions": [".ttl"],
                "description": "Terse RDF Triple Language - human-readable format"
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
                "name": "JSON-LD",
                "value": "jsonld",
                "mime_type": "application/ld+json",
                "extensions": [".jsonld"],
                "description": "JSON format with linked data support"
            },
            {
                "name": "TriG",
                "value": "trig",
                "mime_type": "application/trig",
                "extensions": [".trig"],
                "description": "Extension of Turtle for named graphs"
            },
            {
                "name": "N-Quads",
                "value": "nquads",
                "mime_type": "application/n-quads",
                "extensions": [".nq"],
                "description": "Extension of N-Triples for named graphs"
            },
            {
                "name": "N3",
                "value": "n3",
                "mime_type": "text/n3",
                "extensions": [".n3"],
                "description": "Notation3 - superset of Turtle"
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
