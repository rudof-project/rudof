use crate::{
    rdf_core::{AsyncRDF, NeighsRDF, term::Triple, query::QueryResultFormat, Rdf, Any},
    rdf_impl::SparqlEndpoint
};
use oxrdf::{NamedNode, NamedOrBlankNode as Subject, Term};

#[test]
fn check_sparql() {
    let wikidata = SparqlEndpoint::wikidata().unwrap();

    let q80: Subject = NamedNode::new_unchecked("http://www.wikidata.org/entity/Q80").into();
    let p19: NamedNode = NamedNode::new_unchecked("http://www.wikidata.org/prop/P19");

    let data: Vec<_> = wikidata
        .triples_with_subject(&q80)
        .unwrap()
        .map(Triple::into_predicate)
        .collect();

    assert!(data.contains(&p19));
}

#[test]
fn test_endpoint_creation() {
    let iri = iri_s::IriS::new_unchecked("https://example.org/sparql");
    let prefixmap = prefixmap::PrefixMap::new();
    let endpoint = SparqlEndpoint::new(&iri, &prefixmap).unwrap();
    assert_eq!(endpoint.iri().as_str(), "https://example.org/sparql");
}

#[test]
fn test_wikidata_endpoint() {
    let wikidata = SparqlEndpoint::wikidata().unwrap();
    assert!(wikidata.iri().as_str().contains("wikidata"));
}

#[test]
fn test_with_prefixmap() {
    let mut prefixmap = prefixmap::PrefixMap::new();
    let iri = iri_s::IriS::new_unchecked("https://example.org/");
    prefixmap.add_prefix("", iri.clone()).unwrap();
    let endpoint = SparqlEndpoint::wikidata().unwrap().with_prefixmap(prefixmap);
    // Check that the prefixmap was set by trying to resolve
    let resolved = endpoint.resolve_prefix_local("", "test").unwrap();
    assert!(resolved.as_str().contains("example.org"));
}

#[test]
fn test_qualify_iri() {
    let endpoint = SparqlEndpoint::wikidata().unwrap();
    let iri = NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
    let qualified = endpoint.qualify_iri(&iri);
    assert!(qualified.contains("rdf"));
    assert!(qualified.contains("type"));
}

#[test]
fn test_qualify_subject() {
    let endpoint = SparqlEndpoint::wikidata().unwrap();
    let subject: Subject = NamedNode::new_unchecked("http://www.wikidata.org/entity/Q42").into();
    let qualified = endpoint.qualify_subject(&subject);
    assert!(qualified.contains("Q42"));
}

#[test]
fn test_qualify_term() {
    let endpoint = SparqlEndpoint::wikidata().unwrap();
    let term: Term = NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").into();
    let qualified = endpoint.qualify_term(&term);
    assert!(qualified.contains("rdf"));
    assert!(qualified.contains("type"));
}

#[tokio::test]
async fn test_query_ask_async() {
    let endpoint = SparqlEndpoint::wikidata().unwrap();

    // Test ASK query that should return true
    let query = "ASK { ?s ?p ?o }";
    let result = endpoint.query_ask_async(query).await.unwrap();
    assert!(result);

    // Test ASK query that should return false
    let query = "ASK { ?s <http://example.org/nonexistent> ?o }";
    let result = endpoint.query_ask_async(query).await.unwrap();
    assert!(!result);
}

#[tokio::test]
async fn test_query_construct_async() {
    let endpoint = SparqlEndpoint::wikidata().unwrap();

    let query = "CONSTRUCT { ?s ?p ?o } WHERE { ?s ?p ?o } LIMIT 1";
    let result = endpoint.query_construct_async(query, &QueryResultFormat::Turtle).await.unwrap();
    assert!(!result.is_empty());
    assert!(result.contains("@prefix") || result.contains("<"));
}

#[tokio::test]
async fn test_query_select_async() {
    let endpoint = SparqlEndpoint::wikidata().unwrap();

    let query = "SELECT ?s WHERE { ?s ?p ?o } LIMIT 1";
    let solutions = endpoint.query_select_async(query).await.unwrap();
    assert!(solutions.count() > 0);
}

#[tokio::test]
async fn test_get_predicates_subject() {
    let endpoint = SparqlEndpoint::wikidata().unwrap();
    let subject: Subject = NamedNode::new_unchecked("http://www.wikidata.org/entity/Q42").into();

    let predicates = endpoint.get_predicates_subject(&subject).await.unwrap();
    assert!(!predicates.is_empty());
    // Should contain some common Wikidata predicates
    let has_predicate = predicates.iter().any(|p| p.as_str().contains("wikidata.org"));
    assert!(has_predicate);
}

#[tokio::test]
async fn test_get_objects_for_subject_predicate() {
    let endpoint = SparqlEndpoint::wikidata().unwrap();
    let subject: Subject = NamedNode::new_unchecked("http://www.wikidata.org/entity/Q42").into();
    let predicate = NamedNode::new_unchecked("http://www.wikidata.org/prop/P31");

    let objects = endpoint.get_objects_for_subject_predicate(&subject, &predicate).await.unwrap();
    assert!(!objects.is_empty());
}

#[tokio::test]
async fn test_get_subjects_for_object_predicate() {
    let endpoint = SparqlEndpoint::wikidata().unwrap();

    let object: Term = NamedNode::new_unchecked("http://www.wikidata.org/entity/Q3624078").into(); 
    let predicate = NamedNode::new_unchecked("http://www.wikidata.org/prop/direct/P31"); 

    let subjects = endpoint.get_subjects_for_object_predicate(&object, &predicate).await.unwrap();
    assert!(!subjects.is_empty());
}

#[test]
fn test_triples_matching() {
    let endpoint = SparqlEndpoint::wikidata().unwrap();
    let subject: Subject = NamedNode::new_unchecked("http://www.wikidata.org/entity/Q42").into();

    let triples: Vec<_> = endpoint.triples_matching(&subject, &Any, &Any).unwrap().take(5).collect();
    assert!(!triples.is_empty());
}

#[test]
fn test_show_literal() {
    let endpoint = SparqlEndpoint::wikidata().unwrap();
    let literal = oxrdf::Literal::new_simple_literal("test");
    let colored = endpoint.show_literal(&literal);
    assert!(colored.contains("test"));
    // Should be colored red
    assert!(colored.contains("\x1b[31m"));
}
