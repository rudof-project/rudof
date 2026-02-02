use crate::rdf_core::{
    Any, BuildRDF, NeighsRDF, RDFError, RDFFormat, 
    parser::rdf_node_parser::{
        ParserExt, RDFNodeParse, 
        constructors::{
            IntegersPropertyParser, IriParser, ListParser, SetFocusParser, SingleBoolPropertyParser, SingleIntegerPropertyParser, 
            SingleStringPropertyParser, SingleValuePropertyParser, SatisfyParser
        }
    }, 
    term::Triple
};
use crate::rdf_impl::{InMemoryGraph, ReaderMode};
use crate::rdf_parser;
use iri_s::IriS;
use oxrdf::Literal as OxLiteral;
use oxrdf::NamedNode as OxNamedNode;
use oxrdf::NamedOrBlankNode as OxSubject;
use oxrdf::Term as OxTerm;
use std::collections::HashSet;

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

fn graph_from_str(s: &str) -> InMemoryGraph {
    InMemoryGraph::from_str(s, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap()
}

#[test]
fn test_triples_matching_subject_predicate_and_object() {
    let graph = graph_from_str(DUMMY_GRAPH);
    let x: OxSubject = OxNamedNode::new_unchecked("http://example.org/x").into();
    let p = OxNamedNode::new_unchecked("http://example.org/p");
    let one: OxTerm = OxLiteral::from(1).into();
    let triples = graph.triples_matching(&x, &p, &one).unwrap();
    assert_eq!(triples.count(), 1)
}

#[test]
fn test_triples_matching_subject_and_predicate() {
    let graph = graph_from_str(DUMMY_GRAPH);
    let x: OxSubject = OxNamedNode::new_unchecked("http://example.org/x").into();
    let p = OxNamedNode::new_unchecked("http://example.org/p");
    let triples = graph.triples_matching(&x, &p, &Any).unwrap();
    assert_eq!(triples.count(), 1)
}

#[test]
fn test_triples_matching_subject_and_object() {
    let graph = graph_from_str(DUMMY_GRAPH);
    let x: OxSubject = OxNamedNode::new_unchecked("http://example.org/x").into();
    let one: OxTerm = OxLiteral::from(1).into();
    let triples = graph.triples_matching(&x, &Any, &one).unwrap();
    assert_eq!(triples.count(), 1)
}

#[test]
fn test_triples_matching_predicate_and_object() {
    let graph = graph_from_str(DUMMY_GRAPH);
    let p = OxNamedNode::new_unchecked("http://example.org/p");
    let one: OxTerm = OxLiteral::from(1).into();
    let triples = graph.triples_matching(&Any, &p, &one).unwrap();
    assert_eq!(triples.count(), 1)
}

#[test]
fn test_triples_matching_subject() {
    let graph = graph_from_str(DUMMY_GRAPH);
    let x: OxSubject = OxNamedNode::new_unchecked("http://example.org/x").into();
    let triples = graph.triples_matching(&x, &Any, &Any).unwrap();
    assert_eq!(triples.count(), 2)
}

#[test]
fn test_triples_matching_predicate() {
    let graph = graph_from_str(DUMMY_GRAPH);
    let p = OxNamedNode::new_unchecked("http://example.org/p");
    let triples = graph.triples_matching(&Any, &p, &Any).unwrap();
    assert_eq!(triples.count(), 2)
}

#[test]
fn test_triples_matching_object() {
    let graph = graph_from_str(DUMMY_GRAPH);
    let one: OxTerm = OxLiteral::from(1).into();
    let triples = graph.triples_matching(&Any, &Any, &one).unwrap();
    assert_eq!(triples.count(), 1)
}

#[test]
fn test_incoming_arcs() {
    let graph = graph_from_str(DUMMY_GRAPH);
    let x = OxSubject::NamedNode(OxNamedNode::new_unchecked("http://example.org/x"));
    let p = OxNamedNode::new_unchecked("http://example.org/p");
    let one: OxTerm = OxLiteral::from(1).into();
    let actual = graph.incoming_arcs(&one).unwrap();
    let expected = HashSet::from([x]);
    assert_eq!(actual.get(&p), Some(&expected))
}

#[test]
fn test_outgoing_arcs() {
    let graph = graph_from_str(DUMMY_GRAPH_1);

    let x = OxSubject::NamedNode(OxNamedNode::new_unchecked("http://example.org/x"));
    let p = OxNamedNode::new_unchecked("http://example.org/p");
    let one: OxTerm = OxLiteral::from(1).into();

    let subject: OxSubject = graph
        .triples_matching(&x, &p, &Any)
        .unwrap()
        .map(Triple::into_object)
        .next()
        .unwrap()
        .try_into()
        .unwrap();

    let actual = graph.outgoing_arcs(&subject).unwrap();
    let expected = HashSet::from([one]);

    assert_eq!(actual.get(&p), Some(&expected))
}

#[test]
fn test_add_triple() {
    let mut graph = InMemoryGraph::default();

    let alice = OxSubject::NamedNode(OxNamedNode::new_unchecked("http://example.org/alice"));
    let knows = OxNamedNode::new_unchecked("http://example.org/knows");
    let bob = OxTerm::NamedNode(OxNamedNode::new_unchecked("http://example.org/bob"));

    graph.add_triple(alice, knows, bob).unwrap();

    assert_eq!(graph.len(), 1);
}

#[test]
fn test_rdf_list() {
    let mut graph = graph_from_str(DUMMY_GRAPH_2);

    let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
    let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));

    let parser = SingleValuePropertyParser::new(p).then(move |obj| SetFocusParser::new(obj).with(ListParser::new()));
    let result: Vec<OxTerm> = parser.parse(&x, &mut graph).unwrap();

    assert_eq!(
        result,
        vec![
            OxTerm::from(OxLiteral::from(1)),
            OxTerm::from(OxLiteral::from(2))
        ]
    )
}

