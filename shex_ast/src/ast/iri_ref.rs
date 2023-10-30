use std::{fmt::Display, str::FromStr};

use iri_s::{IriS, IriSError};
use prefixmap::PrefixMap;
use serde_derive::{Deserialize, Serialize};
use void::Void;
use crate::{Deref, DerefError};

#[derive(Deserialize, Serialize, Debug, PartialEq, Hash, Eq, Clone)]
#[serde(try_from = "&str", into = "String")]
pub enum IriRef {
    Iri(IriS),
    Prefixed { prefix: String, local: String },
}

impl IriRef {
    pub fn prefixed(prefix: &str, local: &str) -> IriRef {
        IriRef::Prefixed {
            prefix: prefix.to_string(),
            local: local.to_string(),
        }
    }

    pub fn iri(iri: IriS) -> IriRef {
        IriRef::Iri(iri)
    }

    pub fn to_string(&self) -> String {
        match self {
            IriRef::Iri(iri) => iri.as_str().to_string(),
            IriRef::Prefixed { prefix, local } => {
                format!("{prefix}:{local}")
            }
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
            IriRef::Prefixed { prefix, local } => {
                match prefixmap {
                    None => Err(DerefError::NoPrefixMapPrefixedName { 
                        prefix: prefix.clone(), 
                        local: local.clone() }
                    ),
                    Some(prefixmap) => {
                        let iri = prefixmap.resolve_prefix_local(prefix, local)?;
                        Ok(IriRef::Iri(iri))
                    }
                }
            }
        }
    }
}

impl TryFrom<&str> for IriRef {
    type Error = IriSError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let iri_s = IriS::from_str(s)?;
        Ok(IriRef::Iri(iri_s))
    }
}

impl Into<IriS> for IriRef {
    fn into(self) -> IriS {
        match self {
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

impl Into<String> for IriRef {
    fn into(self) -> String {
        match self {
            IriRef::Iri(i) => i.as_str().to_string(),
            IriRef::Prefixed { prefix, local } => format!("{prefix}:{local}"),
        }
    }
}

impl Display for IriRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IriRef::Iri(i) => write!(f, "{}", i.to_string())?,
            IriRef::Prefixed { prefix, local } => write!(f, "{prefix}:{local}")?,
        }
        Ok(())
    }
}

