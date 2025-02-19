use std::fmt::Display;
use std::str::FromStr;

use iri_s::IriS;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::Deref;
use crate::DerefError;
use crate::IriFromStrError;
use crate::PrefixMap;

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

    pub fn get_iri(&self) -> Result<&IriS, DerefError> {
        match self {
            IriRef::Iri(iri) => Ok(iri),
            IriRef::Prefixed { prefix, local } => {
                Err(DerefError::Underef(prefix.to_string(), local.to_string()))
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
    type Error = IriFromStrError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        FromStr::from_str(value)
    }
}

impl FromStr for IriRef {
    type Err = IriFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match IriS::from_str(s) {
            Ok(iri) => Ok(IriRef::Iri(iri)),
            Err(_) => Err(IriFromStrError(s.to_string())),
        }
    }
}

impl From<String> for IriRef {
    fn from(value: String) -> Self {
        IriRef::from_str(&value).unwrap()
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
