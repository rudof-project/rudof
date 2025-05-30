use crate::async_srdf::AsyncSRDF;
use crate::{FocusRDF, Query, RDFFormat, Rdf, SRDFBuilder, RDF_TYPE_STR};
use async_trait::async_trait;
use colored::*;
use iri_s::IriS;
use oxrdfio::{RdfFormat, RdfSerializer};
use oxrdfxml::RdfXmlParser;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing::debug;

use crate::srdfgraph_error::SRDFGraphError;
use oxrdf::{
    BlankNode as OxBlankNode, Graph, GraphName, Literal as OxLiteral, NamedNode as OxNamedNode,
    NamedNodeRef, Quad, Subject as OxSubject, SubjectRef, Term as OxTerm, TermRef,
    Triple as OxTriple, TripleRef,
};
use oxttl::{NQuadsParser, NTriplesParser, TurtleParser};
use prefixmap::{prefixmap::*, PrefixMapError};

#[derive(Debug, Default, Clone)]
pub struct SRDFGraph {
    focus: Option<OxTerm>,
    graph: Graph,
    pm: PrefixMap,
    base: Option<IriS>,
    bnode_counter: usize,
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
        self.graph
            .iter()
            .map(move |t| triple_to_quad(t, graph_name.clone()))
    }

    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

    pub fn merge_from_reader<R: io::Read>(
        &mut self,
        read: R,
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
                // let mut graph = Graph::default();
                let mut reader = turtle_parser.for_reader(read);
                for triple_result in reader.by_ref() {
                    self.graph.insert(triple_result?.as_ref());
                }
                let prefixes: HashMap<&str, &str> = reader.prefixes().collect();
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
                let mut reader = parser.for_reader(read);
                for triple_result in reader.by_ref() {
                    match triple_result {
                        Err(e) => {
                            if reader_mode.is_strict() {
                                return Err(SRDFGraphError::TurtleError {
                                    data: "Reading n-quads".to_string(),
                                    turtle_error: e,
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
                let mut reader = parser.for_reader(read);
                for triple_result in reader.by_ref() {
                    match triple_result {
                        Err(e) => {
                            debug!("Error captured: {e:?}")
                        }
                        Ok(t) => {
                            self.graph.insert(t.as_ref());
                        }
                    }
                }
            }
            RDFFormat::TriG => todo!(),
            RDFFormat::N3 => todo!(),
            RDFFormat::NQuads => {
                let parser = NQuadsParser::new();
                let mut reader = parser.for_reader(read);
                for triple_result in reader.by_ref() {
                    match triple_result {
                        Err(e) => {
                            debug!("Error captured: {e:?}")
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

    pub fn merge_prefixes(&mut self, prefixmap: PrefixMap) -> Result<(), SRDFGraphError> {
        self.pm.merge(prefixmap)?;
        Ok(())
    }

    pub fn from_reader<R: io::Read>(
        read: R,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<SRDFGraph, SRDFGraphError> {
        let mut srdf_graph = SRDFGraph::new();

        srdf_graph.merge_from_reader(read, format, base, reader_mode)?;
        Ok(srdf_graph)
    }

    pub fn resolve(&self, str: &str) -> Result<OxNamedNode, SRDFGraphError> {
        let r = self.pm.resolve(str)?;
        Ok(Self::cnv_iri(r))
    }

    pub fn show_blanknode(&self, bn: &OxBlankNode) -> String {
        let str: String = format!("{}", bn);
        format!("{}", str.green())
    }

    pub fn show_literal(&self, lit: &OxLiteral) -> String {
        let str: String = format!("{}", lit);
        format!("{}", str.red())
    }

    pub fn from_str(
        data: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<SRDFGraph, SRDFGraphError> {
        Self::from_reader(std::io::Cursor::new(&data), format, base, reader_mode)
    }

    fn cnv_iri(iri: IriS) -> OxNamedNode {
        OxNamedNode::new_unchecked(iri.as_str())
    }

    pub fn add_triple_ref<'a, S, P, O>(
        &mut self,
        subj: S,
        pred: P,
        obj: O,
    ) -> Result<(), SRDFGraphError>
    where
        S: Into<SubjectRef<'a>>,
        P: Into<NamedNodeRef<'a>>,
        O: Into<TermRef<'a>>,
    {
        let subj: SubjectRef<'a> = subj.into();
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
        let reader = BufReader::new(file);
        Self::merge_from_reader(self, reader, format, base, reader_mode)?;
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
        let reader = BufReader::new(file);
        Self::from_reader(reader, format, base, reader_mode)
    }

    pub fn parse_data(
        data: &String,
        format: &RDFFormat,
        base: &Path,
        reader_mode: &ReaderMode,
    ) -> Result<SRDFGraph, SRDFGraphError> {
        let mut attempt = PathBuf::from(base);
        attempt.push(data);
        let base = Some("base:://");
        let data_path = &attempt;
        let graph = Self::from_path(data_path, format, base, reader_mode)?;
        Ok(graph)
    }

    pub fn prefixmap(&self) -> PrefixMap {
        self.pm.clone()
    }
}

impl Rdf for SRDFGraph {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Triple = OxTriple;
    type Err = SRDFGraphError;

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
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => unimplemented!(),
        }
    }

    fn qualify_term(&self, term: &OxTerm) -> String {
        match term {
            OxTerm::BlankNode(bn) => self.show_blanknode(bn),
            OxTerm::Literal(lit) => self.show_literal(lit),
            OxTerm::NamedNode(n) => self.qualify_iri(n),
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => unimplemented!(),
        }
    }

    fn prefixmap(&self) -> Option<prefixmap::PrefixMap> {
        Some(self.pm.clone())
    }
}

impl Query for SRDFGraph {
    fn triples(&self) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        Ok(self.graph.iter().map(TripleRef::into_owned))
    }
}

#[async_trait]
impl AsyncSRDF for SRDFGraph {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Err = SRDFGraphError;

    async fn get_predicates_subject(
        &self,
        subject: &OxSubject,
    ) -> Result<HashSet<OxNamedNode>, SRDFGraphError> {
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

impl SRDFBuilder for SRDFGraph {
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
            Err(_) => Err(SRDFGraphError::BlankNodeId {
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
        SRDFGraph {
            focus: None,
            graph: Graph::new(),
            pm: PrefixMap::new(),
            base: None,
            bnode_counter: 0,
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
    }
}

fn rdf_type() -> OxNamedNode {
    OxNamedNode::new_unchecked(RDF_TYPE_STR)
}

fn triple_to_quad(t: TripleRef, graph_name: GraphName) -> Quad {
    let subj: oxrdf::Subject = t.subject.into();
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use iri_s::IriS;
    use oxrdf::Literal as OxLiteral;
    use oxrdf::NamedNode as OxNamedNode;
    use oxrdf::Subject as OxSubject;
    use oxrdf::Term as OxTerm;

    use crate::iri;
    use crate::matcher::Any;
    use crate::not;
    use crate::ok;
    use crate::property_bool;
    use crate::property_integer;
    use crate::property_integers;
    use crate::property_string;
    use crate::property_value;
    use crate::rdf_list;
    use crate::rdf_parser;
    use crate::satisfy;
    use crate::set_focus;
    use crate::PResult;
    use crate::Query as _;
    use crate::RDFFormat;
    use crate::RDFNodeParse as _;
    use crate::RDFParseError;
    use crate::SRDFBuilder;
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

    const DUMMY_GRAPH_4: &str = r#"
        prefix : <http://example.org/>
        :x :p 1, 2, 3 .
    "#;

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

        let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
        let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));

        let mut parser = property_value(&p).then(move |obj| set_focus(&obj).with(rdf_list()));
        let result: Vec<OxTerm> = parser.parse(&x, graph).unwrap();

        assert_eq!(
            result,
            vec![
                OxTerm::from(OxLiteral::from(1)),
                OxTerm::from(OxLiteral::from(2))
            ]
        )
    }

    #[test]
    fn test_parser() {
        rdf_parser! {
            fn my_ok['a, A, RDF](value: &'a A)(RDF) -> A
            where [
                A: Clone
            ] { ok(&value.clone()) }
        }
        let graph = graph_from_str("prefix : <http://example.org/>");
        let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
        assert_eq!(my_ok(&3).parse(&x, graph).unwrap(), 3)
    }

    #[test]
    fn test_parser_property_integers() {
        let graph = graph_from_str(DUMMY_GRAPH_3);
        let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
        let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));
        let mut parser = property_integers(&p);
        assert_eq!(parser.parse(&x, graph).unwrap(), HashSet::from([1, 2, 3]))
    }

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
    }

    #[test]
    fn test_parser_or() {
        let graph = graph_from_str(DUMMY_GRAPH_5);
        let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
        let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));
        let q = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/q"));
        let mut parser = property_bool(&p).or(property_bool(&q));
        assert!(parser.parse(&x, graph).unwrap())
    }

    #[test]
    fn test_parser_or_enum_1() {
        let graph = graph_from_str(DUMMY_GRAPH_6);
        let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
        let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));
        let parser_a_bool = property_bool(&p).map(A::Bool);
        let parser_a_int = property_integer(&p).map(A::Int);
        let mut parser = parser_a_int.or(parser_a_bool);
        assert_eq!(parser.parse(&x, graph).unwrap(), A::Int(1))
    }

    #[test]
    fn test_parser_or_enum_2() {
        let graph = graph_from_str(DUMMY_GRAPH_7);
        let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
        let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));
        let parser_a_bool = property_bool(&p).map(A::Bool);
        let parser_a_int = property_integer(&p).map(A::Int);
        let mut parser = parser_a_int.or(parser_a_bool);
        assert_eq!(parser.parse(&x, graph).unwrap(), A::Bool(true))
    }

    #[test]
    fn test_parser_and() {
        let graph = graph_from_str(DUMMY_GRAPH_8);
        let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
        let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));
        let q = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/q"));
        let mut parser = property_bool(&p).and(property_integer(&q));
        assert_eq!(parser.parse(&x, graph).unwrap(), (true, 1))
    }

    #[test]
    fn test_parser_map() {
        let graph = graph_from_str(DUMMY_GRAPH_9);
        let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
        let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));
        let mut parser = property_integer(&p).map(|n| n + 1);
        assert_eq!(parser.parse(&x, graph).unwrap(), 2)
    }

    #[test]
    fn test_parser_and_then() {
        let graph = graph_from_str(DUMMY_GRAPH_10);
        let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
        let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));

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

        let mut parser = property_string(&p).and_then(cnv_int);
        assert_eq!(parser.parse(&x, graph).unwrap(), 1)
    }

    #[test]
    fn test_parser_flat_map() {
        let graph = graph_from_str(DUMMY_GRAPH_10);
        let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
        let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));

        fn cnv_int(s: String) -> PResult<isize> {
            s.parse().map_err(|_| RDFParseError::Custom {
                msg: format!("Error converting {s}"),
            })
        }

        let mut parser = property_string(&p).flat_map(cnv_int);
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
        let x = OxNamedNode::new_unchecked("http://example.org/x");
        let iri_s = IriS::from_named_node(&x);
        let term = x.clone().into();
        let mut parser = is_term(&term);
        let result = parser.parse(&iri_s, graph);
        assert!(result.is_ok())
    }

    #[test]
    fn test_not() {
        let graph = graph_from_str(DUMMY_GRAPH_9);
        let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
        let q = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/q"));
        assert!(not(property_value(&q)).parse(&x, graph).is_ok())
    }

    #[test]
    fn test_iri() {
        let graph = SRDFGraph::default();
        let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
        assert_eq!(iri().parse(&x, graph).unwrap(), x)
    }

    #[test]
    fn test_add_triple_ref() {
        let mut graph = SRDFGraph::default();
        let s = OxNamedNode::new_unchecked("http://example.org/x");
        let p = OxNamedNode::new_unchecked("http://example.org/p");
        let o = OxNamedNode::new_unchecked("http://example.org/y");
        graph.add_triple_ref(&s, &p, &o).unwrap();
        assert_eq!(graph.len(), 1);
    }
}
