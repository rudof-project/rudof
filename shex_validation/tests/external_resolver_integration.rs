//! End-to-end tests for EXTERNAL shape resolution.

use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{InMemoryGraph, ReaderMode};
use shex_ast::ir::external_resolver::SchemaExternalResolver;
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::ir::{map_state::MapState, schema_ir::SchemaIR, semantic_actions_registry::SemanticActionsRegistry};
use shex_ast::{Node, ResolveMethod, ShExParser, ir::ast2ir::AST2IR};
use shex_validation::{Validator, ValidatorConfig};

const SCHEMA: &str = "<http://a.example/Sext> EXTERNAL";
const EXTERNS: &str = "<http://a.example/Sext> { <http://a.example/p2> . }";
const DATA: &str = r#"<http://a.example/n1> <http://a.example/p1> <http://a.example/n2> .
<http://a.example/n2> <http://a.example/p2> "X" ."#;

fn compile(schema_src: &str, config: &ValidatorConfig) -> SchemaIR {
    let base = IriS::new_unchecked("http://a.example/");
    let mut ast = ShExParser::parse(schema_src, Some(base.clone()), &base).expect("parse main schema");
    ast = config.external_resolvers().rewrite_ast(ast);
    let mut map_state = MapState::default();
    let mut registry = SemanticActionsRegistry::default();
    registry.set_map_state(&mut map_state);
    let mut compiler = AST2IR::new(&ResolveMethod::default(), map_state);
    let mut compiled = SchemaIR::new(registry);
    compiler
        .compile(&ast, &base, &Some(base.clone()), &mut compiled)
        .expect("compile to IR");
    compiled
}

fn validate(focus: &str, config: ValidatorConfig) -> bool {
    let compiled = compile(SCHEMA, &config);
    let mut validator = Validator::new(&compiled, &config).expect("validator");
    let graph = InMemoryGraph::from_str(DATA, &RDFFormat::Turtle, None, &ReaderMode::Strict).expect("parse graph");
    let node = Node::parse(focus, None).expect("parse focus");
    let shape = ShapeLabel::iri(IriS::new_unchecked("http://a.example/Sext"));
    let result = validator
        .validate_node_shape(&node, &shape, &graph, &compiled, &Some(graph.prefixmap().clone()))
        .expect("validate");
    result.get_info(&node, &shape).expect("status").is_conformant()
}

#[test]
fn default_config_rejects_external_shape() {
    // No SchemaExternalResolver registered → RejectAllExternalResolver answers
    // NonConformant for every EXTERNAL. The focus node has the expected `<p2>`
    // edge but the validator never gets to check it.
    assert!(!validate("http://a.example/n2", ValidatorConfig::default()));
    assert!(!validate("http://a.example/n1", ValidatorConfig::default()));
}

#[test]
fn schema_resolver_substitutes_and_validates() {
    let base = IriS::new_unchecked("http://a.example/");
    let externs_ast = ShExParser::parse(EXTERNS, Some(base.clone()), &base).expect("parse externs");
    let resolver = SchemaExternalResolver::from_schema("test-externs", externs_ast);

    let config_with_externs = || ValidatorConfig::default().with_external_resolver(resolver.clone());

    // n2 has <p2> "X" → conforms to <Sext> { <p2> . }
    assert!(validate("http://a.example/n2", config_with_externs()));

    // n1 has no <p2> → does not conform
    assert!(!validate("http://a.example/n1", config_with_externs()));
}
