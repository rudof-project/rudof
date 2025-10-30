use crate::shape_label_idx::ShapeLabelIdx;
use petgraph::visit::EdgeRef;
use petgraph::{Directed, prelude::GraphMap};
use std::fmt::Display;

#[derive(Debug, Default, Clone)]
pub struct DependencyGraph {
    graph: GraphMap<ShapeLabelIdx, PosNeg, Directed>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        DependencyGraph {
            graph: GraphMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: ShapeLabelIdx, to: ShapeLabelIdx, pos_neg: PosNeg) {
        self.graph.add_edge(from, to, pos_neg);
    }

    pub fn neg_cycles(&self) -> Vec<Vec<(ShapeLabelIdx, ShapeLabelIdx, Vec<ShapeLabelIdx>)>> {
        let mut result = Vec::new();
        let scc = petgraph::algo::tarjan_scc(&self.graph);
        for component in &scc {
            let mut neg_cycle = Vec::new();
            for node in component.iter().as_slice() {
                let edges = self
                    .graph
                    .edges_directed(*node, petgraph::Direction::Outgoing);
                for edge in edges {
                    if component.contains(&edge.target()) && edge.weight().is_neg() {
                        let mut shapes = Vec::new();
                        for node in component.iter() {
                            shapes.push(*node);
                        }
                        let target = edge.target();
                        neg_cycle.push((*node, target, shapes));
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

    pub fn has_neg_cycle(&self) -> bool {
        let neg_cycles = self.neg_cycles();
        !neg_cycles.is_empty()
    }

    pub fn all_edges(&self) -> DependencyGraphIter<'_> {
        DependencyGraphIter {
            inner: self.graph.all_edges(),
        }
    }
}

impl Display for DependencyGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Dependency Graph:")?;
        for (from, to, posneg) in self.all_edges() {
            writeln!(f, "  {} --{}--> {}", from, posneg, to)?;
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
