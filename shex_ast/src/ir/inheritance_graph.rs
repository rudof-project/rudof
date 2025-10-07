use std::fmt::Display;

use crate::ShapeLabelIdx;
use petgraph::algo::toposort;
use petgraph::graphmap::GraphMap;
use petgraph::visit::IntoEdgeReferences;
use petgraph::visit::{Dfs, EdgeRef, Reversed};
use tracing::trace;

#[derive(Debug, Default, Clone)]
pub struct InheritanceGraph {
    graph: GraphMap<ShapeLabelIdx, (), petgraph::Directed>,
}

impl InheritanceGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_edge(&mut self, from: ShapeLabelIdx, to: ShapeLabelIdx) {
        self.graph.add_edge(from, to, ());
    }

    pub fn has_cycles(&self) -> bool {
        match toposort(&self.graph, None) {
            Ok(_order) => false,
            Err(cycle_node) => {
                trace!("Cycle detected at node {:?}", cycle_node.node_id());
                true
            }
        }
    }

    /// In an inheritance graph, these are the nodes that extend the given node
    pub fn descendants(&self, node: &ShapeLabelIdx) -> Vec<ShapeLabelIdx> {
        let mut dfs = Dfs::new(Reversed(&self.graph), *node);
        let mut ancestors = Vec::new();

        while let Some(nx) = dfs.next(Reversed(&self.graph)) {
            if nx != *node {
                ancestors.push(nx);
            }
        }
        ancestors
    }

    /// In an inheritance graph these are the ndoes that are extended by the given node
    pub fn parents(&self, node: &ShapeLabelIdx) -> Vec<ShapeLabelIdx> {
        let mut dfs = Dfs::new(&self.graph, *node);
        let mut parents = Vec::new();

        while let Some(nx) = dfs.next(&self.graph) {
            if nx != *node {
                parents.push(nx);
            }
        }
        parents
    }
}

impl Display for InheritanceGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (source, target, _) in self.graph.edge_references() {
            writeln!(f, "{} -> {} ", source, target,)?;
        }
        Ok(())
    }
}

/// Iterator over the edges of the dependency graph.
/// The iterator yields tuples of the form (from, posneg, to).
pub struct DependencyGraphIter<'a> {
    inner: petgraph::graphmap::AllEdges<'a, ShapeLabelIdx, PosNeg, petgraph::Directed>,
}
impl Iterator for DependencyGraphIter<'_> {
    type Item = (ShapeLabelIdx, PosNeg, ShapeLabelIdx);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(from, to, posneg)| (from, *posneg, to))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PosNeg {
    #[default]
    Pos,
    Neg,
}

impl PosNeg {
    pub fn is_pos(&self) -> bool {
        matches!(self, PosNeg::Pos)
    }

    pub fn is_neg(&self) -> bool {
        matches!(self, PosNeg::Neg)
    }

    pub fn pos() -> PosNeg {
        PosNeg::Pos
    }

    pub fn neg() -> PosNeg {
        PosNeg::Neg
    }

    pub fn change(&self) -> PosNeg {
        match self {
            PosNeg::Pos => PosNeg::Neg,
            PosNeg::Neg => PosNeg::Pos,
        }
    }
}

impl Display for PosNeg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PosNeg::Pos => write!(f, "[+]"),
            PosNeg::Neg => write!(f, "[-]"),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_neg_cycle_no() {
        use super::*;

        let mut graph = DependencyGraph::new();
        graph.add_edge(ShapeLabelIdx::from(0), ShapeLabelIdx::from(1), PosNeg::Pos);
        graph.add_edge(ShapeLabelIdx::from(1), ShapeLabelIdx::from(0), PosNeg::Pos);
        assert!(!graph.has_neg_cycle());
    }

    #[test]
    fn test_neg_cycle_yes() {
        use super::*;

        let mut graph = DependencyGraph::new();
        graph.add_edge(ShapeLabelIdx::from(0), ShapeLabelIdx::from(1), PosNeg::Pos);
        graph.add_edge(ShapeLabelIdx::from(1), ShapeLabelIdx::from(0), PosNeg::Neg);
        assert!(graph.has_neg_cycle());
    }
}
