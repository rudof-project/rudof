//! Simple Implementation of IRIs
//!
use oxiri::{IriParseError, IriRef};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

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

pub trait IRI {
    //    fn to_string(&self) -> String ;
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

#[derive(Debug)]
pub struct IriError {
    msg: String,
}

impl FromStr for IriS {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_iri(s)
    }
}

impl From<IriParseError> for IriError {
    fn from(e: IriParseError) -> Self {
        IriError {
            msg: format!("IriParserError: {:?}", e.to_string()),
        }
    }
}

fn parse_iri(s: &str) -> Result<IriS, IriError> {
    match IriRef::parse(s.to_owned()) {
        Err(e) => Err(IriError {
            msg: format!("Error parsing IRI: {e}"),
        }),
        Ok(iri) => Ok(IriS { iri: iri }),
    }
}

// TODO: I would like to replace the concrete struct IriS by a trait once I know more Rust
impl IRI for IriS {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_iris() {
        let iri = IriS::from_str("http://example.org/").unwrap();
        assert_eq!(iri.to_string(), "<http://example.org/>");
    }
}
