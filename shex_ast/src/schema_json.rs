use std::fmt::Formatter;
use std::str::FromStr;
use std::{fmt::Display, result};

use crate::serde_string_or_struct::*;
use serde::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use void::Void;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SchemaJson {
    #[serde(rename = "@context")]
    context: String,

    #[serde(rename = "type")]
    pub type_: String,

    pub imports: Option<Vec<Iri>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_string_or_struct",
        deserialize_with = "deserialize_opt_string_or_struct"
    )]
    pub start: Option<ShapeExpr>,

    #[serde(default, rename = "startActs")]
    pub start_acts: Option<Vec<SemAct>>,

    pub shapes: Option<Vec<ShapeDecl>>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct StartAction {
    #[serde(rename = "type")]
    type_: String,
    name: IriRef,
    code: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ShapeDecl {
    #[serde(rename = "type")]
    type_: String,

    id: String,

    #[serde(
        rename = "shapeExpr",
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    shape_expr: ShapeExpr,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(transparent)]
pub struct ShapeExprWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    se: ShapeExpr,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum ShapeExpr {
    ShapeOr {
        #[serde(rename = "shapeExprs")]
        shape_exprs: Vec<Box<ShapeExprWrapper>>,
    },
    ShapeAnd {
        #[serde(rename = "shapeExprs")]
        shape_exprs: Vec<Box<ShapeExprWrapper>>,
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

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum XsFacet {
    StringFacet,
    NumericFacet,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(transparent)]
pub struct ValueSetValueWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    vs: ValueSetValue,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(transparent)]
pub struct ObjectValueWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    ov: ObjectValue,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ValueSetValue {
    IriStem {
        #[serde(rename = "type")]
        type_: String,

        stem: IriRef,
    },
    IriStemRange {
        #[serde(rename = "type")]
        type_: String,

        #[serde(
            serialize_with = "serialize_string_or_struct",
            deserialize_with = "deserialize_string_or_struct"
        )]
        stem: IriRefOrWildcard,

        #[serde(skip_serializing_if = "Option::is_none")]
        exclusions: Option<Vec<StringOrIriStemWrapper>>,
    },
    LiteralStem {
        #[serde(rename = "type")]
        type_: String,

        stem: String,
    },
    LiteralStemRange {
        #[serde(rename = "type")]
        type_: String,

        #[serde(
            serialize_with = "serialize_string_or_struct",
            deserialize_with = "deserialize_string_or_struct"
        )]
        stem: StringOrWildcard,

        #[serde(skip_serializing_if = "Option::is_none")]
        exclusions: Option<Vec<StringOrLiteralStemWrapper>>,
    },
    Language {
        #[serde(rename = "type")]
        type_: String,

        #[serde(rename = "languageTag")]
        language_tag: String,
    },
    LanguageStem,
    LanguageStemRange,
    ObjectValue(ObjectValueWrapper),
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum IriRefOrWildcard {
    IriRef(IriRef),
    Wildcard {
        #[serde(rename = "type")]
        type_: String,
    },
}

impl FromStr for IriRefOrWildcard {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(IriRefOrWildcard::IriRef(IriRef {
            value: s.to_string(),
        }))
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrWildcard {
    String(String),
    Wildcard {
        #[serde(rename = "type")]
        type_: String,
    },
}

impl FromStr for StringOrWildcard {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StringOrWildcard::String(s.to_string()))
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(transparent)]
pub struct StringOrLiteralStemWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    s: StringOrLiteralStem,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrLiteralStem {
    String(String),
    LiteralStem { stem: String },
}

impl FromStr for StringOrLiteralStemWrapper {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StringOrLiteralStemWrapper {
            s: StringOrLiteralStem::String(s.to_string()),
        })
    }
}

impl FromStr for StringOrLiteralStem {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StringOrLiteralStem::String(s.to_string()))
    }
}

impl SerializeStringOrStruct for StringOrWildcard {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            StringOrWildcard::String(ref r) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}

