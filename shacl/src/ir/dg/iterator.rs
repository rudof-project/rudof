use petgraph::Directed;
use petgraph::graphmap::AllEdges;
use crate::ir::dg::pos_neg::PosNeg;
use crate::ir::ShapeLabelIdx;

/// Iterator over the edges of the dependency graph
/// The iterator yields tuples of the form (from, posneg, to)
pub struct DependencyGraphIter<'a> {
    inner: AllEdges<'a, ShapeLabelIdx, PosNeg, Directed>,
}

impl<'a> DependencyGraphIter<'a> {
    pub fn new(edges: AllEdges<'a, ShapeLabelIdx, PosNeg, Directed>) -> Self {
        Self { inner: edges }
    }
}

impl Iterator for DependencyGraphIter<'_> {
    type Item = (ShapeLabelIdx, PosNeg, ShapeLabelIdx);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(from, to, posneg)| (from, *posneg, to))
    }
}