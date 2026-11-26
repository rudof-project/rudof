//! Shapes that refer to themselves through AND, OR or EXTENDS. A recursive
//! reference is assumed to hold while it is checked (greatest fixpoint), so
//! `:A @:A AND { :p . }` accepts the nodes that have a `:p`.

use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::ir::{map_state::MapState, schema_ir::SchemaIR, semantic_actions_registry::SemanticActionsRegistry};
use shex_ast::shapemap::{NodeSelector, QueryShapeMap, ShapeSelector};
use shex_ast::{Node, ResolveMethod, ShExParser, ShapeExprLabel, ir::ast2ir::AST2IR};
use shex_validation::{Validator, ValidatorConfig};

const DATA: &str = r#"
prefix : <http://example.org/>
:x :p 1 .
:y :p 1 .
:z :q 1 .
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

/// Validates the nodes `:x`, `:y` and `:z` against `shape` in one shape map
/// (more than one node, as in the case that overflowed the stack), and returns
/// which ones conform.
fn conforming(schema: &str, shape: &str) -> Vec<String> {
    let config = ValidatorConfig::default();
    let compiled = compile(schema, &config);
    let validator = Validator::new(&compiled, &config).expect("validator");
    let graph = OxigraphInMemory::from_str(DATA, &RDFFormat::Turtle, None, &ReaderMode::Strict).expect("parse graph");
    let shape_label = ShapeLabel::iri(IriS::new_unchecked(&format!("http://example.org/{shape}")));
    let mut shapemap = QueryShapeMap::new();
    let nodes = ["x", "y", "z"].map(|n| Node::parse(&format!("http://example.org/{n}"), None).unwrap());
    for node in &nodes {
        shapemap
            .add_association(
                NodeSelector::Node(node.try_into().unwrap()),
                &None,
                ShapeSelector::label(ShapeExprLabel::from(&shape_label)),
                &None,
            )
            .unwrap();
    }
    let result = validator
        .validate_shapemap(&shapemap, &graph, &compiled, &Some(graph.prefixmap().clone()))
        .expect("validate");
    nodes
        .iter()
        .filter(|node| result.get_info(node, &shape_label).unwrap().is_conformant())
        .map(|node| node.to_string().trim_start_matches("http://example.org/").to_string())
        .collect()
}

#[test]
fn self_reference_through_and() {
    let schema = "prefix : <http://example.org/>\n:A @:A AND { :p . }";
    assert_eq!(conforming(schema, "A"), ["x", "y"]);
}

#[test]
fn mutual_reference_through_and() {
    let schema = "prefix : <http://example.org/>\n:A @:B AND { :p . }\n:B @:A AND { :p [1] }";
    assert_eq!(conforming(schema, "A"), ["x", "y"]);
}

#[test]
fn self_reference_through_or() {
    // Under the greatest fixpoint, `:A` holds for every node: assuming it,
    // the first branch of the OR passes
    let schema = "prefix : <http://example.org/>\n:A @:A OR { :p . }";
    assert_eq!(conforming(schema, "A"), ["x", "y", "z"]);
}

#[test]
fn extends_a_shape_that_refers_to_itself_through_and() {
    let schema = "prefix : <http://example.org/>\nABSTRACT :B @:B AND { :p . }\n:C EXTENDS @:B { }";
    assert_eq!(conforming(schema, "C"), ["x", "y"]);
}
