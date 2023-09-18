use async_trait::async_trait;
use colored::*;
use iri_s::IriS;
// use log::debug;
use oxiri::Iri;
use srdf::async_srdf::AsyncSRDF;
use srdf::{SRDFComparisons, SRDF};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::srdfgraph_error::SRDFGraphError;
use oxrdf::{
    BlankNode as OxBlankNode, Graph, Literal as OxLiteral, NamedNode as OxNamedNode,
    Subject as OxSubject, Term as OxTerm, Triple as OxTriple, TripleRef,
};
use prefix_map::prefix_map::*;
use rio_api::model::{Literal, NamedNode, Subject, Term, Triple};
use rio_api::parser::*;
use rio_turtle::*;

#[derive(Debug)]
pub struct SRDFGraph {
    graph: Graph,
    pm: PrefixMap,
}

impl SRDFGraph {
    pub fn new() -> SRDFGraph {
        SRDFGraph {
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
        let pm = PrefixMap::from_hashmap(turtle_parser.prefixes())?;
        Ok(SRDFGraph { graph: graph, pm })
    }

    pub fn resolve(&self, str: &str) -> Result<Option<OxNamedNode>, SRDFGraphError> {
        let r = self.pm.resolve(str).map(|opt| opt.map(Self::cnv_iri))?;
        Ok(r)
    }

    pub fn qualify_named_node(&self, node: &OxNamedNode) -> String {
        let iri = IriS::from_str(node.as_str()).unwrap();
        self.pm.qualify(&iri)
    }

    pub fn qualify_subject(&self, subj: &OxSubject) -> String {
        match subj {
            OxSubject::BlankNode(bn) => self.show_blanknode(bn),
            OxSubject::NamedNode(n) => self.qualify_named_node(n),
        }
    }

    pub fn show_blanknode(&self, bn: &OxBlankNode) -> String {
        let str: String = format!("{}", bn);
        format!("{}", str.green())
    }

    pub fn show_literal(&self, lit: &OxLiteral) -> String {
        let str: String = format!("{}", lit);
        format!("{}", str.red())
    }

    pub fn qualify_term(&self, term: &OxTerm) -> String {
        match term {
            OxTerm::BlankNode(bn) => self.show_blanknode(bn),
            OxTerm::Literal(lit) => self.show_literal(&lit),
            OxTerm::NamedNode(n) => self.qualify_named_node(n),
        }
    }

    pub fn from_str(data: String, base: Option<Iri<String>>) -> Result<SRDFGraph, SRDFGraphError> {
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

    fn cnv_literal(l: Literal) -> OxLiteral {
        match l {
            Literal::Simple { value } => OxLiteral::new_simple_literal(value.to_string()),
            Literal::LanguageTaggedString { value, language } => {
                OxLiteral::new_language_tagged_literal_unchecked(value, language)
            }
            Literal::Typed { value, datatype } => {
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

    pub fn parse_data(data: &String, base: &Path, _debug: u8) -> Result<SRDFGraph, SRDFGraphError> {
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

impl SRDFComparisons for SRDFGraph {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Err = SRDFGraphError;
    fn subject2iri(&self, subject: &OxSubject) -> Option<OxNamedNode> {
        match subject {
            OxSubject::NamedNode(n) => Some(n.clone()),
            _ => None,
        }
    }
    fn subject2bnode(&self, subject: &OxSubject) -> Option<OxBlankNode> {
        match subject {
            OxSubject::BlankNode(b) => Some(b.clone()),
            _ => None,
        }
    }
    fn subject_is_iri(&self, subject: &OxSubject) -> bool {
        match subject {
            OxSubject::NamedNode(_) => true,
            _ => false,
        }
    }
    fn subject_is_bnode(&self, subject: &OxSubject) -> bool {
        match subject {
            OxSubject::BlankNode(_) => true,
            _ => false,
        }
    }

    fn object2iri(&self, object: &OxTerm) -> Option<OxNamedNode> {
        match object {
            OxTerm::NamedNode(n) => Some(n.clone()),
            _ => None,
        }
    }
    fn object2bnode(&self, object: &OxTerm) -> Option<OxBlankNode> {
        match object {
            OxTerm::BlankNode(b) => Some(b.clone()),
            _ => None,
        }
    }
    fn object2literal(&self, object: &OxTerm) -> Option<OxLiteral> {
        match object {
            OxTerm::Literal(l) => Some(l.clone()),
            _ => None,
        }
    }
    fn object_is_iri(&self, object: &OxTerm) -> bool {
        match object {
            OxTerm::NamedNode(_) => true,
            _ => false,
        }
    }
    fn object_is_bnode(&self, object: &OxTerm) -> bool {
        match object {
            OxTerm::BlankNode(_) => true,
            _ => false,
        }
    }

    fn object_is_literal(&self, object: &OxTerm) -> bool {
        match object {
            OxTerm::Literal(_) => true,
            _ => false,
        }
    }

    fn term_as_subject(&self, object: &Self::Term) -> Option<Self::Subject> {
        match object {
            OxTerm::NamedNode(n) => Some(OxSubject::NamedNode(n.clone())),
            OxTerm::BlankNode(b) => Some(OxSubject::BlankNode(b.clone())),
            _ => None,
        }
    }

    fn lexical_form(&self, literal: &OxLiteral) -> String {
        literal.to_string()
    }

    fn lang(&self, literal: &OxLiteral) -> Option<String> {
        literal.language().map(|s| s.to_string())
    }

    fn datatype(&self, literal: &OxLiteral) -> OxNamedNode {
        literal.datatype().into_owned()
    }

    fn iri_s2iri(iri_s: &IriS) -> &OxNamedNode {
        iri_s.as_named_node()
    }

    fn iri_as_term(iri: OxNamedNode) -> OxTerm {
        OxTerm::NamedNode(iri)
    }

    fn iri2iri_s(iri: OxNamedNode) -> IriS {
        IriS::from_named_node(iri)
    }

    fn term2object(term: OxTerm) -> srdf::Object {
        match term {
            OxTerm::BlankNode(bn) => srdf::Object::BlankNode(bn.to_string()),
            OxTerm::Literal(lit) => match lit.destruct() {
                (s, None, None) => srdf::Object::Literal(srdf::literal::Literal::StringLiteral {
                    lexical_form: s,
                    lang: None,
                }),
                (s, None, Some(lang)) => {
                    srdf::Object::Literal(srdf::literal::Literal::StringLiteral {
                        lexical_form: s,
                        lang: Some(srdf::lang::Lang::new(lang)),
                    })
                }
                (s, Some(datatype), _) => {
                    srdf::Object::Literal(srdf::literal::Literal::DatatypeLiteral {
                        lexical_form: s,
                        datatype: Self::iri2iri_s(datatype),
                    })
                }
            },
            OxTerm::NamedNode(iri) => srdf::Object::Iri {
                iri: Self::iri2iri_s(iri),
            },
        }
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

    fn get_subjects_for_object_predicate(
        &self,
        _object: &Self::Term,
        _pred: &Self::IRI,
    ) -> Result<HashSet<Self::Subject>, Self::Err> {
        todo!()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SRDFGraph;
    use oxrdf::{Graph, SubjectRef};
    use rio_api::model::{Literal, Subject};
    use srdf::SRDF;

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
        let parsed_graph = SRDFGraph::from_str(s.to_string(), None).unwrap();
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
}
