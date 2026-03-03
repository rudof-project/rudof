use std::fmt::Display;

use prefixmap::{DerefIri, IriRef};
use serde::Serialize;

use crate::ObjectValue;

#[derive(Debug, PartialEq, Clone, Serialize)]

pub enum Pattern {
    Node(ObjectValue),
    Wildcard,
    Focus,
}

impl Pattern {
    pub fn focus() -> Self {
        Pattern::Focus
    }

    pub fn wildcard() -> Self {
        Pattern::Wildcard
    }

    pub fn node(obj: ObjectValue) -> Self {
        Pattern::Node(obj)
    }

    pub fn prefixed(prefix: &str, local: &str) -> Self {
        Pattern::Node(ObjectValue::iri_ref(IriRef::prefixed(prefix, local)))
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pattern::Node(object_value) => write!(f, "{object_value}"),
            Pattern::Wildcard => write!(f, "_"),
            Pattern::Focus => write!(f, "FOCUS"),
        }
    }
}

impl DerefIri for Pattern {
    fn deref_iri(
        self,
        base: Option<&iri_s::IriS>,
        prefixmap: Option<&prefixmap::PrefixMap>,
    ) -> Result<Self, prefixmap::DerefError>
    where
        Self: Sized,
    {
        match self {
            Pattern::Node(object_value) => {
                let deref = object_value.deref_iri(base, prefixmap)?;
                Ok(Pattern::Node(deref))
            },
            Pattern::Wildcard => Ok(Pattern::Wildcard),
            Pattern::Focus => Ok(Pattern::Focus),
        }
    }
}
