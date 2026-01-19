use crate::error::IriSError;
use oxiri::Iri;
use oxrdf::{NamedNode, NamedOrBlankNode, Term};
use serde::{Serialize, Serializer};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::canonicalize;
use std::path::Path;
use std::str::FromStr;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IriS {
    iri: NamedNode,
}

impl IriS {
    pub fn rdf_type() -> IriS {
        IriS::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")
    }

    pub fn from_url(url: &Url) -> IriS {
        let iri = NamedNode::new_unchecked(url.as_str());
        IriS { iri }
    }

    pub fn from_path(path: &Path) -> Result<IriS, IriSError> {
        let abs_path = if path.is_absolute() {
            Ok(path.to_path_buf())
        } else {
            canonicalize(path).map_err(|e| IriSError::ConvertingPathToIri {
                path: path.to_string_lossy().to_string(),
                error: e.to_string(),
            })
        }?;

        let url = Url::from_file_path(&abs_path).map_err(|_| IriSError::ConvertingPathToIri {
            path: abs_path.to_string_lossy().to_string(),
            error: "Cannot convert path to file URL".to_string(),
        })?;

        let iri = NamedNode::new(url.as_str()).map_err(|e| IriSError::IriParseError {
            str: url.as_str().to_string(),
            error: e.to_string(),
        })?;

        Ok(IriS { iri })
    }

    /// Convert a `NamedNode` to an `IriS`
    pub fn from_named_node(iri: &NamedNode) -> IriS {
        IriS { iri: iri.clone() }
    }

    pub fn from_str_base(str: &str, base: Option<&str>) -> Result<IriS, IriSError> {
        match base {
            Some(base_str) => IriS::from_str(base_str)?.resolve_str(str),
            None => IriS::from_str(str),
        }
    }

    pub fn from_str_base_iri(str: &str, base_iri: &Option<IriS>) -> Result<IriS, IriSError> {
        match base_iri {
            Some(base_iri) => base_iri.resolve_str(str),
            None => IriS::from_str(str),
        }
    }

    /// Get the IRI as a string slice
    pub fn as_str(&self) -> &str {
        self.iri.as_str()
    }

    /// Convert an `IriS` to a `NamedNode`
    pub fn as_named_node(&self) -> &NamedNode {
        &self.iri
    }

    pub fn new_unchecked(str: &str) -> IriS {
        let iri = NamedNode::new_unchecked(str);
        IriS { iri }
    }

    pub fn join(&self, str: &str) -> Result<Self, IriSError> {
        let url = Url::from_str(self.as_str()).map_err(|e| IriSError::IriParseError {
            str: str.to_string(),
            error: e.to_string(),
        })?;

        let joined = url.join(str).map_err(|e| IriSError::JoinError {
            str: str.to_string(),
            current: Box::new(self.clone()),
            error: e.to_string(),
        })?;

        Ok(IriS::new_unchecked(joined.as_str()))
    }

    /// Extends the current IRI with a new string
    ///
    /// This function checks for possible errors returning a Result
    pub fn extend(&self, str: &str) -> Result<Self, IriSError> {
        let current_str = self.iri.as_str();
        let extend_str = if current_str.ends_with("/") || current_str.ends_with("#") {
            format!("{current_str}{str}")
        } else {
            format!("{current_str}/{str}")
        };

        let iri = NamedNode::new(extend_str.as_str()).map_err(|e| IriSError::IriParseError {
            str: extend_str,
            error: e.to_string(),
        })?;

        Ok(IriS { iri })
    }

    /// Extend an IRI with a new string without checking for possible syntactic errors
    pub fn extend_unchecked(&self, str: &str) -> Self {
        let extended_str = format!("{}{}", self.iri.as_str(), str);
        let iri = NamedNode::new_unchecked(extended_str);

        IriS { iri }
    }

    /// Resolve the IRI `other` with this IRI
    pub fn resolve(&self, other: IriS) -> Result<Self, IriSError> {
        let resolved = self.resolve_iri(other.as_str())?;
        let iri = IriS::namednode_from_iri(&resolved)?;
        Ok(IriS { iri })
    }

