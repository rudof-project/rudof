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
    fn add_triple<S, P, O>(&mut self, subj: S, pred: P, obj: O) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        P: Into<Self::IRI>,
        O: Into<Self::Term>;

    /// Removes an RDF triple to the current RDF graph
    fn remove_triple<S, P, O>(&mut self, subj: S, pred: P, obj: O) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        P: Into<Self::IRI>,
        O: Into<Self::Term>;

    /// Adds an `rdf:type` declaration to the current RDF graph
    fn add_type<S, T>(&mut self, node: S, type_: T) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        T: Into<Self::Term>;

    /// Adds an Blank node to the RDF graph and get the node identifier
    fn add_bnode(&mut self) -> Result<Self::BNode, Self::Err>;

    /// Serialize the current graph to a Write implementation
    fn serialize<W: Write>(&self, format: &RDFFormat, writer: &mut W) -> Result<(), Self::Err>;
}
