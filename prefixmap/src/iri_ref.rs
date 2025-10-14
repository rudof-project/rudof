use crate::PrefixMap;
use crate::{Deref, DerefError, PrefixMapError};
use iri_s::{IriS, IriSError};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use thiserror::Error;
use tracing::trace;

#[derive(Deserialize, Serialize, Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
#[serde(try_from = "&str", into = "String")]
pub enum IriRef {
    Iri(IriS),
    Prefixed { prefix: String, local: String },
    RelativeIri(String),
}

#[derive(Debug, Error, Clone)]
#[error("Cannot obtain IRI from prefixed name IriRef {prefix}:{local}")]
pub enum IriRefError {
    #[error("Cannot obtain IRI from {prefix}:{local} without a PrefixMap")]
    IriRefPrefixedLocalError { prefix: String, local: String },
    #[error("Cannot obtain IRI from relative IriRef {str}")]
    IriRefRelativeError { str: String },
}

impl IriRef {
    /// Tries to get the IRI, returns an error if it is a prefixed name
    /// Usually you want to use get_iri_prefixmap instead
    pub fn get_iri(&self) -> Result<IriS, IriRefError> {
        match self {
            IriRef::Iri(iri) => Ok(iri.clone()),
            IriRef::Prefixed { prefix, local } => Err(IriRefError::IriRefPrefixedLocalError {
                prefix: prefix.clone(),
                local: local.clone(),
            }),
            IriRef::RelativeIri(str) => Err(IriRefError::IriRefRelativeError { str: str.clone() }),
        }
    }

    /// Gets the IRI, resolving prefixed names using the provided PrefixMap
    pub fn get_iri_prefixmap(
        &self,
        prefixmap: &PrefixMap,
        base: Option<&IriS>,
    ) -> Result<IriS, PrefixMapError> {
        match self {
            IriRef::Iri(iri) => Ok(iri.clone()),
            IriRef::Prefixed { prefix, local } => prefixmap.resolve_prefix_local(prefix, local),
            IriRef::RelativeIri(str) => {
                trace!("Resolving relative IRI {str} with base {base:?}");
                if let Some(base_iri) = base {
                    let resolved = base_iri.resolve_str(str)?;
                    Ok(resolved)
                } else {
                    Err(PrefixMapError::RelativeIriNoBase { str: str.clone() })
                }
            }
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

    pub fn from_str_base(str: &str, base: Option<&IriS>) -> Result<IriRef, IriSError> {
        if let Some((prefix, local)) = str.split_once(':') {
            if prefix
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
            {
                // Valid prefix
                return Ok(IriRef::Prefixed {
                    prefix: prefix.to_string(),
                    local: local.to_string(),
                });
            }
        }
        let iri_s = IriS::from_str(str)?;
        if let Some(base_iri) = base {
            let resolved = base_iri.resolve(iri_s)?;
            Ok(IriRef::Iri(resolved))
        } else {
            Ok(IriRef::Iri(iri_s))
        }
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
            IriRef::RelativeIri(str) => {
                if let Some(base_iri) = base {
                    let resolved = base_iri.resolve_str(str)?;
                    Ok(IriRef::Iri(resolved))
                } else {
                    Err(DerefError::NoBaseIriForRelative { str: str.clone() })
                }
            }
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
        trace!("Parsing IriRef from str: {s}");
        if let Ok(iri_s) = IriS::from_str(s) {
            trace!("Parsed as full IRI: {iri_s}");
            return Ok(IriRef::Iri(iri_s));
        } else {
            trace!("Not a full IRI, trying as prefixed name");
            if let Some((prefix, local)) = s.split_once(':') {
                trace!("Split into prefix: {prefix}, local: {local}");
                if prefix
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
                {
                    // Valid prefix
                    Ok(IriRef::Prefixed {
                        prefix: prefix.to_string(),
                        local: local.to_string(),
                    })
                } else {
                    Err(IriSError::InvalidPrefixedIri {
                        iri_ref: s.to_string(),
                        prefix: prefix.to_string(),
                        local: local.to_string(),
                    })
                }
            } else {
                Ok(IriRef::RelativeIri(s.to_string()))
            }
        }
    }
}

impl From<IriRef> for IriS {
    fn from(iri_ref: IriRef) -> IriS {
        match iri_ref {
            IriRef::Iri(iri_s) => iri_s,
            IriRef::Prefixed { prefix, local } => {
                panic!("Cannot convert prefixed name {prefix}:{local} to IriS without context")
            }
            IriRef::RelativeIri(_str) => {
                panic!("Cannot convert relative IriRef to IriS without context")
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
            IriRef::RelativeIri(str) => str.to_string(),
        }
    }
}

impl Display for IriRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IriRef::Iri(i) => write!(f, "{i}")?,
            IriRef::Prefixed { prefix, local } => write!(f, "{prefix}:{local}")?,
            IriRef::RelativeIri(str) => write!(f, "{str}")?,
        }
        Ok(())
    }
}
