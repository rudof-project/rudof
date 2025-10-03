use super::{
    dependency_graph::{DependencyGraph, PosNeg},
    node_constraint::NodeConstraint,
    shape::Shape,
};
use crate::{Pred, ShapeLabelIdx, ir::schema_ir::SchemaIR};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Clone)]
pub enum ShapeExpr {
    ShapeOr { exprs: Vec<ShapeLabelIdx> },
    ShapeAnd { exprs: Vec<ShapeLabelIdx> },
    ShapeNot { expr: ShapeLabelIdx },
    NodeConstraint(NodeConstraint),
    Shape(Box<Shape>),
    External {},
    Ref { idx: ShapeLabelIdx },
    Empty,
}

impl ShapeExpr {
    pub fn mk_ref(idx: ShapeLabelIdx) -> ShapeExpr {
        ShapeExpr::Ref { idx }
    }

    pub fn direct_references(&self) -> Vec<ShapeLabelIdx> {
        match self {
            ShapeExpr::ShapeOr { exprs, .. } => exprs.to_vec(),
            ShapeExpr::ShapeAnd { exprs, .. } => exprs.to_vec(),
            ShapeExpr::ShapeNot { expr, .. } => vec![*expr],
            ShapeExpr::NodeConstraint(_) => vec![],
            ShapeExpr::Shape(_s) => vec![],
            ShapeExpr::External {} => vec![],
            ShapeExpr::Ref { idx } => vec![*idx],
            ShapeExpr::Empty => vec![],
        }
    }

    pub fn references(&self, schema: &SchemaIR) -> HashMap<Pred, Vec<ShapeLabelIdx>> {
        match self {
            ShapeExpr::ShapeOr { exprs, .. } => {
                exprs.iter().fold(HashMap::new(), |mut acc, expr| {
                    let refs = schema
                        .find_shape_idx(expr)
                        .map(|(_, expr)| expr.references(schema))
                        .unwrap_or_default();
                    for (p, v) in refs {
                        acc.entry(p).or_insert_with(Vec::new).extend(v);
                    }
                    acc
                })
            }
            ShapeExpr::ShapeAnd { exprs, .. } => {
                exprs.iter().fold(HashMap::new(), |mut acc, expr| {
                    let refs = schema
                        .find_shape_idx(expr)
                        .map(|(_, expr)| expr.references(schema))
                        .unwrap_or_default();
                    for (p, v) in refs {
                        acc.entry(p).or_insert_with(Vec::new).extend(v);
                    }
                    acc
                })
            }
            ShapeExpr::ShapeNot { expr, .. } => schema
                .find_shape_idx(expr)
                .map(|(_, expr)| expr.references(schema))
                .unwrap_or_default(),
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
    pub(crate) fn add_edges<'a>(
        &'a self,
        source: ShapeLabelIdx,
        graph: &mut DependencyGraph,
        pos_neg: PosNeg,
        schema: &'a SchemaIR,
        visited: &mut Vec<&'a ShapeExpr>,
    ) {
        match self {
            ShapeExpr::ShapeOr { exprs, .. } => {
                for expr in exprs {
                    if let Some((_, expr)) = schema.find_shape_idx(expr) {
                        if visited.contains(&expr) {
                            continue;
                        } else {
                            visited.push(expr);
                            expr.add_edges(source, graph, pos_neg, schema, visited);
                        }
                    }
                }
            }
            ShapeExpr::ShapeAnd { exprs, .. } => {
                for expr in exprs {
                    if let Some((_, expr)) = schema.find_shape_idx(expr) {
                        if visited.contains(&expr) {
                            continue;
                        } else {
                            visited.push(expr);
                            expr.add_edges(source, graph, pos_neg, schema, visited);
                        }
                    }
                }
            }
            ShapeExpr::ShapeNot { expr, .. } => {
                if let Some((_, expr)) = schema.find_shape_idx(expr) {
                    if visited.contains(&expr) {
                    } else {
                        visited.push(expr);
                        expr.add_edges(source, graph, pos_neg.change(), schema, visited);
                    }
                }
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
                "OR({})",
                exprs
                    .iter()
                    .map(|e| format!("{e}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            ShapeExpr::ShapeAnd { exprs, .. } => write!(
                f,
                "AND({})",
                exprs
                    .iter()
                    .map(|e| format!("{e}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            ShapeExpr::ShapeNot { expr, .. } => write!(f, "NOT {expr}"),
            ShapeExpr::NodeConstraint(nc) => write!(f, "{nc}"),
            ShapeExpr::Shape(shape) => write!(f, "{shape}"),
            ShapeExpr::External {} => write!(f, "External"),
            ShapeExpr::Ref { idx } => write!(f, "@{idx}"),
            ShapeExpr::Empty => write!(f, "<Empty>"),
        }
    }
}
