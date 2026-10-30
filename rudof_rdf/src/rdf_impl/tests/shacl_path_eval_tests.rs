//! `sh:inversePath` over any path, and sequence paths of any length.

use crate::rdf_core::{NeighsRDF, RDFFormat, SHACLPath};
use crate::rdf_impl::{OxigraphInMemory, ReaderMode};
use oxrdf::Literal as OxLiteral;
use oxrdf::NamedNode as OxNamedNode;
use oxrdf::Term as OxTerm;
use rudof_iri::IriS;
use std::collections::HashSet;

const GRAPH: &str = r#"prefix : <http://example.org/>
    :a :p :b .
    :b :p :c .
    :e :q :b .
    :c :q "v" .
"#;

fn values(graph: &str, focus: OxTerm, path: SHACLPath) -> HashSet<OxTerm> {
    let graph = OxigraphInMemory::from_str(graph, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();
    graph.objects_for_shacl_path(&focus, &path).unwrap()
}

fn node(s: &str) -> OxTerm {
    OxNamedNode::new_unchecked(format!("http://example.org/{s}")).into()
}

fn nodes<const N: usize>(names: [&str; N]) -> HashSet<OxTerm> {
    names.into_iter().map(node).collect()
}

fn p() -> SHACLPath {
    SHACLPath::iri(IriS::new_unchecked("http://example.org/p"))
}

fn q() -> SHACLPath {
    SHACLPath::iri(IriS::new_unchecked("http://example.org/q"))
}

fn inv(path: SHACLPath) -> SHACLPath {
    SHACLPath::inverse(path)
}

#[test]
fn inverse_of_inverse() {
    assert_eq!(values(GRAPH, node("a"), inv(inv(p()))), nodes(["b"]));
}

#[test]
fn inverse_of_alternative() {
    let path = inv(SHACLPath::alternative(vec![p(), q()]));
    assert_eq!(values(GRAPH, node("b"), path), nodes(["a", "e"]));
}

#[test]
fn inverse_of_sequence_reverses_it_and_starts_from_a_literal() {
    let path = inv(SHACLPath::sequence(vec![p(), q()]));
    let v: OxTerm = OxLiteral::new_simple_literal("v").into();
    assert_eq!(values(GRAPH, v, path), nodes(["b"]));
}

#[test]
fn inverse_of_sequence_through_a_literal() {
    let path = inv(SHACLPath::sequence(vec![q(), inv(q())]));
    assert_eq!(values(GRAPH, node("c"), path), nodes(["c"]));
}

#[test]
fn inverse_of_zero_or_more() {
    let path = inv(SHACLPath::zero_or_more(p()));
    assert_eq!(values(GRAPH, node("c"), path), nodes(["a", "b", "c"]));
}

#[test]
fn inverse_of_one_or_more() {
    let path = inv(SHACLPath::one_or_more(p()));
    assert_eq!(values(GRAPH, node("c"), path), nodes(["a", "b"]));
}

#[test]
fn inverse_of_zero_or_one() {
    let path = inv(SHACLPath::zero_or_one(p()));
    assert_eq!(values(GRAPH, node("c"), path), nodes(["b", "c"]));
}

#[test]
fn long_sequence() {
    let graph = "prefix : <http://example.org/> :a :p :a .";
    let path = SHACLPath::sequence(vec![p(); 10_000]);
    assert_eq!(values(graph, node("a"), path), nodes(["a"]));
}

#[test]
fn nested_repetitions_take_polynomial_time() {
    // Each level used to repeat the levels inside it from every node, 27x slower per level
    let graph: String = (0..100)
        .map(|i| {
            format!(
                "<http://example.org/n{i}> <http://example.org/p> <http://example.org/n{}> .\n",
                i + 1
            )
        })
        .collect();
    let mut nested = p();
    let mut alternating = p();
    for _ in 0..6 {
        nested = SHACLPath::zero_or_more(nested);
        alternating = SHACLPath::zero_or_more(inv(alternating));
    }

    assert_eq!(values(&graph, node("n0"), nested).len(), 101);
    assert_eq!(values(&graph, node("n0"), alternating).len(), 101);
}

#[test]
fn directly_nested_repetitions() {
    // Each is evaluated as a single repetition; this checks the values, as memory is not asserted
    let graph: String = (0..10)
        .map(|i| {
            format!(
                "<http://example.org/n{i}> <http://example.org/p> <http://example.org/n{}> .\n",
                i + 1
            )
        })
        .collect();
    for path in [
        SHACLPath::zero_or_more(SHACLPath::zero_or_more(p())),
        SHACLPath::zero_or_more(SHACLPath::one_or_more(p())),
        SHACLPath::one_or_more(SHACLPath::zero_or_more(p())),
        SHACLPath::zero_or_one(SHACLPath::zero_or_more(p())),
    ] {
        assert_eq!(values(&graph, node("n0"), path).len(), 11);
    }
    let plus_plus = SHACLPath::one_or_more(SHACLPath::one_or_more(p()));
    assert_eq!(values(&graph, node("n0"), plus_plus).len(), 10);
}
