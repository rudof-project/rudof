//! Simple Implementation of IRIs
//!
use oxiri::{IriParseError, IriRef};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;
use serde::Serialize;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use serde::de;
use serde::de::Visitor;


#[derive(Error, Debug)]
pub enum IriSError {
    #[error("Converting String to IRI: {error:?}")]
    IriStrError { error: IriParseError },
}

impl From<IriParseError> for IriSError {
    fn from(e: IriParseError) -> IriSError {
        IriSError::IriStrError { error: e }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IriS {
    iri: IriRef<String>,
}

impl IriS {

    pub fn new(str: &str) -> Result<IriS, IriSError> {
        let iri = IriRef::from_str(str)?;
        Ok(IriS { iri: iri })
    }

    pub fn from_iri(iri: &IriRef<String>) -> IriS {
        IriS { iri: iri.clone() }
    }


    pub fn as_str(&self) -> &str {
        self.iri.as_str()
    }

    pub fn extend(&self, str: &str) -> Result<Self, IriError> {
        let s = self.iri.to_string() + str;
        let iri = IriRef::parse(s)?;
        Ok(IriS { iri: iri })
    }

    pub fn is_absolute(&self) -> bool {
        self.iri.is_absolute()
    }
}

impl fmt::Display for IriS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.iri)
    }
}

impl Serialize for IriS {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.iri.as_str())
    }
}

#[derive(Debug, Error)]
pub enum IriError {

    #[error("Iri error: {msg:?}")]
    IriError{ msg: String }
}

impl FromStr for IriS {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_iri(s)
    }
}

impl From<IriParseError> for IriError {
    fn from(e: IriParseError) -> Self {
        IriError::IriError {
            msg: format!("IriParserError: {:?}", e.to_string()),
        }
    }
}

fn parse_iri(s: &str) -> Result<IriS, IriError> {
    match IriRef::parse(s.to_owned()) {
        Err(e) => Err(IriError::IriError {
            msg: format!("Error parsing IRI: {e}"),
        }),
        Ok(iri) => Ok(IriS { iri: iri }),
    }
}

impl Default for IriS {
    
    fn default() -> Self {
        Self { iri: IriRef::from_str(String::default().as_str()).unwrap() }
    }
}

struct IriVisitor ;

impl<'de> Visitor<'de> for IriVisitor {
    type Value = IriS;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an IRI")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error, {
       match IriRef::parse(v.to_owned()) {
         Ok(iri) => Ok(IriS { iri: iri}),
         Err(err) => { 
            Err(E::custom(format!("Cannot parse as Iri: \"{v}\". Error: {err}")))
          }
       }
    }
}

impl <'de> Deserialize<'de> for IriS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
    where D: Deserializer<'de> { 
        deserializer.deserialize_str(IriVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_iris() {
        let iri = IriS::from_str("http://example.org/").unwrap();
        assert_eq!(iri.to_string(), "<http://example.org/>");
    }

    #[test]
    fn obtaining_iri_as_str() {
        let iri = IriS::from_str("http://example.org/p1").unwrap();
        assert_eq!(iri.as_str(), "http://example.org/p1");
    }

}
