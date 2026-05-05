use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::errors::NodeInspectionError;

/// Controls how bare IRI strings are handled before being passed to `ShapeMapParser`.
///
/// Use `Strict` in production pipelines or whenever callers control input format.
/// Use `Lax` for interactive tools, CLI usage, and MCP where convenience matters more.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum IriNormalizationMode {
    /// Auto-wrap bare `://` IRIs in angle brackets (convenient, heuristic-based).
    #[default]
    Lax,
    /// Require angle-bracketed IRIs; refuse to normalize (strict, predictable).
    Strict,
}

impl Display for IriNormalizationMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IriNormalizationMode::Lax => write!(f, "lax"),
            IriNormalizationMode::Strict => write!(f, "strict"),
        }
    }
}

/// Node inspection modes supported by Rudof.
///
/// Controls which edges/arcs are displayed when inspecting a specific node in an RDF graph.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum NodeInspectionMode {
    /// Show only outgoing arcs (where the node is the subject)
    Outgoing,
    /// Show only incoming arcs (where the node is the object)
    Incoming,
    /// Show both incoming and outgoing arcs
    #[default]
    Both,
}

impl NodeInspectionMode {
    pub fn show_outgoing(&self) -> bool {
        matches!(self, NodeInspectionMode::Outgoing | NodeInspectionMode::Both)
    }

    pub fn show_incoming(&self) -> bool {
        matches!(self, NodeInspectionMode::Incoming | NodeInspectionMode::Both)
    }
}

impl Display for NodeInspectionMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            NodeInspectionMode::Outgoing => write!(dest, "outgoing"),
            NodeInspectionMode::Incoming => write!(dest, "incoming"),
            NodeInspectionMode::Both => write!(dest, "both"),
        }
    }
}

impl FromStr for NodeInspectionMode {
    type Err = NodeInspectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "outgoing" => Ok(NodeInspectionMode::Outgoing),
            "incoming" => Ok(NodeInspectionMode::Incoming),
            "both" => Ok(NodeInspectionMode::Both),
            other => Err(NodeInspectionError::UnsupportedNodeInspectionMode {
                mode: other.to_string(),
            }),
        }
    }
}
