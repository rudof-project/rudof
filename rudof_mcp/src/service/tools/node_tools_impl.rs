use crate::service::{errors::*, mcp_service::*};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
};
use rudof_lib::formats::NodeInspectionMode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashSet,
    io::Cursor,
    str::FromStr,
};

use super::helpers::*;

/// Request parameters for inspecting an RDF node.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeInfoRequest {
    /// RDF node to inspect. Can be:
    /// - A full IRI (e.g., "http://example.org/person/1")
    /// - A prefixed name (e.g., "ex:person1" or ":localName")
    pub node: String,

    /// Optional list of predicates to filter the results.
    /// Only arcs with these predicates will be shown.
    pub predicates: Option<Vec<String>>,

    /// Optional maximum depth for traversing connected nodes.
    pub depth: Option<usize>,

    /// Display mode for node information.
    /// Supported: outgoing, incoming, both (default: both)
    pub mode: Option<String>,
}

/// Response containing node information.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeInfoResponse {
    /// Node selector used for inspection.
    pub node: String,
    /// Effective traversal mode.
    pub mode: String,
    /// Human-readable node inspection results.
    pub results: String,
    /// Parsed data extracted from the text tree output.
    pub parsed: ParsedNodeInfo,
}

/// Parsed arc entry extracted from the node tree output.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ParsedArc {
    /// Predicate shown in the rendered output.
    pub predicate: String,
    /// Target node/literal shown in the rendered output.
    pub target: String,
    /// Depth inside the rendered tree (0 means direct arc from the root node).
    pub depth: usize,
}

/// Parsed and summarized information extracted from node inspection text output.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ParsedNodeInfo {
    /// Parsed outgoing arcs.
    pub outgoing_arcs: Vec<ParsedArc>,
    /// Parsed incoming arcs.
    pub incoming_arcs: Vec<ParsedArc>,
    /// High-level stats derived from parsed arcs.
    pub stats: ParsedNodeInfoStats,
}

/// Summary statistics computed from parsed outgoing and incoming arcs.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ParsedNodeInfoStats {
    /// Total number of parsed outgoing arcs.
    pub outgoing_arcs_count: usize,
    /// Total number of parsed incoming arcs.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArcSection {
    Outgoing,
    Incoming,
}

/// Parses `show_node_info` text output into structured outgoing/incoming arcs.
fn parse_node_info_results(output: &str) -> ParsedNodeInfo {
    let mut section: Option<ArcSection> = None;
    let mut outgoing_arcs = Vec::new();
    let mut incoming_arcs = Vec::new();

    for line in output.lines() {
        let trimmed = line.trim();

        if trimmed.eq_ignore_ascii_case("Outgoing arcs") {
            section = Some(ArcSection::Outgoing);
            continue;
        }

        if trimmed.eq_ignore_ascii_case("Incoming arcs") {
            section = Some(ArcSection::Incoming);
            continue;
        }

        if trimmed.is_empty() || trimmed == "▲" {
            continue;
        }

        match section {
            Some(ArcSection::Outgoing) => {
                if let Some(arc) = parse_arc_line(line, " ─► ") {
                    outgoing_arcs.push(arc);
                }
            },
            Some(ArcSection::Incoming) => {
                if let Some(arc) = parse_arc_line(line, " ── ") {
                    incoming_arcs.push(arc);
                }
            },
            None => {},
        }
    }

    let outgoing_predicates_count = outgoing_arcs
        .iter()
        .map(|arc| arc.predicate.as_str())
        .collect::<HashSet<_>>()
        .len();

    let incoming_predicates_count = incoming_arcs
        .iter()
        .map(|arc| arc.predicate.as_str())
        .collect::<HashSet<_>>()
        .len();

    let unique_outgoing_targets_count = outgoing_arcs
        .iter()
        .map(|arc| arc.target.as_str())
        .collect::<HashSet<_>>()
        .len();

    let unique_incoming_sources_count = incoming_arcs
        .iter()
        .map(|arc| arc.target.as_str())
        .collect::<HashSet<_>>()
        .len();

    ParsedNodeInfo {
        stats: ParsedNodeInfoStats {
            outgoing_arcs_count: outgoing_arcs.len(),
            incoming_arcs_count: incoming_arcs.len(),
            outgoing_predicates_count,
            incoming_predicates_count,
            unique_outgoing_targets_count,
            unique_incoming_sources_count,
        },
        outgoing_arcs,
        incoming_arcs,
    }
}

/// Parses a single arc line using the provided rendered delimiter.
fn parse_arc_line(line: &str, delimiter: &str) -> Option<ParsedArc> {
    let (left, right) = line.split_once(delimiter)?;

    let predicate = strip_tree_glyphs(left);
    let target = right.trim().to_string();

    if predicate.is_empty() || target.is_empty() {
        return None;
    }

    Some(ParsedArc {
        predicate,
        target,
        depth: tree_depth(line),
    })
}

/// Removes tree drawing glyphs from the predicate side of a rendered tree line.
fn strip_tree_glyphs(value: &str) -> String {
    value
        .chars()
        .filter(|c| !matches!(c, '│' | '├' | '└' | '─'))
        .collect::<String>()
        .trim()
        .to_string()
}

/// Computes the nesting depth from line prefixes used by `termtree`.
fn tree_depth(line: &str) -> usize {
    let mut remaining = line;
    let mut depth = 0;

    loop {
        if let Some(rest) = remaining.strip_prefix("│   ") {
            depth += 1;
            remaining = rest;
            continue;
        }

        if let Some(rest) = remaining.strip_prefix("    ") {
            depth += 1;
            remaining = rest;
            continue;
        }

        break;
    }

    depth
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
    }): Parameters<NodeInfoRequest>,
) -> Result<CallToolResult, McpError> {
    let mut rudof = service.rudof.lock().await;
    let mode_str = mode.clone().unwrap_or_else(|| "both".to_string());

    // Parse mode - return Tool Execution Error for invalid mode
    let parsed_mode = match parse_optional_value_with_hint(
        mode.as_deref(),
        "mode",
        &format!("Supported modes: {}", NODE_INFO_MODES),
        NodeInspectionMode::from_str,
    ) {
        Ok(value) => value,
        Err(e) => return Ok(e.into_call_tool_result()),
    };

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

    let parsed = parse_node_info_results(&output_str);
    let stats = &parsed.stats;
    let results_size = output_str.chars().count();

    let summary = format!(
        "Node inspection completed.\nNode: {}\nMode: {}\nResults size: {} chars\nOutgoing arcs: {} (predicates: {}, unique targets: {})\nIncoming arcs: {} (predicates: {}, unique sources: {})",
        node,
        mode_str,
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
        mode: mode_str.clone(),
        results: output_str.clone(),
        parsed,
    };

    let structured = serialize_structured(&response, "node_info_impl")?;

    let results_preview = code_block_preview("text", &output_str, DEFAULT_CONTENT_PREVIEW_CHARS);

    let mut result = CallToolResult::success(vec![
        Content::text(summary),
        Content::text(format!("## Results Preview\n\n{}", results_preview)),
    ]);
    result.structured_content = Some(structured);

    Ok(result)
}
