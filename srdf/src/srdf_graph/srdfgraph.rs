use crate::async_srdf::AsyncSRDF;
use crate::matcher::Matcher;
use crate::srdfgraph_error::SRDFGraphError;
use crate::{
    BuildRDF, FocusRDF, NeighsRDF, QueryRDF, QueryResultFormat, QuerySolution, QuerySolutions, RDF_TYPE_STR, RDFFormat,
    Rdf, VarName,
};
use async_trait::async_trait;
use colored::*;
use iri_s::IriS;
use oxigraph::sparql::{QueryResults, SparqlEvaluator};
use oxigraph::store::Store;
use oxjsonld::JsonLdParser;
use oxrdf::{
    BlankNode as OxBlankNode, Graph, GraphName, Literal as OxLiteral, NamedNode as OxNamedNode, NamedNodeRef,
    NamedOrBlankNode as OxSubject, NamedOrBlankNodeRef as OxSubjectRef, Quad, Term as OxTerm, TermRef,
    Triple as OxTriple, TripleRef,
};
use oxrdfio::{JsonLdProfileSet, RdfFormat, RdfSerializer};
use oxrdfxml::RdfXmlParser;
use oxttl::{NQuadsParser, NTriplesParser, TurtleParser};
use prefixmap::error::PrefixMapError;
use prefixmap::map::*;
use serde::Serialize;
use serde::ser::SerializeStruct;
use sparesults::QuerySolution as SparQuerySolution;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing::{debug, trace};

#[derive(Default, Clone)]
pub struct SRDFGraph {
    focus: Option<OxTerm>,
    graph: Graph,
    pm: PrefixMap,
    base: Option<IriS>,
    bnode_counter: usize,
    store: Option<Store>,
}

impl Debug for SRDFGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SRDFGraph")
            .field("triples_count", &self.graph.len())
            .field("prefixmap", &self.pm)
            .field("base", &self.base)
            .finish()
    }
}

