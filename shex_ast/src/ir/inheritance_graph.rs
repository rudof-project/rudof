use std::fmt::Display;

use crate::ShapeLabelIdx;
use petgraph::algo::toposort;
use petgraph::graphmap::GraphMap;
use petgraph::visit::IntoEdgeReferences;
use petgraph::visit::{Dfs, Reversed};
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
            },
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
