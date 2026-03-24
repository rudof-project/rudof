use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::errors::NodeInspectionError;

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
