use iri_s::IriSError;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;

use prefixmap::{Deref, DerefError, IriRef};

use super::bnode::BNode;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(try_from = "&str", into = "String")]
pub enum TripleExprLabel {
    IriRef { value: IriRef },
    BNode { value: BNode },
}

impl Deref for TripleExprLabel {
    fn deref(
        &self,
        base: &Option<iri_s::IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        match self {
            TripleExprLabel::IriRef { value } => {
                let new_value = value.deref(base, prefixmap)?;
                Ok(TripleExprLabel::IriRef { value: new_value })
            }
            TripleExprLabel::BNode { value } => Ok(TripleExprLabel::BNode {
                value: value.clone(),
            }),
        }
    }
}

impl TryFrom<&str> for TripleExprLabel {
    type Error = IriSError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let iri_ref = IriRef::try_from(s)?;
        Ok(TripleExprLabel::IriRef { value: iri_ref })
    }
}

impl Display for TripleExprLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TripleExprLabel::IriRef { value } => value.to_string(),
            TripleExprLabel::BNode { value } => value.to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<TripleExprLabel> for String {
    fn from(val: TripleExprLabel) -> Self {
        val.to_string()
    }
}
