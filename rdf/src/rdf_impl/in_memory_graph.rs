use crate::rdf_core::{
    AsyncRDF, BuildRDF, FocusRDF, NeighsRDF, RDFFormat, Rdf, Matcher,
    query::{QueryRDF, QueryResultFormat, QuerySolution, QuerySolutions, VarName},
    vocab::rdf_type as vocab_rdf_type,
};
use crate::rdf_impl::in_memory_graph_error::InMemoryGraphError;

use async_trait::async_trait;
use colored::*;
use oxigraph::{store::Store, sparql::{QueryResults, SparqlEvaluator}};
use oxrdf::{
    BlankNode as OxBlankNode, Graph, GraphName, Literal as OxLiteral, NamedNode as OxNamedNode,
    NamedNodeRef, NamedOrBlankNode as OxSubject, NamedOrBlankNodeRef as OxSubjectRef, Quad,
    Term as OxTerm, TermRef, Triple as OxTriple, TripleRef
};
use oxjsonld::JsonLdParser;
use oxrdfio::{JsonLdProfileSet, RdfFormat, RdfSerializer};
use oxrdfxml::RdfXmlParser;
use oxttl::{NQuadsParser, NTriplesParser, TurtleParser};
use prefixmap::{PrefixMapError, prefixmap::*};
use iri_s::IriS;
use std::collections::{HashMap, HashSet};
use serde::{ser::SerializeStruct, Serialize};
use sparesults::QuerySolution as SparQuerySolution;
use std::{
    fmt::Debug, fs::File,
    io::{self, BufReader, Cursor, Read, Write}, path::{Path, PathBuf},
    str::FromStr,
};
use tracing::{debug, trace};

/// An RDF graph stored entirely in memory.
///
/// The graph is backed by [`oxrdf::Graph`] and enriched with prefix handling,
/// base IRI support, blank node generation, and optional SPARQL querying via
/// an Oxigraph [`Store`].
#[derive(Default, Clone)]
pub struct InMemoryGraph {
    /// Optional focus term used by [`FocusRDF`] operations.
    focus: Option<OxTerm>,
    
    /// Underlying RDF graph.
    graph: Graph,
    
    /// Prefix map used for CURIE resolution and qualification.
    pm: PrefixMap,
    
    /// Optional base IRI for resolving relative IRIs.
    base: Option<IriS>,
    
    /// Counter used to generate unique blank node identifiers.
    bnode_counter: usize,
    
    /// Optional Oxigraph store used for SPARQL evaluation.
    store: Option<Store>,
}

impl Debug for InMemoryGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InMemoryGraph")
            .field("triples_count", &self.graph.len())
            .field("prefixmap", &self.pm)
            .field("base", &self.base)
            .finish()
    }
}

impl Serialize for InMemoryGraph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("SRDFGraph", 3)?;
        state.serialize_field("triples_count", &self.graph.len())?;
        state.serialize_field("prefixmap", &self.pm)?;
        state.serialize_field("base", &self.base)?;
        state.end()
    }
}

impl InMemoryGraph {
    /// Creates an empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of triples in the graph.
    pub fn len(&self) -> usize {
        self.graph.len()
    }

