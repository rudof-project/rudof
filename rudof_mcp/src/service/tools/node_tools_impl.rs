use crate::service::{errors::*, mcp_service::*};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib_refactored::formats::NodeInspectionMode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{io::Cursor, str::FromStr};

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

    /// Optional maximum depth for traversing connected nodes.
    pub depth: Option<usize>,

    /// Whether to show hyperlinks for connected nodes in the output.
    pub show_hyperlinks: Option<bool>,

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
    Parameters(NodeInfoRequest {
        node,
        predicates,
        depth,
        mode,
        show_hyperlinks,
    }): Parameters<NodeInfoRequest>,
) -> Result<CallToolResult, McpError> {
    let rudof = service.rudof.lock().await;

    // Parse mode - return Tool Execution Error for invalid mode
    let mut parsed_mode = None;
    if let Some(mode) = mode {
        match NodeInspectionMode::from_str(&mode) {
            Ok(mode) => parsed_mode = Some(mode),
            Err(e) => {
                return Ok(ToolExecutionError::with_hint(
                    format!("Invalid mode '{}': {}", mode, e),
                    format!("Supported modes: {}", NODE_INFO_MODES),
                )
                .into_call_tool_result());
            },
        };
    }

    let mut output_buffer = Cursor::new(Vec::new());
    let mut showing_node_info = rudof.show_node_info(&node, &mut output_buffer);
    if let Some(mode) = &parsed_mode {
        showing_node_info = showing_node_info.with_show_node_mode(mode);
    }
    if let Some(predicates) = predicates.as_deref() {
        showing_node_info = showing_node_info.with_predicates(predicates);
    }
    if let Some(depth) = depth {
        showing_node_info = showing_node_info.with_depth(depth);
    }
    if let Some(show_hyperlinks) = show_hyperlinks {
        showing_node_info = showing_node_info.with_show_hyperlinks(show_hyperlinks);
    }
    showing_node_info.execute().map_err(|e| {
        ToolExecutionError::with_hint(
            format!("Node not found: {}", e),
            "Verify the node exists in the loaded RDF data. Use the correct IRI or prefixed name.",
        )
        .into_call_tool_result()
    });

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"node_info_impl", "phase":"utf8_conversion"})),
        )
    })?;

    let detailed_output = format!("## Node Information\n\n```\n{}\n```", output_str);

    let result = CallToolResult::success(vec![Content::text(detailed_output)]);
    Ok(result)
}
