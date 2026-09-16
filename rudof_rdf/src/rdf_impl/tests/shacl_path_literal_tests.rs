//! Regression tests: literal terms are valid SHACL property-path endpoints.
//!
//! A literal cannot be an RDF *subject*, so it has no outgoing arcs and cannot
//! continue a path. It is still a perfectly valid *endpoint* — SHACL property
//! paths are defined over SPARQL property paths, whose results routinely
//! include literals.
//!
//! Before the fix, the recursing branches of `objects_for_shacl_path`
//! (`Sequence`, `ZeroOrMore`, `OneOrMore`) attempted to traverse *from* every
//! object they found, literals included. `objects_for` begins with
//! `term_as_subject(..)?`, which fails on a literal, and the `?` propagated out
//! of the whole call — so a single literal did not merely lose itself, it
//! discarded **every** value on the path, including unrelated valid branches.
//!
//! The W3C data-shapes suite does not cover this: `path-oneOrMore-001` and
//! `path-zeroOrMore-001` use only IRI values, which is why the bug survived.

use crate::rdf_core::{NeighsRDF, RDFFormat, SHACLPath};
use crate::rdf_impl::{OxigraphInMemory, ReaderMode};
use oxrdf::Literal as OxLiteral;
use oxrdf::NamedNode as OxNamedNode;
use oxrdf::Term as OxTerm;
use rudof_iri::IriS;

fn graph_from_str(s: &str) -> OxigraphInMemory {
    OxigraphInMemory::from_str(s, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap()
}

fn iri(s: &str) -> IriS {
    IriS::new_unchecked(&format!("http://example.org/{s}"))
}

fn node(s: &str) -> OxTerm {
    OxNamedNode::new_unchecked(format!("http://example.org/{s}")).into()
}

fn lit(s: &str) -> OxTerm {
    OxLiteral::new_simple_literal(s).into()
}

fn p() -> SHACLPath {
    SHACLPath::iri(iri("p"))
}

fn q() -> SHACLPath {
    SHACLPath::iri(iri("q"))
}

/// `:x :p "leaf" .` — `p+` must yield the literal.
#[test]
fn one_or_more_yields_literal_endpoint() {
    let graph = graph_from_str(
        r#"prefix : <http://example.org/>
        :x :p "leaf" .
    "#,
    );
    let got = graph
        .objects_for_shacl_path(&node("x"), &SHACLPath::one_or_more(p()))
        .unwrap();
    assert!(got.contains(&lit("leaf")), "p+ dropped the literal endpoint: {got:?}");
}

/// `p*` must yield the focus node and the literal.
#[test]
fn zero_or_more_yields_literal_endpoint() {
    let graph = graph_from_str(
        r#"prefix : <http://example.org/>
        :x :p "leaf" .
    "#,
    );
    let got = graph
        .objects_for_shacl_path(&node("x"), &SHACLPath::zero_or_more(p()))
        .unwrap();
    assert!(got.contains(&node("x")), "p* dropped the zero-length result: {got:?}");
    assert!(got.contains(&lit("leaf")), "p* dropped the literal endpoint: {got:?}");
}

/// The regression that matters: a literal on one branch must not discard the
/// IRI values reachable on another.
#[test]
fn one_or_more_literal_does_not_discard_other_branches() {
    let graph = graph_from_str(
        r#"prefix : <http://example.org/>
        :x :p "leaf" .
        :x :p :y .
        :y :p :z .
    "#,
    );
    let got = graph
        .objects_for_shacl_path(&node("x"), &SHACLPath::one_or_more(p()))
        .unwrap();
    assert!(got.contains(&lit("leaf")), "missing literal endpoint: {got:?}");
    assert!(
        got.contains(&node("y")),
        "missing :y — a literal aborted the walk: {got:?}"
    );
    assert!(
        got.contains(&node("z")),
        "missing transitive :z — a literal aborted the walk: {got:?}"
    );
    assert_eq!(got.len(), 3, "unexpected value set: {got:?}");
}

/// A literal at an intermediate position of a sequence cannot continue, but it
/// must not discard the values reached through the sibling branch.
#[test]
fn sequence_literal_intermediate_does_not_discard_other_branches() {
    let graph = graph_from_str(
        r#"prefix : <http://example.org/>
        :x :p "leaf" .
        :x :p :y .
        :y :q :z .
    "#,
    );
    let got = graph
        .objects_for_shacl_path(&node("x"), &SHACLPath::sequence(vec![p(), q()]))
        .unwrap();
    assert_eq!(got.len(), 1, "unexpected value set: {got:?}");
    assert!(
        got.contains(&node("z")),
        "(p q) lost :z because of a literal on :p: {got:?}"
    );
}

/// A literal in the FINAL position of a sequence is a legitimate endpoint.
#[test]
fn sequence_yields_literal_endpoint() {
    let graph = graph_from_str(
        r#"prefix : <http://example.org/>
        :x :p :y .
        :y :q "leaf" .
    "#,
    );
    let got = graph
        .objects_for_shacl_path(&node("x"), &SHACLPath::sequence(vec![p(), q()]))
        .unwrap();
    assert!(
        got.contains(&lit("leaf")),
        "(p q) dropped the literal endpoint: {got:?}"
    );
}

/// Control: `p?` never recursed from its objects, so it was already correct.
/// Pinned so a future refactor of the guard cannot regress it.
#[test]
fn zero_or_one_yields_literal_endpoint() {
    let graph = graph_from_str(
        r#"prefix : <http://example.org/>
        :x :p "leaf" .
    "#,
    );
    let got = graph
        .objects_for_shacl_path(&node("x"), &SHACLPath::zero_or_one(p()))
        .unwrap();
    assert!(got.contains(&lit("leaf")), "p? dropped the literal endpoint: {got:?}");
}

/// A cycle must still terminate once literals are involved.
#[test]
fn one_or_more_terminates_with_cycle_and_literal() {
    let graph = graph_from_str(
        r#"prefix : <http://example.org/>
        :x :p :y .
        :y :p :x .
        :y :p "leaf" .
    "#,
    );
    let got = graph
        .objects_for_shacl_path(&node("x"), &SHACLPath::one_or_more(p()))
        .unwrap();
    assert!(got.contains(&node("x")), "cycle should reach :x: {got:?}");
    assert!(got.contains(&node("y")), "missing :y: {got:?}");
    assert!(got.contains(&lit("leaf")), "missing literal endpoint: {got:?}");
}
