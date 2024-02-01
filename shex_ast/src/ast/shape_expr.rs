use std::result;
use std::str::FromStr;

use iri_s::IriS;
use prefixmap::{Deref, DerefError, IriRef, PrefixMap};
use serde::{Serialize as SerializeTrait, Serializer};
use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::serde_string_or_struct::SerializeStringOrStruct;
use crate::ast::serde_string_or_struct::*;
use crate::{NodeConstraint, RefError, Shape, ShapeExprLabel};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub struct ShapeExprWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    pub se: ShapeExpr,
}

impl Deref for ShapeExprWrapper {
    fn deref(
        &self,
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Self, DerefError> {
        let se = self.se.deref(base, prefixmap)?;
        let sew = ShapeExprWrapper { se: se };
        Ok(sew)
    }
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

    Shape(Shape),

    #[serde(rename="ShapeExternal")]
    External,

    Ref(ShapeExprLabel),
}

impl FromStr for ShapeExpr {
    type Err = RefError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ref_ = ShapeExprLabel::from_str(s)?;
        Ok(ShapeExpr::Ref(ref_))
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
        ShapeExpr::Shape(Shape::default())
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

    pub fn iri_ref(iri_ref: IriRef) -> ShapeExpr {
        ShapeExpr::Ref(ShapeExprLabel::iri_ref(iri_ref))
    }

    pub fn shape_ref(label: ShapeExprLabel) -> ShapeExpr {
        ShapeExpr::Ref(label)
    }

    pub fn any() -> ShapeExpr {
        ShapeExpr::default()
    }

    pub fn shape(shape: Shape) -> ShapeExpr {
        ShapeExpr::Shape(shape)
    }
}

impl Default for ShapeExpr {
    fn default() -> Self {
        ShapeExpr::Shape(Shape::default())
    }
}

impl Deref for ShapeExpr {
    fn deref(
        &self,
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Self, DerefError> {
        match self {
            ShapeExpr::External => Ok(ShapeExpr::External),
            ShapeExpr::ShapeAnd { shape_exprs } => {
                let shape_exprs = <ShapeExpr as Deref>::deref_vec(shape_exprs, base, prefixmap)?;
                Ok(ShapeExpr::ShapeAnd { shape_exprs })
            }
            ShapeExpr::ShapeOr { shape_exprs } => {
                let shape_exprs = <ShapeExpr as Deref>::deref_vec(shape_exprs, base, prefixmap)?;
                Ok(ShapeExpr::ShapeOr { shape_exprs })
            }
            ShapeExpr::ShapeNot { shape_expr } => {
                let shape_expr = <ShapeExpr as Deref>::deref_box(shape_expr, base, prefixmap)?;
                Ok(ShapeExpr::ShapeNot { shape_expr })
            }
            ShapeExpr::Shape(shape) => {
                let shape = shape.deref(base, prefixmap)?;
                Ok(ShapeExpr::Shape(shape))
            }
            ShapeExpr::Ref(ref_) => {
                let ref_ = ref_.deref(base, prefixmap)?;
                Ok(ShapeExpr::Ref(ref_))
            }
            ShapeExpr::NodeConstraint(nc) => {
                let nc = nc.deref(base, prefixmap)?;
                Ok(ShapeExpr::NodeConstraint(nc))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Pattern, StringFacet, XsFacet};

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