impl SerializeStringOrStruct for IriRefOrWildcard {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            IriRefOrWildcard::IriRef(ref r) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(transparent)]
pub struct StringOrIriStemWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    s: StringOrIriStem,
}

impl SerializeStringOrStruct for StringOrIriStem {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            StringOrIriStem::String(ref r) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}

impl SerializeStringOrStruct for StringOrLiteralStem {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            StringOrLiteralStem::String(ref r) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrIriStem {
    String(String),
    IriStem { stem: String },
}

impl FromStr for StringOrIriStem {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StringOrIriStem::String(s.to_string()))
    }
}

impl FromStr for ValueSetValue {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ValueSetValue::ObjectValue(ObjectValueWrapper {
            ov: ObjectValue::IriRef(IriRef {
                value: s.to_string(),
            }),
        }))
    }
}

impl SerializeStringOrStruct for ObjectValue {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            ObjectValue::IriRef(ref r) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}

impl SerializeStringOrStruct for ValueSetValue {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            ValueSetValue::ObjectValue(ObjectValueWrapper {
                ov: ObjectValue::IriRef(ref r),
            }) => r.serialize(serializer),
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

impl FromStr for ShapeExpr {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ShapeExpr::Ref(Ref::IriRef {
            value: s.to_string(),
        }))
    }
}

impl FromStr for TripleExpr {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TripleExpr::TripleExprRef(TripleExprLabel::IriRef {
            value: IriRef {
                value: s.to_string(),
            },
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

impl SerializeStringOrStruct for TripleExpr {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            TripleExpr::TripleExprRef(ref r) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Ref {
    IriRef { value: String },
    BNode { value: String },
}

#[derive(Debug, Clone)]
struct ClosedError;

#[derive(Debug, Clone)]
pub struct FromStrRefError;

impl FromStr for Ref {
    type Err = FromStrRefError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ref::IriRef {
            value: s.to_string(),
        })
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SemAct {
    name: IriRef,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    code: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Annotation {
    predicate: IriRef,
    object: ObjectValue,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ObjectValue {
    IriRef(IriRef),

    ObjectLiteral {
        value: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        language: Option<String>,

        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        type_: Option<String>,
    },
}

impl FromStr for ObjectValue {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ObjectValue::IriRef(IriRef {
            value: s.to_string(),
        }))
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ObjectLiteral {
    value: String,
    language: Option<String>,

    #[serde(rename = "type")]
    type_: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum TripleExpr {
    EachOf {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<TripleExprLabel>,
        expressions: Vec<Box<TripleExprWrapper>>,
        min: Option<i32>,
        max: Option<i32>,
        #[serde(default, rename = "semActs", skip_serializing_if = "Option::is_none")]
        sem_acts: Option<Vec<SemAct>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        annotations: Option<Vec<Annotation>>,
    },
    OneOf {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<TripleExprLabel>,
        expressions: Vec<Box<TripleExprWrapper>>,
        min: Option<i32>,
        max: Option<i32>,

        #[serde(default, rename = "semActs", skip_serializing_if = "Option::is_none")]
        sem_acts: Option<Vec<SemAct>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        annotations: Option<Vec<Annotation>>,
    },
    TripleConstraint {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<TripleExprLabel>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        inverse: Option<bool>,

        predicate: IriRef,

        #[serde(
            default,
            rename = "valueExpr",
            skip_serializing_if = "Option::is_none",
            serialize_with = "serialize_opt_box_string_or_struct",
            deserialize_with = "deserialize_opt_box_string_or_struct"
        )]
        value_expr: Option<Box<ShapeExpr>>,

        min: Option<i32>,

        max: Option<i32>,

        #[serde(default, rename = "semActs", skip_serializing_if = "Option::is_none")]
        sem_acts: Option<Vec<SemAct>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        annotations: Option<Vec<Annotation>>,
    },

    TripleExprRef(TripleExprLabel),
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(transparent)]
pub struct TripleExprWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    te: TripleExpr,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(try_from = "&str")]
pub enum TripleExprLabel {
    IriRef { value: IriRef },
    BNode { value: BNode },
}

#[derive(Debug, Clone)]
pub struct FromStrTripleExprLabelError;

impl Display for FromStrTripleExprLabelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error converting TripleExprLabel")
    }
}

impl TryFrom<&str> for TripleExprLabel {
    type Error = FromStrTripleExprLabelError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(TripleExprLabel::IriRef {
            value: IriRef {
                value: s.to_string(),
            },
        })
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NodeKind {
    Iri,
    BNode,
    NonLiteral,
    Literal,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(try_from = "String")]
pub struct Iri {
    value: String,
}

impl TryFrom<String> for Iri {
    type Error = Void;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Iri { value: s })
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(try_from = "String")]
pub struct IriRef {
    value: String,
}

impl TryFrom<String> for IriRef {
    type Error = Void;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(IriRef { value: s })
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(try_from = "String")]
pub struct BNode {
    value: String,
}

impl TryFrom<String> for BNode {
    type Error = Void;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(BNode { value: s })
    }
}

#[cfg(test)]
mod tests {

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
        let se = serde_json::from_str::<ShapeExpr>(&str).unwrap();
        let expected = ShapeExpr::Shape {
            closed: None,
            extra: None,
            expression: Some(TripleExprWrapper {
                te: TripleExpr::TripleConstraint {
                    id: None,
                    inverse: None,
                    predicate: IriRef {
                        value: "http://a.example/p1".to_string(),
                    },
                    value_expr: None,
                    min: None,
                    max: None,
                    sem_acts: None,
                    annotations: None,
                },
            }),
            sem_acts: None,
            annotations: None,
        };
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
        let se = serde_json::from_str::<ShapeExpr>(&str).unwrap();
        let expected = ShapeExpr::Shape {
            closed: None,
            extra: None,
            expression: Some(TripleExprWrapper {
                te: TripleExpr::TripleConstraint {
                    id: None,
                    inverse: None,
                    predicate: IriRef {
                        value: "http://a.example/p1".to_string(),
                    },
                    value_expr: Some(Box::new(ShapeExpr::Ref(Ref::IriRef {
                        value: "http://all.example/S5".to_string(),
                    }))),
                    min: None,
                    max: None,
                    sem_acts: None,
                    annotations: None,
                },
            }),
            sem_acts: None,
            annotations: None,
        };
        assert_eq!(se, expected);
    }

