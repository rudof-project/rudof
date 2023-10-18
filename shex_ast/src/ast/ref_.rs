use std::str::FromStr;

use iri_s::IriS;
use serde_derive::{Deserialize, Serialize};
use void::Void;

use super::FromStrRefError;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum Ref {
    IriRef { value: String },
    BNode { value: String },
}

impl Ref {
    pub fn iri_unchecked(s: &str) -> Ref {
        Ref::IriRef {
            value: s.to_string(),
        }
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
            Ref::IriRef { value } => value,
            Ref::BNode { value } => value,
        }
    }
}

impl TryFrom<String> for Ref {
    type Error = Void;

    // TODO: We should parse the bnode
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Ref::IriRef { value: s })
    }
}

impl From<IriS> for Ref {
    fn from(iri: IriS) -> Ref {
        Ref::iri_unchecked(iri.as_str())
    }
}

impl FromStr for Ref {
    type Err = FromStrRefError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ref::IriRef {
            value: s.to_string(),
        })
    }
}
