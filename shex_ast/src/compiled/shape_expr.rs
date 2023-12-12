use iri_s::IriS;
use rbe::RbeTable;

use crate::{Cond, Pred, Node, ShapeLabelIdx};

use super::annotation::Annotation;
use super::node_kind::NodeKind;
use super::sem_act::SemAct;
use super::xs_facet::XsFacet;
use super::value_set_value::ValueSetValue;
use super::shape::Shape;


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ShapeExpr {
    ShapeOr {
        exprs: Vec<ShapeExpr>,
    },
    ShapeAnd {
        exprs: Vec<ShapeExpr>,
    },
    ShapeNot {
        expr: Box<ShapeExpr>,
    },
    NodeConstraint {
        node_kind: Option<NodeKind>,
        datatype: Option<IriS>,
        xs_facet: Option<Vec<XsFacet>>,
        values: Option<Vec<ValueSetValue>>,
        cond: Cond,
    },
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
