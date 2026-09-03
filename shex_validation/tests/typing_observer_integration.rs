//! End-to-end tests for `TypingObserver`: observing intermediate validation
//! results as they're cached, and using that hook to recover partial results
//! when validation is cancelled mid-run.

use std::sync::{Arc, Mutex};

use either::Either;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::ir::{map_state::MapState, schema_ir::SchemaIR, semantic_actions_registry::SemanticActionsRegistry};
use shex_ast::shapemap::{NodeSelector, QueryShapeMap, ShapeSelector};
use shex_ast::{Node, ResolveMethod, ShExParser, ShapeExprLabel, ir::ast2ir::AST2IR};
use shex_validation::{Reason, ValidationResult, Validator, ValidatorConfig, ValidatorError};

fn compile(schema_src: &str, config: &ValidatorConfig) -> SchemaIR {
    let base = IriS::new_unchecked("http://a.example/");
    let ast = ShExParser::parse(schema_src, Some(base.clone()), &base).expect("parse schema");
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

fn shapemap(pairs: &[(&str, &str)]) -> QueryShapeMap {
    let mut sm = QueryShapeMap::new();
    for (node, shape) in pairs {
        let node = Node::parse(node, None).expect("parse focus node");
        let shape_label: ShapeExprLabel = (&ShapeLabel::iri(IriS::new_unchecked(shape))).into();
        sm.add_association(
            NodeSelector::Node((&node).try_into().expect("node as object value")),
            &None,
            ShapeSelector::label(shape_label),
            &None,
        )
        .expect("add association");
    }
    sm
}

#[derive(Debug, Default)]
struct RecordingObserver {
    seen: Mutex<Vec<(String, ValidationResult)>>,
}

impl rudof_typing::TypingObserver<Node, shex_ast::ShapeLabelIdx, ValidatorError, Reason> for RecordingObserver {
    fn on_insert(&self, key: &(Node, shex_ast::ShapeLabelIdx), value: &ValidationResult) {
        self.seen.lock().unwrap().push((key.0.to_string(), value.clone()));
    }
}

const NESTED_SCHEMA: &str = r#"
<http://a.example/T> { <http://a.example/q> . }
<http://a.example/S> { <http://a.example/p> @<http://a.example/T> }
"#;
const NESTED_DATA: &str = r#"<http://a.example/n1> <http://a.example/p> <http://a.example/n2> .
<http://a.example/n2> <http://a.example/q> "x" ."#;

#[test]
fn observer_sees_dependency_result_before_the_shape_that_needed_it() {
    let observer = Arc::new(RecordingObserver::default());
    let config = ValidatorConfig::default().with_typing_observer(observer.clone());
    let compiled = compile(NESTED_SCHEMA, &config);
    let mut validator = Validator::new(&compiled, &config).expect("validator");
    let graph =
        OxigraphInMemory::from_str(NESTED_DATA, &RDFFormat::Turtle, None, &ReaderMode::Strict).expect("parse graph");
    let node = Node::parse("http://a.example/n1", None).expect("parse focus");
    let shape = ShapeLabel::iri(IriS::new_unchecked("http://a.example/S"));

    let result = validator
        .validate_node_shape(&node, &shape, &graph, &compiled, &Some(graph.prefixmap().clone()))
        .expect("validate");
    assert!(result.get_info(&node, &shape).expect("status").is_conformant());

    let seen = observer.seen.lock().unwrap();
    // n2@T (S's dependency) is cached before n1@S (the shape that depends on it).
    assert_eq!(
        seen.len(),
        2,
        "expected one cached result per (node, shape) pair: {seen:?}"
    );
    assert_eq!(seen[0].0, "http://a.example/n2");
    assert_eq!(seen[1].0, "http://a.example/n1");
    assert!(matches!(seen[0].1, Either::Right(_)), "n2@T should conform");
    assert!(matches!(seen[1].1, Either::Right(_)), "n1@S should conform");
}

#[test]
fn show_intermediate_results_auto_installs_a_default_observer() {
    // No `with_typing_observer` call at all: `show_intermediate_results(true)`
    // alone must be enough for `Validator::new` to install a default
    // (console-printing) observer, and validation must still succeed.
    let config = ValidatorConfig::default().with_show_intermediate_results(true);
    let compiled = compile(NESTED_SCHEMA, &config);
    let mut validator = Validator::new(&compiled, &config).expect("validator");
    let graph =
        OxigraphInMemory::from_str(NESTED_DATA, &RDFFormat::Turtle, None, &ReaderMode::Strict).expect("parse graph");
    let node = Node::parse("http://a.example/n1", None).expect("parse focus");
    let shape = ShapeLabel::iri(IriS::new_unchecked("http://a.example/S"));

    let result = validator
        .validate_node_shape(&node, &shape, &graph, &compiled, &Some(graph.prefixmap().clone()))
        .expect("validate");
    assert!(result.get_info(&node, &shape).expect("status").is_conformant());
}

#[test]
fn explicit_observer_wins_over_show_intermediate_results() {
    // When both are set, the explicitly-supplied observer must be the one
    // that's actually notified, not silently replaced by the default one.
    let observer = Arc::new(RecordingObserver::default());
    let config = ValidatorConfig::default()
        .with_show_intermediate_results(true)
        .with_typing_observer(observer.clone());
    let compiled = compile(NESTED_SCHEMA, &config);
    let mut validator = Validator::new(&compiled, &config).expect("validator");
    let graph =
        OxigraphInMemory::from_str(NESTED_DATA, &RDFFormat::Turtle, None, &ReaderMode::Strict).expect("parse graph");
    let node = Node::parse("http://a.example/n1", None).expect("parse focus");
    let shape = ShapeLabel::iri(IriS::new_unchecked("http://a.example/S"));

    validator
        .validate_node_shape(&node, &shape, &graph, &compiled, &Some(graph.prefixmap().clone()))
        .expect("validate");

    assert_eq!(
        observer.seen.lock().unwrap().len(),
        2,
        "explicit observer should still be notified"
    );
}

const INDEPENDENT_SCHEMA: &str = r#"
<http://a.example/S1> { <http://a.example/p1> . }
<http://a.example/S2> { <http://a.example/p2> . }
"#;
const INDEPENDENT_DATA: &str = r#"<http://a.example/n1> <http://a.example/p1> "x" .
<http://a.example/n2> <http://a.example/p2> "y" ."#;

#[derive(Debug)]
struct CancelAfterFirstInsert;

impl rudof_typing::TypingObserver<Node, shex_ast::ShapeLabelIdx, ValidatorError, Reason> for CancelAfterFirstInsert {
    fn on_insert(&self, _key: &(Node, shex_ast::ShapeLabelIdx), _value: &ValidationResult) {
        rudof_rdf::cancellation::request_cancellation();
    }
}

#[test]
fn cancelling_mid_validation_still_returns_results_for_finished_independent_pairs() {
    rudof_rdf::cancellation::reset();

    let config = ValidatorConfig::default().with_typing_observer(Arc::new(CancelAfterFirstInsert));
    let compiled = compile(INDEPENDENT_SCHEMA, &config);
    let validator = Validator::new(&compiled, &config).expect("validator");
    let graph = OxigraphInMemory::from_str(INDEPENDENT_DATA, &RDFFormat::Turtle, None, &ReaderMode::Strict)
        .expect("parse graph");

    let n1 = Node::parse("http://a.example/n1", None).expect("parse n1");
    let n2 = Node::parse("http://a.example/n2", None).expect("parse n2");
    let s1 = ShapeLabel::iri(IriS::new_unchecked("http://a.example/S1"));
    let s2 = ShapeLabel::iri(IriS::new_unchecked("http://a.example/S2"));

    let sm = shapemap(&[
        ("http://a.example/n1", "http://a.example/S1"),
        ("http://a.example/n2", "http://a.example/S2"),
    ]);

    // Cancellation is requested from inside the observer once the first pair
    // is cached, simulating a user interrupting the run mid-way. The engine
    // should still hand back a result: the pair proved before cancellation
    // was requested, and `Pending` for the one it never got to.
    let result = validator
        .validate_shapemap(&sm, &graph, &compiled, &Some(graph.prefixmap().clone()))
        .expect("validate_shapemap should succeed with partial results, not error out");

    rudof_rdf::cancellation::reset();

    let status1 = result.get_info(&n1, &s1).expect("status for n1@S1");
    let status2 = result.get_info(&n2, &s2).expect("status for n2@S2");
    let statuses = [status1.is_conformant(), status2.is_conformant()];
    let pendings = [status1.is_pending(), status2.is_pending()];

    assert_eq!(
        statuses.iter().filter(|c| **c).count(),
        1,
        "exactly one pair should have finished before cancellation: {status1:?} / {status2:?}"
    );
    assert_eq!(
        pendings.iter().filter(|p| **p).count(),
        1,
        "exactly one pair should be left pending: {status1:?} / {status2:?}"
    );
}
