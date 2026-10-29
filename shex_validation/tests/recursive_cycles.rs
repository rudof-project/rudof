//! Shapes that refer to themselves through triple constraints, on data where the
//! references form long cycles. Each `(node, shape)` pair must be proved a bounded
//! number of times: following every path through a cycle is exponential in its
//! length.

use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::ir::{map_state::MapState, schema_ir::SchemaIR, semantic_actions_registry::SemanticActionsRegistry};
use shex_ast::shapemap::{NodeSelector, QueryShapeMap, ShapeSelector};
use shex_ast::{Node, ResolveMethod, ShExParser, ShapeExprLabel, ir::ast2ir::AST2IR};
use shex_validation::{Validator, ValidatorConfig};

const KNOWS: &str = "prefix : <http://example.org/>\n:P { :knows @:P {1,2} }";

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

/// Validates `nodes` (local names, in this order) against `:P` and returns the
/// conforming ones.
fn conforming(schema: &str, data: &str, nodes: &[String], config: ValidatorConfig) -> Vec<String> {
    let compiled = compile(schema, &config);
    let validator = Validator::new(&compiled, &config).expect("validator");
    let graph = OxigraphInMemory::from_str(data, &RDFFormat::Turtle, None, &ReaderMode::Strict).expect("parse graph");
    let shape_label = ShapeLabel::iri(IriS::new_unchecked("http://example.org/P"));
    let mut shapemap = QueryShapeMap::new();
    let nodes: Vec<Node> = nodes
        .iter()
        .map(|n| Node::parse(&format!("http://example.org/{n}"), None).unwrap())
        .collect();
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

/// `n` nodes where node `i` knows `i+1` and `i+2` (modulo `n`).
fn ring(n: usize) -> (String, Vec<String>) {
    let mut data = String::from("prefix : <http://example.org/>\n");
    for i in 0..n {
        data.push_str(&format!(":n{i} :knows :n{}, :n{} .\n", (i + 1) % n, (i + 2) % n));
    }
    (data, (0..n).map(|i| format!("n{i}")).collect())
}

#[test]
fn a_long_cycle_is_validated_in_a_bounded_number_of_steps() {
    // Before memoising results that depend on hypotheses, 28 nodes took about
    // 20 s and the number of steps grew by about 1.5x per node.
    let (data, nodes) = ring(60);
    let config = ValidatorConfig::default().with_max_steps(Some(50 * nodes.len()));
    assert_eq!(conforming(KNOWS, &data, &nodes, config), nodes);
}

#[test]
fn one_broken_node_fails_every_node_that_reaches_it() {
    let (mut data, nodes) = ring(30);
    // n7 also knows a literal, which matches no constraint (no EXTRA).
    data.push_str(":n7 :knows 1 .\n");
    let config = ValidatorConfig::default().with_max_steps(Some(50 * nodes.len()));
    assert!(conforming(KNOWS, &data, &nodes, config).is_empty());
}

#[test]
fn a_result_proved_under_a_hypothesis_that_fails_is_not_kept() {
    // :b is proved assuming :a holds while :a is checked; :a then fails (no :p),
    // so :b must fail too when it is validated afterwards.
    let schema = "prefix : <http://example.org/>\n:P { :knows @:P ; :p [1] }";
    let data = "prefix : <http://example.org/>\n:a :knows :b .\n:b :knows :a ; :p 1 .\n";
    let order = ["a".to_string(), "b".to_string()];
    assert!(conforming(schema, data, &order, ValidatorConfig::default()).is_empty());
    let reversed = ["b".to_string(), "a".to_string()];
    assert!(conforming(schema, data, &reversed, ValidatorConfig::default()).is_empty());
}

#[test]
fn a_cycle_next_to_a_failing_branch_keeps_its_conforming_nodes() {
    // :a :b :c form a valid cycle; :d knows :c and :e, and :e fails, so :d fails.
    let schema = "prefix : <http://example.org/>\n:P { :knows @:P {1,2} ; :p [1] }";
    let data = "prefix : <http://example.org/>
:a :knows :b ; :p 1 .
:b :knows :c ; :p 1 .
:c :knows :a ; :p 1 .
:d :knows :c, :e ; :p 1 .
:e :knows :d .
";
    let nodes: Vec<String> = ["d", "a", "b", "c", "e"].map(String::from).to_vec();
    assert_eq!(
        conforming(schema, data, &nodes, ValidatorConfig::default()),
        ["a", "b", "c"]
    );
}
