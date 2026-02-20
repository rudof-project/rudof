use crate::service::{errors::*, mcp_service::*};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::{node_info::*, parse_node_selector};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;

use super::helpers::*;

/// Request parameters for inspecting an RDF node.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeInfoRequest {
    /// RDF node to inspect. Can be:
    /// - A full IRI (e.g., "http://example.org/person/1")
    /// - A prefixed name (e.g., "ex:person1" or ":localName")
    /// - A blank node identifier (e.g., "_:b0")
    pub node: String,

    /// Optional list of predicates to filter the results.
    /// Only arcs with these predicates will be shown.
    pub predicates: Option<Vec<String>>,

    /// Display mode for node information.
    /// Supported: outgoing, incoming, both (default: both)
    pub mode: Option<String>,
}

/// Response containing node information.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeInfoResponse {
    /// The qualified IRI of the RDF subject node
    pub subject: String,
    /// List of outgoing arcs (predicates and their objects)
    pub outgoing: Vec<NodePredicateObjects>,
    /// List of incoming arcs (predicates and their subjects)
    pub incoming: Vec<NodePredicateSubjects>,
    /// Number of distinct outgoing predicates
    pub outgoing_count: usize,
    /// Number of distinct incoming predicates
    pub incoming_count: usize,
    /// Total number of outgoing objects across all predicates
    pub total_outgoing_objects: usize,
    /// Total number of incoming subjects across all predicates
    pub total_incoming_subjects: usize,
}

/// Represents outgoing arcs from a node.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodePredicateObjects {
    /// The qualified IRI of the predicate
    pub predicate: String,
    /// List of object values linked via this predicate
    pub objects: Vec<String>,
}

/// Represents incoming arcs to a node.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodePredicateSubjects {
    /// The qualified IRI of the predicate
    pub predicate: String,
    /// List of subject terms that point to the node via this predicate
    pub subjects: Vec<String>,
}

/// Get detailed information about an RDF node.
///
/// Returns both outgoing arcs (where the node is the subject) and
/// incoming arcs (where the node is the object).
///
/// # Errors
///
/// Returns a Tool Execution Error when:
/// - The node selector is invalid or malformed
/// - The mode is not recognized
/// - The node is not found in the RDF data
pub async fn node_info_impl(
    service: &RudofMcpService,
    Parameters(NodeInfoRequest { node, predicates, mode }): Parameters<NodeInfoRequest>,
) -> Result<CallToolResult, McpError> {
    let rudof = service.rudof.lock().await;
    let rdf = rudof.get_rdf_data();

    // Parse node selector - return Tool Execution Error for invalid input
    let node_selector = match parse_node_selector(&node) {
        Ok(sel) => sel,
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Invalid node selector '{}': {}", node, e),
                "Provide a valid IRI (e.g., 'http://example.org/node'), prefixed name (e.g., 'ex:node'), or blank node (e.g., '_:b0')",
            )
            .into_call_tool_result());
        },
    };

    // Parse mode - return Tool Execution Error for invalid mode
    let mode_str = mode.as_deref().unwrap_or("both");
    let mut options = match NodeInfoOptions::from_mode_str(mode_str) {
        Ok(opts) => opts,
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Invalid mode '{}': {}", mode_str, e),
                format!("Supported modes: {}", NODE_INFO_MODES),
            )
            .into_call_tool_result());
        },
    };
    options.show_colors = false;

    let pred_list: Vec<String> = predicates.unwrap_or_default();

    // Get node info - return Tool Execution Error if node not found
    let node_infos = match get_node_info(rdf, node_selector, &pred_list, &options) {
        Ok(infos) => infos,
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Node not found: {}", e),
                "Verify the node exists in the loaded RDF data. Use the correct IRI or prefixed name.",
            )
            .into_call_tool_result());
        },
    };

    let node_info = &node_infos[0];

    let mut output_buffer = Cursor::new(Vec::new());

    format_node_info_list(&node_infos, rdf, &mut output_buffer, &options).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({"operation":"node_info_impl", "phase":"format_node_info_list"})),
        )
    })?;

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"node_info_impl", "phase":"utf8_conversion"})),
        )
    })?;
    let outgoing_data: Vec<NodePredicateObjects> = node_info
        .outgoing
        .iter()
        .map(|(predicate_iri, objects_vec)| NodePredicateObjects {
            predicate: predicate_iri.to_string(),
            objects: objects_vec.iter().map(|term| term.to_string()).collect(),
        })
        .collect();

    let incoming_data: Vec<NodePredicateSubjects> = node_info
        .incoming
        .iter()
        .map(|(predicate_iri, subjects_vec)| NodePredicateSubjects {
            predicate: predicate_iri.to_string(),
            subjects: subjects_vec.iter().map(|subject| subject.to_string()).collect(),
        })
        .collect();

    // Calculate metadata
    let outgoing_count = outgoing_data.len();
    let incoming_count = incoming_data.len();
    let total_outgoing_objects: usize = outgoing_data.iter().map(|p| p.objects.len()).sum();
    let total_incoming_subjects: usize = incoming_data.iter().map(|p| p.subjects.len()).sum();

    let response = NodeInfoResponse {
        subject: node_info.subject_qualified.clone(),
        outgoing: outgoing_data,
        incoming: incoming_data,
        outgoing_count,
        incoming_count,
        total_outgoing_objects,
        total_incoming_subjects,
    };

    let structured = serde_json::to_value(&response).map_err(|e| {
        internal_error(
            "Serialization error",
            e.to_string(),
            Some(json!({"operation":"node_info_impl", "phase":"serialize_response"})),
        )
    })?;

    // Create a summary text
    let summary = format!(
        "# Node Information: {}\n\n\
        **Mode:** {}\n\
        **Outgoing Predicates:** {}\n\
        **Incoming Predicates:** {}\n\
        **Total Outgoing Objects:** {}\n\
        **Total Incoming Subjects:** {}\n",
        node_info.subject_qualified,
        mode_str,
        outgoing_count,
        incoming_count,
        total_outgoing_objects,
        total_incoming_subjects
    );

    let detailed_output = format!("## Detailed Node Information\n\n```\n{}\n```", output_str);

    let mut result = CallToolResult::success(vec![Content::text(summary), Content::text(detailed_output)]);
    result.structured_content = Some(structured);
    Ok(result)
}
