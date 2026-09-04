use crate::ir::ShapeLabelIdx;
use crate::ir::dg::DependencyGraph;
use petgraph::Outgoing;
use petgraph::algo::tarjan_scc;
use petgraph::prelude::EdgeRef;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

/// How a shape participates in recursion (a cyclic shape reference), if at all.
///
/// A shape is *recursive* when it can reach itself again through the
/// dependency graph (`sh:node`, `sh:property`, `sh:and`, `sh:not`, ...). A
/// negating constraint (`sh:not`, `sh:xone`, ...) that only ever points
/// outside of any cycle can be resolved before the recursive part of the
/// schema is evaluated, exactly like stratified negation in Datalog: the
/// negated shape's answer never depends on the very fixpoint being
/// computed, so it can be settled up front and reused. A negating
/// constraint that instead reaches back into a cycle (its own, or another
/// recursive shape's) has no such safe order and is rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapeRecursionKind {
    /// Not part of any cyclic shape reference.
    NonRecursive,
    /// Part of a cycle built only from monotonic constraints — no negation
    /// is involved in getting back to itself. Supported under both
    /// `cautious` (least fixed point) and `brave` (greatest fixed point)
    /// recursion semantics.
    Positive,
    /// Part of a cycle that also involves a negating constraint, but every
    /// such constraint targets a shape that is itself free of recursion
    /// (directly or transitively) — the classical stratification
    /// condition. Supported under both `cautious` and `brave`.
    Stratified,
    /// Part of a cycle where a negating constraint reaches back into a
    /// cycle — its own, or a different recursive shape's. There is no
    /// well-defined order to evaluate the negation in, so a schema
    /// containing this is rejected regardless of recursion semantics.
    NonStratified,
}

impl Display for ShapeRecursionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ShapeRecursionKind::NonRecursive => "not recursive",
            ShapeRecursionKind::Positive => "positive recursive",
            ShapeRecursionKind::Stratified => "stratified recursive",
            ShapeRecursionKind::NonStratified => "non-stratified recursive",
        };
        write!(f, "{s}")
    }
}

