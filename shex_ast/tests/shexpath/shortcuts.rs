//! The shortcuts, among them the reference implementation's `shortcuts-test.ts`.

use crate::common::*;
use serde_json::json;
use shex_ast::shexpath::ShExPathItem;

#[test]
fn dollar_selects_a_labelled_triple_constraint() {
    assert_eq!(
        labelled(&select(&shortcuts_schema(), "$<t1>")),
        ["TripleConstraint(:t1)"]
    );
}

#[test]
fn dollar_selects_a_labelled_group() {
    assert_eq!(labelled(&select(&shortcuts_schema(), "$<grp>")), ["EachOf(:grp)"]);
}

#[test]
fn dollar_finds_a_label_inside_a_nested_inline_shape() {
    assert_eq!(
        labelled(&select(&shortcuts_schema(), "$<nested>")),
        ["TripleConstraint(:nested)"]
    );
}

#[test]
fn dollar_takes_a_prefixed_name() {
    assert_eq!(
        labelled(&select(&shortcuts_schema(), "$:t1")),
        ["TripleConstraint(:t1)"]
    );
}

#[test]
fn dollar_selects_nothing_for_a_label_the_schema_does_not_declare() {
    assert!(select(&shortcuts_schema(), "$<nope>").is_empty());
}

#[test]
fn dollar_goes_on_being_a_path_afterwards() {
    let doc = shortcuts_schema();
    assert_eq!(
        labelled(&select(&doc, "$<grp>/expressions/*")),
        [
            "TripleConstraint(:t1)",
            "TripleConstraint(:p2)",
            "TripleConstraint(:p3)"
        ]
    );
    let predicate = select(&doc, "$<t1>/predicate");
    assert_eq!(
        predicate.iter().map(ShExPathItem::value).collect::<Vec<_>>(),
        [&json!("http://a.example/p1")]
    );
}

#[test]
fn at_selects_the_shape_expression_through_the_declaration() {
    let doc = shortcuts_schema();
    assert_eq!(labelled(&select(&doc, "@<S1>")), ["Shape"]);
    assert_eq!(labelled(&select(&doc, "@<S2>")), ["NodeConstraint"]);
}

#[test]
fn tilde_selects_the_constraint_on_that_predicate() {
    assert_eq!(
        labelled(&select(&shortcuts_schema(), "@<S1>~<p2>")),
        ["TripleConstraint(:p2)"]
    );
}

#[test]
fn at_selects_nothing_for_a_shape_label_the_schema_does_not_declare() {
    assert!(select(&shortcuts_schema(), "@<nope>").is_empty());
}

#[test]
fn the_three_namespaces_are_kept_apart() {
    let doc = shortcuts_schema();
    assert_eq!(labelled(&select(&doc, "$<t1>")), ["TripleConstraint(:t1)"]);
    // the same item, reached by predicate
    assert_eq!(selects(&doc, "@<S1>~<p1>"), selects(&doc, "$<t1>"));
    // a predicate is not a triple expression label
    assert!(select(&doc, "$<p1>").is_empty());
}

#[test]
fn an_inclusion_and_the_label_reach_the_same_item() {
    let doc = inclusions_schema();
    let by_inclusion = selects(&doc, "@<S1>~:p1");
    assert_eq!(by_inclusion.len(), 1);
    assert_eq!(by_inclusion, selects(&doc, "$<grp>/expressions/*[0]"));
}

#[test]
fn dollar_searches_downward_from_the_value_it_is_given() {
    let doc = shortcuts_schema();
    assert_eq!(labelled(&select(&doc, "@<S1>$<nested>")), ["TripleConstraint(:nested)"]);
    assert!(select(&doc, "@<S2>$<t1>").is_empty());
}

#[test]
fn at_addresses_a_schema_and_selects_nothing_anywhere_else() {
    assert!(select(&shortcuts_schema(), "@<S1>@<S1>").is_empty());
}
