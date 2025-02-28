use std::io::Write;

use iri_s::IriS;
use prefixmap::PrefixMap;

use crate::{Query, RDFFormat};

/// Types that implement this trait can build RDF data
pub trait SRDFBuilder: Query {
    /// Returns an empty RDF graph
    fn empty() -> Self;

    /// Adds an optional IRI as base
    fn add_base(&mut self, base: &Option<IriS>) -> Result<(), Self::Err>;

    /// Adds a prefix declaration to the current RDF graph
    fn add_prefix(&mut self, alias: &str, iri: &IriS) -> Result<(), Self::Err>;

    /// Adds a prefix map declaration to the current RDF graph
    fn add_prefix_map(&mut self, prefix_map: PrefixMap) -> Result<(), Self::Err>;

    /// Adds an RDF triple to the current RDF graph
    fn add_triple(
        &mut self,
        subj: &Self::Subject,
        pred: &Self::IRI,
        obj: &Self::Term,
    ) -> Result<(), Self::Err>;

    /// Removes an RDf triple to the current RDF graph
    fn remove_triple(
        &mut self,
        subj: &Self::Subject,
        pred: &Self::IRI,
        obj: &Self::Term,
    ) -> Result<(), Self::Err>;

    /// Adds an `rdf:type` declaration to the current RDF graph
    fn add_type(&mut self, node: &Self::Term, type_: Self::Term) -> Result<(), Self::Err>;

    /// Serialize the current graph to a Write implementation
    fn serialize<W: Write>(&self, format: &RDFFormat, writer: &mut W) -> Result<(), Self::Err>;
}
