use std::str::FromStr;
use regex::Regex;
use iri_s::{IriS, IriSError};
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

use crate::{IriRef, Deref, DerefError};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(try_from = "&str", into = "String")]
pub enum Ref {
    IriRef { value: IriRef },
    BNode { value: String },
}

impl Ref {
    pub fn iri_unchecked(s: &str) -> Ref {
        Ref::IriRef {
            value: IriS::new_unchecked(s).into(),
        }
    }

    pub fn iri_ref(i: IriRef) -> Ref {
        Ref::IriRef { value: i }
    }

    pub fn bnode_unchecked(s: &str) -> Ref {
        Ref::BNode {
            value: s.to_string(),
        }
    }
}

impl Into<String> for Ref {
    fn into(self) -> String {
        match self {
            Ref::IriRef { value } => value.to_string(),
            Ref::BNode { value } => {
                format!("_:{value}")
            },
        }
    }
}

impl Deref for Ref {
    fn deref(&self, 
        base: &Option<IriS>, 
        prefixmap: &Option<prefixmap::PrefixMap>
    ) -> Result<Self, DerefError> {
        match self {
            Ref::IriRef { value } => {
                let value = value.deref(base, prefixmap)?;
                Ok(Ref::IriRef { value })
            },
            Ref::BNode { value } => {
                Ok(Ref::BNode { value: value.clone() })
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum RefError {
   #[error(transparent)]
   IriSError(#[from] IriSError),

   #[error("Cannot parse as Iri or BNode: {str}")]
   BadRef{ str: String }

}

impl TryFrom<&str> for Ref {
    type Error = RefError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let re_iri = Regex::new(r"<(.*)>").unwrap();
        if let Some(iri_str) = re_iri.captures(s) {
            let iri_s = IriS::from_str(&iri_str[1])?;
            Ok(Ref::IriRef {
                value: IriRef::iri(iri_s),
            })
        } else {
            let re_bnode = Regex::new(r"_:(.*)").unwrap();
            if let Some(bnode_s) = re_bnode.captures(s) {
                Ok(Ref::BNode { value: bnode_s[1].to_string() })
            } else {
                let iri_s = IriS::from_str(s)?;
                Ok(Ref::IriRef {
                    value: IriRef::iri(iri_s),
                })
            }
        }
    }
}

impl From<IriS> for Ref {
    fn from(iri: IriS) -> Ref {
        Ref::iri_unchecked(iri.as_str())
    }
}

impl FromStr for Ref {
    type Err = RefError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}
