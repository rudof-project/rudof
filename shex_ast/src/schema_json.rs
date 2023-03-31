// use std::fmt::{self, Display};
// use std::marker::PhantomData;
use std::result;
use std::str::FromStr;

use crate::serde_string_or_struct::*;
// use serde::de::{self, MapAccess, Visitor};
use serde::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use void::Void;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SchemaJson {
    #[serde(rename = "@context")]
    context: String,

    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "startActs")]
    pub start_acts: Vec<StartAction>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_string_or_struct",
        deserialize_with = "deserialize_opt_string_or_struct"
    )]
    pub start: Option<ShapeExpr>,

    pub imports: Option<Vec<Iri>>,

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
        rename = "ShapeExpr",
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    shape_expr: ShapeExpr,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum ShapeExpr {
    ShapeOr {
        shapeExprs: Vec<Box<ShapeExpr>>,
    },
    ShapeAnd {
        shapeExprs: Vec<Box<ShapeExpr>>,
    },
    ShapeNot {
        shapeExpr: Box<Box<ShapeExpr>>,
    },
    NodeConstraint {
        nodeKind: Option<NodeKind>,
        datatype: Option<IriRef>,
        xsFacet: Vec<XsFacet>,
        values: Option<Vec<ValueSetValue>>,
    },
    Shape {
        closed: Option<bool>,
        extra: Option<Vec<IriRef>>,
        expression: Option<TripleExpr>,
        semActs: Option<Vec<SemAct>>,
        annotations: Option<Vec<Annotation>>,
    },
    ShapeExternal,
    Ref(Ref),
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
enum XsFacet {
    StringFacet,
    NumericFacet,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
enum ValueSetValue {
    ObjectValue,
    IriStem,
    IriStemRange,
    LiteralStem,
    LiteralStemRange,
    Language,
    LanguageStem,
    LanguageStemRange,
}

impl ShapeExpr {
    fn emptyShape() -> ShapeExpr {
        ShapeExpr::Shape {
            closed: None,
            extra: None,
            expression: None,
            semActs: None,
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
    code: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Annotation {
    predicate: IriRef,
    object: ObjectValue,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum ObjectValue {
    IriRef { value: IriRef },
    ObjectLiteral { value: ObjectLiteral },
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
    TripleConstraint {
        id: Option<TripleExprLabel>,
        inverse: Option<bool>,
        predicate: IriRef,

        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            serialize_with = "serialize_opt_box_string_or_struct",
            deserialize_with = "deserialize_opt_box_string_or_struct"
        )]
        valueExpr: Option<Box<ShapeExpr>>,

        min: Option<i32>,
        max: Option<i32>,
        semActs: Option<Vec<SemAct>>,
        annotations: Option<Vec<Annotation>>,
    },
    OneOf {
        expressions: Vec<TripleExpr>,
    },
    EachOf {
        expressions: Vec<TripleExpr>,
    },
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum TripleExprLabel {
    IriTripleExprLabel { value: IriRef },
    BNodeTripleExprLabel { value: BNode },
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
/*
fn string_deser<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    struct StringDeser<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringDeser<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }
    }

    deserializer.deserialize_any(StringDeser(PhantomData))
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

fn opt_string_or_struct<'de, T, D>(deserializer: D) -> Result<Option<Box<T>>, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct OptStringOrStruct<T>(PhantomData<fn() -> Option<Box<T>>>);

    impl<'de, T> Visitor<'de> for OptStringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = Option<Box<T>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<Option<Box<T>>, E>
        where
            E: de::Error,
        {
            println!("String!: {:?}", value);
            Ok(Some(Box::new(FromStr::from_str(value).unwrap())))
        }

        fn visit_map<M>(self, map: M) -> Result<Option<Box<T>>, M::Error>
        where
            M: MapAccess<'de>,
        {
            println!("visit_map...");
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    println!("opt_string_or_struct!");
    deserializer.deserialize_any(OptStringOrStruct(PhantomData))
}
*/
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
            expression: Some(TripleExpr::TripleConstraint {
                id: None,
                inverse: None,
                predicate: IriRef {
                    value: "http://a.example/p1".to_string(),
                },
                valueExpr: None,
                min: None,
                max: None,
                semActs: None,
                annotations: None,
            }),
            semActs: None,
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
            expression: Some(TripleExpr::TripleConstraint {
                id: None,
                inverse: None,
                predicate: IriRef {
                    value: "http://a.example/p1".to_string(),
                },
                valueExpr: Some(Box::new(ShapeExpr::Ref(Ref::IriRef {
                    value: "http://all.example/S5".to_string(),
                }))),
                min: None,
                max: None,
                semActs: None,
                annotations: None,
            }),
            semActs: None,
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
            valueExpr: Some(Box::new(ShapeExpr::Ref(Ref::IriRef {
                value: "http://all.example/S5".to_string(),
            }))),
            max: None,
            min: None,
            semActs: None,
            annotations: None,
        };
        assert_eq!(te, expected);
    }

    #[test]
    fn test_triple_constraint2() {
        let str = r#"{
             "type": "TripleConstraint",
             "predicate": "http://a.example/p1"
        }"#;
        let te = serde_json::from_str::<TripleExpr>(&str).unwrap();
        let expected = TripleExpr::TripleConstraint {
            id: None,
            inverse: None,
            predicate: IriRef {
                value: "http://a.example/p1".to_string(),
            },
            valueExpr: None,
            max: None,
            min: None,
            semActs: None,
            annotations: None,
        };
        assert_eq!(te, expected);
    }
}
