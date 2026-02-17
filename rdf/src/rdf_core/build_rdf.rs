use iri_s::IriS;
use prefixmap::PrefixMap;
use std::io::Write;

use crate::rdf_core::{NeighsRDF, RDFFormat};

/// Trait for building and modifying RDF graphs.
///
/// This trait provides methods for constructing RDF graphs by adding triples,
/// managing namespace prefixes, creating blank nodes, and serializing the
/// graph to various RDF formats. It extends [`NeighsRDF`] with mutation
/// capabilities.
pub trait BuildRDF: NeighsRDF {
    /// Creates a new empty RDF graph.
    ///
    /// Returns a graph with no triples, no prefix declarations, and no base IRI.
    /// This is the starting point for building RDF data.
    fn empty() -> Self;

    /// Sets the base IRI for resolving relative IRI references.
    ///
    /// # Arguments
    ///
    /// * `base` - Optional base IRI for resolving relative references
    fn add_base(&mut self, base: &Option<IriS>) -> Result<(), Self::Err>;

    /// Adds a namespace prefix declaration to the graph.
    ///
    /// # Arguments
    ///
    /// * `alias` - The prefix alias (e.g., "foaf", "ex", "rdf")
    /// * `iri` - The full namespace IRI this prefix represents
    fn add_prefix(&mut self, alias: &str, iri: &IriS) -> Result<(), Self::Err>;

    /// Adds multiple prefix declarations from a prefix map.
    ///
    /// # Arguments
    ///
    /// * `prefix_map` - A map of prefix aliases to namespace IRIs
    fn add_prefix_map(&mut self, prefix_map: PrefixMap) -> Result<(), Self::Err>;

    /// Adds an RDF triple to the graph.
    ///
    /// # Type Parameters
    ///
    /// * `S` - Type convertible to the subject representation
    /// * `P` - Type convertible to the predicate (IRI) representation
    /// * `O` - Type convertible to the object (term) representation
    ///
    /// # Arguments
    ///
    /// * `subj` - The subject of the triple (IRI or blank node)
    /// * `pred` - The predicate of the triple (must be an IRI)
    /// * `obj` - The object of the triple (IRI, blank node, or literal)
    fn add_triple<S, P, O>(&mut self, subj: S, pred: P, obj: O) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        P: Into<Self::IRI>,
        O: Into<Self::Term>;

    /// Removes an RDF triple from the graph.
    ///
    /// # Type Parameters
    ///
    /// * `S` - Type convertible to the subject representation
    /// * `P` - Type convertible to the predicate (IRI) representation
    /// * `O` - Type convertible to the object (term) representation
    ///
    /// # Arguments
    ///
    /// * `subj` - The subject of the triple to remove
    /// * `pred` - The predicate of the triple to remove
    /// * `obj` - The object of the triple to remove
    fn remove_triple<S, P, O>(&mut self, subj: S, pred: P, obj: O) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        P: Into<Self::IRI>,
        O: Into<Self::Term>;

    /// Adds an `rdf:type` declaration to the graph.
    ///
    /// # Type Parameters
    ///
    /// * `S` - Type convertible to the subject representation
    /// * `T` - Type convertible to the term representation (typically an IRI)
    ///
    /// # Arguments
    ///
    /// * `node` - The resource being typed (subject)
    /// * `type_` - The type/class of the resource (typically an IRI)
    fn add_type<S, T>(&mut self, node: S, type_: T) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        T: Into<Self::Term>;

    /// Adds an Blank node to the RDF graph and get the node identifier
    fn add_bnode(&mut self) -> Result<Self::BNode, Self::Err>;

    /// Serializes the graph to an RDF format.
    ///
    /// # Arguments
    ///
    /// * `format` - The RDF serialization format to use
    /// * `writer` - The destination for serialized output
    fn serialize<W: Write>(&self, format: &RDFFormat, writer: &mut W) -> Result<(), Self::Err>;
}
