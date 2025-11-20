use rmcp::{
    ErrorData as McpError,
    model::{Annotated, RawResource, ReadResourceResult, ResourceContents},
};
use serde_json::json;

pub fn get_query_resources() -> Vec<Annotated<RawResource>> {
    vec![
        Annotated {
            raw: RawResource {
                uri: "rudof://formats/query-types".to_string(),
                name: "Supported SPARQL Query Types".to_string(),
                description: Some("List of supported SPARQL query types".to_string()),
                mime_type: Some("application/json".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
        Annotated {
            raw: RawResource {
                uri: "rudof://formats/query-results".to_string(),
                name: "Supported Query Result Formats".to_string(),
                description: Some("List of all supported SPARQL query result formats".to_string()),
                mime_type: Some("application/json".to_string()),
                title: None,
                size: None,
                icons: None,
            },
            annotations: None,
        },
    ]
}

pub fn handle_query_resource(uri: &str) -> Option<Result<ReadResourceResult, McpError>> {
    match uri {
        "rudof://formats/query-types" => Some(get_query_types(uri)),
        "rudof://formats/query-results" => Some(get_query_result_formats(uri)),
        _ => None,
    }
}

fn get_query_types(uri: &str) -> Result<ReadResourceResult, McpError> {
    let types = json!({
        "query_types": [
            {
                "name": "SELECT",
                "description": "Returns a table of variable bindings",
                "example": "SELECT ?subject ?predicate ?object WHERE { ?subject ?predicate ?object }"
            },
            {
                "name": "CONSTRUCT",
                "description": "Returns an RDF graph constructed by substituting variables",
                "example": "CONSTRUCT { ?s ?p ?o } WHERE { ?s ?p ?o }"
            },
            {
                "name": "ASK",
                "description": "Returns a boolean indicating whether a query pattern matches",
                "example": "ASK WHERE { ?s ?p ?o }"
            },
            {
                "name": "DESCRIBE",
                "description": "Returns an RDF graph describing resources",
                "example": "DESCRIBE <http://example.org/resource>"
            }
        ]
    });

    Ok(ReadResourceResult {
        contents: vec![ResourceContents::TextResourceContents {
            uri: uri.to_string(),
            mime_type: Some("application/json".to_string()),
            text: serde_json::to_string_pretty(&types).unwrap(),
            meta: None,
        }],
    })
}

fn get_query_result_formats(uri: &str) -> Result<ReadResourceResult, McpError> {
    let formats = json!({
        "formats": [
            {
                "name": "Internal",
                "value": "internal",
                "description": "Internal format for query results"
            },
            {
                "name": "JSON",
                "value": "json",
                "mime_type": "application/sparql-results+json",
                "extensions": [".srj", ".json"],
                "description": "SPARQL JSON Results format"
            },
            {
                "name": "XML",
                "value": "xml",
                "mime_type": "application/sparql-results+xml",
                "extensions": [".srx", ".xml"],
                "description": "SPARQL XML Results format"
            },
            {
                "name": "CSV",
                "value": "csv",
                "mime_type": "text/csv",
                "extensions": [".csv"],
                "description": "Comma-separated values format"
            },
            {
                "name": "TSV",
                "value": "tsv",
                "mime_type": "text/tab-separated-values",
                "extensions": [".tsv"],
                "description": "Tab-separated values format"
            },
            {
                "name": "Turtle",
                "value": "turtle",
                "mime_type": "text/turtle",
                "extensions": [".ttl"],
                "description": "Terse RDF Triple Language (for CONSTRUCT/DESCRIBE)"
            },
            {
                "name": "N-Triples",
                "value": "ntriples",
                "mime_type": "application/n-triples",
                "extensions": [".nt"],
                "description": "N-Triples format (for CONSTRUCT/DESCRIBE)"
            },
            {
                "name": "RDF/XML",
                "value": "rdfxml",
                "mime_type": "application/rdf+xml",
                "extensions": [".rdf"],
                "description": "RDF/XML format (for CONSTRUCT/DESCRIBE)"
            },
            {
                "name": "TriG",
                "value": "trig",
                "mime_type": "application/trig",
                "description": "Extension of Turtle for named graphs"
            }
        ],
        "default": "internal"
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
