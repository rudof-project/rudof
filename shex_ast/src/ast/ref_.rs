use iri_s::{IriS, IriSError};
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

use crate::{BNode, Deref, DerefError, IriRef};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(try_from = "&str", into = "String")]
pub enum Ref {
    IriRef { value: IriRef },
    BNode { value: BNode },
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

    pub fn bnode(bn: BNode) -> Ref {
        Ref::BNode { value: bn }
    }
}

impl Into<String> for Ref {
    fn into(self) -> String {
        match self {
            Ref::IriRef { value } => value.to_string(),
            Ref::BNode { value } => {
                format!("{value}")
            }
        }
    }
}

impl Deref for Ref {
    fn deref(
        &self,
        base: &Option<IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError> {
        match self {
            Ref::IriRef { value } => {
                let value = value.deref(base, prefixmap)?;
                Ok(Ref::IriRef { value })
            }
            Ref::BNode { value } => Ok(Ref::BNode {
                value: value.clone(),
            }),
        }
    }
}

#[derive(Error, Debug)]
pub enum RefError {
    #[error("Cannot pase as IriS")]
    IriSError(#[from] IriSError),

    #[error("Cannot parse as Iri or BNode: {str}")]
    BadRef { str: String },
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
                Ok(Ref::BNode {
                    value: BNode::new(&bnode_s[1]),
                })
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
