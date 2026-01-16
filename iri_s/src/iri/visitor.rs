use crate::error::IriSError;
use crate::iri::IriS;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::Formatter;
use std::str::FromStr;

struct IriVisitor;

impl Visitor<'_> for IriVisitor {
    type Value = IriS;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an IRI")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match IriS::from_str(v) {
            Ok(iri) => Ok(iri),
            Err(IriSError::IriParseError { str, error: err }) => Err(E::custom(format!(
                "Error parsing \"{v}\" as IRI. String \"{str}\", Error: {err}"
            ))),
            Err(other) => Err(E::custom(format!(
                "Can not parse value \"{v}\" to IRI. Error: {other}"
            )))
        }
    }
}

impl <'de> Deserialize<'de> for IriS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(IriVisitor)
    }
}
