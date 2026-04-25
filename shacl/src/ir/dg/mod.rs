use crate::ir::ShapeLabelIdx;
pub(crate) use crate::ir::dg::iterator::DependencyGraphIter;
pub(crate) use crate::ir::dg::pos_neg::PosNeg;
use petgraph::algo::{is_cyclic_directed, tarjan_scc};
use petgraph::prelude::{EdgeRef, GraphMap};
use petgraph::{Directed, Outgoing};
use std::fmt::{Display, Formatter};

mod iterator;
mod pos_neg;

#[derive(Default, Debug, Clone)]
pub struct DependencyGraph {
    graph: GraphMap<ShapeLabelIdx, PosNeg, Directed>,
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
                    if component.contains(&edge.target()) && !edge.weight().value() {
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
        DependencyGraphIter::new(self.graph.all_edges())
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
