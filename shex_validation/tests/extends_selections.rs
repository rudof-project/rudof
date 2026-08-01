//! End-to-end tests for the selections path (extended parents containing a `ShapeOr`):
//! split-constraint validation and diamond deduplication.
//!
//! The fixtures force `check_node_shape_extends_selections` (some parent resolves to
//! several alternatives) and pin the split-constraint semantics ported from
//! `check_node_shape_extends`: a RESTRICTS-style conjunct is validated against the
//! triples the partition allocated to the buckets in its scope — not against the whole
//! neighbourhood (its CLOSED would reject siblings' triples) and not as a competing
//! partition bucket (it would starve the main constraints).

use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
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
        .compile(&ast, &base, &Some(base.clone()), &mut compiled, config.external_resolvers())
        .expect("compile to IR");
    compiled
}

fn conforms(schema: &str, data: &str, shape: &str) -> bool {
    let config = ValidatorConfig::default();
    let compiled = compile(schema, &config);
    let mut validator = Validator::new(&compiled, &config).expect("validator");
    let graph = OxigraphInMemory::from_str(data, &RDFFormat::Turtle, None, &ReaderMode::Strict).expect("parse graph");
    let node = Node::parse("http://a.example/s", None).expect("parse focus");
    let shape_label = ShapeLabel::iri(IriS::new_unchecked(shape));
    let result = validator
        .validate_node_shape(&node, &shape_label, &graph, &compiled, &Some(graph.prefixmap().clone()))
        .expect("validate");
    result.get_info(&node, &shape_label).expect("status").is_conformant()
}

/// <Par> is a ShapeOr, forcing the selections path.  <Mid>'s RESTRICTS-style conjunct
/// @<MidC> is CLOSED over the values <Base> accepts: it conforms only when validated
/// against <Mid>'s split (the triples allocated to <Mid> and <Base>), never against the
/// whole neighbourhood, which also holds the <Par>-branch and <Bottom> triples.
const SPLIT_SCHEMA: &str = r#"
BASE <http://a.example/>
<Or1>  CLOSED { <p> ["a"] }
<Or2>  CLOSED { <p> ["b"] }
<Par>  @<Or1> OR @<Or2>
<Base> CLOSED { <p> ["g1" "g2"] }
<MidC> CLOSED { <p> ["g1" "g2"] }
<Mid>  EXTENDS @<Base> CLOSED { } AND @<MidC>
<Bottom> EXTENDS @<Par> EXTENDS @<Mid> CLOSED { <p> ["z"] }
"#;

#[test]
fn split_constraint_sees_only_its_scope() {
    // "a" goes to the <Or1> branch, "g1" to <Base>, "z" to <Bottom>'s own bucket.
    // <MidC> (CLOSED { <p> ["g1" "g2"] }) sees only {"g1"} — over the whole
    // neighbourhood its CLOSED would reject "a" and "z".
    let data = r#"<http://a.example/s> <http://a.example/p> "a", "g1", "z" ."#;
    assert!(conforms(SPLIT_SCHEMA, data, "http://a.example/Bottom"));
}

#[test]
fn split_constraint_other_or_branch() {
    let data = r#"<http://a.example/s> <http://a.example/p> "b", "g2", "z" ."#;
    assert!(conforms(SPLIT_SCHEMA, data, "http://a.example/Bottom"));
}

#[test]
fn missing_base_triple_fails() {
    let data = r#"<http://a.example/s> <http://a.example/p> "a", "z" ."#;
    assert!(!conforms(SPLIT_SCHEMA, data, "http://a.example/Bottom"));
}

#[test]
fn two_base_triples_fail() {
    // <Base> has cardinality {1,1}: "g1" and "g2" cannot both be allocated.
    let data = r#"<http://a.example/s> <http://a.example/p> "a", "g1", "g2", "z" ."#;
    assert!(!conforms(SPLIT_SCHEMA, data, "http://a.example/Bottom"));
}

#[test]
fn unallocatable_triple_fails() {
    let data = r#"<http://a.example/s> <http://a.example/p> "c", "g1", "z" ."#;
    assert!(!conforms(SPLIT_SCHEMA, data, "http://a.example/Bottom"));
}

/// A diamond reached through a ShapeOr parent: <D0> arrives via the selected branch of
/// <DOr> and via the direct EXTENDS @<Da>; its bucket must appear once per selection.
const DIAMOND_OR_SCHEMA: &str = r#"
BASE <http://a.example/>
<D0>  CLOSED { <p> ["d1" "d2"] }
<Da>  EXTENDS @<D0> CLOSED { <p> ["da"] }
<Db>  EXTENDS @<D0> CLOSED { <p> ["db"] }
<DOr> @<Da> OR @<Db>
<DBottom> EXTENDS @<DOr> EXTENDS @<Da> CLOSED { <p> ["z"] }
"#;

#[test]
fn diamond_through_or_deduplicated() {
    // One <D0> triple satisfies the shared ancestor for both inheritance paths.
    let data = r#"<http://a.example/s> <http://a.example/p> "d1", "da", "z" ."#;
    assert!(conforms(DIAMOND_OR_SCHEMA, data, "http://a.example/DBottom"));
}

#[test]
fn diamond_through_or_rejects_two_ancestor_triples() {
    let data = r#"<http://a.example/s> <http://a.example/p> "d1", "d2", "da", "z" ."#;
    assert!(!conforms(DIAMOND_OR_SCHEMA, data, "http://a.example/DBottom"));
}

/// A NOT conjunct is not expressible as triple expressions over a split; it must keep
/// the whole-neighbourhood check even on the selections path.
const NOT_SCHEMA: &str = r#"
BASE <http://a.example/>
<NOr1> CLOSED { <p> ["a"] }
<NOr2> CLOSED { <p> ["b"] }
<NPar> @<NOr1> OR @<NOr2>
<NMid> { <q> . } AND NOT { <r> ["bad"] }
<NBottom> EXTENDS @<NPar> EXTENDS @<NMid> { <p> ["z"] }
"#;

#[test]
fn not_conjunct_checked_against_whole_node() {
    let ok = r#"<http://a.example/s> <http://a.example/p> "a", "z" ;
                 <http://a.example/q> "x" ."#;
    assert!(conforms(NOT_SCHEMA, ok, "http://a.example/NBottom"));

    let bad = r#"<http://a.example/s> <http://a.example/p> "a", "z" ;
                  <http://a.example/q> "x" ;
                  <http://a.example/r> "bad" ."#;
    assert!(!conforms(NOT_SCHEMA, bad, "http://a.example/NBottom"));
}
