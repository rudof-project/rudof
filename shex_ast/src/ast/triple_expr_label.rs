use iri_s::IriSError;
use serde_derive::{Deserialize, Serialize};

use super::bnode::BNode;
use super::iri_ref::IriRef;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(try_from = "&str", into = "String")]
pub enum TripleExprLabel {
    IriRef { value: IriRef },
    BNode { value: BNode },
}


impl TryFrom<&str> for TripleExprLabel {
    type Error = IriSError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let iri_ref = IriRef::try_from(s)?;
        Ok(TripleExprLabel::IriRef {
            value: iri_ref,
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
