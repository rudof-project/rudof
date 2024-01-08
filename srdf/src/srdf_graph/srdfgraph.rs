use async_trait::async_trait;
use colored::*;
use iri_s::{IriS, iri};
// use log::debug;
use oxiri::Iri;
use crate::async_srdf::AsyncSRDF;
use crate::{FocusRDF, SRDFBasic, SRDF, Triple as STriple};
use crate::literal::Literal;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::Object;
use crate::lang::Lang;
use crate::srdfgraph_error::SRDFGraphError;
use oxrdf::{
    BlankNode as OxBlankNode, Graph, Literal as OxLiteral, NamedNode as OxNamedNode,
    Subject as OxSubject, Term as OxTerm, Triple as OxTriple, TripleRef
};
use prefixmap::{prefixmap::*, IriRef, PrefixMapError};
use rio_api::model::{Literal as RioLiteral, NamedNode, Subject, Term, Triple, BlankNode};
use rio_api::parser::*;
use rio_turtle::*;

#[derive(Debug)]
pub struct SRDFGraph {
    focus: Option<OxTerm>,
    graph: Graph,
    pm: PrefixMap,
}

impl SRDFGraph {
    pub fn new() -> SRDFGraph {
        SRDFGraph {
            focus: None,
            graph: Graph::new(),
            pm: PrefixMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.graph.len()
    }

    pub fn from_reader<R: BufRead>(
        reader: R,
        base: Option<Iri<String>>,
    ) -> Result<SRDFGraph, SRDFGraphError> {
        let mut turtle_parser = TurtleParser::new(reader, base);
        let mut graph = Graph::default();
        turtle_parser.parse_all(&mut |triple| {
            let ox_triple = Self::cnv(triple);
            let triple_ref: TripleRef = ox_triple.as_ref();
            graph.insert(triple_ref);
            Ok(()) as Result<(), TurtleError>
        })?;
        let prefixes: HashMap<&str, &str> = turtle_parser
            .prefixes()
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect();
        let pm = PrefixMap::from_hashmap(&prefixes)?;
        Ok(SRDFGraph {
            focus: None,
            graph: graph,
            pm,
        })
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

    pub fn from_str(data: &str, base: Option<Iri<String>>) -> Result<SRDFGraph, SRDFGraphError> {
        Self::from_reader(std::io::Cursor::new(&data), base)
    }

    fn cnv_iri(iri: IriS) -> OxNamedNode {
        OxNamedNode::new_unchecked(iri.as_str())
    }

    fn cnv_subject(s: Subject) -> OxSubject {
        match s {
            Subject::NamedNode(n) => {
                OxSubject::NamedNode(OxNamedNode::new_unchecked(n.iri.to_string()))
            }
            Subject::BlankNode(b) => OxSubject::BlankNode(OxBlankNode::new_unchecked(b.id)),
            Subject::Triple(_) => todo!(),
        }
    }

    fn cnv_named_node(s: NamedNode) -> OxNamedNode {
        OxNamedNode::new_unchecked(s.iri)
    }

    fn cnv_literal(l: RioLiteral) -> OxLiteral {
        match l {
            RioLiteral::Simple { value } => OxLiteral::new_simple_literal(value.to_string()),
            RioLiteral::LanguageTaggedString { value, language } => {
                OxLiteral::new_language_tagged_literal_unchecked(value, language)
            }
            RioLiteral::Typed { value, datatype } => {
                OxLiteral::new_typed_literal(value, Self::cnv_named_node(datatype))
            }
        }
    }
    fn cnv_object(s: Term) -> OxTerm {
        match s {
            Term::NamedNode(n) => OxTerm::NamedNode(OxNamedNode::new_unchecked(n.iri.to_string())),
            Term::BlankNode(b) => OxTerm::BlankNode(OxBlankNode::new_unchecked(b.id)),
            Term::Literal(l) => OxTerm::Literal(Self::cnv_literal(l)),
            Term::Triple(_) => todo!(),
        }
    }

    fn cnv(t: Triple) -> OxTriple {
        OxTriple::new(
            Self::cnv_subject(t.subject),
            Self::cnv_named_node(t.predicate),
            Self::cnv_object(t.object),
        )
    }

    pub fn from_path(
        path: &PathBuf,
        base: Option<Iri<String>>,
    ) -> Result<SRDFGraph, SRDFGraphError> {
        let file = File::open(path).map_err(|e| SRDFGraphError::ReadingPathError {
            path_name: path.display().to_string(),
            error: e,
        })?;
        let reader = BufReader::new(file);
        Self::from_reader(reader, base)
    }

    pub fn parse_data(data: &String, base: &Path) -> Result<SRDFGraph, SRDFGraphError> {
        let mut attempt = PathBuf::from(base);
        attempt.push(data);
        let base = Some(Iri::parse("base:://".to_owned()).unwrap());
        let data_path = &attempt;
        let graph = Self::from_path(data_path, base)?;
        Ok(graph)
    }

    pub fn prefixmap(&self) -> PrefixMap {
        self.pm.clone()
    }
}

impl SRDFBasic for SRDFGraph {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Err = SRDFGraphError;

    fn subject_as_iri(subject: &OxSubject) -> Option<OxNamedNode> {
        match subject {
            OxSubject::NamedNode(n) => Some(n.clone()),
            _ => None,
        }
    }
    fn subject_as_bnode(subject: &OxSubject) -> Option<OxBlankNode> {
        match subject {
            OxSubject::BlankNode(b) => Some(b.clone()),
            _ => None,
        }
    }
    fn subject_is_iri(subject: &OxSubject) -> bool {
        match subject {
            OxSubject::NamedNode(_) => true,
            _ => false,
        }
    }
    fn subject_is_bnode(subject: &OxSubject) -> bool {
        match subject {
            OxSubject::BlankNode(_) => true,
            _ => false,
        }
    }

    fn term_as_iri(object: &OxTerm) -> Option<OxNamedNode> {
        match object {
            OxTerm::NamedNode(n) => Some(n.clone()),
            _ => None,
        }
    }
    fn term_as_bnode(object: &OxTerm) -> Option<OxBlankNode> {
        match object {
            OxTerm::BlankNode(b) => Some(b.clone()),
            _ => None,
        }
    }

    fn term_as_literal(object: &OxTerm) -> Option<OxLiteral> {
        match object {
            OxTerm::Literal(l) => Some(l.clone()),
            _ => None,
        }
    }

    fn term_is_iri(object: &OxTerm) -> bool {
        match object {
            OxTerm::NamedNode(_) => true,
            _ => false,
        }
    }
    fn term_is_bnode(object: &OxTerm) -> bool {
        match object {
            OxTerm::BlankNode(_) => true,
            _ => false,
        }
    }

    fn term_is_literal(object: &OxTerm) -> bool {
        match object {
            OxTerm::Literal(_) => true,
            _ => false,
        }
    }

    fn subject_as_term(subject: &Self::Subject) -> Self::Term {
        match subject {
            OxSubject::NamedNode(n) => OxTerm::NamedNode(n.clone()),
            OxSubject::BlankNode(b) => OxTerm::BlankNode(b.clone()),
        }
    }

    fn term_as_subject(object: &Self::Term) -> Option<Self::Subject> {
        match object {
            OxTerm::NamedNode(n) => Some(OxSubject::NamedNode(n.clone())),
            OxTerm::BlankNode(b) => Some(OxSubject::BlankNode(b.clone())),
            _ => None,
        }
    }

    fn lexical_form(literal: &OxLiteral) -> &str {
        literal.value()
    }

    fn lang(literal: &OxLiteral) -> Option<String> {
        literal.language().map(|s| s.to_string())
    }

    fn datatype(literal: &OxLiteral) -> OxNamedNode {
        literal.datatype().into_owned()
    }

    fn iri_s2iri(iri_s: &IriS) -> OxNamedNode {
        iri_s.as_named_node().clone()
    }

    fn iri_as_term(iri: OxNamedNode) -> OxTerm {
        OxTerm::NamedNode(iri)
    }

    fn iri_as_subject(iri: OxNamedNode) -> OxSubject {
        OxSubject::NamedNode(iri)
    }

    fn iri2iri_s(iri: &OxNamedNode) -> IriS {
        IriS::from_named_node(iri)
    }

    fn term_as_object(term: &OxTerm) -> Object {
        match term {
            OxTerm::BlankNode(bn) => Object::BlankNode(bn.to_string()),
            OxTerm::Literal(lit) => {
                let lit = lit.to_owned();
                match lit.destruct() {
                    (s, None, None) => {
                        Object::Literal(Literal::StringLiteral {
                            lexical_form: s,
                            lang: None,
                        })
                    }
                    (s, None, Some(lang)) => {
                        Object::Literal(Literal::StringLiteral {
                            lexical_form: s,
                            lang: Some(Lang::new(lang.as_str())),
                        })
                    }
                    (s, Some(datatype), _) => {
                        let iri_s = Self::iri2iri_s(&datatype);
                        Object::Literal(Literal::DatatypeLiteral {
                            lexical_form: s,
                            datatype: IriRef::Iri(iri_s),
                        })
                    }
                }
            }
            OxTerm::NamedNode(iri) => Object::Iri {
                iri: Self::iri2iri_s(iri),
            },
        }
    }

    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError> {
        let iri = self.pm.resolve_prefix_local(prefix, local)?;
        Ok(iri.clone())
    }

    fn qualify_iri(&self, node: &OxNamedNode) -> String {
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
            OxTerm::Literal(lit) => self.show_literal(&lit),
            OxTerm::NamedNode(n) => self.qualify_iri(n),
        }
    }

    fn prefixmap(&self) -> Option<prefixmap::PrefixMap> { 
        Some(self.pm.clone())
    }

    fn bnode_id2bnode(id: &str) -> Self::BNode {
        OxBlankNode::new_unchecked(id)
    }

    fn bnode_as_term(bnode: Self::BNode) -> Self::Term {
        OxTerm::BlankNode(bnode)
    }
}

impl SRDF for SRDFGraph {
    fn get_predicates_for_subject(
        &self,
        subject: &Self::Subject,
    ) -> Result<HashSet<Self::IRI>, Self::Err> {
        let mut ps = HashSet::new();
        for triple in self.graph.triples_for_subject(subject) {
            let pred = triple.predicate.into_owned();
            ps.insert(pred);
        }
        Ok(ps)
    }

