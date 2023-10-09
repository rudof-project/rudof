use std::result;
use std::str::FromStr;

use serde::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use void::Void;

use super::serde_string_or_struct::SerializeStringOrStruct;
use super::{
    annotation::Annotation, iri_ref::IriRef, sem_act::SemAct, triple_expr::TripleExprWrapper,
    value_set_value::ValueSetValueWrapper, xs_facet::XsFacet,
};
use super::{node_kind::NodeKind, ref_::Ref};
use crate::ast::serde_string_or_struct::*;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(transparent)]
pub struct ShapeExprWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    pub se: ShapeExpr,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum ShapeExpr {
    ShapeOr {
        #[serde(rename = "shapeExprs")]
        shape_exprs: Vec<ShapeExprWrapper>,
    },
    ShapeAnd {
        #[serde(rename = "shapeExprs")]
        shape_exprs: Vec<ShapeExprWrapper>,
    },
    ShapeNot {
        #[serde(rename = "shapeExpr")]
        shape_expr: Box<ShapeExprWrapper>,
    },
    NodeConstraint {
        #[serde(default, rename = "nodeKind", skip_serializing_if = "Option::is_none")]
        node_kind: Option<NodeKind>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        datatype: Option<IriRef>,

        #[serde(default, rename = "xsFacet", skip_serializing_if = "Option::is_none")]
        xs_facet: Option<Vec<XsFacet>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        values: Option<Vec<ValueSetValueWrapper>>,
    },
    Shape {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        closed: Option<bool>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        extra: Option<Vec<IriRef>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        expression: Option<TripleExprWrapper>,

        #[serde(default, rename = "semActs", skip_serializing_if = "Option::is_none")]
        sem_acts: Option<Vec<SemAct>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Vec<Annotation>>,
    },

    ShapeExternal,

    Ref(Ref),
}

impl FromStr for ShapeExpr {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ShapeExpr::Ref(Ref::IriRef {
            value: s.to_string(),
        }))
    }
}

impl SerializeStringOrStruct for ShapeExpr {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            ShapeExpr::Ref(ref r) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}

impl ShapeExpr {
    pub fn empty_shape() -> ShapeExpr {
        ShapeExpr::Shape {
            closed: None,
            extra: None,
            expression: None,
            sem_acts: None,
            annotations: None,
        }
    }
}

impl Default for ShapeExpr {
    fn default() -> Self {
        ShapeExpr::Shape {
            closed: None,
            extra: None,
            expression: None,
            sem_acts: None,
            annotations: None,
        }
    }
}
