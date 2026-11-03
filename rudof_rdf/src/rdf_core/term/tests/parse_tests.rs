//! The term parser reads the RDF 1.2 Turtle syntax of an object, so what it accepts is
//! what a caller can write wherever rudof takes a term as text.

use crate::rdf_core::term::parse_term;
use oxrdf::{BaseDirection, BlankNode, Literal, NamedNode, Term, Triple, vocab::xsd};
use prefixmap::PrefixMap;
use rudof_iri::IriS;

fn prefixmap() -> PrefixMap {
    let mut pm = PrefixMap::new();
    pm.add_prefix("ex", IriS::new_unchecked("http://example.org/"));
    pm.add_prefix("", IriS::new_unchecked("http://example.org/"));
    pm.add_prefix("xsd", IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#"));
    pm
}

fn term(text: &str) -> Term {
    parse_term(text, None, &prefixmap()).unwrap_or_else(|e| panic!("{text} should parse: {e}"))
}

fn error(text: &str) -> String {
    parse_term(text, None, &prefixmap())
        .map(|term| panic!("{text} should not parse, it gave {term}"))
        .unwrap_err()
        .to_string()
}

fn iri(iri: &str) -> Term {
    NamedNode::new_unchecked(iri).into()
}

fn alice() -> Term {
    iri("http://example.org/alice")
}

#[test]
fn test_an_iri_is_written_bracketed_or_prefixed() {
    assert_eq!(term("<http://example.org/alice>"), alice());
    assert_eq!(term("ex:alice"), alice());
    assert_eq!(term(":alice"), alice());
    assert_eq!(term("  ex:alice  "), alice());
}

#[test]
fn test_a_blank_node_keeps_its_label() {
    assert_eq!(term("_:b1"), BlankNode::new_unchecked("b1").into());
}

#[test]
fn test_a_literal_keeps_its_language_and_direction() {
    assert_eq!(term(r#""Alice""#), Literal::new_simple_literal("Alice").into());
    assert_eq!(
        term(r#""Alice"@en"#),
        Literal::new_language_tagged_literal("Alice", "en").unwrap().into()
    );
    assert_eq!(
        term(r#""Shalom"@he--rtl"#),
        Literal::new_directional_language_tagged_literal("Shalom", "he", BaseDirection::Rtl)
            .unwrap()
            .into()
    );
}

#[test]
fn test_a_datatype_is_resolved_and_the_lexical_form_kept() {
    assert_eq!(
        term(r#""23"^^xsd:integer"#),
        Literal::new_typed_literal("23", xsd::INTEGER).into()
    );
    assert_eq!(term("23"), Literal::new_typed_literal("23", xsd::INTEGER).into());
    assert_eq!(term("true"), Literal::new_typed_literal("true", xsd::BOOLEAN).into());

    // The lexical form reaches the graph as it was written, not canonicalized.
    assert_eq!(
        term(r#""2024-01-01T00:00:00.000Z"^^xsd:dateTime"#),
        Literal::new_typed_literal("2024-01-01T00:00:00.000Z", xsd::DATE_TIME).into()
    );
}

#[test]
fn test_a_triple_term_is_a_term() {
    let inner = Triple::new(
        NamedNode::new_unchecked("http://example.org/alice"),
        NamedNode::new_unchecked("http://example.org/name"),
        Literal::new_simple_literal("Alice"),
    );
    assert_eq!(term(r#"<<( ex:alice ex:name "Alice" )>>"#), inner.clone().into());
    assert_eq!(term(r#"<<(ex:alice ex:name "Alice")>>"#), inner.clone().into());

    // `a` is the predicate rdf:type here too.
    assert_eq!(
        term("<<( ex:alice a ex:Person )>>"),
        Triple::new(
            NamedNode::new_unchecked("http://example.org/alice"),
            NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"),
            NamedNode::new_unchecked("http://example.org/Person"),
        )
        .into()
    );

    // A blank node is a subject of one, and another triple term is an object.
    assert_eq!(
        term(r#"<<( _:b1 ex:name "Alice" )>>"#),
        Triple::new(
            BlankNode::new_unchecked("b1"),
            NamedNode::new_unchecked("http://example.org/name"),
            Literal::new_simple_literal("Alice"),
        )
        .into()
    );
    assert_eq!(
        term(r#"<<( ex:bob ex:believes <<( ex:alice ex:name "Alice" )>> )>>"#),
        Triple::new(
            NamedNode::new_unchecked("http://example.org/bob"),
            NamedNode::new_unchecked("http://example.org/believes"),
            Term::from(inner),
        )
        .into()
    );
}

#[test]
fn test_a_relative_iri_is_resolved_against_the_base() {
    let base = IriS::new_unchecked("http://example.org/");

    assert_eq!(parse_term("<alice>", Some(&base), &prefixmap()).unwrap(), alice());
    assert_eq!(
        parse_term(r#"<<( <alice> <name> "Alice" )>>"#, Some(&base), &prefixmap()).unwrap(),
        Triple::new(
            NamedNode::new_unchecked("http://example.org/alice"),
            NamedNode::new_unchecked("http://example.org/name"),
            Literal::new_simple_literal("Alice"),
        )
        .into()
    );
}

#[test]
fn test_a_relative_iri_without_a_base_is_refused() {
    assert!(error("<alice>").contains("not valid RDF 1.2 Turtle syntax"));
}

#[test]
fn test_text_naming_more_than_one_term_is_refused() {
    // A reified triple names a reifier and the triple term it reifies, not one term.
    let err = error("<< ex:a ex:b ex:c >>");
    assert!(err.contains("does not name a single term"), "{err}");

    assert!(error("[ ex:p ex:o ]").contains("does not name a single term"));
    assert!(error("( ex:a ex:b )").contains("does not name a single term"));

    // The empty collection is the single IRI rdf:nil.
    assert_eq!(term("()"), iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#nil"));
}

#[test]
fn test_text_that_is_not_a_term_is_refused() {
    for text in [
        r#""Alice" basura"#,
        "ex:alice basura",
        "",
        "undeclared:thing",
        r#"<<( "Alice" ex:name "Alice" )>>"#,
        "<<( ex:alice ex:name )>>",
        r#"<<( ex:alice ex:name "Alice" )>"#,
        r#""Alice"@he--sideways"#,
    ] {
        assert!(
            error(text).contains("not valid RDF 1.2 Turtle syntax"),
            "{text} gave {}",
            error(text)
        );
    }
}
