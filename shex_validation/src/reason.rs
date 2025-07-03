use std::fmt::Display;

use serde::Serialize;
use shex_ast::{
    ir::{node_constraint::NodeConstraint, shape::Shape, shape_expr::ShapeExpr},
    Node, ShapeLabelIdx,
};

use crate::ValidatorErrors;

/// Reason represents justifications about why a node conforms to some shape
#[derive(Debug, Clone)]
pub enum Reason {
    NodeConstraintPassed {
        node: Node,
        nc: NodeConstraint,
    },
    ShapeAndPassed {
        node: Node,
        se: ShapeExpr,
        reasons: Vec<Vec<Reason>>,
    },
    EmptyPassed {
        node: Node,
    },
    ExternalPassed {
        node: Node,
    },
    ShapeOrPassed {
        node: Node,
        shape_expr: ShapeExpr,
        reasons: Reasons,
    },
    ShapeNotPassed {
        node: Node,
        shape_expr: ShapeExpr,

        // Errors that are evidences that the negation passess
        errors_evidences: ValidatorErrors,
    },
    ShapePassed {
        node: Node,
        shape: Box<Shape>,
    },
    ShapeRefPassed {
        node: Node,
        idx: ShapeLabelIdx,
    },
}

impl Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reason::NodeConstraintPassed { node, nc } => {
                write!(f, "Node constraint passed. Node: {node}, Constraint: {nc}",)
            }
            Reason::ShapeAndPassed { node, se, reasons } => {
                write!(f, "AND passed. Node {node}, and: {se}, reasons:")?;
                for reason in reasons {
                    write!(f, "[")?;
                    for r in reason {
                        write!(f, "{r}, ")?;
                    }
                    write!(f, "], ")?;
                }
                Ok(())
            }
            Reason::ShapePassed { node, shape } => {
                write!(f, "Shape passed. Node {node}, shape: {shape}")
            }
            Reason::ShapeOrPassed {
                node,
                shape_expr,
                reasons,
            } => write!(
                f,
                "Shape OR passed. Node {node}, shape: {shape_expr}, reasons: {reasons}"
            ),
            Reason::ShapeNotPassed {
                node,
                shape_expr,
                errors_evidences,
            } => write!(
                f,
                "Shape NOT passed. Node {node}, shape: {shape_expr}, errors: {errors_evidences}"
            ),
            Reason::ExternalPassed { node } => write!(f, "Shape External passed for node {node}"),
            Reason::EmptyPassed { node } => write!(f, "Shape External passed for node {node}"),
            Reason::ShapeRefPassed { node, idx } => {
                write!(f, "ShapeRef passed. Node {node}, idx: {idx}")
            }
        }
    }
}

impl Reason {
    pub fn as_json(&self) -> serde_json::Value {
        match self {
            Reason::NodeConstraintPassed { node, nc } => {
                serde_json::json!({
                    "type": "NodeConstraintPassed",
                    "node": node.to_string(),
                    "constraint": nc.to_string()
                })
            }
            Reason::ShapeAndPassed { node, se, reasons } => {
                serde_json::json!({
                    "type": "ShapeAndPassed",
                    "node": node.to_string(),
                    "shape_expr": se.to_string(),
                    "reasons": reasons.iter().map(|r| {
                        r.iter().map(|reason| reason.as_json()).collect::<Vec<_>>()
                    }).collect::<Vec<_>>()
                })
            }
            Reason::ShapePassed { node, shape } => {
                serde_json::json!({
                    "type": "ShapePassed",
                    "node": node.to_string(),
                    "shape": shape.to_string()
                })
            }
            Reason::ShapeOrPassed {
                node,
                shape_expr,
                reasons: _,
            } => {
                serde_json::json!({
                        "type": "ShapeOrPassed",
                        "node": node.to_string(),
                        "shape_expr": shape_expr.to_string(),
                        /*"reasons": reasons.iter().map(|reason| {
                    reason.as_json()
                }).collect::<Vec<_>>()*/
                    })
            }
            Reason::ShapeNotPassed {
                node,
                shape_expr,
                errors_evidences: _,
            } => {
                serde_json::json!({
                        "type": "ShapeNotPassed",
                        "node": node.to_string(),
                        "shape_expr": shape_expr.to_string(),
                        /*"errors_evidences": errors_evidences.iter().map(|reason| {
                    reason.as_json()
                }).collect::<Vec<_>>() */
                    })
            }
            Reason::ExternalPassed { node } => serde_json::json!({
                "type": "ExternalPassed",
                "node": node.to_string()
            }),
            Reason::EmptyPassed { node } => {
                serde_json::json!({
                    "type": "EmptyPassed",
                    "node": node.to_string()
                })
            }
            Reason::ShapeRefPassed { node, idx } => {
                serde_json::json!({
                    "type": "ShapeRefPassed",
                    "node": node.to_string(),
                    "idx": idx.to_string()
                })
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Reasons {
    reasons: Vec<Reason>,
}

impl Reasons {
    pub fn new(reasons: Vec<Reason>) -> Reasons {
        Reasons { reasons }
    }
}

impl Display for Reasons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for reason in self.reasons.iter() {
            writeln!(f, "  {reason}")?;
        }
        Ok(())
    }
}

impl Serialize for Reason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(format!("{}", self).as_str())
    }
}
