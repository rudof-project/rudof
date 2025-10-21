use crate::rudof_mcp_service::{errors::*, service::*};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::{
    node_info::*,
    srdf::{Iri, NeighsRDF, Subject, Term},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

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

// Formats an IRI/NamedNode as a plain string (no colors, no prefixing).
// Uses Iri's standard Display implementation (likely the full IRI).
fn format_iri_no_color<I: Iri>(iri: &I) -> String {
    iri.to_string()
}

// Formats an RDF Term as a plain string (no colors, no prefixing).
// Uses Term's standard Display implementation.
fn format_term_no_color<T: Term>(term: &T) -> String {
    term.to_string()
}

// Formats an RDF Subject as a plain string (no colors, no prefixing).
// Uses Subject's standard Display implementation.
fn format_subject_no_color<S: Subject>(subject: &S) -> String {
    subject.to_string()
}

// Convert NodeInfo to text format for MCP response
// This matches the CLI output format for consistency
fn format_node_info_text<S: NeighsRDF>(node_info: &NodeInfo<S>) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        "Information about {}\n",
        node_info.subject_qualified
    ));

    // Outgoing arcs
    if !node_info.outgoing.is_empty() {
        out.push_str("Outgoing arcs\n");
        out.push_str(&format!("{}\n", node_info.subject_qualified));

        let mut preds: Vec<_> = node_info.outgoing.keys().collect();
        preds.sort();

        for pred in preds {
            out.push_str(&format!(" -{}-> \n", format_iri_no_color(pred)));
            if let Some(objs) = node_info.outgoing.get(pred) {
                for o in objs {
                    out.push_str(&format!("       {}\n", format_term_no_color(o)));
                }
            }
        }
    }

    // Incoming arcs
    if !node_info.incoming.is_empty() {
        out.push_str("Incoming arcs\n");
        let object: S::Term = node_info.subject.clone().into();
        out.push_str(&format!("{}\n", format_term_no_color(&object)));

        let mut preds: Vec<_> = node_info.incoming.keys().collect();
        preds.sort();

        for pred in preds {
            out.push_str(&format!(" <-{}-\n", format_iri_no_color(pred)));
            if let Some(subjs) = node_info.incoming.get(pred) {
                for s in subjs {
                    out.push_str(&format!("       {}\n", format_subject_no_color(s)));
                }
            }
        }
    }

    out
}

// Convert NodeInfo to structured JSON response
fn node_info_to_response<S: NeighsRDF>(node_info: &NodeInfo<S>) -> NodeInfoResponse {
    let outgoing = node_info
        .outgoing
        .iter()
        .map(|(p, objs)| {
            let pred_q = format_iri_no_color(p);

            let obj_qs = objs.iter().map(format_term_no_color).collect();
            NodePredicateObjects {
                predicate: pred_q,
                objects: obj_qs,
            }
        })
        .collect();

    let incoming = node_info
        .incoming
        .iter()
        .map(|(p, subs)| {
            let pred_q = format_iri_no_color(p);

            let sub_qs = subs.iter().map(format_subject_no_color).collect();
            NodePredicateSubjects {
                predicate: pred_q,
                subjects: sub_qs,
            }
        })
        .collect();

    NodeInfoResponse {
        subject: node_info.subject_qualified.clone(),
        outgoing,
        incoming,
    }
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
            codes::INVALID_NODE_SELECTOR,
            Some(json!({ "node": node, "error": e.to_string() })),
        )
    })?;

    let mode_str = mode.as_deref().unwrap_or("both");
    let options = NodeInfoOptions::from_mode_str(mode_str)
        .map_err(|_| invalid_request(codes::INVALID_MODE, Some(json!({ "mode": mode_str }))))?;

    let pred_list: Vec<String> = predicates.unwrap_or_default();
    let node_infos = get_node_info(rdf, node_selector, &pred_list, options).map_err(|e| {
        internal_error(
            codes::RDF_ARC_QUERY_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let node_info = node_infos
        .first()
        .ok_or_else(|| internal_error(codes::NODE_NOT_FOUND, Some(json!({ "node": node }))))?;

    let text_output = format_node_info_text(node_info);
    let response = node_info_to_response(node_info);
    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            codes::SERIALIZE_DATA_ERROR,
            Some(json!({ "error": e.to_string() })),
        )
    })?;

    let mut result = CallToolResult::success(vec![Content::text(text_output)]);
    result.structured_content = Some(structured);
    Ok(result)
}
