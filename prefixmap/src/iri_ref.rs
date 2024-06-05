use std::{fmt::Display, str::FromStr};

use crate::PrefixMap;
use crate::{Deref, DerefError};
use iri_s::{IriS, IriSError};
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize, Debug, PartialEq, Hash, Eq, Clone)]
#[serde(try_from = "&str", into = "String")]
pub enum IriRef {
    Iri(IriS),
    Prefixed { prefix: String, local: String },
}

#[derive(Debug, Error)]
#[error("Cannot obtain IRI from prefixed name IriRef {prefix}:{local}")]
pub struct Underef {
    prefix: String,
    local: String,
}

impl IriRef {
    pub fn get_iri(&self) -> Result<IriS, Underef> {
        match self {
            IriRef::Iri(iri) => Ok(iri.clone()),
            IriRef::Prefixed { prefix, local } => Err(Underef {
                prefix: prefix.clone(),
                local: local.clone(),
            }),
        }
    }
    pub fn prefixed(prefix: &str, local: &str) -> IriRef {
        IriRef::Prefixed {
            prefix: prefix.to_string(),
            local: local.to_string(),
        }
    }

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
                    let iri = prefixmap.resolve_prefix_local(prefix, local)?;
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
