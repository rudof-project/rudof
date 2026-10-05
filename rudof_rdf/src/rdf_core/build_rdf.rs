use prefixmap::PrefixMap;
use rudof_iri::IriS;
use std::io::Write;

use crate::rdf_core::vocabs::{RdfVocab, ShaclVocab};
use crate::rdf_core::{BlankNodeMode, NeighsRDF, RDFFormat, SHACLPath};

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
    fn add_base(&mut self, base: &Option<IriS>);

    /// Adds a namespace prefix declaration to the graph.
    ///
    /// # Arguments
    ///
    /// * `alias` - The prefix alias (e.g., "foaf", "ex", "rdf")
    /// * `iri` - The full namespace IRI this prefix represents
    fn add_prefix(&mut self, alias: &str, iri: &IriS);

    /// Replaces the graph prefixmap.
    ///
    /// # Arguments
    ///
    /// * `prefix_map` - A map of prefix aliases to namespace IRIs
    fn set_prefix_map(&mut self, prefix_map: PrefixMap);

    /// Merges the graph prefixmap with another given.
    ///
    /// # Arguments
    ///
    /// * `prefix_map` - A map of prefix aliases to namespace IRIs
    fn merge_prefixes(&mut self, prefix_map: PrefixMap);

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

    /// Adds an RDF collection to the graph, returning the term that denotes the
    /// head of the list (`rdf:nil` for an empty one).
    ///
    /// # Arguments
    ///
    /// * `items` - The elements of the list, in order
    fn add_rdf_list(&mut self, items: Vec<Self::Term>) -> Result<Self::Term, Self::Err> {
        let mut rest: Self::Term = RdfVocab::rdf_nil().into();
        for item in items.into_iter().rev() {
            let node: Self::Subject = self.add_bnode()?.into();
            self.add_triple(node.clone(), RdfVocab::rdf_first(), item)?;
            self.add_triple(node.clone(), RdfVocab::rdf_rest(), rest)?;
            rest = node.into();
        }
        Ok(rest)
    }

    /// Adds a SHACL property path to the graph, returning the term that denotes
    /// it. Complex paths are encoded as blank node structures.
    ///
    /// This is the inverse of
    /// [`get_path_for`](crate::rdf_core::FocusRDF::get_path_for).
    ///
    /// # Arguments
    ///
    /// * `path` - The path to serialize
    fn add_shacl_path(&mut self, path: &SHACLPath) -> Result<Self::Term, Self::Err> {
        match path {
            SHACLPath::Predicate { pred } => Ok(pred.clone().into()),
            SHACLPath::Sequence { paths } => {
                let items = self.add_shacl_paths(paths)?;
                self.add_rdf_list(items)
            },
            SHACLPath::Alternative { paths } => {
                let items = self.add_shacl_paths(paths)?;
                let list = self.add_rdf_list(items)?;
                self.add_bnode_with(ShaclVocab::sh_alternative_path(), list)
            },
            SHACLPath::Inverse { path } => {
                let sub_path = self.add_shacl_path(path)?;
                self.add_bnode_with(ShaclVocab::sh_inverse_path(), sub_path)
            },
            SHACLPath::ZeroOrMore { path } => {
                let sub_path = self.add_shacl_path(path)?;
                self.add_bnode_with(ShaclVocab::sh_zero_or_more_path(), sub_path)
            },
            SHACLPath::OneOrMore { path } => {
                let sub_path = self.add_shacl_path(path)?;
                self.add_bnode_with(ShaclVocab::sh_one_or_more_path(), sub_path)
            },
            SHACLPath::ZeroOrOne { path } => {
                let sub_path = self.add_shacl_path(path)?;
                self.add_bnode_with(ShaclVocab::sh_zero_or_one_path(), sub_path)
            },
        }
    }

    /// Serializes each of the given paths, keeping their order.
    ///
    /// Auxiliary to [`add_shacl_path`](BuildRDF::add_shacl_path).
    fn add_shacl_paths(&mut self, paths: &[SHACLPath]) -> Result<Vec<Self::Term>, Self::Err> {
        paths.iter().map(|path| self.add_shacl_path(path)).collect()
    }

    /// Creates a blank node linked to the given object through the given
    /// predicate, returning the blank node as a term.
    ///
    /// Auxiliary to [`add_shacl_path`](BuildRDF::add_shacl_path), which uses it
    /// for alternative, inverse and quantified paths.
    fn add_bnode_with(&mut self, predicate: IriS, object: Self::Term) -> Result<Self::Term, Self::Err> {
        let node: Self::Subject = self.add_bnode()?.into();
        self.add_triple(node.clone(), predicate, object)?;
        Ok(node.into())
    }

    /// Serializes the graph to an RDF format.
    ///
    /// Blank node identifiers are simplified to short sequential ids
    /// (`_:b0`, `_:b1`, ...); see [`BlankNodeMode`]. Use
    /// [`serialize_with_blank_node_mode`](BuildRDF::serialize_with_blank_node_mode)
    /// to keep the original blank node identifiers instead.
    ///
    /// # Arguments
    ///
    /// * `format` - The RDF serialization format to use
    /// * `writer` - The destination for serialized output
    fn serialize<W: Write>(&self, format: &RDFFormat, writer: &mut W) -> Result<(), Self::Err>;

    /// Serializes the graph to an RDF format, choosing how blank node
    /// identifiers are handled.
    ///
    /// # Arguments
    ///
    /// * `format` - The RDF serialization format to use
    /// * `mode` - How to handle blank node identifiers, see [`BlankNodeMode`]
    /// * `writer` - The destination for serialized output
    fn serialize_with_blank_node_mode<W: Write>(
        &self,
        format: &RDFFormat,
        mode: BlankNodeMode,
        writer: &mut W,
    ) -> Result<(), Self::Err>;
}
