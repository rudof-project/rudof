use crate::ir::shape_label_idx::ShapeLabelIdx;
use petgraph::algo::{is_cyclic_directed, tarjan_scc};
use petgraph::graphmap::AllEdges;
use petgraph::prelude::GraphMap;
use petgraph::visit::EdgeRef;
use petgraph::{Directed, Outgoing};
use std::fmt::{Display, Formatter};

#[derive(Default, Debug, Clone)]
pub(crate) struct DependencyGraph {
    graph: GraphMap<ShapeLabelIdx, PosNeg, Directed>
}

/// Iterator over the edges of the dependency graph
/// The iterator yields tuples of the form (from, posneg, to)
pub(crate) struct DependencyGraphIter<'a> {
    inner: AllEdges<'a, ShapeLabelIdx, PosNeg, Directed>
}

impl Iterator for DependencyGraphIter<'_> {
    type Item = (ShapeLabelIdx, PosNeg, ShapeLabelIdx);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(from, to, posneg)| (from, *posneg, to))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PosNeg {
    #[default]
    Pos,
    Neg
}

impl DependencyGraph {
    pub fn new() -> Self {
        DependencyGraph { graph: GraphMap::new() }
    }

    pub fn neg_cycles(&self) -> Vec<Vec<(ShapeLabelIdx, ShapeLabelIdx, Vec<ShapeLabelIdx>)>> {
        let mut result = Vec::new();
        let scc = tarjan_scc(&self.graph);
        for component in scc {
            let mut neg_cycle = Vec::new();
            for node in component.iter().as_slice() {
                let edges = self.graph.edges_directed(*node, Outgoing);
                for edge in edges {
                    if component.contains(&edge.target()) && edge.weight().value() == false {
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

    pub fn add_edge(&mut self, from: ShapeLabelIdx, to: ShapeLabelIdx, pos_neg: PosNeg) {
        self.graph.add_edge(from, to, pos_neg);
    }

    /// Check if the dependency graph has any cycle (including positive cycles).
    pub fn has_cycles(&self) -> bool {
        is_cyclic_directed(&self.graph)
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

impl PosNeg {
    pub fn value(&self) -> bool {
        match self {
            PosNeg::Pos => true,
            PosNeg::Neg => false,
        }
    }

    pub fn change(&self) -> Self {
        match self {
            PosNeg::Pos => PosNeg::Neg,
            PosNeg::Neg => PosNeg::Pos,
        }
    }
}

impl Display for DependencyGraph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Dependency Graph:")?;
        for (from, posneg, to) in self.all_edges() {
            writeln!(f, "  {} --{}--> {}", from, posneg, to)?;
        }
        Ok(())
    }
}

impl Display for PosNeg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PosNeg::Pos => write!(f, "[+]"),
            PosNeg::Neg => write!(f, "[-]"),
        }
    }
}