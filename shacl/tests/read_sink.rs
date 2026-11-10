use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use rudof_rdf_sink::{GraphSink, RdfSink, TriplesSink};
use shacl::ir::IRSchema;
use shacl::rdf::ShaclParser;
use shacl::validator::processor::{CachedDataValidation, DataValidation, GraphValidation, ShaclProcessor};
use shacl::validator::store::Graph;
use shacl::validator::{FetchStrategy, ShaclConfig, ShaclValidationMode};
use sparql_service::RdfData;
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

const PREFIXES: &str = "prefix : <http://example.org/>\nprefix sh: <http://www.w3.org/ns/shacl#>\n";

const DATA: &str = r#"
    :alice a :Person ; :name "Alice" ; :age 30 .
    :bob :name "Bob" .
"#;

fn data() -> RdfData {
    RdfData::from_str(
        &format!("{PREFIXES}{DATA}"),
        &RDFFormat::Turtle,
        None,
        &ReaderMode::Strict,
    )
    .unwrap()
}

fn shapes(shapes: &str) -> IRSchema {
    let graph = RdfData::from_str(
        &format!("{PREFIXES}{shapes}"),
        &RDFFormat::Turtle,
        None,
        &ReaderMode::Strict,
    )
    .unwrap();
    ShaclParser::new(graph).parse().unwrap().try_into().unwrap()
}

fn sink() -> (Arc<Mutex<GraphSink>>, ShaclConfig) {
    let sink = Arc::new(Mutex::new(GraphSink::new(OxigraphInMemory::new())));
    let config = ShaclConfig::default().with_read_sink(TriplesSink::new(sink.clone()));
    (sink, config)
}

/// The distinct triples the sink kept, in N-Triples.
fn kept(sink: &Arc<Mutex<GraphSink>>) -> BTreeSet<String> {
    let sink = sink.lock().unwrap();
    sink.graph().triples().unwrap().map(|t| t.to_string()).collect()
}

fn ex(triple: &str) -> String {
    triple.replace("<:", "<http://example.org/")
}

fn graph_validation() -> GraphValidation {
    Graph::from(data()).into()
}

#[test]
fn records_the_triples_a_property_shape_reads() {
    let schema = shapes(":S sh:targetNode :alice ; sh:property [ sh:path :name ; sh:minCount 1 ] .");
    let (sink, config) = sink();

    let report = graph_validation()
        .validate(&schema, &ShaclValidationMode::Native, &config)
        .unwrap();

    assert!(report.conforms());
    assert_eq!(kept(&sink), BTreeSet::from([ex("<:alice> <:name> \"Alice\"")]));
}

#[test]
fn records_the_type_triples_behind_a_target_class() {
    let schema = shapes(":S sh:targetClass :Person ; sh:property [ sh:path :age ; sh:minCount 1 ] .");
    let (sink, config) = sink();

    graph_validation()
        .validate(&schema, &ShaclValidationMode::Native, &config)
        .unwrap();

    let kept = kept(&sink);
    assert!(
        kept.contains(&ex(
            "<:alice> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <:Person>"
        )),
        "{kept:?}"
    );
    assert!(
        kept.contains(&ex(
            "<:alice> <:age> \"30\"^^<http://www.w3.org/2001/XMLSchema#integer>"
        )),
        "{kept:?}"
    );
    assert!(!kept.iter().any(|t| t.contains("bob")), "{kept:?}");
}

#[test]
fn records_nothing_once_the_validation_is_over() {
    let schema = shapes(":S sh:targetNode :alice ; sh:property [ sh:path :name ; sh:minCount 1 ] .");
    let (sink, config) = sink();
    let mut validation = graph_validation();

    validation
        .validate(&schema, &ShaclValidationMode::Native, &config)
        .unwrap();
    validation
        .validate(&schema, &ShaclValidationMode::Native, &ShaclConfig::default())
        .unwrap();

    assert_eq!(sink.lock().unwrap().report().triples, 1);
}

#[test]
fn does_not_record_the_data_fetched_in_advance() {
    let schema = shapes(":S sh:targetNode :alice ; sh:property [ sh:path :name ; sh:minCount 1 ] .");
    let (sink, config) = sink();
    let config = config.with_fetch_strategy(FetchStrategy::ByShape);

    CachedDataValidation::new(data())
        .validate(&schema, &ShaclValidationMode::Native, &config)
        .unwrap();

    assert_eq!(kept(&sink), BTreeSet::from([ex("<:alice> <:name> \"Alice\"")]));
}

#[test]
fn a_read_sink_needs_the_native_engine() {
    let schema = shapes(":S sh:targetNode :alice .");
    let (_, config) = sink();

    let error = graph_validation()
        .validate(&schema, &ShaclValidationMode::Sparql, &config)
        .unwrap_err();

    assert!(error.to_string().contains("native engine"), "{error}");
}

#[test]
fn a_processor_that_cannot_record_refuses_the_sink() {
    let schema = shapes(":S sh:targetNode :alice .");
    let (_, config) = sink();
    let mut validation: DataValidation = data().into();

    let error = validation
        .validate(&schema, &ShaclValidationMode::Native, &config)
        .unwrap_err();

    assert!(error.to_string().contains("no read sink"), "{error}");
}
