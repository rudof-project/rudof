use crate::{
    Result, Rudof, RudofConfig,
    formats::{InputSpec, IriNormalizationMode},
    types::{QueryResult, Triple},
};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::{Object, literal::ConcreteLiteral};
use std::{collections::HashSet, str::FromStr};

const GRAPH: &str = r#"
        prefix : <http://example.org/>
        prefix xsd: <http://www.w3.org/2001/XMLSchema#>
        :alice :name "Alice" ;
               :age 23 ;
               :knows :bob .
        :bob :name "Bob" .
    "#;

fn session() -> Rudof {
    let mut rudof = Rudof::new(RudofConfig::default());
    let data = InputSpec::from_str(GRAPH).unwrap();
    rudof.load_data().with_data(&[data]).execute().unwrap();
    rudof
}

fn iri(local: &str) -> IriS {
    IriS::new_unchecked(&format!("http://example.org/{local}"))
}

fn node(local: &str) -> Object {
    Object::iri(iri(local))
}

fn literal(value: &str) -> Object {
    Object::Literal(ConcreteLiteral::str(value))
}

/// The triples matching a pattern, sorted so the assertions do not depend on the
/// iteration order of the graph.
fn triples(rudof: &Rudof, subject: Option<&str>, predicate: Option<&str>, object: Option<&str>) -> Vec<Triple> {
    let mut builder = rudof.triples();
    if let Some(subject) = subject {
        builder = builder.with_subject(subject);
    }
    if let Some(predicate) = predicate {
        builder = builder.with_predicate(predicate);
    }
    if let Some(object) = object {
        builder = builder.with_object(object);
    }
    let mut triples: Vec<Triple> = builder.execute().unwrap().collect::<Result<_>>().unwrap();
    triples.sort_by_key(|triple| triple.to_string());
    triples
}

fn all(rudof: &Rudof) -> Vec<Triple> {
    triples(rudof, None, None, None)
}

#[test]
fn test_triples_patterns() {
    let rudof = session();

    // Every position unset walks the whole graph.
    assert_eq!(all(&rudof).len(), 4);

    // And each position narrows it, in every combination.
    assert_eq!(triples(&rudof, Some(":alice"), None, None).len(), 3);
    assert_eq!(triples(&rudof, None, Some(":name"), None).len(), 2);
    assert_eq!(triples(&rudof, None, None, Some("\"Alice\"")).len(), 1);
    assert_eq!(triples(&rudof, Some(":alice"), Some(":name"), None).len(), 1);
    assert_eq!(triples(&rudof, Some(":alice"), None, Some("\"Alice\"")).len(), 1);
    assert_eq!(triples(&rudof, None, Some(":name"), Some("\"Alice\"")).len(), 1);
    assert_eq!(
        triples(&rudof, Some(":alice"), Some(":name"), Some("\"Alice\"")).len(),
        1
    );

    // A pattern nothing matches is empty, not an error.
    assert!(triples(&rudof, Some(":carol"), None, None).is_empty());
    assert!(triples(&rudof, Some(":alice"), Some(":name"), Some("\"Bob\"")).is_empty());
}

#[test]
fn test_triples_are_the_terms_of_the_data() {
    let rudof = session();
    let triples = triples(&rudof, Some(":alice"), Some(":knows"), None);
    assert_eq!(
        triples,
        vec![Triple {
            subject: node("alice"),
            predicate: iri("knows"),
            object: node("bob"),
        }]
    );
    assert_eq!(
        triples[0].to_string(),
        "http://example.org/alice <http://example.org/knows> http://example.org/bob ."
    );
}

#[test]
fn test_add_triple() {
    let mut rudof = session();
    rudof.add_triple(":carol", ":name", "\"Carol\"").execute().unwrap();

    assert_eq!(all(&rudof).len(), 5);
    assert_eq!(
        triples(&rudof, Some(":carol"), None, None),
        vec![Triple {
            subject: node("carol"),
            predicate: iri("name"),
            object: literal("Carol"),
        }]
    );

    // A graph is a set of triples, so adding one it already has changes nothing.
    rudof.add_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
    assert_eq!(all(&rudof).len(), 5);
}

