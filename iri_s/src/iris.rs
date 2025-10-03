use oxiri::Iri;
use oxrdf::NamedNode;
use oxrdf::NamedOrBlankNode;
use oxrdf::Term;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de;
use serde::de::Visitor;
use std::fmt;
use std::str::FromStr;
use tracing::trace;
use url::Url;

use crate::IriSError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn from_path(path: &std::path::Path) -> Result<IriS, IriSError> {
        let url = Url::from_file_path(path).map_err(|_| IriSError::ConvertingPathToIri {
            path: path.to_path_buf().to_string_lossy().to_string(),
        })?;
        let iri = NamedNode::new(url.as_str()).map_err(|e| IriSError::IriParseError {
            str: url.as_str().to_string(),
            err: e.to_string(),
        })?;
        Ok(IriS { iri })
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

    pub fn join(&self, str: &str) -> Result<Self, IriSError> {
        let url = Url::from_str(self.as_str()).map_err(|e| IriSError::IriParseError {
            str: str.to_string(),
            err: e.to_string(),
        })?;
        let joined = url.join(str).map_err(|e| IriSError::JoinError {
            str: Box::new(str.to_string()),
            current: Box::new(self.clone()),
            err: Box::new(e.to_string()),
        })?;
        Ok(IriS::new_unchecked(joined.as_str()))
    }

    /// Extends the current IRI with a new string
    ///
    /// This function checks for possible errors returning a Result
    pub fn extend(&self, str: &str) -> Result<Self, IriSError> {
        let current_str = self.iri.as_str();
        let extended_str = if current_str.ends_with('/') || current_str.ends_with('#') {
            format!("{current_str}{str}")
        } else {
            format!("{current_str}/{str}")
        };
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
                err: Box::new(e.to_string()),
                base: Box::new(self.clone()),
                other: other_str.to_string(),
            })?;
        let iri = NamedNode::new(resolved.as_str()).map_err(|e| IriSError::IriParseError {
            str: resolved.as_str().to_string(),
            err: e.to_string(),
        })?;
        Ok(IriS { iri })
    }

    /// Resolve `other` with this IRI
    pub fn resolve_str(&self, other: &str) -> Result<Self, IriSError> {
        let str = self.iri.as_str();
        let base = Iri::parse(str).map_err(|e| IriSError::IriParseError {
            str: str.to_string(),
            err: e.to_string(),
        })?;
        let resolved = base
            .resolve(other)
            .map_err(|e| IriSError::IriResolveError {
                err: Box::new(e.to_string()),
                base: Box::new(self.clone()),
                other: other.to_string(),
            })?;
        let iri = NamedNode::new(resolved.as_str()).map_err(|e| IriSError::IriParseError {
            str: resolved.as_str().to_string(),
            err: e.to_string(),
        })?;
        Ok(IriS { iri })
    }

    /// [Dereference](https://www.w3.org/wiki/DereferenceURI) the IRI and get the content available from it
    /// It handles also IRIs with the `file` scheme as local file names. For example: `file:///person.txt`
    ///
    #[cfg(not(target_family = "wasm"))]
    pub fn dereference(&self, base: &Option<IriS>) -> Result<String, IriSError> {
        use reqwest::header;
        use reqwest::header::USER_AGENT;
        use std::fs;

        let url = match base {
            Some(base_iri) => {
                let base =
                    Url::from_str(base_iri.as_str()).map_err(|e| IriSError::UrlParseError {
                        str: self.iri.as_str().to_string(),
                        error: format!("{e}"),
                    })?;
                Url::options()
                    .base_url(Some(&base))
                    .parse(self.iri.as_str())
                    .map_err(|e| IriSError::IriParseErrorWithBase {
                        str: self.iri.as_str().to_string(),
                        base: format!("{base}"),
                        error: format!("{e}"),
                    })?
            }
            None => Url::from_str(self.iri.as_str()).map_err(|e| IriSError::UrlParseError {
                str: self.iri.as_str().to_string(),
                error: format!("{e}"),
            })?,
        };
        match url.scheme() {
            "file" => {
                let path = url
                    .to_file_path()
                    .map_err(|_| IriSError::ConvertingFileUrlToPath {
                        url: format!("{url}"),
                    })?;
                let path_name = path.to_string_lossy().to_string();
                let body = fs::read_to_string(path).map_err(|e| IriSError::IOErrorFile {
                    path: path_name,
                    url: format!("{url}"),
                    error: format!("{e}"),
                })?;
                Ok(body)
            }
            _ => {
                let mut headers = header::HeaderMap::new();
                /* TODO: Add a parameter with the Accept header ?
                headers.insert(
                    ACCEPT,
                    header::HeaderValue::from_static(""),
                );*/
                headers.insert(USER_AGENT, header::HeaderValue::from_static("rudof"));
                let client = reqwest::blocking::Client::builder()
                    .default_headers(headers)
                    .build()
                    .map_err(|e| IriSError::ReqwestClientCreation {
                        error: format!("{e}"),
                    })?;
                let body = client
                    .get(url)
                    .send()
                    .map_err(|e| IriSError::ReqwestError {
                        error: format!("{e}"),
                    })?
                    .text()
                    .map_err(|e| IriSError::ReqwestTextError {
                        error: format!("{e}"),
                    })?;
                Ok(body)
            }
        }
    }

    #[cfg(target_family = "wasm")]
    pub fn dereference(&self, _base: &Option<IriS>) -> Result<String, IriSError> {
        return Err(IriSError::ReqwestClientCreation {
            error: String::from("reqwest is not enabled"),
        });
    }

    /*    pub fn is_absolute(&self) -> bool {
        self.0.is_absolute()
    } */

    pub fn from_str_base(str: &str, base: Option<&str>) -> Result<IriS, IriSError> {
        match base {
            Some(base_str) => {
                let base_iri = IriS::from_str(base_str)?;
                base_iri.resolve_str(str)
            }
            None => IriS::from_str(str),
        }
    }
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

