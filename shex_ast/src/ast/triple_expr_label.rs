use serde_derive::{Deserialize, Serialize};

use super::bnode::BNode;
use super::iri_ref::IriRef;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(try_from = "&str", into = "String")]
pub enum TripleExprLabel {
    IriRef { value: IriRef },
    BNode { value: BNode },
}

#[derive(Debug, Clone)]
pub struct FromStrTripleExprLabelError;

impl Display for FromStrTripleExprLabelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error converting TripleExprLabel")
    }
}

impl TryFrom<&str> for TripleExprLabel {
    type Error = FromStrTripleExprLabelError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(TripleExprLabel::IriRef {
            value: IriRef {
                value: s.to_string(),
            },
        })
    }
}

impl Into<String> for TripleExprLabel {
    fn into(self) -> String {
        match self {
            TripleExprLabel::IriRef { value } => value.into(),
            TripleExprLabel::BNode { value } => value.into(),
        }
    }
}
