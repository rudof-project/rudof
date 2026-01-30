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
use serde::{ser::SerializeStruct, Serialize};
use sparesults::QuerySolution as SparQuerySolution;
use std::{
    fmt::Debug, collections::{HashMap, HashSet}, fs::File,
    io::{self, BufReader, Cursor, Read, Write}, path::{Path, PathBuf},
    str::FromStr,
};
use tracing::{debug, trace};

#[derive(Default, Clone)]
pub struct InMemoryGraph {
    focus: Option<OxTerm>,
    graph: Graph,
    pm: PrefixMap,
    base: Option<IriS>,
    bnode_counter: usize,
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
        let mut state = serializer.serialize_struct("SRDFGraph", 4)?;
        state.serialize_field("triples_count", &self.graph.len())?;
        state.serialize_field("prefixmap", &self.pm)?;
        state.serialize_field("base", &self.base)?;
        state.end()
    }
}

impl InMemoryGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.graph.len()
    }

    pub fn quads(&self) -> impl Iterator<Item = Quad> + '_ {
        let graph_name = GraphName::DefaultGraph;
        self.graph
            .iter()
            .map(move |t| triple_to_quad(t, graph_name.clone()))
    }

    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

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
                let turtle_parser = match base {
                    None => TurtleParser::new(),
                    Some(iri) => TurtleParser::new().with_base_iri(iri)?,
                };
                let mut buffer = Vec::new();
                reader.read_to_end(&mut buffer)?;
                let reader1 = Cursor::new(buffer.clone());
                let mut reader2 = Cursor::new(buffer);
                let mut turtle_reader = turtle_parser.for_reader(reader1);
                for triple_result in turtle_reader.by_ref() {
                    let triple = match triple_result {
                        Err(e) => {
                            if reader_mode.is_strict() {
                                let mut str = String::new();
                                let _ = reader2.read_to_string(&mut str)?;
                                trace!("Error parsing turtle...rest of input: {}", str);
                                return Err(InMemoryGraphError::TurtleParseError {
                                    source_name: source_name.to_string(),
                                    error: e.to_string(),
                                });
                            } else {
                                debug!("Turtle Error captured in Lax mode: {e:?}");
                                continue;
                            }
                        }
                        Ok(t) => t,
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
            }
            RDFFormat::NTriples => {
                let parser = NTriplesParser::new();
                let mut reader = parser.for_reader(reader);
                for triple_result in reader.by_ref() {
                    match triple_result {
                        Err(e) => {
                            if reader_mode.is_strict() {
                                return Err(InMemoryGraphError::NTriplesError {
                                    data: "Reading N-Triples".to_string(),
                                    error: e.to_string(),
                                });
                            } else {
                                debug!("Error captured: {e:?}")
                            }
                        }
                        Ok(t) => {
                            self.graph.insert(t.as_ref());
                        }
                    }
                }
            }
            RDFFormat::RDFXML => {
                let parser = RdfXmlParser::new();
                let mut reader = parser.for_reader(reader);
                for triple_result in reader.by_ref() {
                    match triple_result {
                        Err(e) => {
                            if reader_mode.is_strict() {
                                return Err(InMemoryGraphError::RDFXMLError {
                                    data: "Reading RDF/XML".to_string(),
                                    error: e.to_string(),
                                });
                            } else {
                                debug!("Error captured: {e:?}")
                            }
                        }
                        Ok(t) => {
                            let triple_ref = cnv_triple(&t);
                            self.graph.insert(triple_ref);
                        }
                    }
                }
            }
            RDFFormat::TriG => todo!(),
            RDFFormat::N3 => todo!(),
            RDFFormat::NQuads => {
                let parser = NQuadsParser::new();
                let mut reader = parser.for_reader(reader);
                for triple_result in reader.by_ref() {
                    match triple_result {
                        Err(e) => {
                            if reader_mode.is_strict() {
                                return Err(InMemoryGraphError::NQuadsError {
                                    data: "Reading NQuads".to_string(),
                                    error: e.to_string(),
                                });
                            } else {
                                debug!("NQuads Error captured in Lax mode: {e:?}")
                            }
                        }
                        Ok(t) => {
                            self.graph.insert(t.as_ref());
                        }
                    }
                }
            }
            RDFFormat::JsonLd => {
                let parser = JsonLdParser::new();
                let mut reader = parser.for_reader(reader);
                for triple_result in reader.by_ref() {
                    match triple_result {
                        Err(e) => {
                            if reader_mode.is_strict() {
                                return Err(InMemoryGraphError::JsonLDError {
                                    data: "Reading JSON-LD".to_string(),
                                    error: e.to_string(),
                                });
                            } else {
                                debug!("JSON-LD Error captured in Lax mode: {e:?}")
                            }
                        }
                        Ok(t) => {
                            self.graph.insert(t.as_ref());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn merge_prefixes(&mut self, prefixmap: PrefixMap) -> Result<(), InMemoryGraphError> {
        self.pm.merge(prefixmap)?;
        Ok(())
    }

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

    pub fn resolve(&self, str: &str) -> Result<OxNamedNode, InMemoryGraphError> {
        let r = self.pm.resolve(str)?;
        Ok(Self::cnv_iri(r))
    }

    pub fn show_blanknode(&self, bn: &OxBlankNode) -> String {
        let str: String = format!("{bn}");
        format!("{}", str.green())
    }

    pub fn show_literal(&self, lit: &OxLiteral) -> String {
        let str: String = format!("{lit}");
        format!("{}", str.red())
    }

    pub fn from_str(
        data: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<InMemoryGraph, InMemoryGraphError> {
        Self::from_reader(
            &mut std::io::Cursor::new(&data),
            "String",
            format,
            base,
            reader_mode,
        )
    }

    fn cnv_iri(iri: IriS) -> OxNamedNode {
        OxNamedNode::new_unchecked(iri.as_str())
    }

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
        let subj: OxSubjectRef<'a> = subj.into();
        let pred: NamedNodeRef<'a> = pred.into();
        let obj: TermRef<'a> = obj.into();
        let triple = TripleRef::new(subj, pred, obj);
        self.graph.insert(triple);
        Ok(())
    }

    pub fn merge_from_path<P: AsRef<Path>>(
        &mut self,
        path: P,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<(), InMemoryGraphError> {
        let path_name = path.as_ref().display();
        let file = File::open(path.as_ref()).map_err(|e| InMemoryGraphError::ReadingPathError {
            path_name: path_name.to_string(),
            error: e,
        })?;
        let mut reader = BufReader::new(file);
        Self::merge_from_reader(
            self,
            &mut reader,
            path.as_ref().display().to_string().as_str(),
            format,
            base,
            reader_mode,
        )?;
        Ok(())
    }

    pub fn from_path<P: AsRef<Path>>(
        path: P,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<InMemoryGraph, InMemoryGraphError> {
        let path_name = path.as_ref().display();
        let file = File::open(path.as_ref()).map_err(|e| InMemoryGraphError::ReadingPathError {
            path_name: path_name.to_string(),
            error: e,
        })?;
        let mut reader = BufReader::new(file);
        Self::from_reader(
            &mut reader,
            path.as_ref().display().to_string().as_str(),
            format,
            base,
            reader_mode,
        )
    }

    pub fn parse_data(
        data: &String,
        format: &RDFFormat,
        folder: &Path,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<InMemoryGraph, InMemoryGraphError> {
        let mut attempt = PathBuf::from(folder);
        attempt.push(data);
        let data_path = &attempt;
        let graph = Self::from_path(data_path, format, base, reader_mode)?;
        Ok(graph)
    }

    pub fn prefixmap(&self) -> PrefixMap {
        self.pm.clone()
    }
}

impl Rdf for InMemoryGraph {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Triple = OxTriple;
    type Err = InMemoryGraphError;

    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError> {
        let iri = self.pm.resolve_prefix_local(prefix, local)?;
        Ok(iri.clone())
    }

    fn qualify_iri(&self, node: &Self::IRI) -> String {
        let iri = IriS::from_str(node.as_str()).unwrap();
        self.pm.qualify(&iri)
    }

    fn qualify_subject(&self, subj: &OxSubject) -> String {
        match subj {
            OxSubject::BlankNode(bn) => self.show_blanknode(bn),
            OxSubject::NamedNode(n) => self.qualify_iri(n),
        }
    }

    fn qualify_term(&self, term: &OxTerm) -> String {
        match term {
            OxTerm::BlankNode(bn) => self.show_blanknode(bn),
            OxTerm::Literal(lit) => self.show_literal(lit),
            OxTerm::NamedNode(n) => self.qualify_iri(n),
            OxTerm::Triple(_) => unimplemented!(),
        }
    }

    fn prefixmap(&self) -> Option<prefixmap::PrefixMap> {
        Some(self.pm.clone())
    }
}

impl NeighsRDF for InMemoryGraph {
    fn triples(&self) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        Ok(self.graph.iter().map(TripleRef::into_owned))
    }

    // Optimized version for triples with a specific subject
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

impl FocusRDF for InMemoryGraph {
    fn set_focus(&mut self, focus: &Self::Term) {
        self.focus = Some(focus.clone())
    }

    fn get_focus(&self) -> Option<&Self::Term> {
        self.focus.as_ref()
    }
}

impl BuildRDF for InMemoryGraph {
    fn add_base(&mut self, base: &Option<IriS>) -> Result<(), Self::Err> {
        self.base.clone_from(base);
        Ok(())
    }

    fn add_prefix(&mut self, alias: &str, iri: &IriS) -> Result<(), Self::Err> {
        self.pm.insert(alias, iri)?;
        Ok(())
    }

    fn add_prefix_map(&mut self, prefix_map: PrefixMap) -> Result<(), Self::Err> {
        self.pm = prefix_map.clone();
        Ok(())
    }

    fn add_bnode(&mut self) -> Result<Self::BNode, Self::Err> {
        self.bnode_counter += 1;
        match self.bnode_counter.try_into() {
            Ok(bn) => Ok(OxBlankNode::new_from_unique_id(bn)),
            Err(_) => Err(InMemoryGraphError::BlankNodeId {
                msg: format!("Error converting {} to usize", self.bnode_counter),
            }),
        }
    }

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

    fn add_type<S, T>(&mut self, node: S, type_: T) -> Result<(), Self::Err>
    where
        S: Into<Self::Subject>,
        T: Into<Self::Term>,
    {
        let subject: Self::Subject = node.into();
        let type_: Self::Term = type_.into();
        let triple = OxTriple::new(subject, rdf_type(), type_.clone());
        self.graph.insert(&triple);
        Ok(())
    }

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

fn rdf_type() -> OxNamedNode {
    OxNamedNode::new_unchecked(vocab_rdf_type().as_str())
}

fn triple_to_quad(t: TripleRef, graph_name: GraphName) -> Quad {
    let subj: oxrdf::NamedOrBlankNode = t.subject.into();
    let pred: oxrdf::NamedNode = t.predicate.into();
    let obj: oxrdf::Term = t.object.into();
    Quad::new(subj, pred, obj, graph_name)
}

/// Reader mode when parsing RDF data files
#[derive(Debug, PartialEq, Clone, Default)]
pub enum ReaderMode {
    /// Stops when there is an error
    #[default]
    Strict,

    /// Emits a warning and continues processing
    Lax,
}

impl ReaderMode {
    pub fn is_strict(&self) -> bool {
        matches!(self, ReaderMode::Strict)
    }
}

impl FromStr for ReaderMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "strinct" => Ok(ReaderMode::Strict),
            "lax" => Ok(ReaderMode::Lax),
            _ => Err(format!("Unknown reader mode format: {s}")),
        }
    }
}

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
        let str = String::new();
        if let Some(_store) = &self.store {
            tracing::debug!("Querying in-memory store (we ignore it by now");
        }
        Ok(str)
    }

    fn query_select(&self, query_str: &str) -> Result<QuerySolutions<InMemoryGraph>, InMemoryGraphError>
    where
        Self: Sized,
    {
        let mut sols: QuerySolutions<InMemoryGraph> = QuerySolutions::empty();
        if let Some(store) = &self.store {
            trace!("Querying in-memory store");

            let parsed_query = SparqlEvaluator::new().parse_query(query_str).map_err(|e| {
                InMemoryGraphError::ParsingQueryError {
                    msg: format!("Error parsing query: {}", e),
                }
            })?;
            let new_sol = parsed_query.on_store(store).execute().map_err(|e| {
                InMemoryGraphError::RunningQueryError {
                    query: query_str.to_string(),
                    msg: format!("Error executing query: {}", e),
                }
            })?;
            trace!("Got results from in-memory store");
            let sol = cnv_query_results(new_sol)?;
            sols.extend(sol, self.prefixmap()).map_err(|e| {
                InMemoryGraphError::ExtendingQuerySolutionsError {
                    query: query_str.to_string(),
                    error: format!("{e}"),
                }
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

fn cnv_query_results(
    query_results: QueryResults,
) -> Result<Vec<QuerySolution<InMemoryGraph>>, InMemoryGraphError> {
    let mut results = Vec::new();
    if let QueryResults::Solutions(solutions) = query_results {
        trace!("Converting query solutions");
        let mut counter = 0;
        for solution_action in solutions {
            counter += 1;
            trace!("Converting solution {counter}");
            let solution = solution_action.map_err(|e| InMemoryGraphError::QueryResultError {
                msg: format!("Error getting query solution: {}", e),
            })?;
            let result = cnv_query_solution(solution);
            results.push(result)
        }
    }
    Ok(results)
}

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