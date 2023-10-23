use std::{fmt::Display, str::FromStr};

use iri_s::{IriS, IriSError};
use prefixmap::PrefixMap;
use serde_derive::{Deserialize, Serialize};
use void::Void;

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

    pub fn deref(
        mut self,
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Self, IriSError> {
        self = match self {
            IriRef::Iri(iri_s) => match base {
                None => IriRef::Iri(iri_s),
                Some(base_iri) => {
                    let iri = base_iri.resolve(iri_s)?;
                    IriRef::Iri(iri)
                }
            },
            IriRef::Prefixed { prefix, local } => {
                todo!()
            }
        };
        Ok(self)
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
            IriRef::Iri(i) => i.to_string(),
            IriRef::Prefixed { prefix, local } => format!("{prefix}:{local}"),
        }
    }
}

impl Display for IriRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IriRef::Iri(i) => write!(f, "<{}>", i.to_string())?,
            IriRef::Prefixed { prefix, local } => write!(f, "{prefix}:{local}")?,
        }
        Ok(())
    }
}