impl From<NamedNode> for IriS {
    fn from(iri: NamedNode) -> Self {
        IriS { iri }
    }
}

impl FromStr for IriS {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iri = NamedNode::new(s).map_err(|e| {
            trace!("Error parsing IRI from str: {s}, error: {e}");
            IriSError::IriParseError {
                str: s.to_string(),
                err: e.to_string(),
            }
        })?;
        Ok(IriS { iri })
    }
}

impl From<IriS> for NamedNode {
    fn from(iri: IriS) -> Self {
        NamedNode::new_unchecked(iri.as_str())
    }
}

impl From<IriS> for NamedOrBlankNode {
    fn from(value: IriS) -> Self {
        let named_node: NamedNode = value.into();
        named_node.into()
    }
}

impl From<IriS> for Term {
    fn from(value: IriS) -> Self {
        let named_node: NamedNode = value.into();
        named_node.into()
    }
}

impl Default for IriS {
    fn default() -> Self {
        IriS::new_unchecked(&String::default())
    }
}

struct IriVisitor;

impl Visitor<'_> for IriVisitor {
    type Value = IriS;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an IRI")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match IriS::from_str(v) {
            Ok(iri) => Ok(iri),
            Err(IriSError::IriParseError { str, err }) => Err(E::custom(format!(
                "Error parsing value \"{v}\" as IRI. String \"{str}\", Error: {err}"
            ))),
            Err(other) => Err(E::custom(format!(
                "Can not parse value \"{v}\" to IRI. Error: {other}"
            ))),
        }
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

    #[test]
    fn from_str_base() {
        let iri1 = IriS::from_str_base("name", Some("http://example.org/")).unwrap();
        let iri2 = IriS::from_str_base("http://example.org/name", None).unwrap();
        assert_eq!(iri1, iri2);
    }
}