impl DependencyGraph {
    /// Classifies every shape in the graph by how it participates in
    /// recursion. See [`ShapeRecursionKind`].
    ///
    /// The classification is computed per strongly connected component
    /// (SCC) of the dependency graph, since all shapes in the same SCC are
    /// mutually recursive and therefore share a classification:
    ///
    /// 1. An SCC is *recursive* if it has more than one member, or its
    ///    single member has a direct self-loop.
    /// 2. An SCC *depends on recursion* if it is itself recursive, or any
    ///    SCC it reaches (through the condensation graph, which is always
    ///    a DAG) does.
    /// 3. A recursive SCC is [`ShapeRecursionKind::NonStratified`] if it has
    ///    a negative edge to another member of the same SCC (negation
    ///    embedded directly in the cycle), or a negative edge to a
    ///    different SCC that depends on recursion (negation reaching into
    ///    another cycle).
    /// 4. Otherwise, a recursive SCC is [`ShapeRecursionKind::Stratified`]
    ///    if it has at least one negative edge (necessarily safe, by the
    ///    previous rule), or [`ShapeRecursionKind::Positive`] if it has
    ///    none.
    pub fn shape_recursion_kinds(&self) -> HashMap<ShapeLabelIdx, ShapeRecursionKind> {
        let sccs = tarjan_scc(&self.graph);

        let mut scc_of: HashMap<ShapeLabelIdx, usize> = HashMap::new();
        for (i, component) in sccs.iter().enumerate() {
            for &node in component {
                scc_of.insert(node, i);
            }
        }

        let mut recursive_scc = vec![false; sccs.len()];
        for (i, component) in sccs.iter().enumerate() {
            recursive_scc[i] =
                component.len() > 1 || (component.len() == 1 && self.graph.contains_edge(component[0], component[0]));
        }

        let mut scc_successors: Vec<HashSet<usize>> = vec![HashSet::new(); sccs.len()];
        let mut internal_negative = vec![false; sccs.len()];
        let mut has_negative_edge = vec![false; sccs.len()];
        let mut external_negative_edges: Vec<(usize, usize)> = Vec::new();

        for node in self.graph.nodes() {
            let from_scc = scc_of[&node];
            for edge in self.graph.edges_directed(node, Outgoing) {
                let to_scc = scc_of[&edge.target()];
                if to_scc != from_scc {
                    scc_successors[from_scc].insert(to_scc);
                }
                if !edge.weight().value() {
                    has_negative_edge[from_scc] = true;
                    if to_scc == from_scc {
                        internal_negative[from_scc] = true;
                    } else {
                        external_negative_edges.push((from_scc, to_scc));
                    }
                }
            }
        }

        // The condensation of a graph's SCCs is always a DAG, so this
        // memoized recursion always terminates.
        fn depends_on_recursive(
            i: usize,
            recursive_scc: &[bool],
            successors: &[HashSet<usize>],
            memo: &mut [Option<bool>],
        ) -> bool {
            if let Some(v) = memo[i] {
                return v;
            }
            let result = recursive_scc[i]
                || successors[i]
                    .iter()
                    .any(|&succ| depends_on_recursive(succ, recursive_scc, successors, memo));
            memo[i] = Some(result);
            result
        }

        let mut memo: Vec<Option<bool>> = vec![None; sccs.len()];
        let depends_on_recursive: Vec<bool> = (0..sccs.len())
            .map(|i| depends_on_recursive(i, &recursive_scc, &scc_successors, &mut memo))
            .collect();

        let mut has_unsafe_negative = internal_negative.clone();
        for (from, to) in external_negative_edges {
            if depends_on_recursive[to] {
                has_unsafe_negative[from] = true;
            }
        }

        let mut kind_of_scc = Vec::with_capacity(sccs.len());
        for i in 0..sccs.len() {
            kind_of_scc.push(if !recursive_scc[i] {
                ShapeRecursionKind::NonRecursive
            } else if has_unsafe_negative[i] {
                ShapeRecursionKind::NonStratified
            } else if has_negative_edge[i] {
                ShapeRecursionKind::Stratified
            } else {
                ShapeRecursionKind::Positive
            });
        }

        let mut result = HashMap::new();
        for (i, component) in sccs.iter().enumerate() {
            for &node in component {
                result.insert(node, kind_of_scc[i]);
            }
        }
        result
    }

