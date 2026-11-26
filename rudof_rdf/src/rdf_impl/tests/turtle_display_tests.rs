//! Tests of the Turtle rendering of terms and triples with prefixed names.

use crate::rdf_impl::{show_term, show_triple, used_prefixes};
use oxrdf::{BlankNode, Literal, NamedNode, Term, TermRef, Triple, vocab::xsd};
use prefixmap::PrefixMap;
use std::collections::HashMap;

const EX: &str = "http://example.org/";

fn pm() -> PrefixMap {
    HashMap::from([
        ("", EX),
        ("xsd", "http://www.w3.org/2001/XMLSchema#"),
        ("schema", "http://schema.org/"),
    ])
    .try_into()
    .unwrap()
}

fn iri(local: &str) -> NamedNode {
    NamedNode::new_unchecked(format!("{EX}{local}"))
}

fn show(term: impl Into<Term>) -> String {
    show_term(term.into().as_ref(), &pm())
}

#[test]
fn iris_use_prefixes_when_they_match() {
    assert_eq!(show(iri("alice")), ":alice");
    assert_eq!(show(NamedNode::new_unchecked("http://schema.org/name")), "schema:name");
}

#[test]
fn other_iris_are_written_in_full() {
    assert_eq!(
        show(NamedNode::new_unchecked("http://other.org/x")),
        "<http://other.org/x>"
    );
    // `/` cannot start a local name: not `:/x`
    assert_eq!(
        show(NamedNode::new_unchecked("http://example.org//x")),
        "<http://example.org//x>"
    );
}

#[test]
fn blank_nodes_keep_their_labels() {
    assert_eq!(show(BlankNode::new_unchecked("b1")), "_:b1");
}

#[test]
fn simple_and_language_strings() {
    assert_eq!(show(Literal::new_simple_literal("Bob")), "\"Bob\"");
    assert_eq!(
        show(Literal::new_simple_literal("say \"hi\"\n")),
        "\"say \\\"hi\\\"\\n\""
    );
    assert_eq!(
        show(Literal::new_language_tagged_literal_unchecked("chat", "fr")),
        "\"chat\"@fr"
    );
}

#[test]
fn numbers_and_booleans_use_the_short_form() {
    assert_eq!(show(Literal::new_typed_literal("30", xsd::INTEGER)), "30");
    assert_eq!(show(Literal::new_typed_literal("-7", xsd::INTEGER)), "-7");
    assert_eq!(show(Literal::new_typed_literal("1.5", xsd::DECIMAL)), "1.5");
    assert_eq!(show(Literal::new_typed_literal("true", xsd::BOOLEAN)), "true");
}

#[test]
fn other_typed_literals_use_a_prefixed_datatype() {
    assert_eq!(
        show(Literal::new_typed_literal("2024-01-01", xsd::DATE)),
        "\"2024-01-01\"^^xsd:date"
    );
    // The short form keeps the lexical form: Turtle reads `01` back as "01"^^xsd:integer
    assert_eq!(show(Literal::new_typed_literal("01", xsd::INTEGER)), "01");
    assert_eq!(
        show(Literal::new_typed_literal("TRUE", xsd::BOOLEAN)),
        "\"TRUE\"^^xsd:boolean"
    );
    assert_eq!(
        show(Literal::new_typed_literal(
            "x",
            NamedNode::new_unchecked("http://other.org/dt")
        )),
        "\"x\"^^<http://other.org/dt>"
    );
}

#[test]
fn integers_without_a_xsd_prefix_still_use_the_short_form() {
    let pm: PrefixMap = HashMap::from([("", EX)]).try_into().unwrap();
    let t = Literal::new_typed_literal("3", xsd::INTEGER);
    assert_eq!(show_term(Term::from(t).as_ref(), &pm), "3");
}

#[test]
fn triples_and_triple_terms() {
    let inner = Triple::new(iri("s"), iri("p"), BlankNode::new_unchecked("x"));
    let t = Triple::new(iri("a"), iri("says"), inner);
    assert_eq!(show_triple(&t, &pm()), ":a :says <<( :s :p _:x )>>");
}

#[test]
fn used_prefixes_are_the_ones_in_the_text() {
    let triples = [
        Triple::new(
            iri("a"),
            NamedNode::new_unchecked("http://schema.org/name"),
            Literal::new_simple_literal("A"),
        ),
        Triple::new(iri("a"), iri("age"), Literal::new_typed_literal("3", xsd::INTEGER)),
        Triple::new(
            iri("a"),
            iri("born"),
            Literal::new_typed_literal("2024-01-01", xsd::DATE),
        ),
    ];
    let used = used_prefixes(triples.iter().map(TermRef::Triple), &pm());
    let mut aliases: Vec<_> = used.iter().map(|(a, i)| (a.clone(), i.to_string())).collect();
    aliases.sort();
    assert_eq!(
        aliases,
        vec![
            ("".to_string(), EX.to_string()),
            ("schema".to_string(), "http://schema.org/".to_string()),
            ("xsd".to_string(), "http://www.w3.org/2001/XMLSchema#".to_string()),
        ]
    );
    // The short form of 3 does not use xsd:
    let used = used_prefixes(triples[1..2].iter().map(TermRef::Triple), &pm());
    assert_eq!(
        used.iter().map(|(a, _)| a.clone()).collect::<Vec<_>>(),
        vec!["".to_string()]
    );
}
