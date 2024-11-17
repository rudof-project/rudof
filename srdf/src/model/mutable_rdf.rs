use crate::model::rdf::TObject;
use crate::model::rdf::TPredicate;
use crate::model::rdf::Rdf;
use crate::model::rdf::TSubject;

/// Provides the functionality to implementors of being mutable.
pub trait MutableRdf: Rdf {
    type MutableRdfError;

    fn add_triple(
        &mut self,
        subject: TSubject<Self>,
        predicate: TPredicate<Self>,
        object: TObject<Self>,
    ) -> Result<(), Self::MutableRdfError>;

    fn remove_triple(&mut self, triple: &Self::Triple) -> Result<(), Self::MutableRdfError>;

    fn add_base(&mut self, base: TPredicate<Self>) -> Result<(), Self::MutableRdfError>;

    fn add_prefix(
        &mut self,
        alias: &str,
        iri: TPredicate<Self>,
    ) -> Result<(), Self::MutableRdfError>;
}
