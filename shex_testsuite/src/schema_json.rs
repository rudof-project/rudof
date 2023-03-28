use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};
use void::Void;

#[derive(Deserialize, Serialize, Debug)]
pub struct SchemaJson {
    #[serde(rename = "@context")]
    context: String,

    #[serde(rename = "type")]
    type_: String,

    #[serde(rename = "startActs")]
    start_acts: Vec<StartAction>,

    start: Option<ShapeExpr>,

    imports: Option<Vec<Iri>>,

    shapes: Option<Vec<ShapeDecl>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StartAction {
    #[serde(rename = "type")]
    type_: String,
    name: IriRef,
    code: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ShapeDecl {
    #[serde(rename = "type")]
    type_: String,

    id: String,

    shapeExpr: ShapeExpr,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
enum ShapeExpr {
    ShapeOr { shapeExprs: Vec<ShapeExpr> },
    ShapeAnd { shapeExprs: Vec<ShapeExpr> },
    ShapeNot { shapeExpr: Box<ShapeExpr> },
    NodeConstraint { nodeKind: NodeKind },
    Shape { expression: TripleExpr },
    ShapeExternal,
    ShapeExprRef,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
enum TripleExpr {
    TripleConstraint {
        predicate: String,
        valueExpr: Box<ShapeExpr>,
    },
    OneOf {
        expressions: Vec<TripleExpr>,
    },
    EachOf {
        expressions: Vec<TripleExpr>,
    },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
enum NodeKind {
    Iri,
    BlankNode,
}

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Deserialize, Serialize, Debug)]
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

fn string_deser<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
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