    fn get_objects_for_subject_predicate(
        &self,
        subject: &Self::Subject,
        pred: &Self::IRI,
    ) -> Result<HashSet<Self::Term>, Self::Err> {
        let predicate = pred.as_ref();
        let mut result = HashSet::new();
        for o in self.graph.objects_for_subject_predicate(subject, predicate) {
            result.insert(o.into_owned());
        }
        Ok(result)
    }

    fn subjects_with_predicate_object(
        &self,
        pred: &Self::IRI,
        object: &Self::Term,
    ) -> Result<HashSet<Self::Subject>, Self::Err> {
        let mut result = HashSet::new();
        for subj in self
            .graph
            .subjects_for_predicate_object(pred.as_ref(), object.as_ref())
        {
            result.insert(subj.into_owned());
        }
        Ok(result)
    }

    fn outgoing_arcs(
        &self,
        subject: &Self::Subject,
    ) -> Result<HashMap<Self::IRI, HashSet<Self::Term>>, Self::Err> {
        let mut results: HashMap<Self::IRI, HashSet<Self::Term>> = HashMap::new();
        for triple in self.graph.triples_for_subject(subject) {
            let pred = triple.predicate.into_owned();
            let term = triple.object.into_owned();
            match results.entry(pred) {
                Entry::Occupied(mut vs) => {
                    vs.get_mut().insert(term.clone());
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(HashSet::from([term.clone()]));
                }
            }
        }
        println!("Outgoing arcs for {subject:?} = {results:?}");
        Ok(results)
    }