    /// Resolve `other` with this IRI
    pub fn resolve_str(&self, other: &str) -> Result<Self, IriSError> {
        let resolved = self.resolve_iri(other)?;
        let iri = IriS::namednode_from_iri(&resolved)?;
        Ok(IriS { iri })
    }

    /// Resolve `other` with this IRI
    pub fn resolve_iri(&self, other: &str) -> Result<Iri<String>, IriSError> {
        let base = Iri::parse(self.as_str()).map_err(|e| IriSError::IriParseError {
            str: self.to_string(),
            error: e.to_string(),
        })?;

        base.resolve(other).map_err(|e| IriSError::IriResolveError {
            error: e.to_string(),
            base: Box::new(IriS::new_unchecked(base.as_str())),
            other: other.to_string(),
        })
    }

    pub fn namednode_from_iri(iri: &Iri<String>) -> Result<NamedNode, IriSError> {
        NamedNode::new(iri.as_str()).map_err(|e| IriSError::IriParseError {
            str: iri.as_str().to_string(),
            error: e.to_string(),
        })
    }

    /// [Dereference](https://www.w3.org/wiki/DereferenceURI) the IRI and get the content available from it
    /// It handles also IRIs with the `file` scheme as local file names. For example: `file:///person.txt`
    #[cfg(not(target_family = "wasm"))]
    pub fn dereference(&self, base: &Option<IriS>) -> Result<String, IriSError> {
        use reqwest::blocking::Client;
        use reqwest::header;
        use reqwest::header::HeaderMap;
        use reqwest::header::USER_AGENT;
        use std::fs;

        let url = match base {
            Some(base_iri) => {
                let base =
                    Url::from_str(base_iri.as_str()).map_err(|e| IriSError::UrlParseError {
                        str: base_iri.to_string(),
                        error: e.to_string(),
                    })?;
                Url::options()
                    .base_url(Some(&base))
                    .parse(self.iri.as_str())
                    .map_err(|e| IriSError::IriParseErrorWithBase {
                        str: self.iri.as_str().to_string(),
                        base: format!("{base}"),
                        error: e.to_string(),
                    })?
            }
            None => Url::from_str(self.iri.as_str()).map_err(|e| IriSError::UrlParseError {
                str: self.iri.as_str().to_string(),
                error: e.to_string(),
            })?,
        };

        match url.scheme() {
            "file" => {
                let path = url
                    .to_file_path()
                    .map_err(|_| IriSError::ConvertingFileUrlToPath {
                        url: url.to_string(),
                    })?;
                let path_name = path.to_string_lossy().to_string();
                let body = fs::read_to_string(path).map_err(|e| IriSError::IOErrorFile {
                    path: path_name,
                    url: url.to_string(),
                    error: e.to_string(),
                })?;
                Ok(body)
            }
            _ => {
                let mut headers = HeaderMap::new();
                /* TODO: Add a parameter with the Accept header ?
                headers.insert(
                    ACCEPT,
                    header::HeaderValue::from_static(""),
                );*/
                headers.insert(USER_AGENT, header::HeaderValue::from_static("rudof"));
                let client = Client::builder()
                    .default_headers(headers)
                    .build()
                    .map_err(|e| IriSError::ReqwestClientCreation {
                        error: e.to_string(),
                    })?;
                let res = client
                    .get(url)
                    .send()
                    .map_err(|e| IriSError::ReqwestError {
                        error: e.to_string(),
                    })?
                    .text()
                    .map_err(|e| IriSError::ReqwestTextError {
                        error: e.to_string(),
                    })?;
                Ok(res)
            }
        }
    }

    #[cfg(target_family = "wasm")]
    pub fn dereference(&self, _base: &Option<IriS>) -> Result<String, IriSError> {
        Err(IriSError::ReqwestClientCreation {
            error: "rewquest is not enabled".to_string(),
        })
    }
}

impl Display for IriS {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
        let iri = NamedNode::new(s).map_err(|e| IriSError::IriParseError {
            str: s.to_string(),
            error: e.to_string(),
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