    /// Returns an iterator over all triples as quads in the default graph.
    pub fn quads(&self) -> impl Iterator<Item = Quad> + '_ {
        let graph_name = GraphName::DefaultGraph;
        self.graph
            .iter()
            .map(move |t| triple_to_quad(t, graph_name.clone()))
    }

    /// Returns `true` if the graph contains no triples.
    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

    /// Merges RDF data from a reader into the current graph.
    ///
    /// The parsing behavior depends on [`RDFFormat`] and [`ReaderMode`].
    /// Prefixes and base IRI are merged when available.
    ///
    /// # Parameters
    ///
    /// * `reader` - Input stream containing RDF data
    /// * `source_name` - Name used for error reporting
    /// * `format` - RDF serialization format
    /// * `base` - Optional base IRI for resolving relative IRIs
    /// * `reader_mode` - Controls error handling (strict or lax)
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails in strict mode or if I/O errors occur.
    pub fn merge_from_reader<R: io::Read>(
        &mut self,
        reader: &mut R,
        source_name: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<(), InMemoryGraphError> {
        match format {
            RDFFormat::Turtle => {
                self.parse_turtle(reader, source_name, base, reader_mode)?;
            }
            RDFFormat::NTriples => {
                self.parse_ntriples(reader, reader_mode)?;
            }
            RDFFormat::RDFXML => {
                self.parse_rdfxml(reader, reader_mode)?;
            }
            RDFFormat::TriG => {
                todo!();
            }
            RDFFormat::N3 => {
                todo!();
            }
            RDFFormat::NQuads => {
                self.parse_nquads(reader, reader_mode)?;
            }
            RDFFormat::JsonLd => {
                self.parse_jsonld(reader, reader_mode)?;
            }
        }
        Ok(())
    }

    /// Parses Turtle data and merges it into the graph.
    ///
    /// # Parameters
    ///
    /// * `reader` - Input stream containing Turtle data
    /// * `source_name` - Name used for error reporting
    /// * `base` - Optional base IRI for resolving relative IRIs
    /// * `reader_mode` - Controls error handling (strict or lax)
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails in strict mode.
    fn parse_turtle<R: io::Read>(
        &mut self,
        reader: &mut R,
        source_name: &str,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<(), InMemoryGraphError> {
        let turtle_parser = match base {
            None => TurtleParser::new(),
            Some(iri) => TurtleParser::new().with_base_iri(iri)?,
        };
        
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let reader1 = Cursor::new(&buffer);
        let mut reader2 = Cursor::new(&buffer);
        let mut turtle_reader = turtle_parser.for_reader(reader1);
        
        for triple_result in turtle_reader.by_ref() {
            let triple = match handle_parse_error(
                triple_result,
                reader_mode,
                |e| InMemoryGraphError::TurtleParseError {
                    source_name: source_name.to_string(),
                    error: e,
                },
            )? {
                Some(t) => t,
                None => continue,
            };
            self.graph.insert(triple.as_ref());
        }
        
        let prefixes: HashMap<&str, &str> = turtle_reader.prefixes().collect();
        self.base = match (&self.base, base) {
            (None, None) => None,
            (Some(b), None) => Some(b.clone()),
            (_, Some(b)) => Some(IriS::new_unchecked(b)),
        };
        let pm = PrefixMap::from_hashmap(&prefixes)?;
        self.merge_prefixes(pm)?;
        
        Ok(())
    }

    /// Parses N-Triples data and merges it into the graph.
    ///
    /// # Parameters
    ///
    /// * `reader` - Input stream containing N-Triples data
    /// * `reader_mode` - Controls error handling (strict or lax)
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails in strict mode.
    fn parse_ntriples<R: io::Read>(
        &mut self,
        reader: &mut R,
        reader_mode: &ReaderMode,
    ) -> Result<(), InMemoryGraphError> {
        let parser = NTriplesParser::new();
        let mut nt_reader = parser.for_reader(reader);
        
        for triple_result in nt_reader.by_ref() {
            let triple = match handle_parse_error(
                triple_result,
                reader_mode,
                |e| InMemoryGraphError::NTriplesError {
                    data: "Reading N-Triples".to_string(),
                    error: e,
                },
            )? {
                Some(t) => t,
                None => continue,
            };
            self.graph.insert(triple.as_ref());
        }
        
        Ok(())
    }

    /// Parses RDF/XML data and merges it into the graph.
    ///
    /// # Parameters
    ///
    /// * `reader` - Input stream containing RDF/XML data
    /// * `reader_mode` - Controls error handling (strict or lax)
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails in strict mode.
    fn parse_rdfxml<R: io::Read>(
        &mut self,
        reader: &mut R,
        reader_mode: &ReaderMode,
    ) -> Result<(), InMemoryGraphError> {
        let parser = RdfXmlParser::new();
        let mut xml_reader = parser.for_reader(reader);
        
        for triple_result in xml_reader.by_ref() {
            let triple = match handle_parse_error(
                triple_result,
                reader_mode,
                |e| InMemoryGraphError::RDFXMLError {
                    data: "Reading RDF/XML".to_string(),
                    error: e,
                },
            )? {
                Some(t) => t,
                None => continue,
            };
            let triple_ref = cnv_triple(&triple);
            self.graph.insert(triple_ref);
        }
        
        Ok(())
    }

    /// Parses N-Quads data and merges it into the graph.
    ///
    /// # Parameters
    ///
    /// * `reader` - Input stream containing N-Quads data
    /// * `reader_mode` - Controls error handling (strict or lax)
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails in strict mode.
    fn parse_nquads<R: io::Read>(
        &mut self,
        reader: &mut R,
        reader_mode: &ReaderMode,
    ) -> Result<(), InMemoryGraphError> {
        let parser = NQuadsParser::new();
        let mut nq_reader = parser.for_reader(reader);
        
        for triple_result in nq_reader.by_ref() {
            let triple = match handle_parse_error(
                triple_result,
                reader_mode,
                |e| InMemoryGraphError::NQuadsError {
                    data: "Reading NQuads".to_string(),
                    error: e,
                },
            )? {
                Some(t) => t,
                None => continue,
            };
            self.graph.insert(triple.as_ref());
        }
        
        Ok(())
    }

    /// Parses JSON-LD data and merges it into the graph.
    ///
    /// # Parameters
    ///
    /// * `reader` - Input stream containing JSON-LD data
    /// * `reader_mode` - Controls error handling (strict or lax)
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails in strict mode.
    fn parse_jsonld<R: io::Read>(
        &mut self,
        reader: &mut R,
        reader_mode: &ReaderMode,
    ) -> Result<(), InMemoryGraphError> {
        let parser = JsonLdParser::new();
        let mut jsonld_reader = parser.for_reader(reader);
        
        for triple_result in jsonld_reader.by_ref() {
            let triple = match handle_parse_error(
                triple_result,
                reader_mode,
                |e| InMemoryGraphError::JsonLDError {
                    data: "Reading JSON-LD".to_string(),
                    error: e,
                },
            )? {
                Some(t) => t,
                None => continue,
            };
            self.graph.insert(triple.as_ref());
        }
        
        Ok(())
    }

    /// Merges a prefix map into the graph's prefix map.
    ///
    /// # Parameters
    ///
    /// * `prefixmap` - The prefix map to merge
    ///
    /// # Errors
    ///
    /// Returns an error if merging fails due to conflicting prefixes.
    pub fn merge_prefixes(&mut self, prefixmap: PrefixMap) -> Result<(), InMemoryGraphError> {
        self.pm.merge(prefixmap)?;
        Ok(())
    }

    /// Builds a new graph from a reader.
    ///
    /// This is a convenience constructor that creates an empty graph and merges
    /// data from the reader.
    ///
    /// # Parameters
    ///
    /// * `read` - Input stream containing RDF data
    /// * `source_name` - Name used for error reporting
    /// * `format` - RDF serialization format
    /// * `base` - Optional base IRI for resolving relative IRIs
    /// * `reader_mode` - Controls error handling (strict or lax)
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails.
    pub fn from_reader<R: io::Read>(
        read: &mut R,
        source_name: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<InMemoryGraph, InMemoryGraphError> {
        let mut srdf_graph = InMemoryGraph::new();
        srdf_graph.merge_from_reader(read, source_name, format, base, reader_mode)?;
        Ok(srdf_graph)
    }

    /// Resolves a CURIE or prefixed name into a full IRI.
    ///
    /// Uses the graph's prefix map to expand the prefixed form.
    ///
    /// # Parameters
    ///
    /// * `str` - The CURIE or prefixed name to resolve (e.g., "ex:Alice")
    ///
    /// # Errors
    ///
    /// Returns an error if the prefix is not defined or if the string format is invalid.
    pub fn resolve(&self, str: &str) -> Result<OxNamedNode, InMemoryGraphError> {
        let r = self.pm.resolve(str)?;
        Ok(Self::cnv_iri(r))
    }

    /// Formats a blank node for display with color.
    ///
    /// Returns a green-colored string representation.
    ///
    /// # Parameters
    ///
    /// * `bn` - The blank node to format
    pub fn show_blanknode(&self, bn: &OxBlankNode) -> String {
        format!("{}", bn.to_string().green())
    }

    /// Formats a literal for display with color.
    ///
    /// Returns a red-colored string representation.
    ///
    /// # Parameters
    ///
    /// * `lit` - The literal to format
    pub fn show_literal(&self, lit: &OxLiteral) -> String {
        format!("{}", lit.to_string().red())
    }

    /// Builds a graph from a string.
    ///
    /// # Parameters
    ///
    /// * `data` - RDF data as a string
    /// * `format` - RDF serialization format
    /// * `base` - Optional base IRI for resolving relative IRIs
    /// * `reader_mode` - Controls error handling (strict or lax)
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails.
    pub fn from_str(
        data: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<InMemoryGraph, InMemoryGraphError> {
        Self::from_reader(
            &mut Cursor::new(data),
            "String",
            format,
            base,
            reader_mode,
        )
    }

    /// Converts an [`IriS`] into an Oxigraph named node.
    ///
    /// # Parameters
    ///
    /// * `iri` - The IRI to convert
    ///
    /// # Returns
    ///
    /// An Oxigraph named node representation of the IRI.
    fn cnv_iri(iri: IriS) -> OxNamedNode {
        OxNamedNode::new_unchecked(iri.as_str())
    }

    /// Adds a triple using borrowed references.
    ///
    /// This method avoids cloning the triple components by working with references.
    ///
    /// # Parameters
    ///
    /// * `subj` - Triple subject (named node or blank node)
    /// * `pred` - Triple predicate (named node)
    /// * `obj` - Triple object (term)
    ///
    /// # Errors
    ///
    /// This method currently always returns `Ok(())`.
    pub fn add_triple_ref<'a, S, P, O>(
        &mut self,
        subj: S,
        pred: P,
        obj: O,
    ) -> Result<(), InMemoryGraphError>
    where
        S: Into<OxSubjectRef<'a>>,
        P: Into<NamedNodeRef<'a>>,
        O: Into<TermRef<'a>>,
    {
        let triple = TripleRef::new(subj.into(), pred.into(), obj.into());
        self.graph.insert(triple);
        Ok(())
    }

    /// Merges RDF data from a filesystem path.
    ///
    /// Opens the file and delegates to [`merge_from_reader`](Self::merge_from_reader).
    ///
    /// # Parameters
    ///
    /// * `path` - Path to the RDF file
    /// * `format` - RDF serialization format
    /// * `base` - Optional base IRI for resolving relative IRIs
    /// * `reader_mode` - Controls error handling (strict or lax)
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or if parsing fails.
    pub fn merge_from_path<P: AsRef<Path>>(
        &mut self,
        path: P,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<(), InMemoryGraphError> {
        let path_ref = path.as_ref();
        let file = File::open(path_ref).map_err(|e| InMemoryGraphError::ReadingPathError {
            path_name: path_ref.display().to_string(),
            error: e,
        })?;
        let mut reader = BufReader::new(file);
        self.merge_from_reader(
            &mut reader,
            &path_ref.display().to_string(),
            format,
            base,
            reader_mode,
        )
    }

    /// Builds a graph from a filesystem path.
    ///
    /// Creates a new empty graph and merges data from the file.
    ///
    /// # Parameters
    ///
    /// * `path` - Path to the RDF file
    /// * `format` - RDF serialization format
    /// * `base` - Optional base IRI for resolving relative IRIs
    /// * `reader_mode` - Controls error handling (strict or lax)
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or if parsing fails.
    pub fn from_path<P: AsRef<Path>>(
        path: P,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<InMemoryGraph, InMemoryGraphError> {
        let path_ref = path.as_ref();
        let file = File::open(path_ref).map_err(|e| InMemoryGraphError::ReadingPathError {
            path_name: path_ref.display().to_string(),
            error: e,
        })?;
        let mut reader = BufReader::new(file);
        Self::from_reader(
            &mut reader,
            &path_ref.display().to_string(),
            format,
            base,
            reader_mode,
        )
    }

    /// Parses data from a relative path within a folder.
    ///
    /// Convenience method that joins the data file name with the folder path.
    ///
    /// # Parameters
    ///
    /// * `data` - Relative file name within the folder
    /// * `format` - RDF serialization format
    /// * `folder` - Base directory path
    /// * `base` - Optional base IRI for resolving relative IRIs
    /// * `reader_mode` - Controls error handling (strict or lax)
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or if parsing fails.
    pub fn parse_data(
        data: &str,
        format: &RDFFormat,
        folder: &Path,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<InMemoryGraph, InMemoryGraphError> {
        let data_path = folder.join(data);
        Self::from_path(&data_path, format, base, reader_mode)
    }

    /// Returns a reference to the prefix map.
    pub fn prefixmap(&self) -> &PrefixMap {
        &self.pm
    }
}

/// Implementation of the core `Rdf` trait.
///
/// This implementation provides the fundamental RDF operations including type definitions,
/// prefix resolution, and term qualification (converting full IRIs to prefixed names).
impl Rdf for InMemoryGraph {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Triple = OxTriple;
    type Err = InMemoryGraphError;

    /// Resolves a prefix and local name to a full IRI.
    ///
    /// # Parameters
    ///
    /// * `prefix` - The namespace prefix (e.g., "foaf")
    /// * `local` - The local name (e.g., "Person")
    ///
    /// # Returns
    ///
    /// The full IRI (e.g., "http://xmlns.com/foaf/0.1/Person")
    ///
    /// # Errors
    ///
    /// Returns an error if the prefix is not defined in the prefix map.
    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError> {
        let iri = self.pm.resolve_prefix_local(prefix, local)?;
        Ok(iri)
    }

    /// Converts a full IRI to a qualified (prefixed) name if possible.
    ///
    /// If the IRI matches a known namespace prefix, it returns a shortened form
    /// (e.g., "foaf:Person"). Otherwise, it returns the full IRI.
    ///
    /// # Parameters
    ///
    /// * `node` - The named node to qualify
    ///
    /// # Returns
    ///
    /// A string representation, either prefixed or full IRI.
    fn qualify_iri(&self, node: &Self::IRI) -> String {
        let iri = IriS::from_str(node.as_str())
            .expect("OxNamedNode should always contain valid IRI");
        self.pm.qualify(&iri)
    }

    /// Converts a subject (named node or blank node) to a qualified string.
    ///
    /// Named nodes are qualified using the prefix map. Blank nodes are formatted
    /// with color for display.
    ///
    /// # Parameters
    ///
    /// * `subj` - The subject to qualify
    ///
    /// # Returns
    ///
    /// A string representation of the subject.
    fn qualify_subject(&self, subj: &OxSubject) -> String {
        match subj {
            OxSubject::BlankNode(bn) => self.show_blanknode(bn),
            OxSubject::NamedNode(n) => self.qualify_iri(n),
        }
    }

    /// Converts an RDF term (named node, blank node, or literal) to a qualified string.
    ///
    /// Different term types are formatted differently:
    /// - Named nodes: qualified using prefix map
    /// - Blank nodes: formatted with green color
    /// - Literals: formatted with red color
    /// - RDF-star triples: not yet supported
    ///
    /// # Parameters
    ///
    /// * `term` - The term to qualify
    ///
    /// # Returns
    ///
    /// A string representation of the term.
    fn qualify_term(&self, term: &OxTerm) -> String {
        match term {
            OxTerm::BlankNode(bn) => self.show_blanknode(bn),
            OxTerm::Literal(lit) => self.show_literal(lit),
            OxTerm::NamedNode(n) => self.qualify_iri(n),
            OxTerm::Triple(_) => unimplemented!("RDF-star triples not yet supported"),
        }
    }

    /// Returns a reference to the graph's prefix map.
    ///
    /// # Returns
    ///
    /// `Some(&PrefixMap)` containing all defined namespace prefixes.
    fn prefixmap(&self) -> Option<&PrefixMap> {
        Some(&self.pm)
    }
}

/// Implementation of the `NeighsRDF` trait for navigating graph neighbors.
///
/// This implementation provides methods to iterate over triples in the graph,
/// optionally filtered by subject, predicate, or object patterns.
impl NeighsRDF for InMemoryGraph {
    /// Returns an iterator over all triples in the graph.
    ///
    /// # Returns
    ///
    /// An iterator that yields owned triples.
    fn triples(&self) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        Ok(self.graph.iter().map(TripleRef::into_owned))
    }

    /// Returns an iterator over triples with a specific subject.
    ///
    /// This is optimized to use the underlying graph's subject index.
    ///
    /// # Parameters
    ///
    /// * `subject` - The subject to filter by
    fn triples_with_subject(
        &self,
        subject: &Self::Subject,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        // Collect the triples into a Vec to avoid the lifetime dependency on subject
        let triples: Vec<_> = self
            .graph
            .triples_for_subject(subject)
            .map(TripleRef::into_owned)
            .collect();
        Ok(triples.into_iter())
    }

    /// Returns an iterator over triples matching a pattern.
    ///
    /// This method filters triples based on patterns for subject, predicate, and object.
    /// Each pattern can be a specific value or a wildcard matcher.
    ///
    /// # Parameters
    ///
    /// * `subject` - Pattern matcher for the subject
    /// * `predicate` - Pattern matcher for the predicate
    /// * `object` - Pattern matcher for the object
    ///
    /// # Returns
    ///
    /// An iterator that yields triples matching all three patterns.
    fn triples_matching<S, P, O>(
        &self,
        subject: &S,
        predicate: &P,
        object: &O,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err>
    where
        S: Matcher<Self::Subject>,
        P: Matcher<Self::IRI>,
        O: Matcher<Self::Term>,
    {
        // TODO: Implement this function in a way that it does not retrieve all triples
        let triples = self.triples()?.filter_map(move |triple| {
            match subject == &triple.subject
                && predicate == &triple.predicate
                && object == &triple.object
            {
                true => Some(triple),
                false => None,
            }
        });
        Ok(triples)
    }
}

#[async_trait]
impl AsyncRDF for InMemoryGraph {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Err = InMemoryGraphError;

    /// Returns all predicates associated with a given subject.
    ///
    /// # Parameters
    ///
    /// * `subject` - The subject node to query
    ///
    /// # Errors
    ///
    /// This method currently always returns `Ok`.
    async fn get_predicates_subject(
        &self,
        subject: &OxSubject,
    ) -> Result<HashSet<OxNamedNode>, InMemoryGraphError> {
        let mut results = HashSet::new();
        for triple in self.graph.triples_for_subject(subject) {
            let predicate: OxNamedNode = triple.predicate.to_owned().into();
            results.insert(predicate);
        }
        Ok(results)
    }

    /// Returns all objects for a given subject-predicate pair.
    ///
    /// # Parameters
    ///
    /// * `subject` - The subject node
    /// * `pred` - The predicate node
    ///
    /// # Errors
    ///
    /// This method currently always returns `Ok`.
    async fn get_objects_for_subject_predicate(
        &self,
        subject: &OxSubject,
        pred: &OxNamedNode,
    ) -> Result<HashSet<OxTerm>, InMemoryGraphError> {
        let mut results = HashSet::new();
        for triple in self.graph.triples_for_subject(subject) {
            let predicate: OxNamedNode = triple.predicate.to_owned().into();
            if predicate.eq(pred) {
                let object: OxTerm = triple.object.to_owned().into();
                results.insert(object);
            }
        }
        Ok(results)
    }

    /// Returns all subjects for a given object-predicate pair.
    ///
    /// # Parameters
    ///
    /// * `object` - The object term
    /// * `pred` - The predicate node
    ///
    /// # Errors
    ///
    /// This method currently always returns `Ok`.
    async fn get_subjects_for_object_predicate(
        &self,
        object: &OxTerm,
        pred: &OxNamedNode,
    ) -> Result<HashSet<OxSubject>, InMemoryGraphError> {
        let mut results = HashSet::new();
        for triple in self.graph.triples_for_object(object) {
            let predicate: OxNamedNode = triple.predicate.to_owned().into();
            if predicate.eq(pred) {
                let subject: OxSubject = triple.subject.to_owned().into();
                results.insert(subject);
            }
        }
        Ok(results)
    }
}

/// Implementation of the `FocusRDF` trait for managing a focus term.
///
/// The focus term is used to track a specific RDF term of interest during graph operations.
/// This can be useful for navigation, querying, or maintaining context during traversals.
impl FocusRDF for InMemoryGraph {
    /// Sets the focus term for this graph.
    ///
    /// # Parameters
    ///
    /// * `focus` - The term to set as the current focus
    fn set_focus(&mut self, focus: &Self::Term) {
        self.focus = Some(focus.clone());
    }

    /// Returns the current focus term, if one is set.
    ///
    /// # Returns
    ///
    /// * `Some(&Term)` - If a focus term has been set
    /// * `None` - If no focus term is currently set
    fn get_focus(&self) -> Option<&Self::Term> {
        self.focus.as_ref()
    }
}

/// Implementation of the `BuildRDF` trait for constructing and modifying RDF graphs.
///
/// This implementation provides methods to build RDF graphs programmatically by adding
/// prefixes, base IRIs, blank nodes, triples, and types. It also supports serialization
/// to various RDF formats.
impl BuildRDF for InMemoryGraph {
    /// Sets the base IRI for the graph.
    ///
    /// The base IRI is used to resolve relative IRIs during parsing and serialization.
    ///
    /// # Parameters
    ///
    /// * `base` - Optional base IRI to set
    fn add_base(&mut self, base: &Option<IriS>) -> Result<(), Self::Err> {
        self.base = base.clone();
        Ok(())
    }

    /// Adds a prefix mapping to the graph's prefix map.
    ///
    /// Prefix mappings are used to abbreviate IRIs in qualified form (CURIEs).
    ///
    /// # Parameters
    ///
    /// * `alias` - The prefix alias (e.g., "ex", "foaf")
    /// * `iri` - The full IRI that the alias represents
    ///
    /// # Errors
    ///
    /// Returns an error if the prefix cannot be added to the prefix map.
    fn add_prefix(&mut self, alias: &str, iri: &IriS) -> Result<(), Self::Err> {
        self.pm.insert(alias, iri)?;
        Ok(())
    }

    /// Replaces the entire prefix map with a new one.
    ///
    /// # Parameters
    ///
    /// * `prefix_map` - The new prefix map to use
    fn add_prefix_map(&mut self, prefix_map: PrefixMap) -> Result<(), Self::Err> {
        self.pm = prefix_map;
        Ok(())
    }

    /// Generates a new unique blank node.
    ///
    /// Each call to this method increments an internal counter to ensure uniqueness.
    ///
    /// # Errors
    ///
    /// Returns an error if the blank node counter overflows (extremely unlikely).
    fn add_bnode(&mut self) -> Result<Self::BNode, Self::Err> {
        self.bnode_counter += 1;
        let bn = u128::try_from(self.bnode_counter).map_err(|_| {
            InMemoryGraphError::BlankNodeId {
                msg: format!("Blank node counter overflow: {}", self.bnode_counter),
            }
        })?;
        Ok(OxBlankNode::new_from_unique_id(bn))
    }

    /// Adds a triple to the graph.
    ///
    /// # Parameters
    ///
    /// * `subj` - The subject of the triple
    /// * `pred` - The predicate of the triple
    /// * `obj` - The object of the triple
    fn add_triple<S, P, O>(&mut self, subj: S, pred: P, obj: O) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        P: Into<Self::IRI>,
        O: Into<Self::Term>,
    {
        let triple = OxTriple::new(subj.into(), pred.into(), obj.into());
        self.graph.insert(&triple);
        Ok(())
    }

    /// Removes a triple from the graph.
    ///
    /// # Parameters
    ///
    /// * `subj` - The subject of the triple to remove
    /// * `pred` - The predicate of the triple to remove
    /// * `obj` - The object of the triple to remove
    fn remove_triple<S, P, O>(&mut self, subj: S, pred: P, obj: O) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        P: Into<Self::IRI>,
        O: Into<Self::Term>,
    {
        let triple = OxTriple::new(subj.into(), pred.into(), obj.into());
        self.graph.remove(&triple);
        Ok(())
    }

    /// Adds an `rdf:type` assertion to the graph.
    ///
    /// This is a convenience method that adds a triple with `rdf:type` as the predicate.
    ///
    /// # Parameters
    ///
    /// * `node` - The subject that has the type
    /// * `type_` - The type (class) of the subject
    fn add_type<S, T>(&mut self, node: S, type_: T) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        T: Into<Self::Term>,
    {
        let triple = OxTriple::new(node.into(), rdf_type(), type_.into());
        self.graph.insert(&triple);
        Ok(())
    }

    /// Creates a new empty graph.
    ///
    /// # Returns
    ///
    /// A new `InMemoryGraph` with no triples, prefixes, or base IRI.
    fn empty() -> Self {
        InMemoryGraph {
            focus: None,
            graph: Graph::new(),
            pm: PrefixMap::new(),
            base: None,
            bnode_counter: 0,
            store: None,
        }
    }

    /// Serializes the graph to a writer in the specified RDF format.
    ///
    /// All prefixes defined in the graph's prefix map are included in the serialization.
    ///
    /// # Parameters
    ///
    /// * `format` - The RDF serialization format to use
    /// * `write` - The writer to serialize to
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails or if the writer encounters an I/O error.
    fn serialize<W: Write>(&self, format: &RDFFormat, write: &mut W) -> Result<(), Self::Err> {
        let mut serializer = RdfSerializer::from_format(cnv_rdf_format(format));

        for (prefix, iri) in &self.pm.map {
            serializer = serializer.with_prefix(prefix, iri.as_str()).unwrap();
        }

        let mut writer = serializer.for_writer(write);
        for triple in self.graph.iter() {
            writer.serialize_triple(triple)?;
        }
        writer.finish()?;
        Ok(())
    }
}


/// Converts an RDF format enum to the Oxigraph RdfFormat type.
///
/// # Parameters
///
/// * `rdf_format` - The RDF format to convert
///
/// # Returns
///
/// The corresponding Oxigraph RdfFormat.
fn cnv_rdf_format(rdf_format: &RDFFormat) -> RdfFormat {
    match rdf_format {
        RDFFormat::NTriples => RdfFormat::NTriples,
        RDFFormat::Turtle => RdfFormat::Turtle,
        RDFFormat::RDFXML => RdfFormat::RdfXml,
        RDFFormat::TriG => RdfFormat::TriG,
        RDFFormat::N3 => RdfFormat::N3,
        RDFFormat::NQuads => RdfFormat::NQuads,
        RDFFormat::JsonLd => RdfFormat::JsonLd {
            profile: JsonLdProfileSet::empty(),
        },
    }
}

/// Returns the RDF type predicate IRI.
///
/// # Returns
///
/// An Oxigraph named node representing `rdf:type`.
fn rdf_type() -> OxNamedNode {
    OxNamedNode::new_unchecked(vocab_rdf_type().as_str())
}

/// Converts a triple reference to a quad with the specified graph name.
///
/// # Parameters
///
/// * `t` - The triple reference to convert
/// * `graph_name` - The graph name to use for the quad
///
/// # Returns
///
/// A quad representing the triple in the specified graph.
fn triple_to_quad(t: TripleRef, graph_name: GraphName) -> Quad {
    let subj: oxrdf::NamedOrBlankNode = t.subject.into();
    let pred: oxrdf::NamedNode = t.predicate.into();
    let obj: oxrdf::Term = t.object.into();
    Quad::new(subj, pred, obj, graph_name)
}

/// Helper function to handle parse errors consistently.
///
/// This function implements a consistent error handling strategy across all parsers.
/// In strict mode, errors are propagated. In lax mode, errors are logged and parsing continues.
///
/// # Parameters
///
/// * `result` - The parse result to handle
/// * `reader_mode` - Controls error handling behavior
/// * `error_constructor` - Function to construct an appropriate error type
///
/// # Returns
///
/// * `Ok(Some(value))` - Parsing succeeded
/// * `Ok(None)` - Parsing failed in lax mode (skip this item)
/// * `Err(error)` - Parsing failed in strict mode
fn handle_parse_error<T, E: std::fmt::Display>(
    result: Result<T, E>,
    reader_mode: &ReaderMode,
    error_constructor: impl FnOnce(String) -> InMemoryGraphError,
) -> Result<Option<T>, InMemoryGraphError> {
    match result {
        Ok(val) => Ok(Some(val)),
        Err(e) => {
            if reader_mode.is_strict() {
                Err(error_constructor(e.to_string()))
            } else {
                debug!("Error captured in Lax mode: {e}");
                Ok(None)
            }
        }
    }
}

/// Reader mode when parsing RDF data files.
///
/// Controls how parsing errors are handled during RDF data ingestion.
///
/// # Variants
///
/// * `Strict` - Stops when there is an error
/// * `Lax` - Emits a warning and continues processing
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum ReaderMode {
    /// Stops when there is an error.
    #[default]
    Strict,

    /// Emits a warning and continues processing.
    Lax,
}

impl ReaderMode {
    /// Returns `true` if this is strict mode.
    pub fn is_strict(&self) -> bool {
        matches!(self, ReaderMode::Strict)
    }
}

impl FromStr for ReaderMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "strict" => Ok(ReaderMode::Strict),
            "lax" => Ok(ReaderMode::Lax),
            _ => Err(format!("Unknown reader mode format: {s}. Expected 'strict' or 'lax'")),
        }
    }
}

/// Converts an owned triple to a triple reference.
///
/// # Parameters
///
/// * `t` - The owned triple to convert
///
/// # Returns
///
/// A triple reference with the same subject, predicate, and object.
fn cnv_triple(t: &OxTriple) -> TripleRef<'_> {
    TripleRef::new(
        OxSubjectRef::from(&t.subject),
        NamedNodeRef::from(&t.predicate),
        TermRef::from(&t.object),
    )
}

impl QueryRDF for InMemoryGraph {
    fn query_construct(
        &self,
        _query_str: &str,
        _format: &QueryResultFormat,
    ) -> Result<String, InMemoryGraphError>
    where
        Self: Sized,
    {
        if self.store.is_some() {
            debug!("CONSTRUCT queries not yet implemented for in-memory store");
        }
        Ok(String::new())
    }

    /// Executes a SPARQL SELECT query against the graph.
    ///
    /// # Parameters
    ///
    /// * `query_str` - The SPARQL SELECT query string
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The query cannot be parsed
    /// * The query execution fails
    /// * The results cannot be processed
    ///
    /// # Returns
    ///
    /// Query solutions containing the results of the SELECT query.
    fn query_select(&self, query_str: &str) -> Result<QuerySolutions<InMemoryGraph>, InMemoryGraphError>
    where
        Self: Sized,
    {
        let mut sols = QuerySolutions::empty();
        
        if let Some(store) = &self.store {
            let parsed_query = SparqlEvaluator::new()
                .parse_query(query_str)
                .map_err(|e| InMemoryGraphError::ParsingQueryError {
                    msg: format!("Error parsing query: {}", e),
                })?;
                
            let query_results = parsed_query
                .on_store(store)
                .execute()
                .map_err(|e| InMemoryGraphError::RunningQueryError {
                    query: query_str.to_string(),
                    msg: format!("Error executing query: {}", e),
                })?;
                
            let solutions = cnv_query_results(query_results)?;
            
            sols.extend(solutions, self.prefixmap().clone())
                .map_err(|e| InMemoryGraphError::ExtendingQuerySolutionsError {
                    query: query_str.to_string(),
                    error: e.to_string(),
                })?;
        } else {
            trace!("No in-memory store to query");
        }
        
        Ok(sols)
    }

    fn query_ask(&self, _query: &str) -> Result<bool, Self::Err> {
        todo!()
    }
}

/// Converts Oxigraph query results to internal query solutions.
///
/// # Parameters
///
/// * `query_results` - The query results from Oxigraph
///
/// # Errors
///
/// Returns an error if any solution cannot be processed.
///
/// # Returns
///
/// A vector of query solutions.
fn cnv_query_results(
    query_results: QueryResults,
) -> Result<Vec<QuerySolution<InMemoryGraph>>, InMemoryGraphError> {
    let QueryResults::Solutions(solutions) = query_results else {
        return Ok(Vec::new());
    };
    
    solutions
        .enumerate()
        .map(|(idx, solution_result)| {
            solution_result
                .map(cnv_query_solution)
                .map_err(|e| InMemoryGraphError::QueryResultError {
                    msg: format!("Error getting query solution: {}", e),
                })
        })
        .collect()
}

/// Converts a single Oxigraph query solution to internal representation.
///
/// # Parameters
///
/// * `qs` - The Oxigraph query solution
///
/// # Returns
///
/// An internal query solution with variables and values.
fn cnv_query_solution(qs: SparQuerySolution) -> QuerySolution<InMemoryGraph> {
    let mut variables = Vec::new();
    let mut values = Vec::new();
    for v in qs.variables() {
        let varname = VarName::new(v.as_str());
        variables.push(varname);
    }
    for t in qs.values() {
        let term = t.clone();
        values.push(term)
    }
    QuerySolution::new(variables, values)
}