#[test]
fn test_add_triple_every_kind_of_term() {
    let mut rudof = session();
    rudof
        .add_triple("<http://example.org/carol>", "<http://example.org/name>", "\"Carol\"")
        .execute()
        .unwrap();
    rudof.add_triple("_:b1", ":name", "\"Blank\"").execute().unwrap();
    rudof.add_triple(":carol", ":age", "30").execute().unwrap();
    rudof
        .add_triple(":carol", ":height", "\"1.7\"^^xsd:decimal")
        .execute()
        .unwrap();
    rudof.add_triple(":carol", ":label", "\"Carol\"@en").execute().unwrap();
    rudof.add_triple(":carol", ":knows", ":alice").execute().unwrap();

    assert_eq!(all(&rudof).len(), 10);
    // The typed literal keeps its datatype, rather than degrading to a string.
    let height = &triples(&rudof, Some(":carol"), Some(":height"), None)[0].object;
    assert_ne!(height, &literal("1.7"), "{height} lost its datatype");
    assert!(height.to_string().contains("1.7"), "{height}");
    // And a blank node is a subject like any other.
    assert_eq!(triples(&rudof, Some("_:b1"), None, None).len(), 1);
}

#[test]
fn test_remove_triple() {
    let mut rudof = session();
    rudof.remove_triple(":alice", ":name", "\"Alice\"").execute().unwrap();

    assert_eq!(all(&rudof).len(), 3);
    assert!(triples(&rudof, Some(":alice"), Some(":name"), None).is_empty());
    // The other subject's triple with the same predicate is untouched.
    assert_eq!(triples(&rudof, Some(":bob"), Some(":name"), None).len(), 1);

    // Removing a triple that is not there changes nothing.
    rudof.remove_triple(":alice", ":name", "\"Alice\"").execute().unwrap();
    rudof.remove_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
    assert_eq!(all(&rudof).len(), 3);
}

#[test]
fn test_add_and_remove_triples_in_batches() {
    let mut rudof = session();

    rudof
        .add_triples(&[
            (":carol", ":name", "\"Carol\""),
            (":carol", ":knows", ":alice"),
            // Already in the data: a no-op, as for the single-triple operation.
            (":bob", ":name", "\"Bob\""),
        ])
        .execute()
        .unwrap();
    assert_eq!(all(&rudof).len(), 6);

    rudof
        .remove_triples(&[
            (":carol", ":name", "\"Carol\""),
            (":alice", ":age", "23"),
            // Not in the data: a no-op.
            (":dave", ":name", "\"Dave\""),
        ])
        .execute()
        .unwrap();
    assert_eq!(all(&rudof).len(), 4);
    assert!(triples(&rudof, Some(":carol"), Some(":name"), None).is_empty());
    assert_eq!(triples(&rudof, Some(":carol"), Some(":knows"), None).len(), 1);
    assert!(triples(&rudof, Some(":alice"), Some(":age"), None).is_empty());

    // An empty batch is allowed and changes nothing.
    rudof.add_triples(&[]).execute().unwrap();
    rudof.remove_triples(&[]).execute().unwrap();
    assert_eq!(all(&rudof).len(), 4);
}

#[test]
fn test_a_batch_with_an_unparsable_triple_changes_nothing() {
    let mut rudof = session();

    // The bad triple is in the middle of the batch, and the ones before it are not
    // applied either: every triple is parsed before any of them is written.
    let err = rudof
        .add_triples(&[
            (":carol", ":name", "\"Carol\""),
            ("\"Carol\"", ":name", "\"Carol\""),
            (":dave", ":name", "\"Dave\""),
        ])
        .execute()
        .unwrap_err();
    assert!(err.to_string().contains("cannot be the subject"), "{err}");
    assert_eq!(all(&rudof).len(), 4);

    let err = rudof
        .remove_triples(&[(":alice", ":name", "\"Alice\""), (":bob", "\"name\"", "\"Bob\"")])
        .execute()
        .unwrap_err();
    assert!(err.to_string().contains("Iri ref"), "{err}");
    assert_eq!(all(&rudof).len(), 4);
}

#[test]
fn test_add_and_remove_a_triple_with_the_session_prefixes() {
    let mut rudof = Rudof::new(RudofConfig::default());
    rudof.add_prefix("ex", "http://example.org/").execute().unwrap();
    let data = InputSpec::from_str(GRAPH).unwrap();
    rudof.load_data().with_data(&[data]).execute().unwrap();

    // `ex:` is declared by the session, not by the data, which uses `:`.
    rudof.add_triple("ex:carol", "ex:name", "\"Carol\"").execute().unwrap();
    assert_eq!(triples(&rudof, Some(":carol"), None, None).len(), 1);
    rudof
        .remove_triple("ex:carol", "ex:name", "\"Carol\"")
        .execute()
        .unwrap();
    assert!(triples(&rudof, Some(":carol"), None, None).is_empty());
}

/// A session whose configured base IRI is the one the data uses, so a relative IRI
/// written in a term denotes the same node as the data's `:` prefix.
fn session_with_base() -> Rudof {
    let config = RudofConfig::from_str("base_iri = \"http://example.org/\"\n").unwrap();
    let mut rudof = Rudof::new(config);
    let data = InputSpec::from_str(GRAPH).unwrap();
    rudof.load_data().with_data(&[data]).execute().unwrap();
    rudof
}

