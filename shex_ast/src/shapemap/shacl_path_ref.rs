use prefixmap::error::PrefixMapError;
use prefixmap::{DerefIri, IriRef, PrefixMap};
use rudof_rdf::rdf_core::SHACLPath;
use serde::Serialize;

/// SHACLPathRef is similar to SHACLPath but uses IriRef for predicates
/// This is useful when parsing ShapeMaps where predicates may be given as prefixed names
/// And there is no prefixmap to resolve them yet
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum SHACLPathRef {
    Predicate { pred: IriRef },
    Alternative { paths: Vec<SHACLPathRef> },
    Sequence { paths: Vec<SHACLPathRef> },
    Inverse { path: Box<SHACLPathRef> },
    ZeroOrMore { path: Box<SHACLPathRef> },
    OneOrMore { path: Box<SHACLPathRef> },
    ZeroOrOne { path: Box<SHACLPathRef> },
}

impl SHACLPathRef {
    pub fn predicate(pred: IriRef) -> Self {
        SHACLPathRef::Predicate { pred }
    }

    pub fn to_shaclpath(&self, prefixmap: &PrefixMap) -> Result<SHACLPath, PrefixMapError> {
        match self {
            SHACLPathRef::Predicate { pred } => {
                let iri = prefixmap.resolve_iriref(pred.clone())?;
                Ok(SHACLPath::iri(iri))
            },
            SHACLPathRef::Alternative { paths } => {
                let spaths = paths
                    .iter()
                    .map(|p| p.to_shaclpath(prefixmap))
                    .collect::<Result<Vec<SHACLPath>, PrefixMapError>>()?;
                Ok(SHACLPath::alternative(spaths))
            },
            SHACLPathRef::Sequence { paths } => {
                let spaths = paths
                    .iter()
                    .map(|p| p.to_shaclpath(prefixmap))
                    .collect::<Result<Vec<SHACLPath>, PrefixMapError>>()?;
                Ok(SHACLPath::sequence(spaths))
            },
            SHACLPathRef::Inverse { path } => {
                let spath = path.to_shaclpath(prefixmap)?;
                Ok(SHACLPath::inverse(spath))
            },
            SHACLPathRef::ZeroOrMore { path } => {
                let spath = path.to_shaclpath(prefixmap)?;
                Ok(SHACLPath::zero_or_more(spath))
            },
            SHACLPathRef::OneOrMore { path } => {
                let spath = path.to_shaclpath(prefixmap)?;
                Ok(SHACLPath::one_or_more(spath))
            },
            SHACLPathRef::ZeroOrOne { path } => {
                let spath = path.to_shaclpath(prefixmap)?;
                Ok(SHACLPath::zero_or_one(spath))
            },
        }
    }
}

impl DerefIri for SHACLPathRef {
    fn deref_iri(self, base: Option<&iri_s::IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, prefixmap::DerefError>
    where
        Self: Sized,
    {
        match self {
            SHACLPathRef::Predicate { pred } => {
                let pred = pred.deref_iri(base, prefixmap)?;
                Ok(SHACLPathRef::Predicate { pred })
            },
            SHACLPathRef::Alternative { paths } => {
                let paths = paths
                    .into_iter()
                    .map(|p| p.deref_iri(base, prefixmap))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(SHACLPathRef::Alternative { paths })
            },
            SHACLPathRef::Sequence { paths } => {
                let paths = paths
                    .into_iter()
                    .map(|p| p.deref_iri(base, prefixmap))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(SHACLPathRef::Sequence { paths })
            },
            SHACLPathRef::Inverse { path } => {
                let path = path.deref_iri(base, prefixmap)?;
                Ok(SHACLPathRef::Inverse { path })
            },
            SHACLPathRef::ZeroOrMore { path } => {
                let path = path.deref_iri(base, prefixmap)?;
                Ok(SHACLPathRef::ZeroOrMore { path })
            },
            SHACLPathRef::OneOrMore { path } => {
                let path = path.deref_iri(base, prefixmap)?;
                Ok(SHACLPathRef::OneOrMore { path })
            },
            SHACLPathRef::ZeroOrOne { path } => {
                let path = path.deref_iri(base, prefixmap)?;
                Ok(SHACLPathRef::ZeroOrOne { path })
            },
        }
    }
}
