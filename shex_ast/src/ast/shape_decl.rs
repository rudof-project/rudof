use super::shape_expr::ShapeExpr;
use crate::ast::deserialize_string_or_struct;
use crate::ast::serialize_string_or_struct;
use crate::Ref;
use crate::ShapeLabel;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ShapeDecl {
    #[serde(rename = "type")]
    pub type_: String,

    pub id: String,

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
            id: label.into(),
            shape_expr,
        }
    }
}