impl Serialize for SRDFGraph {
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

impl SRDFGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.graph.len()
    }

    pub fn quads(&self) -> impl Iterator<Item = Quad> + '_ {
        let graph_name = GraphName::DefaultGraph;
        self.graph.iter().map(move |t| triple_to_quad(t, graph_name.clone()))
    }

    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

    pub fn merge_from_reader<R: Read>(
        &mut self,
        reader: &mut R,
        source_name: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<(), SRDFGraphError> {
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
                                return Err(SRDFGraphError::TurtleParseError {
                                    source_name: source_name.to_string(),
                                    error: e.to_string(),
                                });
                            } else {
                                debug!("Turtle Error captured in Lax mode: {e:?}");
                                continue;
                            }
                        },
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
                let pm = PrefixMap::from_hashmap(prefixes)?;
                self.merge_prefixes(pm)?;
            },
            RDFFormat::NTriples => {
                let parser = NTriplesParser::new();
                let mut reader = parser.for_reader(reader);
                for triple_result in reader.by_ref() {
                    match triple_result {
                        Err(e) => {
                            if reader_mode.is_strict() {
                                return Err(SRDFGraphError::NTriplesError {
                                    data: "Reading N-Triples".to_string(),
                                    error: e.to_string(),
                                });
                            } else {
                                debug!("Error captured: {e:?}")
                            }
                        },
                        Ok(t) => {
                            self.graph.insert(t.as_ref());
                        },
                    }
                }
            },
            RDFFormat::RdfXml => {
                let parser = RdfXmlParser::new();
                let mut reader = parser.for_reader(reader);
                for triple_result in reader.by_ref() {
                    match triple_result {
                        Err(e) => {
                            if reader_mode.is_strict() {
                                return Err(SRDFGraphError::RDFXMLError {
                                    data: "Reading RDF/XML".to_string(),
                                    error: e.to_string(),
                                });
                            } else {
                                debug!("Error captured: {e:?}")
                            }
                        },
                        Ok(t) => {
                            let triple_ref = cnv_triple(&t);
                            self.graph.insert(triple_ref);
                        },
                    }
                }
            },
            RDFFormat::TriG => todo!(),
            RDFFormat::N3 => todo!(),
            RDFFormat::NQuads => {
                let parser = NQuadsParser::new();
                let mut reader = parser.for_reader(reader);
                for triple_result in reader.by_ref() {
                    match triple_result {
                        Err(e) => {
                            if reader_mode.is_strict() {
                                return Err(SRDFGraphError::NQuadsError {
                                    data: "Reading NQuads".to_string(),
                                    error: e.to_string(),
                                });
                            } else {
                                debug!("NQuads Error captured in Lax mode: {e:?}")
                            }
                        },
                        Ok(t) => {
                            self.graph.insert(t.as_ref());
                        },
                    }
                }
            },
            RDFFormat::JsonLd => {
                let parser = JsonLdParser::new();
                let mut reader = parser.for_reader(reader);
                for triple_result in reader.by_ref() {
                    match triple_result {
                        Err(e) => {
                            if reader_mode.is_strict() {
                                return Err(SRDFGraphError::JsonLDError {
                                    data: "Reading JSON-LD".to_string(),
                                    error: e.to_string(),
                                });
                            } else {
                                debug!("JSON-LD Error captured in Lax mode: {e:?}")
                            }
                        },
                        Ok(t) => {
                            self.graph.insert(t.as_ref());
                        },
                    }
                }
            },
        }
        Ok(())
    }

    pub fn merge_prefixes(&mut self, prefixmap: PrefixMap) -> Result<(), SRDFGraphError> {
        self.pm.merge(prefixmap)?;
        Ok(())
    }

    pub fn from_reader<R: Read>(
        read: &mut R,
        source_name: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<SRDFGraph, SRDFGraphError> {
        let mut srdf_graph = SRDFGraph::new();

        srdf_graph.merge_from_reader(read, source_name, format, base, reader_mode)?;
        Ok(srdf_graph)
    }

    pub fn resolve(&self, str: &str) -> Result<OxNamedNode, SRDFGraphError> {
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
    ) -> Result<SRDFGraph, SRDFGraphError> {
        Self::from_reader(&mut Cursor::new(&data), "String", format, base, reader_mode)
    }

    fn cnv_iri(iri: IriS) -> OxNamedNode {
        OxNamedNode::new_unchecked(iri.as_str())
    }

    pub fn add_triple_ref<'a, S, P, O>(&mut self, subj: S, pred: P, obj: O) -> Result<(), SRDFGraphError>
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
    ) -> Result<(), SRDFGraphError> {
        let path_name = path.as_ref().display();
        let file = File::open(path.as_ref()).map_err(|e| SRDFGraphError::ReadingPathError {
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
    ) -> Result<SRDFGraph, SRDFGraphError> {
        let path_name = path.as_ref().display();
        let file = File::open(path.as_ref()).map_err(|e| SRDFGraphError::ReadingPathError {
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
    ) -> Result<SRDFGraph, SRDFGraphError> {
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

impl Rdf for SRDFGraph {
    type Subject = OxSubject;
    type IRI = OxNamedNode;
    type Term = OxTerm;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Triple = OxTriple;
    type Err = SRDFGraphError;

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

    fn prefixmap(&self) -> Option<PrefixMap> {
        Some(self.pm.clone())
    }

    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError> {
        let iri = self.pm.resolve_prefix_local(prefix, local)?;
        Ok(iri.clone())
    }
}

impl NeighsRDF for SRDFGraph {
    fn triples(&self) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        Ok(self.graph.iter().map(TripleRef::into_owned))
    }

    fn triples_matching<S, P, O>(
        &self,
        subject: S,
        predicate: P,
        object: O,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Err>
    where
        S: Matcher<Self::Subject>,
        P: Matcher<Self::IRI>,
        O: Matcher<Self::Term>,
    {
        // TODO: Implement this function in a way that it does not retrieve all triples
        let triples = self.triples()?.filter_map(move |triple| {
            match subject == triple.subject && predicate == triple.predicate && object == triple.object {
                true => Some(triple),
                false => None,
            }
        });
        Ok(triples)
    }

    // Optimized version for triples with a specific subject
    fn triples_with_subject(&self, subject: Self::Subject) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        // Collect the triples into a Vec to avoid the lifetime dependency on subject
        let triples: Vec<_> = self
            .graph
            .triples_for_subject(&subject)
            .map(TripleRef::into_owned)
            .collect();
        Ok(triples.into_iter())
    }
}

#[async_trait]
impl AsyncSRDF for SRDFGraph {
    type Subject = OxSubject;
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Term = OxTerm;
    type Err = SRDFGraphError;

    async fn get_predicates_subject(&self, subject: &OxSubject) -> Result<HashSet<OxNamedNode>, SRDFGraphError> {
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
    ) -> Result<HashSet<OxTerm>, SRDFGraphError> {
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
    ) -> Result<HashSet<OxSubject>, SRDFGraphError> {
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

impl FocusRDF for SRDFGraph {
    fn set_focus(&mut self, focus: &Self::Term) {
        self.focus = Some(focus.clone())
    }

    fn get_focus(&self) -> &Option<Self::Term> {
        &self.focus
    }
}

impl BuildRDF for SRDFGraph {
    fn empty() -> Self {
        SRDFGraph {
            focus: None,
            graph: Graph::new(),
            pm: PrefixMap::new(),
            base: None,
            bnode_counter: 0,
            store: None,
        }
    }

    fn add_base(&mut self, base: &Option<IriS>) -> Result<(), Self::Err> {
        self.base.clone_from(base);
        Ok(())
    }

    fn add_prefix(&mut self, alias: &str, iri: &IriS) -> Result<(), Self::Err> {
        self.pm.add_prefix(alias, iri.clone())?;
        Ok(())
    }

    fn add_prefix_map(&mut self, prefix_map: PrefixMap) -> Result<(), Self::Err> {
        self.pm = prefix_map.clone();
        Ok(())
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

    fn add_bnode(&mut self) -> Result<Self::BNode, Self::Err> {
        self.bnode_counter += 1;
        match self.bnode_counter.try_into() {
            Ok(bn) => Ok(OxBlankNode::new_from_unique_id(bn)),
            Err(_) => Err(SRDFGraphError::BlankNodeId {
                msg: format!("Error converting {} to usize", self.bnode_counter),
            }),
        }
    }

    fn serialize<W: Write>(&self, format: &RDFFormat, write: &mut W) -> Result<(), Self::Err> {
        let mut serializer = RdfSerializer::from_format(cnv_rdf_format(format));

        for (prefix, iri) in &self.pm.map {
            serializer = serializer.with_prefix(prefix, iri.as_str())?;
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
        RDFFormat::RdfXml => RdfFormat::RdfXml,
        RDFFormat::TriG => RdfFormat::TriG,
        RDFFormat::N3 => RdfFormat::N3,
        RDFFormat::NQuads => RdfFormat::NQuads,
        RDFFormat::JsonLd => RdfFormat::JsonLd {
            profile: JsonLdProfileSet::empty(),
        },
    }
}

fn rdf_type() -> OxNamedNode {
    OxNamedNode::new_unchecked(RDF_TYPE_STR)
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

impl QueryRDF for SRDFGraph {
    fn query_select(&self, query_str: &str) -> Result<QuerySolutions<SRDFGraph>, SRDFGraphError>
    where
        Self: Sized,
    {
        let mut sols: QuerySolutions<SRDFGraph> = QuerySolutions::empty();
        if let Some(store) = &self.store {
            trace!("Querying in-memory store");

            let parsed_query =
                SparqlEvaluator::new()
                    .parse_query(query_str)
                    .map_err(|e| SRDFGraphError::ParsingQueryError {
                        msg: format!("Error parsing query: {}", e),
                    })?;
            let new_sol = parsed_query
                .on_store(store)
                .execute()
                .map_err(|e| SRDFGraphError::RunningQueryError {
                    query: query_str.to_string(),
                    msg: format!("Error executing query: {}", e),
                })?;
            trace!("Got results from in-memory store");
            let sol = cnv_query_results(new_sol)?;
            sols.extend(sol, self.prefixmap())
                .map_err(|e| SRDFGraphError::ExtendingQuerySolutionsError {
                    query: query_str.to_string(),
                    error: format!("{e}"),
                })?;
        } else {
            trace!("No in-memory store to query");
        }
        Ok(sols)
    }

    fn query_construct(&self, _query_str: &str, _format: &QueryResultFormat) -> Result<String, SRDFGraphError>
    where
        Self: Sized,
    {
        let str = String::new();
        if let Some(_store) = &self.store {
            debug!("Querying in-memory store (we ignore it by now");
        }
        Ok(str)
    }

    fn query_ask(&self, _query: &str) -> Result<bool, Self::Err> {
        todo!()
    }
}

fn cnv_query_results(query_results: QueryResults) -> Result<Vec<QuerySolution<SRDFGraph>>, SRDFGraphError> {
    let mut results = Vec::new();
    if let QueryResults::Solutions(solutions) = query_results {
        trace!("Converting query solutions");
        let mut counter = 0;
        for solution_action in solutions {
            counter += 1;
            trace!("Converting solution {counter}");
            let solution = solution_action.map_err(|e| SRDFGraphError::QueryResultError {
                msg: format!("Error getting query solution: {}", e),
            })?;
            let result = cnv_query_solution(solution);
            results.push(result)
        }
    }
    Ok(results)
}

fn cnv_query_solution(qs: SparQuerySolution) -> QuerySolution<SRDFGraph> {
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

#[cfg(test)]
mod tests {
    use crate::neighs_rdf::NeighsRDF;
    use iri_s::IriS;
    use oxrdf::Literal as OxLiteral;
    use oxrdf::NamedNode as OxNamedNode;
    use oxrdf::NamedOrBlankNode as OxSubject;
    use oxrdf::Term as OxTerm;
    use std::collections::HashSet;

    use crate::iri;
    use crate::matcher::Any;
    use crate::not;
    use crate::property_bool;
    use crate::property_integer;
    use crate::property_integers;
    use crate::property_string;
    use crate::property_value;
    use crate::rdf_list;
    use crate::rdf_parser;
    use crate::satisfy;
    use crate::set_focus;
    // use crate::Query as _;
    use crate::BuildRDF;
    use crate::PResult;
    use crate::RDFFormat;
    use crate::RDFNodeParse as _;
    use crate::RDFParseError;
    use crate::Triple;

    use super::ReaderMode;
    use super::SRDFGraph;

    const DUMMY_GRAPH: &str = r#"
        prefix : <http://example.org/>
        :x :p 1 .
        :y :p "String" .
        :y :q 2 .
        :z :r 3 .
        :x :s 4 .
    "#;

    const DUMMY_GRAPH_1: &str = r#"
        prefix : <http://example.org/>
        prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        :x :p [ :p 1 ] .
    "#;

    const DUMMY_GRAPH_2: &str = r#"
        prefix : <http://example.org/>
        :x :p (1 2) .
    "#;

    const DUMMY_GRAPH_3: &str = r#"
        prefix : <http://example.org/>
        :x :p 1, 2, 3, 2 .
    "#;

    /*const DUMMY_GRAPH_4: &str = r#"
        prefix : <http://example.org/>
        :x :p 1, 2, 3 .
    "#;*/

    const DUMMY_GRAPH_5: &str = r#"
        prefix : <http://example.org/>
        :x :p 1, 2 ;
        :q true .
    "#;

    const DUMMY_GRAPH_6: &str = r#"
        prefix : <http://example.org/>
        :x :p 1 .
    "#;

    const DUMMY_GRAPH_7: &str = r#"
        prefix : <http://example.org/>
        :x :p true .
    "#;

    const DUMMY_GRAPH_8: &str = r#"
        prefix : <http://example.org/>
        :x :p true ;
        :q 1    .
    "#;

    const DUMMY_GRAPH_9: &str = r#"
        prefix : <http://example.org/>
        :x :p 1 .
    "#;

    const DUMMY_GRAPH_10: &str = r#"
        prefix : <http://example.org/>
        :x :p "1" .
    "#;

    #[derive(Debug, PartialEq)]
    enum A {
        Int(isize),
        Bool(bool),
    }

    fn graph_from_str(s: &str) -> SRDFGraph {
        SRDFGraph::from_str(s, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap()
    }

    #[test]
    fn test_triples_matching_subject_predicate_and_object() {
        let graph = graph_from_str(DUMMY_GRAPH);
        let x: OxSubject = OxNamedNode::new_unchecked("http://example.org/x").into();
        let p = OxNamedNode::new_unchecked("http://example.org/p");
        let one: OxTerm = OxLiteral::from(1).into();
        let triples = graph.triples_matching(x, p, one).unwrap();
        assert_eq!(triples.count(), 1)
    }

    #[test]
    fn test_triples_matching_subject_and_predicate() {
        let graph = graph_from_str(DUMMY_GRAPH);
        let x: OxSubject = OxNamedNode::new_unchecked("http://example.org/x").into();
        let p = OxNamedNode::new_unchecked("http://example.org/p");
        let triples = graph.triples_matching(x, p, Any).unwrap();
        assert_eq!(triples.count(), 1)
    }

    #[test]
    fn test_triples_matching_subject_and_object() {
        let graph = graph_from_str(DUMMY_GRAPH);
        let x: OxSubject = OxNamedNode::new_unchecked("http://example.org/x").into();
        let one: OxTerm = OxLiteral::from(1).into();
        let triples = graph.triples_matching(x, Any, one).unwrap();
        assert_eq!(triples.count(), 1)
    }

    #[test]
    fn test_triples_matching_predicate_and_object() {
        let graph = graph_from_str(DUMMY_GRAPH);
        let p = OxNamedNode::new_unchecked("http://example.org/p");
        let one: OxTerm = OxLiteral::from(1).into();
        let triples = graph.triples_matching(Any, p, one).unwrap();
        assert_eq!(triples.count(), 1)
    }

    #[test]
    fn test_triples_matching_subject() {
        let graph = graph_from_str(DUMMY_GRAPH);
        let x: OxSubject = OxNamedNode::new_unchecked("http://example.org/x").into();
        let triples = graph.triples_matching(x, Any, Any).unwrap();
        assert_eq!(triples.count(), 2)
    }

    #[test]
    fn test_triples_matching_predicate() {
        let graph = graph_from_str(DUMMY_GRAPH);
        let p = OxNamedNode::new_unchecked("http://example.org/p");
        let triples = graph.triples_matching(Any, p, Any).unwrap();
        assert_eq!(triples.count(), 2)
    }

    #[test]
    fn test_triples_matching_object() {
        let graph = graph_from_str(DUMMY_GRAPH);
        let one: OxTerm = OxLiteral::from(1).into();
        let triples = graph.triples_matching(Any, Any, one).unwrap();
        assert_eq!(triples.count(), 1)
    }

    #[test]
    fn test_incoming_arcs() {
        let graph = graph_from_str(DUMMY_GRAPH);
        let x = OxSubject::NamedNode(OxNamedNode::new_unchecked("http://example.org/x"));
        let p = OxNamedNode::new_unchecked("http://example.org/p");
        let one: OxTerm = OxLiteral::from(1).into();
        let actual = graph.incoming_arcs(one).unwrap();
        let expected = HashSet::from([x]);
        assert_eq!(actual.get(&p), Some(&expected))
    }

    #[test]
    fn test_outgoing_arcs() {
        let graph = graph_from_str(DUMMY_GRAPH_1);

        let x = OxSubject::NamedNode(OxNamedNode::new_unchecked("http://example.org/x"));
        let p = OxNamedNode::new_unchecked("http://example.org/p");
        let one: OxTerm = OxLiteral::from(1).into();

        let subject = graph
            .triples_matching(x, p.clone(), Any)
            .unwrap()
            .map(Triple::into_object)
            .next()
            .unwrap()
            .try_into()
            .unwrap();

        let actual = graph.outgoing_arcs(subject).unwrap();
        let expected = HashSet::from([one]);

        assert_eq!(actual.get(&p), Some(&expected))
    }

    #[test]
    fn test_add_triple() {
        let mut graph = SRDFGraph::default();

        let alice = OxSubject::NamedNode(OxNamedNode::new_unchecked("http://example.org/alice"));
        let knows = OxNamedNode::new_unchecked("http://example.org/knows");
        let bob = OxTerm::NamedNode(OxNamedNode::new_unchecked("http://example.org/bob"));

        graph.add_triple(alice, knows, bob).unwrap();

        assert_eq!(graph.len(), 1);
    }

    #[test]
    fn test_rdf_list() {
        let graph = graph_from_str(DUMMY_GRAPH_2);

        let x = OxNamedNode::new_unchecked("https://example.org/x").into();
        let p = OxNamedNode::new_unchecked("https://example.org/p").into();

        let mut parser = property_value(p).then(move |obj| set_focus(&obj).with(rdf_list()));
        let result: Vec<OxTerm> = parser.parse(&x, graph).unwrap();

        assert_eq!(
            result,
            vec![OxTerm::from(OxLiteral::from(1)), OxTerm::from(OxLiteral::from(2))]
        )
    }

    /*
    #[test]
    fn test_parser() {
        rdf_parser! {
            fn my_ok['a, A, RDF](value: &'a A)(RDF) -> A
            where [
                A: Clone
            ] { ok(value) }
        }
        let graph = graph_from_str("prefix : <http://example.org/>");
        let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
        assert_eq!(my_ok(&3).parse(&x, graph).unwrap(), 3)
    }*/

    #[test]
    fn test_parser_property_integers() {
        let graph = graph_from_str(DUMMY_GRAPH_3);
        let x = OxNamedNode::new_unchecked("https://example.org/x").into();
        let p = OxNamedNode::new_unchecked("https://example.org/p").into();
        let mut parser = property_integers(p);
        assert_eq!(parser.parse(&x, graph).unwrap(), HashSet::from([1, 2, 3]))
    }

    /*
    #[test]
    fn test_parser_then_mut() {
        let graph = graph_from_str(DUMMY_GRAPH_4);
        let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
        let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));

        let mut parser = property_integers(&p).then_mut(move |ns| {
            ns.extend(vec![4, 5]);
            ok(ns)
        });

        assert_eq!(
            parser.parse(&x, graph).unwrap(),
            HashSet::from([1, 2, 3, 4, 5])
        )
    } */

    #[test]
    fn test_parser_or() {
        let graph = graph_from_str(DUMMY_GRAPH_5);
        let x = OxNamedNode::new_unchecked("https://example.org/x").into();
        let p = OxNamedNode::new_unchecked("https://example.org/p").into();
        let q = OxNamedNode::new_unchecked("https://example.org/q").into();
        let mut parser = property_bool(p).or(property_bool(q));
        assert!(parser.parse(&x, graph).unwrap())
    }

    #[test]
    fn test_parser_or_enum_1() {
        let graph = graph_from_str(DUMMY_GRAPH_6);
        let x: IriS = OxNamedNode::new_unchecked("https://example.org/x").into();
        let p: IriS = OxNamedNode::new_unchecked("https://example.org/p").into();
        let parser_a_bool = property_bool(p.clone()).map(A::Bool);
        let parser_a_int = property_integer(p).map(A::Int);
        let mut parser = parser_a_int.or(parser_a_bool);
        assert_eq!(parser.parse(&x, graph).unwrap(), A::Int(1))
    }

    #[test]
    fn test_parser_or_enum_2() {
        let graph = graph_from_str(DUMMY_GRAPH_7);
        let x: IriS = OxNamedNode::new_unchecked("https://example.org/x").into();
        let p: IriS = OxNamedNode::new_unchecked("https://example.org/p").into();
        let parser_a_bool = property_bool(p.clone()).map(A::Bool);
        let parser_a_int = property_integer(p).map(A::Int);
        let mut parser = parser_a_int.or(parser_a_bool);
        assert_eq!(parser.parse(&x, graph).unwrap(), A::Bool(true))
    }

    #[test]
    fn test_parser_and() {
        let graph = graph_from_str(DUMMY_GRAPH_8);
        let x: IriS = OxNamedNode::new_unchecked("https://example.org/x").into();
        let p: IriS = OxNamedNode::new_unchecked("https://example.org/p").into();
        let q: IriS = OxNamedNode::new_unchecked("https://example.org/q").into();
        let mut parser = property_bool(p).and(property_integer(q));
        assert_eq!(parser.parse(&x, graph).unwrap(), (true, 1))
    }

    #[test]
    fn test_parser_map() {
        let graph = graph_from_str(DUMMY_GRAPH_9);
        let x: IriS = OxNamedNode::new_unchecked("https://example.org/x").into();
        let p: IriS = OxNamedNode::new_unchecked("https://example.org/p").into();
        let mut parser = property_integer(p).map(|n| n + 1);
        assert_eq!(parser.parse(&x, graph).unwrap(), 2)
    }

    #[test]
    fn test_parser_and_then() {
        let graph = graph_from_str(DUMMY_GRAPH_10);
        let x = OxNamedNode::new_unchecked("https://example.org/x").into();
        let p = OxNamedNode::new_unchecked("https://example.org/p").into();

        struct IntConversionError(String);

        fn cnv_int(s: String) -> Result<isize, IntConversionError> {
            s.parse().map_err(|_| IntConversionError(s))
        }

        impl From<IntConversionError> for RDFParseError {
            fn from(error: IntConversionError) -> RDFParseError {
                RDFParseError::Custom {
                    msg: format!("Int conversion error: {}", error.0),
                }
            }
        }

        let mut parser = property_string(p).and_then(cnv_int);
        assert_eq!(parser.parse(&x, graph).unwrap(), 1)
    }

    #[test]
    fn test_parser_flat_map() {
        let graph = graph_from_str(DUMMY_GRAPH_10);
        let x = OxNamedNode::new_unchecked("https://example.org/x").into();
        let p = OxNamedNode::new_unchecked("https://example.org/p").into();

        fn cnv_int(s: String) -> PResult<isize> {
            s.parse().map_err(|_| RDFParseError::Custom {
                msg: format!("Error converting {s}"),
            })
        }

        let mut parser = property_string(p).flat_map(cnv_int);
        assert_eq!(parser.parse(&x, graph).unwrap(), 1)
    }

    #[test]
    fn test_rdf_parser_macro() {
        rdf_parser! {
              fn is_term['a, RDF](term: &'a RDF::Term)(RDF) -> ()
              where [
              ] {
                let name = format!("is_{term}");
                satisfy(|t| { t == *term }, name.as_str())
              }
        }

        let graph = graph_from_str(DUMMY_GRAPH_9);
        let x = OxNamedNode::new_unchecked("https://example.org/x");
        let iri_s: IriS = x.into();
        let term = iri_s.clone().into();
        let mut parser = is_term(&term);
        let result = parser.parse(&iri_s, graph);
        assert!(result.is_ok())
    }

    #[test]
    fn test_not() {
        let graph = graph_from_str(DUMMY_GRAPH_9);
        let x: IriS = OxNamedNode::new_unchecked("https://example.org/x").into();
        let q: IriS = OxNamedNode::new_unchecked("https://example.org/q").into();
        assert!(not(property_value(q)).parse(&x, graph).is_ok())
    }

    #[test]
    fn test_iri() {
        let graph = SRDFGraph::default();
        let x = OxNamedNode::new_unchecked("https://example.org/x");
        let x_iri = x.clone().into();
        assert_eq!(iri().parse(&x_iri, graph).unwrap(), x)
    }

    #[test]
    fn test_add_triple_ref() {
        let mut graph = SRDFGraph::default();
        let s = OxNamedNode::new_unchecked("https://example.org/x");
        let p = OxNamedNode::new_unchecked("https://example.org/p");
        let o = OxNamedNode::new_unchecked("https://example.org/y");
        graph.add_triple_ref(&s, &p, &o).unwrap();
        assert_eq!(graph.len(), 1);
    }
}
