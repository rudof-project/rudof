use super::shape_expr::ShapeExpr;
use crate::ast::deserialize_string_or_struct;
use crate::ast::serialize_string_or_struct;
use crate::ShapeExprLabel;
use prefixmap::Deref;
use prefixmap::DerefError;
use prefixmap::IriRef;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ShapeDecl {
    #[serde(rename = "type")]
    pub type_: String,

    pub id: ShapeExprLabel,

    #[serde(rename = "abstract", default = "default_abstract")]
    pub is_abstract: bool,

    #[serde(
        rename = "shapeExpr",
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    pub shape_expr: ShapeExpr,
}

fn default_abstract() -> bool {
    false
}

impl ShapeDecl {
    pub fn new(label: ShapeExprLabel, shape_expr: ShapeExpr, is_abstract: bool) -> Self {
        ShapeDecl {
            type_: "ShapeDecl".to_string(),
            is_abstract,
            id: label,
            shape_expr,
        }
    }

    pub fn with_is_abstract(mut self, is_abstract: bool) -> Self {
        self.is_abstract = is_abstract;
        self
    }
}

impl Deref for ShapeDecl {
    fn deref(
        &self,
        base: &Option<iri_s::IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        let id = self.id.deref(base, prefixmap)?;
        let shape_expr = self.shape_expr.deref(base, prefixmap)?;
        Ok(ShapeDecl {
            type_: self.type_.clone(),
            is_abstract: self.is_abstract.clone(),
            id,
            shape_expr,
        })
    }
}
