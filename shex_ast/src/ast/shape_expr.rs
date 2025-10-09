use iri_s::IriS;
use prefixmap::{Deref, DerefError, IriRef, PrefixMap};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;

use super::serde_string_or_struct::SerializeStringOrStruct;
use crate::Annotation;
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
        let sew = ShapeExprWrapper { se };
        Ok(sew)
    }
}

impl From<ShapeExpr> for ShapeExprWrapper {
    fn from(shape_expr: ShapeExpr) -> Self {
        Self { se: shape_expr }
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

    #[serde(rename = "ShapeExternal")]
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
    fn serialize_string_or_struct<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            ShapeExpr::Ref(r) => r.serialize(serializer),
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

    pub fn shape_not(se: ShapeExpr) -> ShapeExpr {
        ShapeExpr::ShapeNot {
            shape_expr: Box::new(ShapeExprWrapper { se }),
        }
    }

    pub fn or(ses: Vec<ShapeExpr>) -> ShapeExpr {
        /*let shape_exprs = ses
        .into_iter()
        .map(|shape_expression| Box::new(shape_expression.into()))
        .collect(); */
        let mut shape_exprs = Vec::new();
        for se in ses {
            shape_exprs.push(se.into())
        }
        ShapeExpr::ShapeOr { shape_exprs }
    }

    pub fn and(ses: Vec<ShapeExpr>) -> ShapeExpr {
        /* let shape_exprs = ses
        .into_iter()
        .map(|shape_expression| Box::new(shape_expression.into()))
        .collect(); */
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

    pub fn add_annotation(&mut self, annotation: Annotation) {
        match self {
            Self::Shape(s) => s.add_annotation(annotation),
            _ => todo!(),
        };
    }

    pub fn has_annotations(&self) -> bool {
        match self {
            Self::Shape(s) => s.has_annotations(),
            _ => todo!(),
        }
    }

    pub fn annotations(&self) -> Option<impl Iterator<Item = &Annotation>> {
        match self {
            Self::Shape(s) => s.annotations(),
            _ => todo!(),
        }
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
                Ok(ShapeExpr::ShapeAnd {
                    shape_exprs: shape_exprs.clone(),
                })
            }
            ShapeExpr::ShapeOr { shape_exprs } => {
                let shape_exprs = <ShapeExpr as Deref>::deref_vec(shape_exprs, base, prefixmap)?;
                Ok(ShapeExpr::ShapeOr {
                    shape_exprs: shape_exprs.clone(),
                })
            }
            ShapeExpr::ShapeNot { shape_expr } => {
                let shape_expr = Box::new(shape_expr.deref(base, prefixmap)?);
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
