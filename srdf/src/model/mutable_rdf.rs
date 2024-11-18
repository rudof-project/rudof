use crate::model::rdf::Rdf;

use super::Iri;
use super::Subject;
use super::Term;
use super::Triple;

/// Provides the functionality to implementors of being mutable.
pub trait MutableRdf: Rdf {
    type MutableRdfError;

    fn add_triple<S, P, O>(
        &mut self,
        subject: S,
        predicate: P,
        object: O,
    ) -> Result<(), Self::MutableRdfError>
    where
        S: Subject,
        P: Iri,
        O: Term;

    fn remove_triple<T: Triple>(&mut self, triple: T) -> Result<(), Self::MutableRdfError>;

    fn add_base<I: Iri>(&mut self, base: I) -> Result<(), Self::MutableRdfError>;
    fn add_prefix<I: Iri>(&mut self, alias: &str, iri: I) -> Result<(), Self::MutableRdfError>;
}
