use crate::PrefixMap;
use crate::{Deref, DerefError, PrefixMapError};
use iri_s::{IriS, IriSError};
use serde::de::Visitor;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Serialize, Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
#[serde(into = "String")]
pub enum IriRef {
    Iri(IriS),
    Prefixed { prefix: String, local: String },
}

#[derive(Debug, Error, Clone)]
#[error("Cannot obtain IRI from prefixed name IriRef {prefix}:{local}")]
pub struct IriRefError {
    prefix: String,
    local: String,
}

impl IriRef {
    /// Tries to get the IRI, returns an error if it is a prefixed name
    /// Usually you want to use get_iri_prefixmap instead
    pub fn get_iri(&self) -> Result<IriS, IriRefError> {
        match self {
            IriRef::Iri(iri) => Ok(iri.clone()),
            IriRef::Prefixed { prefix, local } => Err(IriRefError {
                prefix: prefix.clone(),
                local: local.clone(),
            }),
        }
    }

    /// Gets the IRI, resolving prefixed names using the provided PrefixMap
    pub fn get_iri_prefixmap(&self, prefixmap: &PrefixMap) -> Result<IriS, PrefixMapError> {
        match self {
            IriRef::Iri(iri) => Ok(iri.clone()),
            IriRef::Prefixed { prefix, local } => prefixmap.resolve_prefix_local(prefix, local),
        }
    }

    /// Creates a prefixed name IriRef from the given prefix and local part
    pub fn prefixed(prefix: &str, local: &str) -> IriRef {
        IriRef::Prefixed {
            prefix: prefix.to_string(),
            local: local.to_string(),
        }
    }

    /// Creates an IriRef from an IriS
    pub fn iri(iri: IriS) -> IriRef {
        IriRef::Iri(iri)
    }
}

impl Deref for IriRef {
    fn deref(
        &self,
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Self, DerefError> {
        match self {
            IriRef::Iri(iri_s) => match base {
                None => Ok(IriRef::Iri(iri_s.clone())),
                Some(base_iri) => {
                    let iri = base_iri.resolve(iri_s.clone())?;
                    Ok(IriRef::Iri(iri))
                }
            },
            IriRef::Prefixed { prefix, local } => match prefixmap {
                None => Err(DerefError::NoPrefixMapPrefixedName {
                    prefix: prefix.clone(),
                    local: local.clone(),
                }),
                Some(prefixmap) => {
                    let iri = prefixmap.resolve_prefix_local(prefix, local).map_err(|e| {
                        DerefError::DerefPrefixMapError {
                            alias: prefix.to_string(),
                            local: local.to_string(),
                            error: Box::new(e),
                        }
                    })?;
                    Ok(IriRef::Iri(iri))
                }
            },
        }
    }
}

impl TryFrom<&str> for IriRef {
    type Error = IriSError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        FromStr::from_str(value)
    }
}

impl FromStr for IriRef {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iri_s = IriS::from_str(s)?;
        Ok(IriRef::Iri(iri_s))
    }
}

impl From<IriRef> for IriS {
    fn from(iri_ref: IriRef) -> IriS {
        match iri_ref {
            IriRef::Iri(iri_s) => iri_s,
            IriRef::Prefixed { prefix, local } => {
                panic!("Cannot convert prefixed name {prefix}:{local} to IriS without context")
            }
        }
    }
}

impl From<IriS> for IriRef {
    fn from(i: IriS) -> IriRef {
        IriRef::Iri(i)
    }
}

impl From<IriRef> for String {
    fn from(iri_ref: IriRef) -> String {
        match iri_ref {
            IriRef::Iri(i) => i.as_str().to_string(),
            IriRef::Prefixed { prefix, local } => format!("{prefix}:{local}"),
        }
    }
}

impl Display for IriRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IriRef::Iri(i) => write!(f, "{i}")?,
            IriRef::Prefixed { prefix, local } => write!(f, "{prefix}:{local}")?,
        }
        Ok(())
    }
}

struct IriRefVisitor;

impl<'de> Visitor<'de> for IriRefVisitor {
    type Value = IriRef;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid IRI or a prefixed name")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match IriRef::from_str(v) {
            Ok(iri_ref) => Ok(iri_ref),
            Err(_) => {
                // Try to parse as prefixed name
                let parts: Vec<&str> = v.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let prefix = parts[0].to_string();
                    let local = parts[1].to_string();
                    Ok(IriRef::Prefixed { prefix, local })
                } else {
                    Err(E::custom(format!("Invalid IRI or prefixed name: {}", v)))
                }
            }
        }
    }
}

impl<'de> Deserialize<'de> for IriRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(IriRefVisitor)
    }
}
