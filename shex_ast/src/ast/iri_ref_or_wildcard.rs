use iri_s::IriSError;
use prefixmap::IriRef;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use std::{result, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
pub enum IriRefOrWildcard {
    IriRef(IriRef),
    Wildcard,
}

impl FromStr for IriRefOrWildcard {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iri_ref = IriRef::try_from(s)?;
        Ok(IriRefOrWildcard::IriRef(iri_ref))
    }
}

impl Serialize for IriRefOrWildcard {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            IriRefOrWildcard::IriRef(iri) => serializer.serialize_str(iri.to_string().as_str()),
            IriRefOrWildcard::Wildcard => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "Wildcard")?;
                map.end()
            }
        }
    }
}
