//! Child steps.

use crate::common::*;
use serde_json::json;

#[test]
fn shapes_star_selects_the_shape_declarations() {
    let doc = issue();
    let items = select(&doc, "/shapes/*");
    assert_eq!(locations(&items), ["/shapes/0", "/shapes/1", "/shapes/2", "/shapes/3"]);
    assert!(items.iter().all(|item| item.value()["type"] == json!("ShapeDecl")));
}

#[test]
fn the_leading_slash_has_no_effect() {
    let doc = issue();
    assert_eq!(selects(&doc, "shapes/*"), selects(&doc, "/shapes/*"));
    assert_eq!(selects(&doc, "//shapes//*"), selects(&doc, "/shapes/*"));
}

#[test]
fn each_step_applies_to_every_item_of_the_previous_one() {
    let doc = issue();
    assert_eq!(
        selects(&doc, "/shapes/*/shapeExpr"),
        [
            "/shapes/0/shapeExpr",
            "/shapes/1/shapeExpr",
            "/shapes/2/shapeExpr",
            "/shapes/3/shapeExpr"
        ]
    );
}

#[test]
fn an_array_is_an_item_in_its_own_right() {
    let doc = issue();
    let list = select(&doc, "/shapes/*/shapeExpr/extends");
    assert_eq!(locations(&list), ["/shapes/3/shapeExpr/extends"]);
    assert!(list[0].value().is_array());

    let members = select(&doc, "/shapes/*/shapeExpr/extends/*");
    assert_eq!(
        members.iter().map(|item| item.value().clone()).collect::<Vec<_>>(),
        [
            json!("http://ex.example/schema#UserShape"),
            json!("http://ex.example/schema#AgentShape")
        ]
    );
    assert_eq!(
        locations(&members),
        ["/shapes/3/shapeExpr/extends/0", "/shapes/3/shapeExpr/extends/1"]
    );
}

#[test]
fn an_attribute_the_item_does_not_have_selects_nothing() {
    let doc = issue();
    assert!(select(&doc, "/shapes/*/shapeExpr/closed").is_empty());
    assert!(select(&doc, "/start").is_empty());
    assert!(select(&doc, "/shapes/predicate").is_empty());
}

#[test]
fn a_child_step_on_an_atomic_value_selects_nothing() {
    let doc = issue();
    assert!(select(&doc, "/shapes/*/id/*").is_empty());
    assert!(select(&doc, "/@context/type").is_empty());
}

#[test]
fn star_selects_every_property_value_of_an_object() {
    let doc = issue();
    let mut found = selects(&doc, "/*");
    found.sort();
    assert_eq!(found, ["/@context", "/shapes", "/type"]);
    assert_eq!(
        select(&doc, "/@context")[0].value(),
        &json!("http://www.w3.org/ns/shex.jsonld")
    );
}

#[test]
fn a_path_is_relative_to_the_value_it_is_given() {
    let doc = issue();
    let context = select(&doc, "/shapes/*/shapeExpr")[..1].to_vec();
    let path =
        shex_ast::shexpath::ShExPathParser::parse("expression/expressions/*", None, None).expect("the path is parsed");
    let items = doc.evaluate_from(&path, context).expect("the path is evaluated");
    assert_eq!(
        locations(&items),
        [
            "/shapes/0/shapeExpr/expression/expressions/0",
            "/shapes/0/shapeExpr/expression/expressions/1",
            "/shapes/0/shapeExpr/expression/expressions/2",
            "/shapes/0/shapeExpr/expression/expressions/3"
        ]
    );
}

#[test]
fn shapes_selects_nothing_from_a_value_that_is_not_the_schema() {
    let doc = issue();
    let context = select(&doc, "/shapes/*/shapeExpr");
    let path = shex_ast::shexpath::ShExPathParser::parse("/shapes/*", None, None).expect("the path is parsed");
    assert!(
        doc.evaluate_from(&path, context)
            .expect("the path is evaluated")
            .is_empty()
    );
}
