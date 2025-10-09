use crate::ValidatorErrors;
use serde::{Serialize, ser::SerializeMap};
use shex_ast::{
    Node, ShapeLabelIdx,
    ir::{node_constraint::NodeConstraint, shape::Shape, shape_expr::ShapeExpr},
};
use std::fmt::Display;

/// Reason represents justifications about why a node conforms to some shape
#[derive(Debug, Clone)]
pub enum Reason {
    ShapeExtendsPassed {
        node: Node,
        shape: Shape,
        reasons: Reasons,
    },
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
        shape_expr: ShapeLabelIdx,
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
            Reason::ShapeExtendsPassed {
                node,
                shape,
                reasons,
            } => write!(
                f,
                "Shape extends passed. Node {node}, shape: {shape}, reasons: {reasons}"
            ),
        }
    }
}

impl Reason {
    pub fn as_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
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
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("reason", &self.to_string())?;
        map.end()
    }
}
