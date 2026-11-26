//! End-to-end tests for the lazy checking of AND/OR/NOT: a component is only
//! proved when the connective needs it, so an OR whose first branch passes
//! never follows (or, on a SPARQL endpoint, queries) its other branches.
//!
//! The fixture mirrors a Wikidata "instance of a subclass of human" check: the
//! class of the focus node is the target class itself, above which sits a whole
//! superclass hierarchy that the recursive branch could walk.

use std::sync::{Arc, Mutex};

use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::ir::{map_state::MapState, schema_ir::SchemaIR, semantic_actions_registry::SemanticActionsRegistry};
use shex_ast::{Node, ResolveMethod, ShExParser, ir::ast2ir::AST2IR};
use shex_validation::{Reason, ValidationResult, Validator, ValidatorConfig, ValidatorError};

const SCHEMA: &str = r#"
prefix : <http://a.example/>

:Person EXTRA :p31 { :p31 @:HumanOrSubclass }
:HumanOrSubclass [ :q5 ] OR EXTRA :p279 { :p279 @:HumanOrSubclass + }
:NotHuman NOT [ :q5 ] AND EXTRA :p279 { :p279 . + }
"#;

const DATA: &str = r#"
prefix : <http://a.example/>

:q42 :p31 :q5 .
:q5 :p279 :s1, :s2 .
:s1 :p279 :s3, :s4 .
:s2 :p279 :s5 .

:alice :p31 :c1 .
:c1 :p279 :c2 .
:c2 :p279 :q5 .

:rex :p31 :dog .
:dog :p279 :animal .
"#;

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

#[derive(Debug, Default)]
struct RecordingObserver {
    nodes: Mutex<Vec<String>>,
}

impl rudof_typing::TypingObserver<Node, shex_ast::ShapeLabelIdx, ValidatorError, Reason> for RecordingObserver {
    fn on_insert(&self, key: &(Node, shex_ast::ShapeLabelIdx), _value: &ValidationResult) {
        self.nodes.lock().unwrap().push(key.0.to_string());
    }
}

/// Validates `node@shape` and returns whether it conforms and every node a
/// result was proved for along the way.
fn validate(node: &str, shape: &str) -> (bool, Vec<String>) {
    let observer = Arc::new(RecordingObserver::default());
    let config = ValidatorConfig::default().with_typing_observer(observer.clone());
    let compiled = compile(SCHEMA, &config);
    let mut validator = Validator::new(&compiled, &config).expect("validator");
    let graph = OxigraphInMemory::from_str(DATA, &RDFFormat::Turtle, None, &ReaderMode::Strict).expect("parse graph");
    let node = Node::parse(&format!("http://a.example/{node}"), None).expect("parse focus");
    let shape = ShapeLabel::iri(IriS::new_unchecked(&format!("http://a.example/{shape}")));
    let result = validator
        .validate_node_shape(&node, &shape, &graph, &compiled, &Some(graph.prefixmap().clone()))
        .expect("validate");
    let conforms = result.get_info(&node, &shape).expect("status").is_conformant();
    let nodes = observer.nodes.lock().unwrap().clone();
    (conforms, nodes)
}

#[test]
fn or_stops_at_the_first_branch_that_passes() {
    let (conforms, nodes) = validate("q42", "Person");
    assert!(conforms);
    // :q5 matches [ :q5 ] on its own: none of its superclasses is visited.
    let superclasses: Vec<_> = nodes.iter().filter(|n| n.contains("/s")).collect();
    assert!(superclasses.is_empty(), "superclasses were proved: {nodes:?}");
}

#[test]
fn or_still_follows_the_second_branch_when_the_first_fails() {
    let (conforms, _) = validate("alice", "Person");
    assert!(conforms, ":alice is an instance of a subclass of a subclass of :q5");
}

#[test]
fn or_fails_when_every_branch_fails() {
    let (conforms, _) = validate("rex", "Person");
    assert!(!conforms);
}

#[test]
fn and_with_not_is_checked_lazily() {
    assert!(validate("dog", "NotHuman").0);
    // NOT [ :q5 ] fails first, so the shape of :q5 is never checked.
    let (conforms, nodes) = validate("q5", "NotHuman");
    assert!(!conforms);
    assert!(nodes.iter().all(|n| n.ends_with("/q5")), "{nodes:?}");
}
