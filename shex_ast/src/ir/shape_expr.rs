use super::{
    dependency_graph::{DependencyGraph, PosNeg},
    node_constraint::NodeConstraint,
    shape::Shape,
};
use crate::ShapeLabelIdx;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum ShapeExpr {
    ShapeOr {
        exprs: Vec<ShapeExpr>,
        display: String,
    },
    ShapeAnd {
        exprs: Vec<ShapeExpr>,
        display: String,
    },
    ShapeNot {
        expr: Box<ShapeExpr>,
        display: String,
    },
    NodeConstraint(NodeConstraint),
    Shape(Shape),
    External {},
    Ref {
        idx: ShapeLabelIdx,
    },
    Empty,
}

impl ShapeExpr {
    pub fn mk_ref(idx: ShapeLabelIdx) -> ShapeExpr {
        ShapeExpr::Ref { idx }
    }

    /// Adds PosNeg edges to the dependency graph.
    pub(crate) fn add_edges(
        &self,
        source: ShapeLabelIdx,
        graph: &mut DependencyGraph,
        pos_neg: PosNeg,
    ) {
        match self {
            ShapeExpr::ShapeOr { exprs, .. } => {
                for expr in exprs {
                    expr.add_edges(source, graph, pos_neg);
                }
            }
            ShapeExpr::ShapeAnd { exprs, .. } => {
                for expr in exprs {
                    expr.add_edges(source, graph, pos_neg);
                }
            }
            ShapeExpr::ShapeNot { expr, .. } => {
                expr.add_edges(source, graph, pos_neg.change());
            }
            ShapeExpr::NodeConstraint(_) => {}
            ShapeExpr::Shape(_) => {}
            ShapeExpr::External {} => {}
            ShapeExpr::Ref { idx } => {
                graph.add_edge(source, *idx, pos_neg);
            }
            ShapeExpr::Empty => {}
        }
    }
}

impl Display for ShapeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeExpr::ShapeOr { display, .. } => write!(f, "{display}"),
            ShapeExpr::ShapeAnd { display, .. } => write!(f, "{display}"),
            ShapeExpr::ShapeNot { display, .. } => write!(f, "{display}"),
            ShapeExpr::NodeConstraint(nc) => write!(f, "{nc}"),
            ShapeExpr::Shape(shape) => write!(f, "{shape}"),
            ShapeExpr::External {} => write!(f, "External"),
            ShapeExpr::Ref { .. } => todo!(),
            ShapeExpr::Empty => write!(f, "<Empty>"),
        }
    }
}
