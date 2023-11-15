use super::shape_expr::ShapeExpr;
use crate::Deref;
use crate::DerefError;
use crate::IriRef;
use crate::ast::deserialize_string_or_struct;
use crate::ast::serialize_string_or_struct;
use crate::Ref;
use crate::ShapeLabel;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ShapeDecl {
    #[serde(rename = "type")]
    pub type_: String,

    pub id: Ref,

    #[serde(
        rename = "shapeExpr",
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    pub shape_expr: ShapeExpr,
}

impl ShapeDecl {
    pub fn new(label: Ref, shape_expr: ShapeExpr) -> Self {
        ShapeDecl {
            type_: "ShapeDecl".to_string(),
            id: label,
            shape_expr,
        }
    }

    
}


impl Deref for ShapeDecl {
    fn deref(&self, 
        base: &Option<iri_s::IriS>, 
        prefixmap: &Option<prefixmap::PrefixMap>
    ) -> Result<Self, DerefError> where Self: Sized {
        let id = self.id.deref(base, prefixmap)?;
        let shape_expr = self.shape_expr.deref(base, prefixmap)?;
        Ok(ShapeDecl {
            type_: self.type_.clone(),
            id,
            shape_expr
        })
    }
}