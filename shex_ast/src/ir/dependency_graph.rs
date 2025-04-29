use petgraph::graph::Graph;

use crate::ShapeLabelIdx;

#[derive(Debug, Default)]
pub(crate) struct DependencyGraph {
    graph: Graph<ShapeLabelIdx, PosNeg>,
}

#[derive(Debug)]
pub(crate) enum PosNeg {
    Pos,
    Neg,
}
