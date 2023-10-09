use super::shape_expr::ShapeExpr;
use crate::schema_json::deserialize_string_or_struct;
use crate::schema_json::serialize_string_or_struct;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
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