#[test]
fn test_a_relative_iri_is_resolved_against_the_base() {
    let mut rudof = session_with_base();

    // Every position of the triple: the relative IRIs used to reach the graph as they
    // were written, and serializing wrote out something that is not valid RDF.
    rudof.add_triple("<carol>", "<name>", "\"Carol\"").execute().unwrap();
    rudof.add_triple("<carol>", "<knows>", "<alice>").execute().unwrap();

    assert_eq!(
        triples(&rudof, Some("<carol>"), None, None),
        vec![
            Triple {
                subject: node("carol"),
                predicate: iri("knows"),
                object: node("alice"),
            },
            Triple {
                subject: node("carol"),
                predicate: iri("name"),
                object: literal("Carol"),
            },
        ]
    );

    // So the resolved term denotes the node the data's prefix does, and the data that
    // comes out carries absolute IRIs only.
    assert_eq!(
        triples(&rudof, Some(":carol"), None, None),
        triples(&rudof, Some("<carol>"), None, None)
    );
    let mut turtle = Vec::new();
    rudof.serialize_data(&mut turtle).execute().unwrap();
    let turtle = String::from_utf8(turtle).unwrap();
    assert!(turtle.contains(":carol"), "{turtle}");
    assert!(!turtle.contains("<carol>"), "{turtle}");

    // And a removal written the same way removes what was added.
    rudof.remove_triple("<carol>", "<name>", "\"Carol\"").execute().unwrap();
    assert_eq!(triples(&rudof, Some(":carol"), Some(":name"), None).len(), 0);
}

#[test]
fn test_a_relative_datatype_is_resolved_against_the_base() {
    let mut rudof = session_with_base();
    rudof
        .add_triple("<carol>", "<age>", "\"23\"^^<integer>")
        .execute()
        .unwrap();

    assert_eq!(
        triples(&rudof, Some("<carol>"), None, None),
        triples(
            &rudof,
            Some(":carol"),
            None,
            Some("\"23\"^^<http://example.org/integer>")
        )
    );
}

#[test]
fn test_triple_operations_need_rdf_data() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let err = rudof.add_triple(":a", ":p", ":b").execute().unwrap_err();
    assert!(err.to_string().contains("No RDF data loaded"), "{err}");
    let err = rudof.remove_triple(":a", ":p", ":b").execute().unwrap_err();
    assert!(err.to_string().contains("No RDF data loaded"), "{err}");
    let err = rudof.triples().execute().unwrap_err();
    assert!(err.to_string().contains("No RDF data loaded"), "{err}");
}

#[test]
fn test_triple_operations_reject_terms_that_are_not_terms() {
    let mut rudof = session();

    // A literal cannot be a subject.
    let err = rudof.add_triple("\"Alice\"", ":p", ":b").execute().unwrap_err();
    assert!(err.to_string().contains("cannot be the subject"), "{err}");

    // A predicate must be an IRI.
    let err = rudof.add_triple(":a", "\"name\"", ":b").execute().unwrap_err();
    assert!(err.to_string().contains("Iri ref"), "{err}");
    // A bare IRI is not accepted as a predicate even in the lax mode, which relaxes
    // the subject and the object only: it is read as a prefixed name, whose local part
    // cannot hold the rest of the IRI.
    let err = rudof
        .add_triple(":a", "http://example.org/name", ":b")
        .execute()
        .unwrap_err();
    assert!(err.to_string().contains("Unexpected trailing input"), "{err}");

    // An undeclared prefix is an error rather than an unresolved term.
    let err = rudof.add_triple("other:a", ":p", ":b").execute().unwrap_err();
    assert!(err.to_string().contains("other"), "{err}");

    // A bare IRI is accepted in the default lax mode, which wraps it in angle
    // brackets. In the strict one it is read as a prefixed name and rejected.
    rudof
        .add_triple("http://example.org/carol", ":name", "\"Carol\"")
        .execute()
        .unwrap();
    let err = rudof
        .add_triple("http://example.org/dave", ":name", "\"Dave\"")
        .with_iri_mode(IriNormalizationMode::Strict)
        .execute()
        .unwrap_err();
    assert!(err.to_string().contains("Unexpected trailing input"), "{err}");

    // A selector that picks nodes out of the data is not a term.
    let err = rudof
        .triples()
        .with_subject("{FOCUS :knows :bob}")
        .execute()
        .unwrap_err();
    assert!(err.to_string().contains("is not a term"), "{err}");
}

