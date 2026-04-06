use crate::service::errors::*;
use crate::service::mcp_service::RudofMcpService;
use crate::service::resources::{json_resource_result, make_resource};
use crate::service::tools::helpers::{RDF_FORMAT_ENTRIES, format_entries_json};
use rmcp::{
    ErrorData as McpError,
    model::{Annotated, RawResource, ReadResourceResult, ResourceContents},
};
use rudof_lib::formats::ResultDataFormat;
use serde_json::json;
use std::str::FromStr;

/// Returns the list of available RDF data resources.
///
/// These resources allow clients to access the currently loaded RDF
/// data in various serialization formats.
pub fn get_data_resources() -> Vec<Annotated<RawResource>> {
    vec![
        make_resource(
            "rudof://current-data",
            "Current RDF Data (Turtle)",
            "Currently loaded RDF data in Turtle format",
            "text/turtle",
        ),
        make_resource(
            "rudof://formats/rdf",
            "Supported RDF Formats",
            "List of all supported RDF data formats for import/export",
            "application/json",
        ),
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
    let mut rudof = service.rudof.lock().await;

    let rdf_format = ResultDataFormat::from_str(format_str).map_err(|e| {
        invalid_params_error(
            "Invalid format parameter",
            e.to_string(),
            Some(json!({"phase":"parse_format","param":"format","value":format_str})),
        )
    })?;

    let mut buffer = Vec::new();
    rudof
        .serialize_data(&mut buffer)
        .with_result_data_format(&rdf_format)
        .execute()
        .map_err(|e| {
            internal_error(
                "Serialization error",
                e.to_string(),
                Some(json!({"operation":"export_rdf_data", "phase":"serialize_data"})),
            )
        })?;

    let text = String::from_utf8(buffer).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"export_rdf_data", "phase":"utf8_conversion"})),
        )
    })?;

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

    Ok(ReadResourceResult::new(vec![
        ResourceContents::TextResourceContents {
            uri: uri.to_string(),
            mime_type: Some(mime_type.to_string()),
            text,
            meta: None,
        },
    ]))
}

fn get_rdf_formats(uri: &str) -> Result<ReadResourceResult, McpError> {
    let formats = format_entries_json(RDF_FORMAT_ENTRIES, "turtle");
    json_resource_result(uri, &formats)
}
