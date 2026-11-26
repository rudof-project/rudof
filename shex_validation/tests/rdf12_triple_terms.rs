//! End-to-end test for issue #827: the validation engine used to panic with
//! `todo!()` whenever the focus node being validated was itself an RDF 1.2
//! triple term (`<<( s p o )>>`). This exercises that path by validating a
//! shape against a triple-term node that has an outgoing annotation triple.

use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_core::term::{IriOrBlankNode, Object};
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::ir::{map_state::MapState, schema_ir::SchemaIR, semantic_actions_registry::SemanticActionsRegistry};
use shex_ast::{Node, ResolveMethod, ShExParser, ir::ast2ir::AST2IR};
use shex_validation::{Validator, ValidatorConfig};

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

/// The focus node is the RDF 1.2 triple term `<<( <s> <p> "o" )>>`. RDF 1.2
/// only allows triple terms in object position, so the data asserts it as
/// the object of `:x :asserts <<( :s :p "o" )>>`. Validating a shape with an
/// inverse `^<asserts>` constraint against that triple-term node requires
/// converting the focus node back into an RDF term to look up its incoming
/// arcs -- the exact step that previously hit `todo!()`.
#[test]
fn validates_shape_on_triple_term_focus_node() {
    let data = r#"
        @prefix : <http://a.example/> .
        :x :asserts <<( :s :p "o" )>> .
    "#;
    let schema = r#"
        BASE <http://a.example/>
        <M> { ^<asserts> [<http://a.example/x>] }
    "#;

    let config = ValidatorConfig::default();
    let compiled = compile(schema, &config);
    let mut validator = Validator::new(&compiled, &config).expect("validator");
    let graph = OxigraphInMemory::from_str(data, &RDFFormat::Turtle, None, &ReaderMode::Strict).expect("parse graph");

    let triple_term = Object::Triple {
        subject: Box::new(IriOrBlankNode::Iri(IriS::new_unchecked("http://a.example/s"))),
        predicate: IriS::new_unchecked("http://a.example/p"),
        object: Box::new(Object::str("o")),
    };
    let node = Node::new(triple_term);
    let shape_label = ShapeLabel::iri(IriS::new_unchecked("http://a.example/M"));

    let result = validator
        .validate_node_shape(&node, &shape_label, &graph, &compiled, &Some(graph.prefixmap().clone()))
        .expect("validate");

    assert!(
        result.get_info(&node, &shape_label).expect("status").is_conformant(),
        "triple-term focus node should conform to <M>"
    );
}
