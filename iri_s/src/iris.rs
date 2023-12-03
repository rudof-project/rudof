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
    // TODO: Maybe define this one as const...
    pub fn rdf_type() -> IriS {
        IriS::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")
    }

    pub fn xsd() -> IriS {
        IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#")
    }

    pub fn xsd_integer() -> IriS {
        IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#integer")
    }

    pub fn xsd_decimal() -> IriS {
        IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#decimal")
    }

    pub fn xsd_double() -> IriS {
        IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#double")
    }

    pub fn xsd_boolean() -> IriS {
        IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#boolean")
    }

    pub fn new_unchecked(str: &str) -> IriS {
        let iri = NamedNode::new_unchecked(str);
        IriS { iri }
    }

    pub fn as_str(&self) -> &str {
        self.iri.as_str()
    }

    pub fn from_named_node(iri: NamedNode) -> IriS {
        IriS { iri }
    }

    pub fn as_named_node(&self) -> &NamedNode {
        &self.iri
    }

    pub fn extend(&self, str: &str) -> Result<Self, IriSError> {
        let extended_str = format!("{}{}", self.iri.as_str(), str);
        let iri = NamedNode::new(extended_str)?;
        Ok(IriS { iri })
    }

    pub fn resolve(&self, other: IriS) -> Result<Self, IriSError> {
        let base = Iri::parse(self.iri.as_str())?;
        let other_str = other.as_str();
        let resolved = base.resolve(other_str)?;
        let iri = NamedNode::new(resolved.as_str())?;
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
        let iri = NamedNode::new(s)?;
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

/*impl PartialEq for IriS {
    fn eq(&self, other: &Self) -> bool {
        self.iri.as_str() == other.iri.as_str()
    }
}*/

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

    #[test]
    fn extending_iri() {
        let base = NamedNode::new("http://example.org/").unwrap();
        let base_iri = IriS::from_named_node(base);
        let extended = base_iri.extend("knows").unwrap();
        assert_eq!(extended.as_str(), "http://example.org/knows");
    }

    #[test]
    fn comparing_iris() {
        let iri1 = IriS::from_named_node(NamedNode::new_unchecked("http://example.org/name"));
        let iri2 = IriS::from_named_node(NamedNode::new_unchecked("http://example.org/name"));
        assert_eq!(iri1, iri2);
    }
}
