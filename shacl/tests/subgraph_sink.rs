//! The valid and invalid subgraphs of a SHACL validation.

use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use rudof_rdf_sink::{GraphSink, TriplesSink};
use shacl::ir::IRSchema;
use shacl::rdf::ShaclParser;
use shacl::validator::processor::{GraphValidation, ShaclProcessor};
use shacl::validator::store::Graph;
use shacl::validator::{ShaclConfig, ShaclValidationMode};
use sparql_service::RdfData;
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

const PREFIXES: &str = "prefix : <http://example.org/>\nprefix sh: <http://www.w3.org/ns/shacl#>\nprefix xsd: <http://www.w3.org/2001/XMLSchema#>\n";

const DATA: &str = r#"
    :alice a :Person ; :name "Alice" ; :knows :bob .
    :bob   a :Person ; :name "Bob" ; :knows :carol .
    :carol a :Person ; :name 23 .
    :dave  a :Person .
"#;

fn rdf(text: &str) -> RdfData {
    RdfData::from_str(
        &format!("{PREFIXES}{text}"),
        &RDFFormat::Turtle,
        None,
        &ReaderMode::Strict,
    )
    .unwrap()
}

fn shapes(shapes: &str) -> IRSchema {
    ShaclParser::new(rdf(shapes)).parse().unwrap().try_into().unwrap()
}

type Sink = Arc<Mutex<GraphSink>>;

fn new_sink() -> Sink {
    Arc::new(Mutex::new(GraphSink::new(OxigraphInMemory::new())))
}

/// The distinct triples the sink kept, in N-Triples, with `:` for the example namespace.
fn kept(sink: &Sink) -> BTreeSet<String> {
    let sink = sink.lock().unwrap();
    sink.graph()
        .triples()
        .unwrap()
        .map(|t| {
            t.to_string()
                .replace("http://example.org/", ":")
                .replace("http://www.w3.org/1999/02/22-rdf-syntax-ns#type", "a")
        })
        .collect()
}

fn set(triples: &[&str]) -> BTreeSet<String> {
    triples.iter().map(|t| t.to_string()).collect()
}

/// Validates `DATA` against `schema`, returning whether it conforms and the
/// triples read, the valid ones and the invalid ones.
fn validate(schema: &str) -> (bool, BTreeSet<String>, BTreeSet<String>, BTreeSet<String>) {
    let (read, valid, invalid) = (new_sink(), new_sink(), new_sink());
    let config = ShaclConfig::default()
        .with_read_sink(TriplesSink::new(read.clone()))
        .with_valid_sink(TriplesSink::new(valid.clone()))
        .with_invalid_sink(TriplesSink::new(invalid.clone()));
    let mut validation: GraphValidation = Graph::from(rdf(DATA)).into();
    let report = validation
        .validate(&shapes(schema), &ShaclValidationMode::Native, &config)
        .unwrap();
    (report.conforms(), kept(&read), kept(&valid), kept(&invalid))
}

#[test]
fn splits_the_triples_read_by_the_conformance_of_each_focus_node() {
    let (conforms, read, valid, invalid) = validate(
        ":S sh:targetClass :Person ; sh:property [ sh:path :name ; sh:minCount 1 ; sh:datatype xsd:string ] .",
    );

    assert!(!conforms);
    assert_eq!(
        valid,
        set(&[
            "<:alice> <a> <:Person>",
            "<:alice> <:name> \"Alice\"",
            "<:bob> <a> <:Person>",
            "<:bob> <:name> \"Bob\"",
        ])
    );
    assert_eq!(
        invalid,
        set(&[
            "<:carol> <a> <:Person>",
            "<:carol> <:name> \"23\"^^<http://www.w3.org/2001/XMLSchema#integer>",
            "<:dave> <a> <:Person>",
        ])
    );
    assert_eq!(read, valid.union(&invalid).cloned().collect());
}

#[test]
fn a_conformant_node_brings_the_nodes_it_relies_on() {
    let (conforms, _, valid, invalid) = validate(
        r#"
        :S sh:targetNode :alice ;
           sh:property [ sh:path :knows ; sh:node :Named ] .
        :Named sh:property [ sh:path :name ; sh:minCount 1 ] .
        "#,
    );

    assert!(conforms);
    assert_eq!(valid, set(&["<:alice> <:knows> <:bob>", "<:bob> <:name> \"Bob\""]));
    assert!(invalid.is_empty(), "{invalid:?}");
}

#[test]
fn a_non_conformant_node_puts_what_it_read_in_the_invalid_subgraph() {
    let (conforms, _, valid, invalid) = validate(
        r#"
        :S sh:targetNode :bob ;
           sh:property [ sh:path :knows ; sh:node :Named ] .
        :Named sh:property [ sh:path :name ; sh:datatype xsd:string ] .
        "#,
    );

    assert!(!conforms);
    assert!(valid.is_empty(), "{valid:?}");
    assert_eq!(
        invalid,
        set(&[
            "<:bob> <:knows> <:carol>",
            "<:carol> <:name> \"23\"^^<http://www.w3.org/2001/XMLSchema#integer>",
        ])
    );
}

#[test]
fn a_triple_read_by_a_conformant_node_is_valid_even_if_a_failing_one_read_it() {
    let (_, _, valid, invalid) = validate(
        r#"
        :Knows sh:targetNode :alice, :carol ;
           sh:property [ sh:path :knows ; sh:minCount 1 ] .
        :Named sh:targetNode :alice, :bob ;
           sh:property [ sh:path :name ; sh:datatype xsd:integer ] .
        "#,
    );

    // :alice conforms to :Knows but not to :Named, so the triple about her
    // name is invalid, while the one about whom she knows is valid
    assert!(valid.contains("<:alice> <:knows> <:bob>"), "{valid:?}");
    assert!(invalid.contains("<:alice> <:name> \"Alice\""), "{invalid:?}");
    assert!(invalid.contains("<:bob> <:name> \"Bob\""), "{invalid:?}");
    assert!(valid.is_disjoint(&invalid));
}

#[test]
fn subgraph_sinks_need_the_native_engine() {
    let config = ShaclConfig::default().with_valid_sink(TriplesSink::new(new_sink()));
    let mut validation: GraphValidation = Graph::from(rdf(DATA)).into();

    let error = validation
        .validate(
            &shapes(":S sh:targetNode :alice ."),
            &ShaclValidationMode::Sparql,
            &config,
        )
        .unwrap_err();

    assert!(error.to_string().contains("native engine"), "{error}");
}

#[test]
fn the_report_is_the_same_with_and_without_subgraph_sinks() {
    let schema =
        shapes(":S sh:targetClass :Person ; sh:property [ sh:path :name ; sh:minCount 1 ; sh:datatype xsd:string ] .");
    let validate = |config: &ShaclConfig| {
        let mut validation: GraphValidation = Graph::from(rdf(DATA)).into();
        let report = validation
            .validate(&schema, &ShaclValidationMode::Native, config)
            .unwrap();
        let mut results: Vec<String> = report.results().iter().map(|r| format!("{r:?}")).collect();
        results.sort();
        (report.conforms(), results)
    };
    let tracked = ShaclConfig::default().with_invalid_sink(TriplesSink::new(new_sink()));

    assert_eq!(validate(&ShaclConfig::default()), validate(&tracked));
}
