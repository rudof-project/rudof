use oxiri::Iri;
use oxrdf::NamedNode;
use serde::de;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;
use std::str::FromStr;

use crate::IriSError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IriS {
    iri: NamedNode,
}

impl IriS {
    pub fn rdf_type() -> IriS {
        IriS::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")
    }

    pub fn new_unchecked(str: &str) -> IriS {
        let iri = NamedNode::new_unchecked(str);
        IriS { iri }
    }

    pub fn as_str(&self) -> &str {
        self.iri.as_str()
    }

    /// Convert a `NamedNode` to an `IriS`
    pub fn from_named_node(iri: &NamedNode) -> IriS {
        IriS { iri: iri.clone() }
    }

    /// Convert an `IriS` to a `NamedNode`
    pub fn as_named_node(&self) -> &NamedNode {
        &self.iri
    }

    /// Extend an IRI with a new string
    ///
    /// This function is safe as it checks for possible errors
    pub fn extend(&self, str: &str) -> Result<Self, IriSError> {
        let extended_str = format!("{}{}", self.iri.as_str(), str);
        let iri = NamedNode::new(extended_str.as_str()).map_err(|e| IriSError::IriParseError {
            str: extended_str,
            err: e.to_string(),
        })?;
        Ok(IriS { iri })
    }

    /// Extend an IRI with a new string without checking for possible syntactic errors
    ///
    pub fn extend_unchecked(&self, str: &str) -> Self {
        let extended_str = format!("{}{}", self.iri.as_str(), str);
        let iri = NamedNode::new_unchecked(extended_str);
        IriS { iri }
    }

    /// Resolve the IRI `other` with this IRI
    pub fn resolve(&self, other: IriS) -> Result<Self, IriSError> {
        let str = self.iri.as_str();
        let base = Iri::parse(str).map_err(|e| IriSError::IriParseError {
            str: str.to_string(),
            err: e.to_string(),
        })?;
        let other_str = other.as_str();
        let resolved = base
            .resolve(other_str)
            .map_err(|e| IriSError::IriResolveError {
                err: e.to_string(),
                base: self.clone(),
                other: other.clone(),
            })?;
        let iri = NamedNode::new(resolved.as_str()).map_err(|e| IriSError::IriParseError {
            str: resolved.as_str().to_string(),
            err: e.to_string(),
        })?;
        Ok(IriS { iri })
    }

    /*    pub fn is_absolute(&self) -> bool {
        self.0.is_absolute()
    } */
}

impl fmt::Display for IriS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.iri.as_str())
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

impl FromStr for IriS {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iri = NamedNode::new(s).map_err(|e| IriSError::IriParseError {
            str: s.to_string(),
            err: e.to_string(),
        })?;
        Ok(IriS { iri })
    }
}

/*impl TryFrom<&str> for IriS {
    type Error = IriSError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let iri = NamedNode::new(value)?;
        Ok(IriS { iri })
    }
}*/

impl Default for IriS {
    fn default() -> Self {
        IriS::new_unchecked(&String::default())
    }
}

struct IriVisitor;

impl<'de> Visitor<'de> for IriVisitor {
    type Value = IriS;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an IRI")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        IriS::from_str(v)
            .map_err(|e| E::custom(format!("Cannot parse as Iri: \"{v}\". Error: {e}")))
    }
}

impl<'de> Deserialize<'de> for IriS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(IriVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_iris() {
        let iri = IriS::from_str("http://example.org/").unwrap();
        assert_eq!(iri.to_string(), "http://example.org/");
    }

    #[test]
    fn obtaining_iri_as_str() {
        let iri = IriS::from_str("http://example.org/p1").unwrap();
        assert_eq!(iri.as_str(), "http://example.org/p1");
    }

    #[test]
    fn extending_iri() {
        let base = NamedNode::new("http://example.org/").unwrap();
        let base_iri = IriS::from_named_node(&base);
        let extended = base_iri.extend("knows").unwrap();
        assert_eq!(extended.as_str(), "http://example.org/knows");
    }

    #[test]
    fn comparing_iris() {
        let iri1 = IriS::from_named_node(&NamedNode::new_unchecked("http://example.org/name"));
        let iri2 = IriS::from_named_node(&NamedNode::new_unchecked("http://example.org/name"));
        assert_eq!(iri1, iri2);
    }
}
