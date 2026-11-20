//! Junctions, among them the reference implementation's `junctions-test.ts`.

use crate::common::*;
use shex_ast::shexpath::ShExPathError;

const ALL: &str = "@<S1>/expression/expressions/*";
const P1: &str = "@<S1>~<p1>";
const P2: &str = "@<S1>~<p2>";
const P4: &str = "@<S2>~<p4>";

#[test]
fn intersection_selects_what_every_operand_selected() {
    assert_eq!(
        predicates(&junctions_schema(), &format!("{ALL} intersection {P2}")),
        ["p2"]
    );
}

#[test]
fn intersection_selects_nothing_when_they_share_nothing() {
    let doc = junctions_schema();
    assert!(select(&doc, &format!("{P1} intersection {P2}")).is_empty());
    assert!(select(&doc, &format!("{P1} intersection {P4}")).is_empty());
}

#[test]
fn intersection_does_not_care_which_side_is_bigger() {
    assert_eq!(
        predicates(&junctions_schema(), &format!("{P2} intersection {ALL}")),
        ["p2"]
    );
}

#[test]
fn intersection_keeps_the_order_of_the_first_operand() {
    assert_eq!(
        predicates(&junctions_schema(), &format!("{ALL} intersection {ALL}")),
        ["p1", "p2", "p3"]
    );
}

#[test]
fn intersection_narrows_with_every_further_operand() {
    assert_eq!(
        predicates(
            &junctions_schema(),
            &format!("{ALL} intersection {ALL} intersection {P2}")
        ),
        ["p2"]
    );
}

#[test]
fn intersection_binds_tighter_than_union() {
    assert_eq!(
        predicates(
            &junctions_schema(),
            &format!("{ALL} intersection {P1} union {P2} intersection {P2}")
        ),
        ["p1", "p2"]
    );
}

#[test]
fn intersection_says_each_item_once() {
    let doc = junctions_schema();
    let twice = "/shapes/*/ancestor::";
    assert_eq!(select(&doc, twice).len(), 4);
    assert_eq!(select(&doc, &format!("{twice} intersection {twice}")).len(), 2);
}

#[test]
fn union_takes_either_once() {
    let doc = junctions_schema();
    assert_eq!(predicates(&doc, &format!("{P1} union {P2}")), ["p1", "p2"]);
    assert_eq!(predicates(&doc, &format!("{P1} union {P1}")), ["p1"]);
}

#[test]
fn a_sequence_keeps_what_union_drops() {
    assert_eq!(predicates(&junctions_schema(), &format!("{P1} , {P1}")), ["p1", "p1"]);
}

#[test]
fn comma_binds_loosest_and_intersection_tightest() {
    // (p1) , (p1 union (all intersection p2))
    assert_eq!(
        predicates(
            &junctions_schema(),
            &format!("{P1} , {P1} union {ALL} intersection {P2}")
        ),
        ["p1", "p1", "p2"]
    );
}

#[test]
fn union_and_intersection_compare_items_by_value() {
    // "two structurally identical triple constraints in different shapes are one item to them"
    let doc = document(
        "BASE <http://a.example/>
         <A> { <p> . }
         <B> { <p> . }",
    );
    assert_eq!(
        selects(&doc, "@<A>~<p> union @<B>~<p>"),
        ["/shapes/0/shapeExpr/expression"]
    );
    assert_eq!(
        selects(&doc, "@<A>~<p> intersection @<B>~<p>"),
        ["/shapes/0/shapeExpr/expression"]
    );
    assert_eq!(selects(&doc, "@<A>~<p> , @<B>~<p>").len(), 2);
}

#[test]
fn every_operand_is_evaluated_from_the_same_value() {
    let doc = junctions_schema();
    let context = select(&doc, "@<S1>/expression");
    let path = shex_ast::shexpath::ShExPathParser::parse("expressions/*[0] , expressions/*[2]", None, None)
        .expect("the path is parsed");
    let items = doc.evaluate_from(&path, context).expect("the path is evaluated");
    assert_eq!(
        locations(&items),
        [
            "/shapes/0/shapeExpr/expression/expressions/0",
            "/shapes/0/shapeExpr/expression/expressions/2"
        ]
    );
}

#[test]
fn a_junction_in_a_filter() {
    assert_eq!(
        selects(
            &junctions_schema(),
            "/shapes/*[shapeExpr/closed , shapeExpr/expression/expressions]"
        ),
        ["/shapes/0"]
    );
}

#[test]
fn a_parenthesized_step_is_not_evaluated() {
    let doc = junctions_schema();
    let result = doc.select("/shapes/(*)");
    assert!(
        matches!(result, Err(ShExPathError::NotImplemented { .. })),
        "{result:?}"
    );
}
