use crate::service::{errors::*, mcp_service::*};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
};
use rudof_lib::formats::{IriNormalizationMode, NodeInspectionMode};
use rudof_lib::types::{ArcDirection, NeighborArc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashSet, io::Cursor, num::NonZeroUsize};

use super::helpers::*;
use super::params::NodeModeParam;

/// Request parameters for inspecting an RDF node.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeInfoRequest {
    /// The RDF node to inspect. Accepted forms:
    /// - Full IRI in angle brackets: "<http://example.org/alice>"
    /// - Prefixed name: "ex:alice" or ":localName"
    /// - Blank node ID: "_:b0"
    pub node: String,

    /// Predicate IRIs to filter results. When provided, only arcs with these predicates
    /// are shown. When omitted, all arcs are returned.
    pub predicates: Option<Vec<String>>,

    /// Maximum traversal depth for the tree output. Default: 1 (direct arcs only).
    pub depth: Option<NonZeroUsize>,

    /// Which arcs to include: outgoing (node as subject), incoming (node as
    /// object), or both. Default: both.
    pub mode: Option<NodeModeParam>,

    /// IRI parsing strictness for `node`.
    /// false (default): bare "http://..." IRIs are auto-wrapped in "<>".
    /// true: bare IRIs cause a parse error — use "<http://...>" explicitly.
    pub strict_iris: Option<bool>,
}

/// Response containing node information.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeInfoResponse {
    /// Node selector used for inspection.
    pub node: String,
    /// Effective traversal mode.
    pub mode: NodeModeParam,
    /// The arcs where the node is the subject, in depth-first order.
    pub outgoing_arcs: Vec<NeighborArc>,
    /// The arcs where the node is the object, in depth-first order.
    pub incoming_arcs: Vec<NeighborArc>,
    /// Counts over the arcs above.
    pub stats: NodeInfoStats,
    /// Human-readable node inspection results.
    pub results: String,
}

/// Summary statistics computed from the outgoing and incoming arcs.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeInfoStats {
    /// Total number of outgoing arcs.
    pub outgoing_arcs_count: usize,
    /// Total number of incoming arcs.
    pub incoming_arcs_count: usize,
    /// Number of unique predicates among outgoing arcs.
    pub outgoing_predicates_count: usize,
    /// Number of unique predicates among incoming arcs.
    pub incoming_predicates_count: usize,
    /// Number of unique outgoing targets.
    pub unique_outgoing_targets_count: usize,
    /// Number of unique incoming sources.
    pub unique_incoming_sources_count: usize,
}