    #[test]
    fn test_triple_constraint1() {
        let str = r#"{
 "type": "TripleConstraint",
 "predicate": "http://a.example/p1",
 "valueExpr": "http://all.example/S5"
}"#;
        let te = serde_json::from_str::<TripleExpr>(&str).unwrap();
        let expected = TripleExpr::TripleConstraint {
            id: None,
            inverse: None,
            predicate: IriRef {
                value: "http://a.example/p1".to_string(),
            },
            value_expr: Some(Box::new(ShapeExpr::Ref(Ref::IriRef {
                value: "http://all.example/S5".to_string(),
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
        match serde_json::from_str::<ShapeExpr>(&str) {
            Ok(v) => {
                println!("Value parsed: {:?}", v);
                let serialized = serde_json::to_string(&v).unwrap();
                println!("serialized: {}", serialized);
                assert!(true)
            }
            Err(e) => assert!(false, "Error parsing: {}", e),
        }
    }

    #[test]
    fn test_triple() {
        let str = r#"{
            "type": "Shape",
            "expression": "http://all.example/S2e"
          }"#;
        match serde_json::from_str::<ShapeExpr>(&str) {
            Ok(v) => {
                println!("Value parsed: {:?}", v);
                let serialized = serde_json::to_string(&v).unwrap();
                println!("serialized: {}", serialized);
                assert!(true)
            }
            Err(e) => assert!(false, "Error parsing: {}", e),
        }
    }
}
