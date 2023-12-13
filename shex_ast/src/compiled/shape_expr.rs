use std::fmt::Display;

use iri_s::IriS;
use rbe::RbeTable;

use crate::{Cond, Node, Pred, ShapeLabelIdx};

use super::annotation::Annotation;
use super::node_constraint::NodeConstraint;
use super::node_kind::NodeKind;
use super::sem_act::SemAct;
use super::shape::Shape;
use super::value_set_value::ValueSetValue;
use super::xs_facet::XsFacet;

#[derive(Debug, PartialEq, Clone)]
pub enum ShapeExpr {
    ShapeOr { exprs: Vec<ShapeExpr> },
    ShapeAnd { exprs: Vec<ShapeExpr> },
    ShapeNot { expr: Box<ShapeExpr> },
    NodeConstraint(NodeConstraint),
    Shape(Shape),
    External {},
    Ref { idx: ShapeLabelIdx },
    Empty,
}

impl ShapeExpr {
    pub fn mk_ref(idx: ShapeLabelIdx) -> ShapeExpr {
        ShapeExpr::Ref { idx }
    }
}

impl Display for ShapeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ShapeExpr: {self:?}")
    }
}