    fn incoming_arcs(
        &self,
        object: &Self::Term,
    ) -> Result<HashMap<Self::IRI, HashSet<Self::Subject>>, Self::Err> {
        let mut results: HashMap<Self::IRI, HashSet<Self::Subject>> = HashMap::new();
        for triple in self.graph.triples_for_object(object) {
            let pred = triple.predicate.into_owned();
            let subj = triple.subject.into_owned();
            match results.entry(pred) {
                Entry::Occupied(mut vs) => {
                    vs.get_mut().insert(subj.clone());
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(HashSet::from([subj.clone()]));
                }
            }
        }
        Ok(results)
    }

    fn outgoing_arcs_from_list(
        &self,
        subject: &Self::Subject,
        preds: Vec<Self::IRI>,
    ) -> Result<(HashMap<Self::IRI, HashSet<Self::Term>>, Vec<Self::IRI>), Self::Err> {
        let mut results: HashMap<Self::IRI, HashSet<Self::Term>> = HashMap::new();
        let mut remainder = Vec::new();
        for triple in self.graph.triples_for_subject(subject) {
            let pred = triple.predicate.into_owned();
            let term = triple.object.into_owned();
            if preds.contains(&pred) {
                match results.entry(pred) {
                    Entry::Occupied(mut vs) => {
                        vs.get_mut().insert(term.clone());
                    }
                    Entry::Vacant(vacant) => {
                        vacant.insert(HashSet::from([term.clone()]));
                    }
                }
            } else {
                remainder.push(pred)
            }
        }
        Ok((results, remainder))
    }

