use crate::ir::ShapeLabelIdx;
pub(crate) use crate::ir::dg::iterator::DependencyGraphIter;
pub(crate) use crate::ir::dg::pos_neg::PosNeg;
use petgraph::algo::{is_cyclic_directed, tarjan_scc};
use petgraph::prelude::{EdgeRef, GraphMap};
use petgraph::{Directed, Outgoing};
use std::collections::HashMap;
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

    /// Returns shape indices grouped into topological levels such that dependencies
    /// appear at lower levels than the shapes that depend on them.
    ///
    /// Level 0 contains shapes that do not depend on any other shape.
    /// Level N contains shapes whose dependencies are all in levels 0..N-1.
    ///
    /// Shapes involved in cycles are not reachable via Kahn's algorithm and are
    /// therefore omitted from the result (the caller should handle them separately,
    /// e.g. by falling back to sequential validation).
    pub fn topological_levels(&self) -> Vec<Vec<ShapeLabelIdx>> {
        // Edge A -> B means "A depends on B".
        // We want to validate B before A, so we assign B a lower level.
        //
        // Algorithm: Kahn's on the dependency graph using out-degree.
        //   - remaining_deps[A] = number of shapes A still depends on
        //   - dependents_of[B]  = shapes that depend on B
        //
        // Level 0: shapes with remaining_deps == 0 (depend on nothing).
        // When a shape is added to a level, decrement remaining_deps of every
        // shape that depends on it; newly-zero shapes enter the next level.

        let mut remaining_deps: HashMap<ShapeLabelIdx, usize> = HashMap::new();
        let mut dependents_of: HashMap<ShapeLabelIdx, Vec<ShapeLabelIdx>> = HashMap::new();

        for node in self.graph.nodes() {
            remaining_deps.entry(node).or_insert(0);
        }

        for node in self.graph.nodes() {
            for edge in self.graph.edges_directed(node, Outgoing) {
                let to = edge.target();
                *remaining_deps.get_mut(&node).unwrap() += 1;
                dependents_of.entry(to).or_default().push(node);
            }
        }

        let mut levels: Vec<Vec<ShapeLabelIdx>> = Vec::new();
        let mut current: Vec<ShapeLabelIdx> = remaining_deps
            .iter()
            .filter(|&(_, count)| *count == 0)
            .map(|(&node, _)| node)
            .collect();
        current.sort_unstable();

        while !current.is_empty() {
            levels.push(current.clone());
            let mut next: Vec<ShapeLabelIdx> = Vec::new();
            for done in &current {
                for dependent in dependents_of.get(done).into_iter().flatten() {
                    let count = remaining_deps.get_mut(dependent).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        next.push(*dependent);
                    }
                }
            }
            next.sort_unstable();
            current = next;
        }

        levels
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
