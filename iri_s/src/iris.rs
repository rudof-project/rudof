use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs;
use std::ops::Deref;
use std::str::FromStr;

use oxiri::Iri;
use reqwest::header;
use reqwest::header::USER_AGENT;
use serde::de;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use url::Url;

use crate::error::GenericIriError;

pub type IriS = GenericIri<String>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericIri<T> {
    iri: Iri<T>,
}

impl<T: Deref<Target = str>> GenericIri<T> {
    pub fn new_unchecked(str: T) -> Self {
        Self {
            iri: Iri::parse_unchecked(str),
        }
    }

    pub fn as_ref(&self) -> GenericIri<&str> {
        GenericIri::<&str> {
            iri: self.iri.as_ref(),
        }
    }

    pub fn as_str(&self) -> &str {
        self.iri.as_str()
    }

    /// [Dereference](https://www.w3.org/wiki/DereferenceURI) the IRI and get
    /// the content available from it. It handles also IRIs with the `file`
    ///  scheme as local file names. For example: `file:///person.txt`
    pub fn dereference(&self, base: &Option<GenericIri<T>>) -> Result<String, GenericIriError> {
        let url = match base {
            Some(base_iri) => {
                let base =
                    Url::from_str(base_iri.as_str()).map_err(|e| GenericIriError::UrlParse {
                        str: self.iri.as_str().to_string(),
                        error: format!("{e}"),
                    })?;
                Url::options()
                    .base_url(Some(&base))
                    .parse(self.iri.as_str())
                    .map_err(|e| GenericIriError::IriWithBaseParse {
                        str: self.iri.as_str().to_string(),
                        base: base.to_string(),
                        err: e.to_string(),
                    })?
            }
            None => Url::from_str(self.iri.as_str()).map_err(|e| GenericIriError::UrlParse {
                str: self.iri.as_str().to_string(),
                error: format!("{e}"),
            })?,
        };
        match url.scheme() {
            "file" => {
                let path =
                    url.to_file_path()
                        .map_err(|_| GenericIriError::ConvertingFileUrlToPath {
                            url: format!("{url}"),
                        })?;
                let path_name = path.to_string_lossy().to_string();
                let body = fs::read_to_string(path).map_err(|e| GenericIriError::IO {
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
                    .map_err(|e| GenericIriError::ReqwestClientCreation {
                        error: format!("{e}"),
                    })?;
                let body = client
                    .get(url)
                    .send()
                    .map_err(|e| GenericIriError::Reqwest {
                        error: format!("{e}"),
                    })?
                    .text()
                    .map_err(|e| GenericIriError::ReqwestText {
                        error: format!("{e}"),
                    })?;
                Ok(body)
            }
        }
    }

    /// Parse a string as an IRI, with this IRI as the base.
    pub fn join(&self, str: &str) -> Result<GenericIri<String>, GenericIriError> {
        let url = Url::from_str(self.as_str()).map_err(|e| GenericIriError::IriParse {
            str: str.to_string(),
            err: e.to_string(),
        })?;
        let joined = url
            .join(str)
            .map_err(|e| GenericIriError::Join {
                str: str.to_string(),
                current: self.to_string(),
                err: e.to_string(),
            })?
            .to_string();
        Ok(GenericIri::<String>::new_unchecked(joined))
    }

    /// Extends the current IRI with a new string
    ///
    /// This function checks for possible errors returning a Result
    pub fn extend(&self, str: &str) -> Result<GenericIri<String>, GenericIriError> {
        let current_str = self.iri.as_str();
        let extended_str = if current_str.ends_with('/') || current_str.ends_with('#') {
            format!("{}{}", current_str, str)
        } else {
            format!("{}/{}", current_str, str)
        };
        let iri = match Iri::parse(extended_str.clone()) {
            Ok(iri) => iri,
            Err(e) => {
                return Err(GenericIriError::IriParse {
                    str: extended_str,
                    err: e.to_string(),
                })
            }
        };
        Ok(GenericIri::<String> { iri })
    }

    /// Extend an IRI with a new string without checking for possible syntactic
    /// errors
    pub fn extend_unchecked(&self, str: &str) -> GenericIri<String> {
        let extended_str = format!("{}{}", self.iri.as_str(), str);
        GenericIri::<String>::new_unchecked(extended_str)
    }

    /// Resolve the IRI `other` with this IRI
    pub fn resolve(&self, other: Self) -> Result<GenericIri<String>, GenericIriError> {
        let iri = self.iri.resolve_unchecked(other.as_str());
        Ok(GenericIri::<String> { iri })
    }
}

impl<T: Deref<Target = str>> Display for GenericIri<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.iri.as_str())
    }
}

impl<T: Deref<Target = str>> Serialize for GenericIri<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.iri.as_str())
    }
}

impl FromStr for GenericIri<String> {
    type Err = GenericIriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iri = Iri::parse(s).map_err(|e| GenericIriError::IriParse {
            str: s.to_string(),
            err: e.to_string(),
        })?;
        Ok(Self { iri: iri.into() })
    }
}

impl Default for GenericIri<String> {
    fn default() -> Self {
        Self::new_unchecked(String::default())
    }
}

struct IriVisitor;

impl<'de> Visitor<'de> for IriVisitor {
    type Value = GenericIri<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an IRI")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        match Self::Value::from_str(v) {
            Ok(iri) => Ok(iri),
            Err(GenericIriError::IriParse { str, err }) => Err(E::custom(format!(
                "Error parsing value \"{v}\" as IRI. String \"{str}\", Error: {err}"
            ))),
            Err(other) => Err(E::custom(format!(
                "Can not parse value \"{v}\" to IRI. Error: {other}"
            ))),
        }
    }
}

impl<'de> Deserialize<'de> for GenericIri<String> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(IriVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_iris() {
        let iri = GenericIri::<String>::from_str("http://example.org/").unwrap();
        assert_eq!(iri.to_string(), "http://example.org/");
    }

    #[test]
    fn obtaining_iri_as_str() {
        let iri = GenericIri::<String>::from_str("http://example.org/p1").unwrap();
        assert_eq!(iri.as_str(), "http://example.org/p1");
    }

    #[test]
    fn extending_iri() {
        let base_iri = GenericIri::<String>::from_str("http://example.org/").unwrap();
        let extended = base_iri.extend("knows").unwrap();
        assert_eq!(extended.as_str(), "http://example.org/knows");
    }

    #[test]
    fn comparing_iris() {
        let iri1 = GenericIri::<String>::from_str("http://example.org/name").unwrap();
        let iri2 = GenericIri::<String>::from_str("http://example.org/name").unwrap();
        assert_eq!(iri1, iri2);
    }
}
