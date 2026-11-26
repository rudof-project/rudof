//! A shape can't extend itself, directly or indirectly: the schemas of the
//! ShExTest `negativeStructure` manifest with a cycle through EXTENDS are
//! rejected when they are compiled. A cycle without EXTENDS is a recursive
//! reference, and is valid.

use rudof_iri::IriS;
use shex_ast::ir::{map_state::MapState, schema_ir::SchemaIR, semantic_actions_registry::SemanticActionsRegistry};
use shex_ast::{ResolveMethod, ShExParser, ir::ast2ir::AST2IR};

fn compile(schema_src: &str) -> Result<SchemaIR, String> {
    let base = IriS::new_unchecked("http://example.org/");
    let ast = ShExParser::parse(schema_src, Some(base.clone()), &base).expect("parse schema");
    let mut map_state = MapState::default();
    let registry = SemanticActionsRegistry::default();
    registry.set_map_state(&mut map_state);
    let mut compiler = AST2IR::new(&ResolveMethod::default(), map_state);
    let mut compiled = SchemaIR::new(registry);
    compiler
        .compile(&ast, &base, &Some(base.clone()), &mut compiled, &Default::default())
        .map_err(|e| e.to_string())?;
    Ok(compiled)
}

#[test]
fn extends_cycles_of_the_test_suite_are_rejected() {
    let dir = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../shex_testsuite/shexTest/negativeStructure"
    );
    for (name, expected) in [
        ("Cycle1Extends", "[:S]"),
        ("Cycle2Extends", "[:S, :T]"),
        ("Cycle2ExtendsOr", "[:S, :Any]"),
        ("Cycle2ExtendsNegation", "[:S, :T]"),
    ] {
        let schema = std::fs::read_to_string(format!("{dir}/{name}.shex")).expect("read schema");
        let error = compile(&schema)
            .err()
            .unwrap_or_else(|| panic!("{name} should be rejected"));
        assert!(
            error.starts_with("A shape extends itself, directly or indirectly: ") && error.contains(expected),
            "{name}: {error}"
        );
    }
}

#[test]
fn recursive_references_without_extends_are_valid() {
    let schema = "prefix : <http://example.org/>\n:A @:A AND { :p . }\n:B EXTENDS @:A { }";
    compile(schema).expect("a recursive reference is not an EXTENDS cycle");
}
