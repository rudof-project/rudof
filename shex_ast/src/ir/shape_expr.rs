use super::{
    dependency_graph::{DependencyGraph, PosNeg},
    node_constraint::NodeConstraint,
    shape::Shape,
};
use crate::{Pred, ShapeLabelIdx};
use std::{collections::HashMap, fmt::Display};

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

    pub fn direct_references(&self) -> Vec<ShapeLabelIdx> {
        match self {
            ShapeExpr::ShapeOr { exprs, .. } => exprs
                .iter()
                .flat_map(|expr| expr.direct_references())
                .collect(),
            ShapeExpr::ShapeAnd { exprs, .. } => exprs
                .iter()
                .flat_map(|expr| expr.direct_references())
                .collect(),
            ShapeExpr::ShapeNot { expr, .. } => expr.direct_references(),
            ShapeExpr::NodeConstraint(_) => vec![],
            ShapeExpr::Shape(_s) => vec![],
            ShapeExpr::External {} => vec![],
            ShapeExpr::Ref { idx } => vec![*idx],
            ShapeExpr::Empty => vec![],
        }
    }

    pub fn references(&self) -> HashMap<Pred, Vec<ShapeLabelIdx>> {
        match self {
            ShapeExpr::ShapeOr { exprs, .. } => {
                exprs.iter().fold(HashMap::new(), |mut acc, expr| {
                    let refs = expr.references();
                    for (pred, idxs) in refs {
                        acc.entry(pred).or_default().extend(idxs);
                    }
                    acc
                })
            }
            ShapeExpr::ShapeAnd { exprs, .. } => {
                exprs.iter().fold(HashMap::new(), |mut acc, expr| {
                    let refs = expr.references();
                    for (pred, idxs) in refs {
                        acc.entry(pred).or_default().extend(idxs);
                    }
                    acc
                })
            }
            ShapeExpr::ShapeNot { expr, .. } => expr.references(),
            ShapeExpr::NodeConstraint(_nc) => HashMap::new(),
            ShapeExpr::Shape(s) => s.references().clone(),
            ShapeExpr::External {} => HashMap::new(),
            ShapeExpr::Ref { idx } => {
                let mut map = HashMap::new();
                map.insert(Pred::default(), vec![*idx]);
                map
            }
            ShapeExpr::Empty => HashMap::new(),
        }
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
            ShapeExpr::Shape(s) => s.add_edges(source, graph, pos_neg),
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
            ShapeExpr::ShapeOr { exprs, .. } => write!(
                f,
                "{}",
                exprs
                    .iter()
                    .map(|e| format!("{e}"))
                    .collect::<Vec<_>>()
                    .join(" OR ")
            ),
            ShapeExpr::ShapeAnd { exprs, .. } => write!(
                f,
                "{}",
                exprs
                    .iter()
                    .map(|e| format!("{e}"))
                    .collect::<Vec<_>>()
                    .join(" AND ")
            ),
            ShapeExpr::ShapeNot { expr, .. } => write!(f, "NOT {expr}"),
            ShapeExpr::NodeConstraint(nc) => write!(f, "Node constraint: {nc}"),
            ShapeExpr::Shape(shape) => write!(f, "{shape}"),
            ShapeExpr::External {} => write!(f, "External"),
            ShapeExpr::Ref { idx } => write!(f, "@{idx}"),
            ShapeExpr::Empty => write!(f, "<Empty>"),
        }
    }
}