impl NodeInfoStats {
    /// Counts the arcs of each direction, their predicates and their far ends.
    fn of(outgoing_arcs: &[NeighborArc], incoming_arcs: &[NeighborArc]) -> Self {
        let unique =
            |arcs: &[NeighborArc], key: fn(&NeighborArc) -> String| arcs.iter().map(key).collect::<HashSet<_>>().len();
        let predicate = |arc: &NeighborArc| arc.predicate.to_string();
        let neighbor = |arc: &NeighborArc| arc.neighbor.to_string();

        Self {
            outgoing_arcs_count: outgoing_arcs.len(),
            incoming_arcs_count: incoming_arcs.len(),
            outgoing_predicates_count: unique(outgoing_arcs, predicate),
            incoming_predicates_count: unique(incoming_arcs, predicate),
            unique_outgoing_targets_count: unique(outgoing_arcs, neighbor),
            unique_incoming_sources_count: unique(incoming_arcs, neighbor),
        }
    }
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
        strict_iris,
    }): Parameters<NodeInfoRequest>,
) -> Result<CallToolResult, McpError> {
    let mut rudof = service.rudof.lock().await;
    let mode = mode.unwrap_or(NodeModeParam::Both);
    let parsed_mode: NodeInspectionMode = mode.into();

    let iri_mode = if strict_iris.unwrap_or(false) {
        IriNormalizationMode::Strict
    } else {
        IriNormalizationMode::Lax
    };

    let mut output_buffer = Cursor::new(Vec::new());
    let mut showing_node_info = rudof
        .show_node_info(&node, &mut output_buffer)
        .with_iri_mode(iri_mode)
        .with_show_node_mode(&parsed_mode);
    if let Some(predicates) = predicates.as_deref() {
        showing_node_info = showing_node_info.with_predicates(predicates);
    }
    if let Some(depth) = depth {
        showing_node_info = showing_node_info.with_depth(depth.get());
    }

    if let Err(e) = showing_node_info.with_show_colors(false).execute() {
        return Ok(ToolExecutionError::with_hint(
            format!("Node not found: {}", e),
            "Verify the node exists in the loaded RDF data. Use the correct IRI or prefixed name.",
        )
        .into_call_tool_result());
    }

    let output_bytes = output_buffer.into_inner();
    let output_str = String::from_utf8(output_bytes).map_err(|e| {
        internal_error(
            "Conversion error",
            e.to_string(),
            Some(json!({"operation":"node_info_impl", "phase":"utf8_conversion"})),
        )
    })?;

    let mut walking_neighborhood = rudof
        .node_neighborhood(&node)
        .with_mode(&parsed_mode)
        .with_iri_mode(iri_mode);
    if let Some(predicates) = predicates.as_deref() {
        walking_neighborhood = walking_neighborhood.with_predicates(predicates);
    }
    if let Some(depth) = depth {
        walking_neighborhood = walking_neighborhood.with_depth(depth.get());
    }
    let arcs = match walking_neighborhood
        .execute()
        .and_then(|arcs| arcs.collect::<Result<Vec<_>, _>>())
    {
        Ok(arcs) => arcs,
        Err(e) => {
            return Ok(ToolExecutionError::with_hint(
                format!("Failed to walk the neighborhood of {}: {}", node, e),
                "Give the predicates as angle-bracketed IRIs or prefixed names",
            )
            .into_call_tool_result());
        },
    };
    let (outgoing_arcs, incoming_arcs): (Vec<_>, Vec<_>) = arcs
        .into_iter()
        .partition(|arc| arc.direction == ArcDirection::Outgoing);
    let stats = NodeInfoStats::of(&outgoing_arcs, &incoming_arcs);
    let results_size = output_str.chars().count();

    let summary = format!(
        "Node inspection completed.\nNode: {}\nMode: {}\nResults size: {} chars\nOutgoing arcs: {} (predicates: {}, unique targets: {})\nIncoming arcs: {} (predicates: {}, unique sources: {})",
        node,
        mode,
        results_size,
        stats.outgoing_arcs_count,
        stats.outgoing_predicates_count,
        stats.unique_outgoing_targets_count,
        stats.incoming_arcs_count,
        stats.incoming_predicates_count,
        stats.unique_incoming_sources_count,
    );

    let response = NodeInfoResponse {
        node: node.clone(),
        mode,
        outgoing_arcs,
        incoming_arcs,
        stats,
        results: output_str.clone(),
    };

    let structured = serialize_structured(&response, "node_info_impl")?;

    let mut result = CallToolResult::success(vec![ContentBlock::text(summary)]);
    result.structured_content = Some(structured);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rudof_lib::formats::InputSpec;
    use rudof_lib::types::Object;

    async fn inspect(node: &str, mode: Option<NodeModeParam>) -> NodeInfoResponse {
        let service = RudofMcpService::new();
        service
            .rudof
            .lock()
            .await
            .load_data()
            .with_data(&[InputSpec::str(
                r#"prefix : <http://example.org/>
                   :alice :name "Alice"@en ; :knows :bob .
                   :bob   :knows :alice ."#,
            )])
            .execute()
            .expect("the test data should load");

        let result = node_info_impl(
            &service,
            Parameters(NodeInfoRequest {
                node: node.to_string(),
                predicates: None,
                depth: None,
                mode,
                strict_iris: None,
            }),
        )
        .await
        .expect("inspection should not fail at the protocol level");

        serde_json::from_value(result.structured_content.expect("the tool reports structured content"))
            .expect("the structured content should be a NodeInfoResponse")
    }

    #[tokio::test]
    async fn an_arc_keeps_the_language_tag_of_its_literal() {
        let response = inspect(":alice", None).await;

        let name = response
            .outgoing_arcs
            .iter()
            .find(|arc| arc.predicate.as_str() == "http://example.org/name")
            .expect(":alice has a :name arc");

        // The rendered tree writes a literal as `"Alice"@en`; the arc carries the lexical
        // form and the tag apart, which is what the glyph scraping used to lose.
        match &name.neighbor {
            Object::Literal(literal) => {
                assert_eq!(literal.lexical_form(), "Alice");
                assert_eq!(
                    literal.lang().map(|lang| lang.as_str().to_string()),
                    Some("en".to_string())
                );
            },
            other => panic!("the :name arc should reach a literal, not {other}"),
        }
    }

    #[tokio::test]
    async fn the_arcs_are_split_by_direction_and_counted() {
        let response = inspect(":alice", None).await;

        assert_eq!(response.outgoing_arcs.len(), 2);
        assert_eq!(response.incoming_arcs.len(), 1);
        assert!(
            response
                .outgoing_arcs
                .iter()
                .all(|arc| arc.direction == ArcDirection::Outgoing)
        );
        assert!(
            response
                .incoming_arcs
                .iter()
                .all(|arc| arc.direction == ArcDirection::Incoming)
        );

        assert_eq!(response.stats.outgoing_arcs_count, 2);
        assert_eq!(response.stats.outgoing_predicates_count, 2);
        assert_eq!(response.stats.incoming_arcs_count, 1);
        assert_eq!(response.stats.unique_incoming_sources_count, 1);
    }

    #[tokio::test]
    async fn outgoing_mode_leaves_the_incoming_arcs_out() {
        let response = inspect(":alice", Some(NodeModeParam::Outgoing)).await;

        assert_eq!(response.outgoing_arcs.len(), 2);
        assert!(response.incoming_arcs.is_empty());
        assert_eq!(response.stats.incoming_arcs_count, 0);
    }
}
