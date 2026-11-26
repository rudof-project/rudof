//! The entries of the reference implementation's `__tests__/Manifest.json`
//! ([ShapePath.js](https://github.com/shexSpec/ShapePath.js)), evaluated against the schemas of the
//! ShEx test suite they name.
//!
//! The manifest checks `shapePathSchemaMatch`, the schema elements a path selects; its
//! `shapePathDataMatch`, what a validator matched, is out of the scope of ShExPath.

use crate::common::select;
use serde_json::{Value, json};
use shex_ast::compact::ShExParser;
use shex_ast::shexpath::{ShExJDocument, ShExPathItem, shexj_document};
use std::path::Path;

/// A schema of the ShEx test suite, by name.
fn shex_test(name: &str) -> ShExJDocument {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../shex_testsuite/shexTest/schemas")
        .join(format!("{name}.shex"));
    let schema = match ShExParser::parse_buf(&path, None) {
        Ok(schema) => schema,
        Err(e) => panic!("{} is not parsed: {e}", path.display()),
    };
    match shexj_document(&schema) {
        Ok(doc) => doc,
        Err(e) => panic!("the schema is not written as ShExJ: {e}"),
    }
}

fn values<'a>(items: &[ShExPathItem<'a>]) -> Vec<&'a Value> {
    items.iter().map(ShExPathItem::value).collect()
}

fn constraint(predicate: &str) -> Value {
    json!({"type": "TripleConstraint", "predicate": format!("http://a.example/{predicate}")})
}

/// `1dotShapeAND1dot3X`: `<S1> { <p1> . } AND { <p2> . } AND { <p3> . }`.
fn shape_and() -> ShExJDocument {
    shex_test("1dotShapeAND1dot3X")
}

/// The predicates of the triple constraints `S1` of `3EachdotExtra3` has.
fn predicates_of(shape: &Value) -> Vec<&str> {
    shape["expression"]["expressions"]
        .as_array()
        .map(|expressions| expressions.iter().filter_map(|tc| tc["predicate"].as_str()).collect())
        .unwrap_or_default()
}

// The first two entries expect the ShExJ of before `ShapeDecl`, where a shape expression carried
// its own `id`. In today's ShExJ, which rudof writes, `@` lands on the shape expression the
// declaration holds, and `/shapes/*` selects the declaration.

#[test]
fn at_s1() {
    let doc = shex_test("3EachdotExtra3");
    let items = select(&doc, "@<http://a.example/S1>");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].value()["type"], json!("Shape"));
    assert_eq!(
        predicates_of(items[0].value()),
        ["http://a.example/p1", "http://a.example/p2", "http://a.example/p3"]
    );
}

#[test]
fn compile_at_s1() {
    let doc = shex_test("3EachdotExtra3");
    let items = select(&doc, "/shapes/*[id=<http://a.example/S1>]");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].value()["type"], json!("ShapeDecl"));
    assert_eq!(items[0].value()["id"], json!("http://a.example/S1"));
    assert_eq!(
        predicates_of(&items[0].value()["shapeExpr"]),
        ["http://a.example/p1", "http://a.example/p2", "http://a.example/p3"]
    );
}

#[test]
fn at_s1_tilde_p2() {
    let doc = shape_and();
    assert_eq!(
        values(&select(&doc, "@<http://a.example/S1>~<http://a.example/p2>")),
        [&constraint("p2")]
    );
}

/// The entries that spell `@` out were written for the ShExJ of before `ShapeDecl`: they go from
/// a member of `shapes` straight to `thisShapeExpr::Shape`. Today that member is a `ShapeDecl`, for
/// which `thisShapeExpr::` yields nothing, as the spec says, so the path selects nothing; with the
/// `/shapeExpr` step that `@` now has, it selects what the manifest expects.
fn selects_p2_once_the_declaration_is_stepped_over(path: &str) {
    let doc = shape_and();
    assert!(select(&doc, path).is_empty(), "{path}");
    let path = path.replacen("[assert count() = 1]", "[assert count() = 1]/shapeExpr", 1);
    assert_eq!(values(&select(&doc, &path)), [&constraint("p2")], "{path}");
}

#[test]
fn compile_at_s1_tilde_p2() {
    selects_p2_once_the_declaration_is_stepped_over(
        "/shapes/*[id=<http://a.example/S1>][assert count() = 1]/thisShapeExpr::Shape/expression\n         /thisTripleExpr::TripleConstraint[predicate=<http://a.example/p2>]",
    );
}

#[test]
fn child_axis() {
    selects_p2_once_the_declaration_is_stepped_over(
        "/child::*/*[id=<http://a.example/S1>][assert count() = 1]/thisShapeExpr::Shape/child::expression\n         /thisTripleExpr::TripleConstraint[predicate=<http://a.example/p2>]",
    );
}

#[test]
fn self_axis() {
    selects_p2_once_the_declaration_is_stepped_over(
        "/self::/shapes/*/self::[id=<http://a.example/S1>][assert count() = 1]/thisShapeExpr::Shape\n         /child::expression/thisTripleExpr::TripleConstraint[predicate=<http://a.example/p2>]",
    );
}

#[test]
fn parent_axis() {
    selects_p2_once_the_declaration_is_stepped_over(
        "/shapes/parent::/shapes/*[id=<http://a.example/S1>][assert count() = 1]/thisShapeExpr::Shape\n         /expression/parent::/expression/thisTripleExpr::TripleConstraint[predicate=<http://a.example/p2>]",
    );
}

#[test]
fn ancestor_axis() {
    selects_p2_once_the_declaration_is_stepped_over(
        "/shapes/*/ancestor::/shapes/*[id=<http://a.example/S1>][assert count() = 1]/thisShapeExpr::Shape\n         /expression/thisTripleExpr::TripleConstraint[predicate=<http://a.example/p2>]",
    );
}

#[test]
fn union_of_two_constraints() {
    let doc = shape_and();
    assert_eq!(
        values(&select(
            &doc,
            "@<http://a.example/S1>~<http://a.example/p3> union @<http://a.example/S1>~<http://a.example/p2>"
        )),
        [&constraint("p3"), &constraint("p2")]
    );
}

#[test]
fn union_of_a_constraint_with_itself() {
    let doc = shape_and();
    assert_eq!(
        values(&select(
            &doc,
            "@<http://a.example/S1>~<http://a.example/p2> union @<http://a.example/S1>~<http://a.example/p2>"
        )),
        [&constraint("p2")]
    );
}

#[test]
fn sequence_of_a_constraint_with_itself() {
    let doc = shape_and();
    assert_eq!(
        values(&select(
            &doc,
            "@<http://a.example/S1>~<http://a.example/p2>, @<http://a.example/S1>~<http://a.example/p2>"
        )),
        [&constraint("p2"), &constraint("p2")]
    );
}