#[test]
fn test_parser_property_integers() {
    let mut graph = graph_from_str(DUMMY_GRAPH_3);
    let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
    let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));
    let parser = IntegersPropertyParser::new(p);
    let mut result = parser.parse(&x, &mut graph).unwrap();
    result.sort();

    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_parser_or() {
    let mut graph = graph_from_str(DUMMY_GRAPH_5);
    let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
    let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));
    let q = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/q"));
    let parser = SingleBoolPropertyParser::new(p).or(SingleBoolPropertyParser::new(q));
    assert!(parser.parse(&x, &mut graph).unwrap())
}

#[test]
fn test_parser_or_enum_1() {
    let mut graph = graph_from_str(DUMMY_GRAPH_6);
    let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
    let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));
    let parser_a_bool = SingleBoolPropertyParser::new(p.clone()).map(A::Bool);
    let parser_a_int = SingleIntegerPropertyParser::new(p).map(A::Int);
    let parser = parser_a_int.or(parser_a_bool);
    assert_eq!(parser.parse(&x, &mut graph).unwrap(), A::Int(1))
}

#[test]
fn test_parser_or_enum_2() {
    let mut graph = graph_from_str(DUMMY_GRAPH_7);
    let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
    let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));
    let parser_a_bool = SingleBoolPropertyParser::new(p.clone()).map(A::Bool);
    let parser_a_int = SingleIntegerPropertyParser::new(p).map(A::Int);
    let parser = parser_a_int.or(parser_a_bool);
    assert_eq!(parser.parse(&x, &mut graph).unwrap(), A::Bool(true))
}

#[test]
fn test_parser_and() {
    let mut graph = graph_from_str(DUMMY_GRAPH_8);
    let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
    let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));
    let q = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/q"));
    let parser = SingleBoolPropertyParser::new(p).and(SingleIntegerPropertyParser::new(q));
    assert_eq!(parser.parse(&x, &mut graph).unwrap(), (true, 1))
}

#[test]
fn test_parser_map() {
    let mut graph = graph_from_str(DUMMY_GRAPH_9);
    let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
    let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));
    let parser = SingleIntegerPropertyParser::new(p).map(|n| n + 1);
    assert_eq!(parser.parse(&x, &mut graph).unwrap(), 2)
}

#[test]
fn test_parser_and_then() {
    let mut graph = graph_from_str(DUMMY_GRAPH_10);
    let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
    let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));

    fn cnv_int(s: String) -> Result<isize, RDFError> {
        s.parse().map_err(|_| RDFError::DefaultError {
            msg: format!("Error converting {s}"),
        })
    }

    let parser = SingleStringPropertyParser::new(p).and_then(cnv_int);
    assert_eq!(parser.parse(&x, &mut graph).unwrap(), 1)
}

#[test]
fn test_parser_flat_map() {
    let mut graph = graph_from_str(DUMMY_GRAPH_10);
    let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
    let p = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/p"));

    fn cnv_int(s: String) -> Result<isize, RDFError> {
        s.parse().map_err(|_| RDFError::DefaultError {
            msg: format!("Error converting {s}"),
        })
    }

    let parser = SingleStringPropertyParser::new(p).flat_map(cnv_int);
    assert_eq!(parser.parse(&x,&mut graph).unwrap(), 1)
}

#[test]
fn test_rdf_parser_macro() {
    rdf_parser! {
          fn is_term['a, RDF](term: &'a RDF::Term)(RDF) -> ()
          where [
          ] {
            let name = format!("is_{term}");
            SatisfyParser::new(|t| { t == *term }, name.as_str())
          }
    }

    let mut graph = graph_from_str(DUMMY_GRAPH_9);
    let x = OxNamedNode::new_unchecked("http://example.org/x");
    let iri_s = IriS::from_named_node(&x);
    let term = x.clone().into();
    let parser = is_term(&term);
    let result = parser.parse(&iri_s, &mut graph);
    assert!(result.is_ok())
}

#[test]
fn test_not() {
    let mut graph = graph_from_str(DUMMY_GRAPH_9);
    let x = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/x"));
    let q = IriS::from_named_node(&OxNamedNode::new_unchecked("http://example.org/q"));
    assert!(SingleValuePropertyParser::new(q).not().parse(&x, &mut graph).is_ok())
}

#[test]
fn test_iri() {
    let mut graph = InMemoryGraph::default();
    let x = OxNamedNode::new_unchecked("http://example.org/x");
    let x_iri = IriS::from_named_node(&x);
    assert_eq!(IriParser::new().parse(&x_iri, &mut graph).unwrap(), x)
}

#[test]
fn test_add_triple_ref() {
    let mut graph = InMemoryGraph::default();
    let s = OxNamedNode::new_unchecked("http://example.org/x");
    let p = OxNamedNode::new_unchecked("http://example.org/p");
    let o = OxNamedNode::new_unchecked("http://example.org/y");
    graph.add_triple_ref(&s, &p, &o).unwrap();
    assert_eq!(graph.len(), 1);
}
