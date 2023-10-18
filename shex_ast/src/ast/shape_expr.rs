use std::result;
use std::str::FromStr;

use iri_s::{IriSError, IriS};
use serde::{Deserializer, Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use void::Void;

use super::serde_string_or_struct::SerializeStringOrStruct;
use super::ValueSetValue;
use super::{
    annotation::Annotation, iri_ref::IriRef, sem_act::SemAct, triple_expr::TripleExprWrapper,
    value_set_value::ValueSetValueWrapper, xs_facet::XsFacet,
};
use super::{node_kind::NodeKind, ref_::Ref};
use crate::ast::serde_string_or_struct::*;
use crate::{Node, NodeConstraint, TripleExpr};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub struct ShapeExprWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    pub se: ShapeExpr,
}

impl Into<ShapeExprWrapper> for ShapeExpr {
    fn into(self) -> ShapeExprWrapper {
        ShapeExprWrapper { se: self }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
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

    NodeConstraint(NodeConstraint),

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

    External,

    Ref(Ref),
}

impl FromStr for ShapeExpr {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iri_s = IriS::from_str(s)?;
        Ok(ShapeExpr::Ref(Ref::IriRef {
            value: iri_s,
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

    pub fn external() -> ShapeExpr {
        ShapeExpr::External
    }

    pub fn not(se: ShapeExpr) -> ShapeExpr {
        ShapeExpr::ShapeNot {
            shape_expr: Box::new(se.into()),
        }
    }

    pub fn or(ses: Vec<ShapeExpr>) -> ShapeExpr {
        let mut shape_exprs = Vec::new();
        for se in ses {
            shape_exprs.push(se.into())
        }
        ShapeExpr::ShapeOr { shape_exprs }
    }

    pub fn and(ses: Vec<ShapeExpr>) -> ShapeExpr {
        let mut shape_exprs = Vec::new();
        for se in ses {
            shape_exprs.push(se.into())
        }
        ShapeExpr::ShapeAnd { shape_exprs }
    }

    pub fn node_constraint(nc: NodeConstraint) -> ShapeExpr {
        ShapeExpr::NodeConstraint(nc)
    }

    pub fn shape_ref(ref_: Ref) -> ShapeExpr {
        ShapeExpr::Ref(ref_)
    }

    pub fn any() -> ShapeExpr {
        ShapeExpr::default()
    }

    pub fn shape(
        closed: Option<bool>,
        extra: Option<Vec<IriRef>>,
        expression: Option<TripleExpr>,
        sem_acts: Option<Vec<SemAct>>,
        annotations: Option<Vec<Annotation>>,
    ) -> ShapeExpr {
        ShapeExpr::Shape {
            closed,
            extra,
            expression: expression.map(|e| e.into()),
            sem_acts,
            annotations,
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

#[cfg(test)]
mod tests {
    use crate::{Pattern, StringFacet};

    use super::*;

    #[test]
    fn test_serde_xsfacet_pattern() {
        let facets: Vec<XsFacet> = vec![XsFacet::StringFacet(StringFacet::Pattern(Pattern::new(
            "o*",
        )))];
        let nc = NodeConstraint::new().with_xsfacets(facets);
        let se = ShapeExpr::NodeConstraint(nc);
        let json_nc = serde_json::to_string(&se).unwrap();
        assert_eq!(json_nc, "{\"type\":\"NodeConstraint\",\"pattern\":\"o*\"}");
    }
}
