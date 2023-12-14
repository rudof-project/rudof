use super::{node_constraint::NodeConstraint, shape::Shape};
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
}

impl Display for ShapeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeExpr::ShapeOr { display, .. } => write!(f, "{display}"),
            ShapeExpr::ShapeAnd { exprs, display } => write!(f, "{display}"),
            ShapeExpr::ShapeNot { display, .. } => write!(f, "{display}"),
            ShapeExpr::NodeConstraint(nc) => write!(f, "{nc}"),
            ShapeExpr::Shape(shape) => write!(f, "{shape}"),
            ShapeExpr::External {} => write!(f, "External"),
            ShapeExpr::Ref { idx } => todo!(),
            ShapeExpr::Empty => write!(f, "<Empty>"),
        }
    }
}
