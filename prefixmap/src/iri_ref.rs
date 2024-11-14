use std::fmt::Display;
use std::str::FromStr;

use api::Iri;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use thiserror::Error;

use crate::Deref;
use crate::DerefError;
use crate::PrefixMap;

#[derive(Deserialize, Serialize, Debug, PartialEq, Hash, Eq, Clone)]
#[serde(try_from = "I", into = "String")]
pub enum IriRef<I: Iri + Clone> {
    Iri(I),
    Prefixed { prefix: String, local: String },
}

impl<I: Iri + Clone> IriRef<I> {
    pub fn prefixed(prefix: &str, local: &str) -> IriRef<I> {
        IriRef::Prefixed {
            prefix: prefix.to_string(),
            local: local.to_string(),
        }
    }

    pub fn iri(iri: I) -> IriRef<I> {
        IriRef::Iri(iri)
    }

    pub fn get_iri(&self) -> Result<&I, Underef> {
        match self {
            IriRef::Iri(iri) => Ok(iri),
            IriRef::Prefixed { prefix, local } => Err(Underef {
                prefix: prefix.to_string(),
                local: local.to_string(),
            }),
        }
    }
}

impl<I: Iri + Clone> Deref<I> for IriRef<I> {
    fn deref(
        &self,
        base: &Option<I>,
        prefixmap: &Option<PrefixMap<I>>,
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

#[derive(Error, Debug, Clone)]
#[error("Error parsing the {} IRI from a String", ._0)]
pub struct IriFromStrError(String);

impl<I: Iri + Clone + FromStr> FromStr for IriRef<I> {
    type Err = IriFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match I::from_str(s) {
            Ok(iri) => Ok(IriRef::Iri(iri)),
            Err(_) => Err(IriFromStrError(s.to_string())),
        }
    }
}

impl<I: Iri + Clone + FromStr> From<String> for IriRef<I> {
    fn from(value: String) -> Self {
        IriRef::from_str(&value).unwrap()
    }
}

impl<I: Iri + Clone + Display> Display for IriRef<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IriRef::Iri(i) => write!(f, "{i}")?,
            IriRef::Prefixed { prefix, local } => write!(f, "{prefix}:{local}")?,
        }
        Ok(())
    }
}
