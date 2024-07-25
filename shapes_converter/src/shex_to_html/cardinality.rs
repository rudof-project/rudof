// use serde::Serialize;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug, PartialEq, Clone, Default)]
pub enum Cardinality {
    #[default]
    OneOne,

    Star,
    Plus,
    Optional,
    Range(i32, i32),
    Fixed(i32),
}

impl Serialize for Cardinality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Cardinality::OneOne => serializer.serialize_str("Exactly one"),
            Cardinality::Star => serializer.serialize_str("Zero or more"),
            Cardinality::Plus => serializer.serialize_str("One or more"),
            Cardinality::Optional => serializer.serialize_str("Zero or one (optional)"),
            Cardinality::Range(_, _) => todo!(),
            Cardinality::Fixed(_) => todo!(),
        }
    }
}
