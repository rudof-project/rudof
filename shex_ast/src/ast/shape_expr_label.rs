use std::{fmt::Display, str::FromStr};

use iri_s::{IriS, IriSError};
use regex::Regex;
use serde::{Deserialize, Serialize};

use prefixmap::{Deref, DerefError, IriRef};
use thiserror::Error;

use super::bnode::BNode;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(try_from = "&str", into = "String")]
pub enum ShapeExprLabel {
    IriRef { value: IriRef },
    BNode { value: BNode },
    Start,
}

impl ShapeExprLabel {
    pub fn iri_unchecked(s: &str) -> Self {
        ShapeExprLabel::IriRef {
            value: IriS::new_unchecked(s).into(),
        }
    }

    pub fn iri_ref(i: IriRef) -> Self {
        ShapeExprLabel::IriRef { value: i }
    }

    pub fn iri(iri: IriS) -> Self {
        ShapeExprLabel::IriRef {
            value: IriRef::iri(iri),
        }
    }

    pub fn bnode(bn: BNode) -> Self {
        ShapeExprLabel::BNode { value: bn }
    }

    pub fn prefixed(alias: &str, local: &str) -> Self {
        ShapeExprLabel::IriRef {
            value: IriRef::prefixed(alias, local),
        }
    }

    pub fn start() -> Self {
        ShapeExprLabel::Start
    }
}

impl Deref for ShapeExprLabel {
    fn deref(
        &self,
        base: &Option<iri_s::IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        match self {
            ShapeExprLabel::IriRef { value } => {
                let new_value = value.deref(base, prefixmap)?;
                Ok(ShapeExprLabel::IriRef { value: new_value })
            }
            ShapeExprLabel::BNode { value } => Ok(ShapeExprLabel::BNode {
                value: value.clone(),
            }),
            ShapeExprLabel::Start => Ok(ShapeExprLabel::Start),
        }
    }
}

impl TryFrom<&str> for ShapeExprLabel {
    type Error = RefError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let re_iri = Regex::new(r"<(.*)>").unwrap();
        if let Some(iri_str) = re_iri.captures(s) {
            let iri_s = IriS::from_str(&iri_str[1])?;
            Ok(ShapeExprLabel::IriRef {
                value: IriRef::iri(iri_s),
            })
        } else {
            let re_bnode = Regex::new(r"_:(.*)").unwrap();
            if let Some(bnode_s) = re_bnode.captures(s) {
                Ok(ShapeExprLabel::BNode {
                    value: BNode::new(&bnode_s[1]),
                })
            } else {
                let iri_s = IriS::from_str(s)?;
                Ok(ShapeExprLabel::IriRef {
                    value: IriRef::iri(iri_s),
                })
            }
        }
    }
}

impl Display for ShapeExprLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ShapeExprLabel::IriRef { value } => value.to_string(),
            ShapeExprLabel::BNode { value } => value.to_string(),
            ShapeExprLabel::Start => "START".to_string(),
        };
        write!(f, "{str}")
    }
}

impl From<ShapeExprLabel> for String {
    fn from(val: ShapeExprLabel) -> Self {
        val.to_string()
    }
}

impl From<&ShapeExprLabel> for String {
    fn from(val: &ShapeExprLabel) -> Self {
        val.to_string()
    }
}

impl FromStr for ShapeExprLabel {
    type Err = RefError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}

#[derive(Error, Debug)]
pub enum RefError {
    #[error("Cannot parse as IriS")]
    IriSError(#[from] IriSError),

    #[error("Cannot parse as Iri or BNode: {str}")]
    BadRef { str: String },
}
