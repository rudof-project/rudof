use crate::rudof_mcp_service::{errors::*, service::*};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::{
    node_info::*,
    parse_node_selector,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeInfoRequest {
    /// Node IRI or prefixed name (e.g. ":a" or "http://example.org/a")
    pub node: String,
    /// Optional list of predicates to filter outgoing arcs
    pub predicates: Option<Vec<String>>,
    /// Optional mode: "incoming", "outgoing", or "both" (default "both")
    pub mode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeInfoResponse {
    /// The qualified IRI of the RDF subject node.
    pub subject: String,
    /// List of outgoing arcs from the subject node.
    pub outgoing: Vec<NodePredicateObjects>,
    /// List of incoming arcs to the subject node.
    pub incoming: Vec<NodePredicateSubjects>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodePredicateObjects {
    /// The qualified IRI of the predicate.
    pub predicate: String,
    /// List of qualified object terms linked via this predicate.
    pub objects: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodePredicateSubjects {
    /// The qualified IRI of the predicate.
    pub predicate: String,
    /// List of qualified subject terms that point to the node via this predicate.
    pub subjects: Vec<String>,
}

pub async fn node_info_impl(
    service: &RudofMcpService,
    Parameters(NodeInfoRequest {
        node,
        predicates,
        mode,
    }): Parameters<NodeInfoRequest>,
) -> Result<CallToolResult, McpError> {
    let rudof = service.rudof.lock().await;
    let rdf = rudof.get_rdf_data();

    let node_selector = parse_node_selector(&node).map_err(|e| {
        invalid_request(
            error_messages::INVALID_NODE_SELECTOR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let mode_str = mode.as_deref().unwrap_or("both");
    let options = NodeInfoOptions::from_mode_str(mode_str)
        .map_err(|e| invalid_request(error_messages::INVALID_NODE_MODE, Some(json!({ "error": e.to_string() }))))?;

    let pred_list: Vec<String> = predicates.unwrap_or_default();
    let node_infos = get_node_info(rdf, node_selector, &pred_list, &options).map_err(|e| {
        internal_error(
            error_messages::RDF_ARC_QUERY_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let node_info = node_infos
        .first()
        .ok_or_else(|| internal_error(error_messages::NODE_NOT_FOUND, Some(json!({ "error": node }))))?;

    let mut output_buffer = Cursor::new(Vec::new());

    format_node_info_list(&node_infos, rdf, &mut output_buffer, &options).map_err(|e| {
        internal_error(
            error_messages::SERIALIZE_DATA_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            error_messages::SERIALIZE_DATA_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    // Función auxiliar para mapear el HashMap de arcos salientes a Vec<NodePredicateObjects>
    let outgoing_mapped: Vec<NodePredicateObjects> = node_info.outgoing
        .iter()
        .map(|(predicate_iri, objects_vec)| NodePredicateObjects {
            // Convertir S::IRI a String
            predicate: predicate_iri.to_string(), 
            // Convertir Vec<S::Term> a Vec<String>
            objects: objects_vec.iter().map(|term| term.to_string()).collect(),
        })
        .collect();

    // Función auxiliar para mapear el HashMap de arcos entrantes a Vec<NodePredicateSubjects>
    let incoming_mapped: Vec<NodePredicateSubjects> = node_info.incoming
        .iter()
        .map(|(predicate_iri, subjects_vec)| NodePredicateSubjects {
            // Convertir S::IRI a String
            predicate: predicate_iri.to_string(),
            // Convertir Vec<S::Subject> a Vec<String>
            subjects: subjects_vec.iter().map(|subject| subject.to_string()).collect(),
        })
        .collect();

    // Construir la respuesta estructurada
    let response = NodeInfoResponse {
        // Se asume que subject_qualified contiene la representación en String del subject
        subject: node_info.subject_qualified.clone(), 
        outgoing: outgoing_mapped,
        incoming: incoming_mapped,
    };

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            error_messages::SERIALIZE_DATA_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let mut result = CallToolResult::success(vec![Content::text(output_str)]);
    result.structured_content = Some(structured);
    Ok(result)
}
