use std::fmt::Display;

use crate::ShapeLabelIdx;
use petgraph::dot::{Config, Dot};
use petgraph::graphmap::GraphMap;
use petgraph::visit::EdgeRef;

#[derive(Debug, Default)]
pub(crate) struct DependencyGraph {
    graph: GraphMap<ShapeLabelIdx, PosNeg, petgraph::Directed>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_pos_edge(&mut self, from: ShapeLabelIdx, to: ShapeLabelIdx) {
        let f = self.graph.add_node(from);
        let t = self.graph.add_node(to);
        self.graph.add_edge(f, t, PosNeg::pos());
    }

    pub fn add_neg_edge(&mut self, from: ShapeLabelIdx, to: ShapeLabelIdx) {
        let f = self.graph.add_node(from);
        let t = self.graph.add_node(to);
        self.graph.add_edge(f, t, PosNeg::neg());
    }

    pub fn neg_cycles(&self) -> Vec<Vec<(ShapeLabelIdx, ShapeLabelIdx, Vec<ShapeLabelIdx>)>> {
        let mut result = Vec::new();
        let scc = petgraph::algo::tarjan_scc(&self.graph);
        for component in &scc {
            println!("Component: {:?}", component);
            let mut neg_cycle = Vec::new();
            for node in component.iter().as_slice() {
                let edges = self
                    .graph
                    .edges_directed(*node, petgraph::Direction::Outgoing);
                for edge in edges {
                    if component.contains(&edge.target()) && edge.weight().is_neg() {
                        let mut shapes = Vec::new();
                        for node in component.iter() {
                            shapes.push(node.clone());
                        }
                        let target = edge.target();
                        neg_cycle.push((node.clone(), target, shapes));
                        break;
                    }
                }
            }
            if !neg_cycle.is_empty() {
                result.push(neg_cycle);
            }
        }
        result
    }
}

impl Display for DependencyGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            Dot::with_config(&self.graph, &[Config::GraphContentOnly])
        )
    }
}

#[derive(Debug, Default)]
pub(crate) enum PosNeg {
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
}

#[cfg(test)]
mod tests {

    /*#[test]
    fn test_neg_cycle_no() {
        use super::*;

        let mut graph = DependencyGraph::new();
        graph.add_pos_edge(ShapeLabelIdx::from(0), ShapeLabelIdx::from(1));
        graph.add_pos_edge(ShapeLabelIdx::from(1), ShapeLabelIdx::from(0));
        println!("Graph: {}", graph);
        let neg_cycles = graph.neg_cycles();
        println!("Neg cycles: {:?}", neg_cycles);
        assert!(neg_cycles.is_empty());
    } */

    #[test]
    fn test_neg_cycle_yes() {
        use super::*;

        let mut graph = DependencyGraph::new();
        graph.add_pos_edge(ShapeLabelIdx::from(0), ShapeLabelIdx::from(1));
        graph.add_neg_edge(ShapeLabelIdx::from(1), ShapeLabelIdx::from(0));
        println!("Graph: {}", graph);
        let neg_cycles = graph.neg_cycles();
        println!("Neg cycles: {:?}", neg_cycles);
        assert!(neg_cycles.is_empty());
    }
}