    fn triples_with_predicate(
        &self,
        pred: &Self::IRI
    ) -> Result<Vec<crate::Triple<Self>>, Self::Err> {
        let mut result = Vec::new();
        for triple in self.graph.triples_for_predicate(pred) {
           let subj = triple.subject.into_owned();
           let pred = triple.predicate.into_owned();
           let obj = triple.object.into_owned();
           result.push(STriple::new(subj,pred,obj))
        }
        Ok(result)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SRDFGraph;
    use oxrdf::{Graph, SubjectRef};
    use rio_api::model::{Literal, Subject};
    use crate::SRDF;

    #[test]
    fn parse_turtle() {
        let mut graph = Graph::default();
        let s = r#"PREFIX : <http://example.org/>
           :alice :name "Alice" ;
                  :knows [ :name "Bob" ], _:1 .
           _:1    :name "Carol" . 
         "#;
        let mut count = 0;
        let mut parser = TurtleParser::new(std::io::Cursor::new(&s), None);

        let res = parser.parse_all(&mut |triple| {
            count += 1;
            let t = SRDFGraph::cnv(triple);
            graph.insert(t.as_ref());
            Ok(()) as Result<(), TurtleError>
        });
        assert!(res.is_ok());
        assert_eq!(graph.len(), 5);
        let alice = OxNamedNode::new_unchecked("http://example.org/alice");
        assert_eq!(graph.triples_for_subject(alice.as_ref()).count(), 3);
        assert_eq!(count, 5)
    }

    #[tokio::test]
    async fn parse_get_predicates() {
        use crate::srdfgraph::AsyncSRDF;

        let s = r#"PREFIX : <http://example.org/>
            PREFIX schema: <https://schema.org/>

            :alice schema:name "Alice" ;
                   schema:knows :bob, :carol ;
                   :age  23 .
         "#;
        let parsed_graph = SRDFGraph::from_str(s, None).unwrap();
        let alice = OxSubject::NamedNode(OxNamedNode::new_unchecked("http://example.org/alice"));
        let knows = OxNamedNode::new_unchecked("https://schema.org/knows");
        let bag_preds = parsed_graph.get_predicates_subject(&alice).await.unwrap();
        assert_eq!(bag_preds.contains(&knows), true);
        let bob = OxTerm::NamedNode(OxNamedNode::new_unchecked("http://example.org/bob"));
        let alice_knows =
            AsyncSRDF::get_objects_for_subject_predicate(&parsed_graph, &alice, &knows)
                .await
                .unwrap();
        assert_eq!(alice_knows.contains(&bob), true);
    }

    #[test]
    fn test_rdf_nil() {
        use crate::SRDFBasic;
        use crate::SRDF;

        let s = r#"prefix : <http://example.org/>
        prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        
        :x :p rdf:nil .
        "#;

        let graph = SRDFGraph::from_str(s, None).unwrap();
        // let p = SRDFBasic::iri_s2iri(&iri!("http://example.org/p"));
        // let x = SRDFBasic::iri_as_subject(iri!("http://example.org/x"));
        // let rs = SRDF::get_objects_for_subject_predicate(&graph, x, p);
        // let mut parser = property_values(&p);
        // let result = parser.parse(&x, &graph).unwrap();
    }


    #[test]
    fn test_parser() {
        use crate::{RDFNodeParse, rdf_parser, ok};
        rdf_parser!{
            fn my_ok['a, A, RDF](value: &'a A)(RDF) -> A
            where [
                A: Clone
            ] { ok(&value.clone()) }
        }
        let s = r#"prefix : <http://example.org/>"#;
        let mut graph = SRDFGraph::from_str(s, None).unwrap();
        let x = iri!("http://example.org/x");
        assert_eq!(my_ok(&3).parse(&x, &mut graph).unwrap(), 3)
    } 

    #[test]
    fn test_parser_property_integers() {
        use crate::{RDFNodeParse, property_integers};
        let s = r#"prefix : <http://example.org/>
          :x :p 1, 2, 3, 2 .
        "#;
        let mut graph = SRDFGraph::from_str(s, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");
        let mut parser = property_integers(&p);
        assert_eq!(parser.parse(&x, &mut graph).unwrap(), HashSet::from([1, 2, 3]))
    }

    #[test]
    fn test_parser_then_mut() {
        use crate::{RDFNodeParse, ok, property_integers};
        let s = r#"prefix : <http://example.org/>
          :x :p 1, 2, 3 .
        "#;
        let mut graph = SRDFGraph::from_str(s, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");
        let mut parser = property_integers(&p).then_mut(move |ns| {
            ns.extend(vec![4, 5]);
            ok(ns)
         });
        assert_eq!(parser.parse(&x, &mut graph).unwrap(), HashSet::from([1, 2, 3, 4, 5]))
    }

    #[test]
    fn test_parser_or() {
        use crate::{RDFNodeParse, property_bool};
        let s = r#"prefix : <http://example.org/>
          :x :p 1, 2 ;
             :q true .
        "#;
        let mut graph = SRDFGraph::from_str(s, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");
        let q = iri!("http://example.org/q");
        let mut parser = property_bool(&p).or(property_bool(&q));
        assert_eq!(parser.parse(&x, &mut graph).unwrap(), true)
    }

    #[test]
    fn test_parser_and() {
        use crate::{RDFNodeParse, property_bool, property_integer};
        let s = r#"prefix : <http://example.org/>
          :x :p true ;
             :q 1    .
        "#;
        let mut graph = SRDFGraph::from_str(s, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");
        let q = iri!("http://example.org/q");
        let mut parser = property_bool(&p).and(property_integer(&q));
        assert_eq!(parser.parse(&x, &mut graph).unwrap(), (true, 1))
    }

    #[test]
    fn test_parser_map() {
        use crate::{RDFNodeParse, property_integer};
        let s = r#"prefix : <http://example.org/>
          :x :p 1 . 
        "#;
        let mut graph = SRDFGraph::from_str(s, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");
        let mut parser = property_integer(&p).map(|n| n + 1);
        assert_eq!(parser.parse(&x, &mut graph).unwrap(), 2)
    }

    #[test]
    fn test_parser_and_then() {
        use crate::{RDFNodeParse, RDFParseError, property_string};
        let s = r#"prefix : <http://example.org/>
          :x :p "1" .
        "#;
        let mut graph = SRDFGraph::from_str(s, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");
        struct IntConversionError(String);
        fn cnv_int(s: String) -> Result<isize, IntConversionError> {
           s.parse().map_err(|_| IntConversionError(s))
        }

        impl Into<RDFParseError> for IntConversionError {
            fn into(self) -> RDFParseError {
                RDFParseError::Custom { msg: format!("Int conversion error: {}", self.0)}
            }
        }
        
        let mut parser = property_string(&p).and_then(cnv_int);
        assert_eq!(parser.parse(&x, &mut graph).unwrap(), 1)
    }

    #[test]
    fn test_parser_flat_map() {
        use crate::{RDFNodeParse, RDFParseError, property_string, PResult};
        let s = r#"prefix : <http://example.org/>
          :x :p "1" .
        "#;
        let mut graph = SRDFGraph::from_str(s, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");
        
        fn cnv_int(s: String) -> PResult<isize> {
           s.parse().map_err(|_| RDFParseError::Custom{ msg: format!("Error converting {s}")})
        }

        let mut parser = property_string(&p).flat_map(cnv_int);
        assert_eq!(parser.parse(&x, &mut graph).unwrap(), 1)
    }


    #[test]
    fn test_rdf_parser_macro() {
        use iri_s::{IriS, iri};
        use crate::SRDFGraph;
        use crate::{SRDFBasic, rdf_parser, satisfy, RDFNodeParse};
        
        rdf_parser! {
              fn is_term['a, RDF](term: &'a RDF::Term)(RDF) -> ()
              where [
              ] { 
               let name = format!("is_{term}");
               satisfy(|t| { t == *term }, name.as_str()) 
              }
        }
        
        let s = r#"prefix : <http://example.org/>
                   :x :p 1.
        "#;
        let mut graph = SRDFGraph::from_str(s, None).unwrap();
        let x = iri!("http://example.org/x");
        let term = <SRDFGraph as SRDFBasic>::iri_s2term(&x);
        assert_eq!(is_term(&term).parse(&x, &mut graph).unwrap(), ()) 
    }

}
