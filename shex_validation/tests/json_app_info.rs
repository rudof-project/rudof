//! The `appInfo` of a validation result shows its errors and reasons as the
//! `reason` does: with shape labels and prefixed names, not with the internal
//! indexes of the shapes or the compiled triple expressions.

use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::ir::{map_state::MapState, schema_ir::SchemaIR, semantic_actions_registry::SemanticActionsRegistry};
use shex_ast::{Node, ResolveMethod, ShExParser, ir::ast2ir::AST2IR};
use shex_validation::{Validator, ValidatorConfig};

const SCHEMA: &str = r#"
prefix : <http://example.org/>

:T { :p @:U ; :q [ 1 ] }
:U { :r . }
"#;

const DATA: &str = r#"
prefix : <http://example.org/>

:ok :p :u ; :q 1 .
:u :r 2 .

:ko :p :y ; :q 1 .
"#;

fn compile(schema_src: &str, config: &ValidatorConfig) -> SchemaIR {
    let base = IriS::new_unchecked("http://example.org/");
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

/// The `info` entries of the `appInfo` of `node@:T`.
fn app_info_entries(node: &str) -> Vec<String> {
    let config = ValidatorConfig::default();
    let compiled = compile(SCHEMA, &config);
    let mut validator = Validator::new(&compiled, &config).expect("validator");
    let graph = OxigraphInMemory::from_str(DATA, &RDFFormat::Turtle, None, &ReaderMode::Strict).expect("parse graph");
    let node = Node::parse(&format!("http://example.org/{node}"), None).expect("parse focus");
    let shape = ShapeLabel::iri(IriS::new_unchecked("http://example.org/T"));
    let result = validator
        .validate_node_shape(&node, &shape, &graph, &compiled, &Some(graph.prefixmap().clone()))
        .expect("validate");
    let app_info = result.get_info(&node, &shape).expect("status").app_info();
    app_info["info"]
        .as_array()
        .expect("info entries")
        .iter()
        .flat_map(|entry| entry.as_object().expect("info entry").values())
        .map(|value| value.as_str().expect("info text").to_string())
        .collect()
}

#[test]
fn errors_show_shape_labels_and_prefixed_names() {
    let errors = app_info_entries("ko");
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].starts_with("Shape :T failed for node :ko"), "{errors:?}");
    assert!(errors[0].contains(":y as :U"), "{errors:?}");
    assert!(!errors[0].contains("http://"), "{errors:?}");
}

#[test]
fn reasons_show_shape_labels_and_prefixed_names() {
    let reasons = app_info_entries("ok");
    assert_eq!(reasons, vec!["Shape passed :ok@:T".to_string()]);
}