#[test]
fn test_triple_operations_reject_terms_with_trailing_input() {
    let mut rudof = session();

    let err = rudof
        .add_triple(":carol", ":greeting", "\"Hola\"@es--rtl")
        .execute()
        .unwrap_err();
    assert!(err.to_string().contains("Unexpected trailing input: --rtl"), "{err}");
    let err = rudof
        .add_triple(":carol", ":name", "\"Carol\" and trailing garbage")
        .execute()
        .unwrap_err();
    assert!(err.to_string().contains("Unexpected trailing input"), "{err}");

    let err = rudof
        .add_triple(":carol basura", ":name", "\"Carol\"")
        .execute()
        .unwrap_err();
    assert!(err.to_string().contains("Unexpected trailing input: basura"), "{err}");
    let err = rudof
        .add_triple(":carol", ":name basura", "\"Carol\"")
        .execute()
        .unwrap_err();
    assert!(err.to_string().contains("Unexpected trailing input: basura"), "{err}");

    let err = rudof
        .remove_triple(":alice", ":name", "\"Alice\"@es--rtl")
        .execute()
        .unwrap_err();
    assert!(err.to_string().contains("Unexpected trailing input: --rtl"), "{err}");

    let err = rudof.triples().with_object("\"Alice\" basura").execute().unwrap_err();
    assert!(err.to_string().contains("Unexpected trailing input: basura"), "{err}");

    // Nothing of this changed the data.
    assert_eq!(all(&rudof).len(), 4);
}

#[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
#[test]
fn test_triple_updates_are_refused_on_endpoint_data() {
    let mut rudof = Rudof::new(RudofConfig::default());
    // No request is made: the update is refused before any query would be sent to
    // the endpoint. The local in-memory graph of such a session is deliberately not
    // written to instead, since `triples` reads it and the endpoint's as one.
    rudof
        .load_data()
        .with_endpoint("http://example.org/sparql")
        .execute()
        .unwrap();

    for err in [
        rudof
            .add_triple("<http://example.org/a>", "<http://example.org/p>", "\"x\"")
            .execute()
            .unwrap_err(),
        rudof
            .remove_triple("<http://example.org/a>", "<http://example.org/p>", "\"x\"")
            .execute()
            .unwrap_err(),
    ] {
        assert!(err.to_string().contains("cannot be modified"), "{err}");
        assert!(err.to_string().contains("example.org/sparql"), "{err}");
    }
}

/// How many `:name` values a SPARQL query finds in the loaded data.
fn names_found(rudof: &mut Rudof) -> usize {
    let query = InputSpec::from_str("SELECT ?name WHERE { ?s <http://example.org/name> ?name }").unwrap();
    rudof.load_sparql_query(&query).execute().unwrap();
    rudof.run_query().execute().unwrap();
    match rudof.query_results() {
        Some(QueryResult::Select(solutions)) => solutions.iter().count(),
        other => panic!("expected SELECT solutions, got {other:?}"),
    }
}

#[test]
fn test_added_triples_reach_serialization_and_queries() {
    let mut rudof = session();
    rudof.add_triple(":carol", ":name", "\"Carol\"").execute().unwrap();

    let mut turtle = Vec::new();
    rudof.serialize_data(&mut turtle).execute().unwrap();
    let turtle = String::from_utf8(turtle).unwrap();
    assert!(turtle.contains("Carol"), "{turtle}");

    // A query runs against a store built from the graph, which the addition has to
    // invalidate, or the new triple would be invisible to SPARQL.
    assert_eq!(names_found(&mut rudof), 3);

    rudof.remove_triple(":carol", ":name", "\"Carol\"").execute().unwrap();
    assert_eq!(names_found(&mut rudof), 2);
}

/// A base direction is a component of the literal, so a directional language-tagged
/// string and the same string with only the language tag are two terms, and the triples
/// holding them two triples.
#[test]
fn test_a_base_direction_tells_two_triples_apart() {
    let mut rudof = Rudof::new(RudofConfig::default());
    let graph = InputSpec::from_str(
        r#"
            prefix : <http://example.org/>
            :alice :greeting "Shalom"@he--rtl , "Shalom"@he .
        "#,
    )
    .unwrap();
    rudof.load_data().with_data(&[graph]).execute().unwrap();

    let triples = triples(&rudof, Some(":alice"), Some(":greeting"), None);
    assert_eq!(triples.len(), 2, "{triples:?}");
    assert_ne!(triples[0], triples[1]);
    assert_eq!(triples.iter().collect::<HashSet<_>>().len(), 2);

    let rendered: Vec<String> = triples.iter().map(|triple| triple.object.to_string()).collect();
    assert!(rendered.contains(&"\"Shalom\"@he--rtl".to_string()), "{rendered:?}");
    assert!(rendered.contains(&"\"Shalom\"@he".to_string()), "{rendered:?}");
}
