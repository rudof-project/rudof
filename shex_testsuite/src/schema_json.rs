use std::fmt::{self, Display};
use std::marker::PhantomData;
use std::str::FromStr;

use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};
use void::Void;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SchemaJson {
    #[serde(rename = "@context")]
    context: String,

    #[serde(rename = "type")]
    type_: String,

    #[serde(rename = "startActs")]
    start_acts: Vec<StartAction>,

    #[serde(default, deserialize_with = "deserialize_opt_string_or_struct")]
    start: Option<ShapeExpr>,

    imports: Option<Vec<Iri>>,

    shapes: Option<Vec<ShapeDecl>>,
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

    #[serde(deserialize_with = "deserialize_string_or_struct")]
    shapeExpr: ShapeExpr,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
enum ShapeExpr {
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
        nodeKind: NodeKind,
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

pub fn deserialize_opt_box_string_or_struct<'de, T, D>(d: D) -> Result<Option<Box<T>>, D::Error>
where
    T: Deserialize<'de> + FromStr,
    <T as FromStr>::Err: Display,
    D: Deserializer<'de>,
{
    /// Declare an internal visitor type to handle our input.
    struct OptBoxStringOrStruct<T>(PhantomData<T>);

    impl<'de, T> de::Visitor<'de> for OptBoxStringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr,
        <T as FromStr>::Err: Display,
    {
        type Value = Option<Box<T>>;

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize_string_or_struct(deserializer).map(|e| Some(Box::new(e)))
        }

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "a null, a string or a map")
        }
    }

    d.deserialize_option(OptBoxStringOrStruct(PhantomData))
}

pub fn deserialize_opt_string_or_struct<'de, T, D>(d: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de> + FromStr,
    <T as FromStr>::Err: Display,
    D: Deserializer<'de>,
{
    /// Declare an internal visitor type to handle our input.
    struct OptStringOrStruct<T>(PhantomData<T>);

    impl<'de, T> de::Visitor<'de> for OptStringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr,
        <T as FromStr>::Err: Display,
    {
        type Value = Option<T>;

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize_string_or_struct(deserializer).map(Some)
        }

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "a null, a string or a map")
        }
    }

    d.deserialize_option(OptStringOrStruct(PhantomData))
}

pub fn deserialize_string_or_struct<'de, T, D>(d: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr,
    <T as FromStr>::Err: Display,
    D: Deserializer<'de>,
{
    /// Declare an internal visitor type to handle our input.
    struct StringOrStruct<T>(PhantomData<T>);

    impl<'de, T> de::Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr,
        <T as FromStr>::Err: Display,
    {
        type Value = T;

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            FromStr::from_str(value).map_err(|err| {
                // Just convert the underlying error type into a string and
                // pass it to serde as a custom error.
                de::Error::custom(format!("{}", err))
            })
        }

        fn visit_map<M>(self, visitor: M) -> Result<T, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            let mvd = de::value::MapAccessDeserializer::new(visitor);
            Deserialize::deserialize(mvd)
        }

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "a string or a map")
        }
    }

    d.deserialize_any(StringOrStruct(PhantomData))
}

/*
impl<'de> Deserialize<'de> for ShapeExpr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ShapeExprVisitor;

        impl<'de> de::Visitor<'de> for ShapeExprVisitor {
            type Value = ShapeExpr;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("expected ShapeExpr")
            }

            fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ShapeExpr::Ref(Ref::IriRef {
                    value: str.to_string(),
                }))
                // ShapeExpr::from_str(value)
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                if let Some("type") = map.next_key()? {
                    let value = map.next_value()?;
                    match value {
                        "ShapeExternal" => Ok(ShapeExpr::ShapeExternal),
                        "Shape" => {
                            let closed = if let Some("closed") = map.next_key()? {
                                let value = map.next_value()?;
                                match value {
                                    "true" => Ok(Some(true)),
                                    "false" => Ok(Some(false)),
                                    other => Err(de::Error::invalid_value(
                                        de::Unexpected::Other(other),
                                        &"true or false",
                                    )),
                                }
                            } else {
                                Ok(None)
                            }?;
                            let extra: Option<Vec<IriRef>> =
                                if let Some("extra") = map.next_key()? {
                                    let value = map.next_value()?;
                                    // de::Deserialize::deserialize(value);
                                    Ok(None)
                                } else {
                                    Ok(None)
                                }?;
                            let expression: Option<TripleExpr> =
                                if let Some("expression") = map.next_key()? {
                                    let value = map.next_value()?;
                                    // de::Deserialize::deserialize(value);
                                    Ok(None)
                                } else {
                                    Ok(None)
                                }?;
                            let expression: Option<TripleExpr> =
                                if let Some("expression") = map.next_key()? {
                                    let value = map.next_value()?;
                                    // de::Deserialize::deserialize(value);
                                    Ok(None)
                                } else {
                                    Ok(None)
                                }?;
                            let semActs: Option<Vec<SemAct>> =
                                if let Some("semActs") = map.next_key()? {
                                    let value = map.next_value()?;
                                    // de::Deserialize::deserialize(value);
                                    Ok(None)
                                } else {
                                    Ok(None)
                                }?;
                            let annotations: Option<Vec<Annotation>> =
                                if let Some("annotations") = map.next_key()? {
                                    let value = map.next_value()?;
                                    // de::Deserialize::deserialize(value);
                                    Ok(None)
                                } else {
                                    Ok(None)
                                }?;
                            Ok(ShapeExpr::Shape {
                                closed: closed,
                                extra: extra,
                                expression: expression,
                                semActs: semActs,
                                annotations: annotations,
                            })
                        }
                        _ => Ok(ShapeExpr::ShapeExternal),
                    }
                } else {
                    Err(de::Error::missing_field("type"))
                }
            }
        }
        deserializer.deserialize_any(ShapeExprVisitor {})
    }
}
*/
impl FromStr for ShapeExpr {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ShapeExpr::Ref(Ref::IriRef {
            value: s.to_string(),
        }))
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
enum Ref {
    IriRef { value: String },
    BNode { value: String },
}

#[derive(Debug, Clone)]
struct ClosedError;

#[derive(Debug, Clone)]
struct FromStrRefError;

impl FromStr for Ref {
    type Err = FromStrRefError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("FromStr for Ref...{s}");
        Ok(Ref::IriRef {
            value: s.to_string(),
        })
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct SemAct {
    name: IriRef,
    code: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Annotation {
    predicate: IriRef,
    object: ObjectValue,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
enum ObjectValue {
    IriRef { value: IriRef },
    ObjectLiteral { value: ObjectLiteral },
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct ObjectLiteral {
    value: String,
    language: Option<String>,

    #[serde(rename = "type")]
    type_: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
enum TripleExpr {
    TripleConstraint {
        id: Option<TripleExprLabel>,
        inverse: Option<bool>,
        predicate: IriRef,

        #[serde(default, deserialize_with = "deserialize_opt_box_string_or_struct")]
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
enum TripleExprLabel {
    IriTripleExprLabel { value: IriRef },
    BNodeTripleExprLabel { value: BNode },
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum NodeKind {
    Iri,
    BlankNode,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(try_from = "String")]
struct Iri {
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
struct IriRef {
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
struct BNode {
    value: String,
}

impl TryFrom<String> for BNode {
    type Error = Void;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(BNode { value: s })
    }
}

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
