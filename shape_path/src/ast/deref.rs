//! The resolution of the IRIs a path is written with.

use super::{Comparison, Filter, Operand, RValue, Selector, ShExPath, Step};
use prefixmap::error::DerefError;
use prefixmap::{DerefIri, PrefixMap};
use rudof_iri::IriS;

/// Resolves the IRIs of a path: an `IRIREF` against the base and a prefixed name with the prefixes.
impl DerefIri for ShExPath {
    fn deref_iri(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError> {
        match self {
            ShExPath::Sequence(paths) => Ok(ShExPath::Sequence(paths.deref_iri(base, prefixmap)?)),
            ShExPath::Union(paths) => Ok(ShExPath::Union(paths.deref_iri(base, prefixmap)?)),
            ShExPath::Intersection(paths) => Ok(ShExPath::Intersection(paths.deref_iri(base, prefixmap)?)),
            ShExPath::Path(steps) => Ok(ShExPath::Path(steps.deref_iri(base, prefixmap)?)),
        }
    }
}

impl DerefIri for Step {
    fn deref_iri(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError> {
        let selector = match self.selector {
            Selector::Path(path) => Selector::Path(path.deref_iri(base, prefixmap)?),
            other => other,
        };
        Ok(Step {
            selector,
            filters: self.filters.deref_iri(base, prefixmap)?,
        })
    }
}

impl DerefIri for Filter {
    fn deref_iri(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError> {
        match self {
            Filter::TermType(term_type) => Ok(Filter::TermType(term_type)),
            Filter::Test {
                assert,
                operand,
                comparison,
            } => Ok(Filter::Test {
                assert,
                operand: operand.deref_iri(base, prefixmap)?,
                comparison: comparison
                    .map(|c| -> Result<Comparison, DerefError> {
                        Ok(Comparison {
                            comparator: c.comparator,
                            rvalue: c.rvalue.deref_iri(base, prefixmap)?,
                        })
                    })
                    .transpose()?,
            }),
        }
    }
}

impl DerefIri for Operand {
    fn deref_iri(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError> {
        match self {
            Operand::Path(path) => Ok(Operand::Path(path.deref_iri(base, prefixmap)?)),
            other => Ok(other),
        }
    }
}

impl DerefIri for RValue {
    fn deref_iri(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError> {
        match self {
            RValue::Iri(iri) => Ok(RValue::Iri(iri.deref_iri(base, prefixmap)?)),
            other @ RValue::Integer(_) => Ok(other),
        }
    }
}