    /// Whether every recursive shape in the graph is safe to validate under
    /// the current cautious/brave semantics — i.e. none of them is
    /// [`ShapeRecursionKind::NonStratified`].
    pub fn is_stratified(&self) -> bool {
        !self
            .shape_recursion_kinds()
            .values()
            .any(|kind| *kind == ShapeRecursionKind::NonStratified)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::dg::PosNeg;

    fn idx(n: usize) -> ShapeLabelIdx {
        ShapeLabelIdx::new(n)
    }

    #[test]
    fn non_recursive_shape_is_classified_as_such() {
        let mut dg = DependencyGraph::new();
        dg.add_edge(idx(0), idx(1), PosNeg::Pos);
        let kinds = dg.shape_recursion_kinds();
        assert_eq!(kinds[&idx(0)], ShapeRecursionKind::NonRecursive);
        assert_eq!(kinds[&idx(1)], ShapeRecursionKind::NonRecursive);
        assert!(dg.is_stratified());
    }

    #[test]
    fn self_loop_is_recursive() {
        let mut dg = DependencyGraph::new();
        dg.add_edge(idx(0), idx(0), PosNeg::Pos);
        assert_eq!(dg.shape_recursion_kinds()[&idx(0)], ShapeRecursionKind::Positive);
    }

    #[test]
    fn purely_positive_cycle_is_positive() {
        let mut dg = DependencyGraph::new();
        dg.add_edge(idx(0), idx(1), PosNeg::Pos);
        dg.add_edge(idx(1), idx(0), PosNeg::Pos);
        let kinds = dg.shape_recursion_kinds();
        assert_eq!(kinds[&idx(0)], ShapeRecursionKind::Positive);
        assert_eq!(kinds[&idx(1)], ShapeRecursionKind::Positive);
        assert!(dg.is_stratified());
    }

    #[test]
    fn negation_of_a_non_recursive_shape_is_stratified() {
        let mut dg = DependencyGraph::new();
        // 0 <-> 1 form a positive cycle; 0 also negates 2, which is outside
        // any cycle and doesn't itself depend on anything recursive.
        dg.add_edge(idx(0), idx(1), PosNeg::Pos);
        dg.add_edge(idx(1), idx(0), PosNeg::Pos);
        dg.add_edge(idx(0), idx(2), PosNeg::Neg);
        let kinds = dg.shape_recursion_kinds();
        assert_eq!(kinds[&idx(0)], ShapeRecursionKind::Stratified);
        assert_eq!(kinds[&idx(1)], ShapeRecursionKind::Stratified);
        assert_eq!(kinds[&idx(2)], ShapeRecursionKind::NonRecursive);
        assert!(dg.is_stratified());
    }

    #[test]
    fn negation_embedded_in_the_cycle_itself_is_non_stratified() {
        let mut dg = DependencyGraph::new();
        dg.add_edge(idx(0), idx(1), PosNeg::Pos);
        dg.add_edge(idx(1), idx(0), PosNeg::Neg);
        let kinds = dg.shape_recursion_kinds();
        assert_eq!(kinds[&idx(0)], ShapeRecursionKind::NonStratified);
        assert_eq!(kinds[&idx(1)], ShapeRecursionKind::NonStratified);
        assert!(!dg.is_stratified());
    }

    #[test]
    fn negation_of_a_different_recursive_shape_is_non_stratified() {
        let mut dg = DependencyGraph::new();
        // 0 <-> 1 is one cycle; 2 <-> 3 is another. 0 negates 2, which is
        // itself recursive, so the negated set isn't settled up front.
        dg.add_edge(idx(0), idx(1), PosNeg::Pos);
        dg.add_edge(idx(1), idx(0), PosNeg::Pos);
        dg.add_edge(idx(2), idx(3), PosNeg::Pos);
        dg.add_edge(idx(3), idx(2), PosNeg::Pos);
        dg.add_edge(idx(0), idx(2), PosNeg::Neg);
        let kinds = dg.shape_recursion_kinds();
        assert_eq!(kinds[&idx(0)], ShapeRecursionKind::NonStratified);
        assert_eq!(kinds[&idx(1)], ShapeRecursionKind::NonStratified);
        assert_eq!(kinds[&idx(2)], ShapeRecursionKind::Positive);
        assert_eq!(kinds[&idx(3)], ShapeRecursionKind::Positive);
        assert!(!dg.is_stratified());
    }

    #[test]
    fn negation_of_a_shape_that_transitively_depends_on_recursion_is_non_stratified() {
        let mut dg = DependencyGraph::new();
        // 0 <-> 1 is a cycle. 0 negates 2, which is not itself recursive,
        // but 2 depends on 3, which is part of an unrelated 3 <-> 4 cycle
        // (with no edge from 2/3/4 back towards 0/1) — so 2's answer isn't
        // settled until the 3/4 fixpoint is, and negating it from inside
        // the 0/1 cycle is unsafe.
        dg.add_edge(idx(0), idx(1), PosNeg::Pos);
        dg.add_edge(idx(1), idx(0), PosNeg::Pos);
        dg.add_edge(idx(0), idx(2), PosNeg::Neg);
        dg.add_edge(idx(2), idx(3), PosNeg::Pos);
        dg.add_edge(idx(3), idx(4), PosNeg::Pos);
        dg.add_edge(idx(4), idx(3), PosNeg::Pos);
        let kinds = dg.shape_recursion_kinds();
        assert_eq!(kinds[&idx(0)], ShapeRecursionKind::NonStratified);
        assert_eq!(kinds[&idx(1)], ShapeRecursionKind::NonStratified);
        assert_eq!(kinds[&idx(2)], ShapeRecursionKind::NonRecursive);
        assert_eq!(kinds[&idx(3)], ShapeRecursionKind::Positive);
        assert!(!dg.is_stratified());
    }
}
