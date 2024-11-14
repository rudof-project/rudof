use crate::rdf::Object;
use crate::rdf::Predicate;
use crate::rdf::Rdf;
use crate::rdf::Subject;

/// Provides the functionality to implementors of being mutable.
pub trait MutableRdf: Rdf {
    type MutableRdfError;

    fn add_triple(
        &mut self,
        subject: Subject<Self>,
        predicate: Predicate<Self>,
        object: Object<Self>,
    ) -> Result<(), Self::MutableRdfError>;

    fn remove_triple(&mut self, triple: &Self::Triple) -> Result<(), Self::MutableRdfError>;

    fn add_base(&mut self, base: &Predicate<Self>) -> Result<(), Self::Error>;

    fn add_prefix(&mut self, alias: &str, iri: &Predicate<Self>) -> Result<(), Self::Error>;
}
