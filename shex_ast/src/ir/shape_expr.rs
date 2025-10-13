use super::{
    dependency_graph::{DependencyGraph, PosNeg},
    node_constraint::NodeConstraint,
    shape::Shape,
};
use crate::{Expr, Pred, ShapeLabelIdx, ir::schema_ir::SchemaIR};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    vec,
};

#[derive(Debug, PartialEq, Clone, Default)]
pub enum ShapeExpr {
    ShapeOr {
        exprs: Vec<ShapeLabelIdx>,
    },
    ShapeAnd {
        exprs: Vec<ShapeLabelIdx>,
    },
    ShapeNot {
        expr: ShapeLabelIdx,
    },
    NodeConstraint(NodeConstraint),
    Shape(Box<Shape>),
    External {},
    Ref {
        idx: ShapeLabelIdx,
    },

    #[default]
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

    pub fn get_triple_exprs(&self, schema: &SchemaIR) -> Vec<Expr> {
        match self {
            ShapeExpr::ShapeOr { .. } => Vec::new(), // Should be an error? extending from OR
            ShapeExpr::ShapeAnd { exprs, .. } => exprs
                .iter()
                .flat_map(|e| {
                    let info = schema.find_shape_idx(e).unwrap();
                    info.expr().get_triple_exprs(schema)
                })
                .collect(),
            ShapeExpr::ShapeNot { .. } => Vec::new(), /*schema
            .find_shape_idx(expr)
            .map(|info| info.expr().get_triple_exprs(schema))
            .unwrap_or_default() */
            ShapeExpr::NodeConstraint(_nc) => vec![],
            ShapeExpr::Shape(s) => vec![s.triple_expr().clone()],
            ShapeExpr::External {} => vec![],
            ShapeExpr::Ref { idx } => {
                let info = schema.find_shape_idx(idx).unwrap();
                info.expr().get_triple_exprs(schema)
            }
            ShapeExpr::Empty => vec![],
        }
    }

    pub fn references(&self, schema: &SchemaIR) -> HashMap<Pred, Vec<ShapeLabelIdx>> {
        match self {
            ShapeExpr::ShapeOr { exprs, .. } => {
                exprs.iter().fold(HashMap::new(), |mut acc, expr| {
                    let refs = schema
                        .find_shape_idx(expr)
                        .map(|info| info.expr().references(schema))
                        .unwrap_or_default();
                    for (p, v) in refs {
                        acc.entry(p).or_default().extend(v);
                    }
                    acc
                })
            }
            ShapeExpr::ShapeAnd { exprs, .. } => {
                exprs.iter().fold(HashMap::new(), |mut acc, expr| {
                    let refs = schema
                        .find_shape_idx(expr)
                        .map(|info| info.expr().references(schema))
                        .unwrap_or_default();
                    for (p, v) in refs {
                        acc.entry(p).or_default().extend(v);
                    }
                    acc
                })
            }
            ShapeExpr::ShapeNot { expr, .. } => schema
                .find_shape_idx(expr)
                .map(|info| info.expr().references(schema))
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

    /// Get the predicates used in this shape expression.
    /// For ShapeOr, ShapeAnd and ShapeNot, it will recursively get the predicates from
    /// the referenced shapes.
    /// For Shape, it will get the predicates from the shape.
    /// For NodeConstraint and External, it will return an empty set.
    /// For Ref, it will get the predicates from the referenced shape.
    /// For Empty, it will return an empty set.
    /// This will used the direct predicates of the shape, not the ones from extends.
    /// To get the predicates including extends, use the method preds_extends of `SchemaIR`
    pub fn preds(&self, schema: &SchemaIR) -> HashSet<Pred> {
        let mut visited = HashSet::new();
        self.preds_visited(schema, &mut visited)
    }

    fn preds_visited(
        &self,
        schema: &SchemaIR,
        visited: &mut HashSet<ShapeLabelIdx>,
    ) -> HashSet<Pred> {
        let mut preds = HashSet::new();
        match self {
            ShapeExpr::ShapeOr { exprs, .. } => {
                for e in exprs {
                    let info = schema.find_shape_idx(e).unwrap();
                    visited.insert(*e);
                    let expr = info.expr();
                    preds.extend(expr.preds_visited(schema, visited));
                }
            }
            ShapeExpr::ShapeAnd { exprs, .. } => {
                for e in exprs {
                    let info = schema.find_shape_idx(e).unwrap();
                    visited.insert(*e);
                    let expr = info.expr();
                    preds.extend(expr.preds_visited(schema, visited));
                }
            }
            ShapeExpr::ShapeNot { expr, .. } => {
                let info = schema.find_shape_idx(expr).unwrap();
                visited.insert(*expr);
                let expr = info.expr();
                preds.extend(expr.preds_visited(schema, visited));
            }
            ShapeExpr::NodeConstraint(_nc) => {}
            ShapeExpr::Shape(s) => {
                preds.extend(s.preds());
            }
            ShapeExpr::External {} => {}
            ShapeExpr::Ref { idx } => {
                if visited.contains(idx) {
                    return preds;
                }
                visited.insert(*idx);
                let info = schema.find_shape_idx(idx).unwrap();
                let expr = info.expr();
                preds.extend(expr.preds(schema));
            }
            ShapeExpr::Empty => {}
        }
        preds
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
                    if let Some(info) = schema.find_shape_idx(expr) {
                        let expr = info.expr();
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
                    if let Some(info) = schema.find_shape_idx(expr) {
                        let expr = info.expr();
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
                if let Some(info) = schema.find_shape_idx(expr) {
                    let expr = info.expr();
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

    // TODO: Improve the visualization of shape expressions
    pub fn show_qualified(
        &self,
        _prefixmap: &prefixmap::PrefixMap,
    ) -> Result<String, prefixmap::PrefixMapError> {
        match self {
            ShapeExpr::ShapeOr { exprs, .. } => Ok(format!(
                "OR({})",
                exprs
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            ShapeExpr::ShapeAnd { exprs, .. } => Ok(format!(
                "AND({})",
                exprs
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            ShapeExpr::ShapeNot { expr, .. } => Ok(format!("NOT {}", expr.to_string())),
            ShapeExpr::NodeConstraint(nc) => Ok(nc.to_string()),
            ShapeExpr::Shape(shape) => Ok(shape.to_string()),
            ShapeExpr::External {} => Ok("External".to_string()),
            ShapeExpr::Ref { idx } => Ok(format!("@{}", idx)),
            ShapeExpr::Empty => Ok("{}".to_string()),
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
