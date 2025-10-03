pub mod annotation;
pub mod bnode;
pub mod exclusion;
pub mod iri_exclusion;
pub mod iri_or_str;
pub mod iri_ref_or_wildcard;
pub mod lang_or_wildcard;
pub mod language_exclusion;
pub mod literal_exclusion;
pub mod node_constraint;
pub mod node_kind;
pub mod object_value;
pub mod schema;
pub mod schema_json_error;
pub mod sem_act;
pub mod serde_string_or_struct;
pub mod shape;
pub mod shape_decl;
pub mod shape_expr;
pub mod shape_expr_label;
pub mod simple_repr_schema;
pub mod start_action;
pub mod string_or_iri_stem;
pub mod string_or_literal_stem;
pub mod string_or_wildcard;
pub mod triple_expr;
pub mod triple_expr_label;
pub mod value_set_value;
pub mod xs_facet;

use crate::ast::iri_exclusion::*;
use crate::ast::language_exclusion::*;
use crate::ast::literal_exclusion::*;
use crate::ast::serde_string_or_struct::*;
use crate::exclusion::*;
pub use annotation::*;
pub use bnode::*;
pub use iri_or_str::*;
pub use iri_ref_or_wildcard::*;
pub use lang_or_wildcard::*;
pub use node_constraint::*;
pub use node_kind::*;
pub use object_value::*;
pub use schema::*;
pub use schema_json_error::*;
pub use sem_act::*;
pub use shape::*;
pub use shape_decl::*;
pub use shape_expr::*;
pub use shape_expr_label::*;
pub use simple_repr_schema::*;
pub use start_action::*;
pub use string_or_iri_stem::*;
pub use string_or_literal_stem::*;
pub use string_or_wildcard::*;
pub use triple_expr::*;
pub use triple_expr_label::*;
pub use value_set_value::*;
pub use xs_facet::*;

const BOOLEAN_STR: &str = "http://www.w3.org/2001/XMLSchema#boolean";
const INTEGER_STR: &str = "http://www.w3.org/2001/XMLSchema#integer";
const BYTE_STR: &str = "http://www.w3.org/2001/XMLSchema#byte";
const SHORT_STR: &str = "http://www.w3.org/2001/XMLSchema#short";
const NON_NEGATIVE_INTEGER_STR: &str = "http://www.w3.org/2001/XMLSchema#nonNegativeInteger";
const UNSIGNED_LONG_STR: &str = "http://www.w3.org/2001/XMLSchema#unsignedLong";
const UNSIGNED_INT_STR: &str = "http://www.w3.org/2001/XMLSchema#unsignedInt";
const UNSIGNED_SHORT_STR: &str = "http://www.w3.org/2001/XMLSchema#unsignedShort";
const UNSIGNED_BYTE_STR: &str = "http://www.w3.org/2001/XMLSchema#unsignedByte";
const POSITIVE_INTEGER_STR: &str = "http://www.w3.org/2001/XMLSchema#positiveInteger";
const NEGATIVE_INTEGER_STR: &str = "http://www.w3.org/2001/XMLSchema#negativeInteger";
const NON_POSITIVE_INTEGER_STR: &str = "http://www.w3.org/2001/XMLSchema#nonPositiveInteger";

const LONG_STR: &str = "http://www.w3.org/2001/XMLSchema#long";
const DOUBLE_STR: &str = "http://www.w3.org/2001/XMLSchema#double";
const FLOAT_STR: &str = "http://www.w3.org/2001/XMLSchema#float";
const DECIMAL_STR: &str = "http://www.w3.org/2001/XMLSchema#decimal";
const DATETIME_STR: &str = "http://www.w3.org/2001/XMLSchema#datetime";

#[derive(Debug, Clone)]
pub struct FromStrRefError;

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use iri_s::IriS;
    use prefixmap::IriRef;

    use super::*;

    #[test]
    fn test_shape_expr_triple_constraint() {
        let str = r#"{
            "type": "Shape",
            "expression": {
              "type": "TripleConstraint",
              "predicate": "http://a.example/p1"
            }
          }"#;
        let se = serde_json::from_str::<ShapeExpr>(str).unwrap();
        let expected = ShapeExpr::Shape(Shape::default().with_expression(
            TripleExpr::TripleConstraint {
                id: None,
                negated: None,
                inverse: None,
                predicate: IriS::new_unchecked("http://a.example/p1").into(),
                value_expr: None,
                min: None,
                max: None,
                sem_acts: None,
                annotations: None,
            },
        ));
        assert_eq!(se, expected);
    }

    #[test]
    fn test_shape_expr_ref() {
        let str = r#"{
            "type": "Shape",
            "expression": {
              "type": "TripleConstraint",
              "predicate": "http://a.example/p1",
              "valueExpr": "http://all.example/S5"
            }
          }"#;
        let se = serde_json::from_str::<ShapeExpr>(str).unwrap();
        let expected = ShapeExpr::Shape(Shape::default().with_expression(
            TripleExpr::TripleConstraint {
                id: None,
                negated: None,
                inverse: None,
                predicate: IriS::new_unchecked("http://a.example/p1").into(),
                value_expr: Some(Box::new(ShapeExpr::Ref(ShapeExprLabel::IriRef {
                    value: IriRef::iri(IriS::new_unchecked("http://all.example/S5")),
                }))),
                min: None,
                max: None,
                sem_acts: None,
                annotations: None,
            },
        ));
        assert_eq!(se, expected);
    }

    #[test]
    fn test_triple_constraint1() {
        let str = r#"{
 "type": "TripleConstraint",
 "predicate": "http://a.example/p1",
 "valueExpr": "http://all.example/S5"
}"#;
        let te = serde_json::from_str::<TripleExpr>(str).unwrap();
        let p1 = IriS::from_str("http://a.example/p1").unwrap();
        let s5 = IriS::from_str("http://all.example/S5").unwrap();
        let expected = TripleExpr::TripleConstraint {
            id: None,
            negated: None,
            inverse: None,
            predicate: p1.into(),
            value_expr: Some(Box::new(ShapeExpr::Ref(ShapeExprLabel::IriRef {
                value: IriRef::iri(s5),
            }))),
            max: None,
            min: None,
            sem_acts: None,
            annotations: None,
        };
        assert_eq!(te, expected);
    }

    #[test]
    fn test_json() {
        let str = r#"{
            "type": "NodeConstraint",
            "values": [
                {
                    "value": "0",
                    "type": "http://www.w3.org/2001/XMLSchema#integer"
                }
             ]
          }"#;

        let shape_expr = serde_json::from_str::<ShapeExpr>(str);
        if let Ok(v) = &shape_expr {
            let _serialized = serde_json::to_string(v).unwrap();
        }
        assert!(shape_expr.is_ok())
    }

    #[test]
    fn test_triple() {
        let str = r#"{
            "type": "Shape",
            "expression": "http://all.example/S2e"
          }"#;

        let shape_expr = serde_json::from_str::<ShapeExpr>(str);
        if let Ok(v) = &shape_expr {
            serde_json::to_string(v).unwrap();
        }
        assert!(shape_expr.is_ok())
    }
}
