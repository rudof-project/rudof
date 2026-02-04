use prefixmap::error::PrefixMapError;
use prefixmap::{IriRef, PrefixMap};
use serde::Serialize;
use srdf::SHACLPath;

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
            }
            SHACLPathRef::Alternative { paths } => {
                let spaths = paths
                    .iter()
                    .map(|p| p.to_shaclpath(prefixmap))
                    .collect::<Result<Vec<SHACLPath>, PrefixMapError>>()?;
                Ok(SHACLPath::alternative(spaths))
            }
            SHACLPathRef::Sequence { paths } => {
                let spaths = paths
                    .iter()
                    .map(|p| p.to_shaclpath(prefixmap))
                    .collect::<Result<Vec<SHACLPath>, PrefixMapError>>()?;
                Ok(SHACLPath::sequence(spaths))
            }
            SHACLPathRef::Inverse { path } => {
                let spath = path.to_shaclpath(prefixmap)?;
                Ok(SHACLPath::inverse(spath))
            }
            SHACLPathRef::ZeroOrMore { path } => {
                let spath = path.to_shaclpath(prefixmap)?;
                Ok(SHACLPath::zero_or_more(spath))
            }
            SHACLPathRef::OneOrMore { path } => {
                let spath = path.to_shaclpath(prefixmap)?;
                Ok(SHACLPath::one_or_more(spath))
            }
            SHACLPathRef::ZeroOrOne { path } => {
                let spath = path.to_shaclpath(prefixmap)?;
                Ok(SHACLPath::zero_or_one(spath))
            }
        }
    }
}
