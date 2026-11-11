use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use rudof_rdf_sink::{GraphSink, RdfSink, TriplesSink};
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::ir::{map_state::MapState, schema_ir::SchemaIR, semantic_actions_registry::SemanticActionsRegistry};
use shex_ast::{Node, ResolveMethod, ShExParser, ir::ast2ir::AST2IR};
use shex_validation::{Validator, ValidatorConfig};
use std::sync::{Arc, Mutex};

const SCHEMA: &str = r#"
prefix : <http://a.example/>

:Person { :name . ; ^:knows . * }
"#;

const DATA: &str = r#"
prefix : <http://a.example/>

:alice :name "Alice" .
:bob :knows :alice .
:carol :name "Carol" .
"#;

fn compile(config: &ValidatorConfig) -> SchemaIR {
    let base = IriS::new_unchecked("http://a.example/");
    let ast = ShExParser::parse(SCHEMA, Some(base.clone()), &base).expect("parse schema");
    let mut map_state = MapState::default();
    let registry = SemanticActionsRegistry::default();
    registry.set_map_state(&mut map_state);
    let mut compiler = AST2IR::new(&ResolveMethod::default(), map_state);
    let mut compiled = SchemaIR::new(registry);
    compiler
        .compile(
            &ast,
            &base,
            &Some(base.clone()),
            &mut compiled,
            config.external_resolvers(),
        )
        .expect("compile to IR");
    compiled
}

/// Validates `:alice@:Person` `runs` times with the same sink and returns it.
fn validate_alice(runs: usize) -> Arc<Mutex<GraphSink>> {
    let sink = Arc::new(Mutex::new(GraphSink::new(OxigraphInMemory::new())));
    let config = ValidatorConfig::default().with_read_sink(TriplesSink::new(sink.clone()));
    let compiled = compile(&config);
    let mut validator = Validator::new(&compiled, &config).expect("validator");
    let graph = OxigraphInMemory::from_str(DATA, &RDFFormat::Turtle, None, &ReaderMode::Strict).expect("parse graph");
    let node = Node::parse("http://a.example/alice", None).expect("parse focus");
    let shape = ShapeLabel::iri(IriS::new_unchecked("http://a.example/Person"));
    for _ in 0..runs {
        let result = validator
            .validate_node_shape(&node, &shape, &graph, &compiled, &None)
            .expect("validate");
        assert!(result.get_info(&node, &shape).expect("status").is_conformant());
    }
    sink
}

fn sorted_triples(sink: &GraphSink) -> Vec<String> {
    let mut triples: Vec<String> = sink.graph().quads().map(|q| q.to_string()).collect();
    triples.sort();
    triples
}

#[test]
fn the_sink_receives_the_outgoing_and_incoming_arcs_read() {
    let sink = validate_alice(1);
    let sink = sink.lock().unwrap();
    assert_eq!(
        sorted_triples(&sink),
        vec![
            r#"<http://a.example/alice> <http://a.example/name> "Alice""#.to_string(),
            "<http://a.example/bob> <http://a.example/knows> <http://a.example/alice>".to_string(),
        ]
    );
    assert_eq!(sink.report().triples, 2);
}

#[test]
fn the_report_counts_every_read_and_the_graph_keeps_one_copy() {
    let sink = validate_alice(2);
    let sink = sink.lock().unwrap();
    assert_eq!(sink.report().triples, 4);
    assert_eq!(sink.graph().len(), 2);
}
