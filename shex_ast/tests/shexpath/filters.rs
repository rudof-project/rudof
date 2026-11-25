//! Filters, among them the reference implementation's `filters-test.ts`.

use crate::common::*;
use serde_json::json;
use shex_ast::shexpath::ShExPathError;

/// The three constraints of `<S1>`, in the order the schema lists them.
const CONSTRAINTS: &str = "@<http://a.example/S1>/expression/expressions/*";
/// All five, from both shapes, so a position spans more than one input item.
const ALL: &str = "/shapes/*/shapeExpr/expression/expressions/*";
/// The constraints of `<#IssueShape>` in the example schema of the spec.
const ISSUE_CONSTRAINTS: &str = "@<#IssueShape>/expression/expressions/*";

fn issue_constraint(n: usize) -> String {
    format!("/shapes/0/shapeExpr/expression/expressions/{n}")
}

#[test]
fn without_a_filter_everything_is_selected() {
    assert_eq!(predicates(&two_shapes(), CONSTRAINTS), ["p1", "p2", "p3"]);
}

#[test]
fn a_number_picks_one_item_counting_from_0() {
    let doc = two_shapes();
    assert_eq!(predicates(&doc, &format!("{CONSTRAINTS}[0]")), ["p1"]);
    assert_eq!(predicates(&doc, &format!("{CONSTRAINTS}[1]")), ["p2"]);
    assert_eq!(predicates(&doc, &format!("{CONSTRAINTS}[2]")), ["p3"]);
}

#[test]
fn a_number_picks_nothing_when_there_is_no_such_position() {
    let doc = two_shapes();
    assert!(predicates(&doc, &format!("{CONSTRAINTS}[3]")).is_empty());
    assert!(predicates(&doc, &format!("{CONSTRAINTS}[-1]")).is_empty());
}

#[test]
fn a_number_means_the_same_as_the_index_it_abbreviates() {
    let doc = two_shapes();
    for n in 0..3 {
        assert_eq!(
            predicates(&doc, &format!("{CONSTRAINTS}[index() = {n}]")),
            predicates(&doc, &format!("{CONSTRAINTS}[{n}]")),
            "{n}"
        );
    }
}

#[test]
fn a_filter_applies_to_what_the_filter_before_it_left() {
    let doc = two_shapes();
    assert_eq!(predicates(&doc, &format!("{CONSTRAINTS}[1][0]")), ["p2"]);
    assert!(predicates(&doc, &format!("{CONSTRAINTS}[1][1]")).is_empty());
}

#[test]
fn a_position_counts_across_everything_the_step_selected() {
    let doc = two_shapes();
    assert_eq!(predicates(&doc, ALL), ["p1", "p2", "p3", "p4", "p5"]);
    assert_eq!(predicates(&doc, &format!("{ALL}[0]")), ["p1"]);
    assert_eq!(predicates(&doc, &format!("{ALL}[3]")), ["p4"]);
    assert_eq!(predicates(&doc, &format!("{ALL}[4]")), ["p5"]);
}

#[test]
fn count_is_the_size_of_the_value_being_trimmed() {
    let doc = two_shapes();
    assert_eq!(
        predicates(&doc, &format!("{CONSTRAINTS}[count() = 3]")),
        ["p1", "p2", "p3"]
    );
    assert!(predicates(&doc, &format!("{CONSTRAINTS}[count() = 2]")).is_empty());
}

#[test]
fn a_bare_index_is_true_for_every_position_but_the_first() {
    assert_eq!(
        predicates(&two_shapes(), &format!("{CONSTRAINTS}[index()]")),
        ["p2", "p3"]
    );
}

#[test]
fn an_assertion_that_holds_keeps_every_item() {
    assert_eq!(
        predicates(&two_shapes(), &format!("{CONSTRAINTS}[assert count() = 3]")),
        ["p1", "p2", "p3"]
    );
}

#[test]
fn an_assertion_that_does_not_hold_is_an_error() {
    let doc = two_shapes();
    let result = doc.select(&format!("{CONSTRAINTS}[assert count() = 2]"));
    assert!(
        matches!(result, Err(ShExPathError::AssertionFailed { .. })),
        "{result:?}"
    );
}

#[test]
fn an_assertion_on_nothing_is_not_tested() {
    assert!(select(&two_shapes(), "/start[assert count() = 1]").is_empty());
}

#[test]
fn an_attribute_selects_by_its_value() {
    let doc = two_shapes();
    let items = select(&doc, "/shapes/*[id = <http://a.example/S2>]");
    assert_eq!(locations(&items), ["/shapes/1"]);
    assert_eq!(items[0].value()["id"], json!("http://a.example/S2"));
    assert_eq!(
        predicates(&doc, "@<http://a.example/S2>/expression/expressions/*"),
        ["p4", "p5"]
    );
}

#[test]
fn an_attribute_value_nothing_has_selects_nothing() {
    assert!(select(&two_shapes(), "/shapes/*[id = <http://a.example/nope>]").is_empty());
}

#[test]
fn the_filter_examples_of_the_spec() {
    let doc = issue();
    let all: Vec<String> = (0..4).map(issue_constraint).collect();
    assert_eq!(selects(&doc, ISSUE_CONSTRAINTS), all);
    assert_eq!(selects(&doc, &format!("{ISSUE_CONSTRAINTS}[1]")), [issue_constraint(1)]);
    assert_eq!(
        selects(&doc, &format!("{ISSUE_CONSTRAINTS}[index() = 1]")),
        [issue_constraint(1)]
    );
    assert_eq!(
        selects(&doc, &format!("{ISSUE_CONSTRAINTS}[predicate = :name]")),
        [issue_constraint(0)]
    );
    assert_eq!(
        selects(&doc, "@<#IssueShape>/expression/expressions/TripleConstraint"),
        all
    );
    assert_eq!(selects(&doc, &format!("{ISSUE_CONSTRAINTS}[assert count() = 4]")), all);
    doc.select(&format!("{ISSUE_CONSTRAINTS}[assert count() = 5]"))
        .unwrap_err();
    assert_eq!(selects(&doc, "/shapes/*[0]"), ["/shapes/0"]);
    assert_eq!(
        selects(&doc, "/shapes/*[id = <#IssueShape>][assert count() = 1]/shapeExpr"),
        ["/shapes/0/shapeExpr"]
    );
}

#[test]
fn a_path_keeps_the_items_it_selects_something_from() {
    let doc = issue();
    // "[valueExpr] keeps the constraints that have one."
    assert_eq!(
        selects(&doc, "/shapes/*/shapeExpr/expression/expressions/*[valueExpr]").len(),
        4
    );
    assert!(select(&doc, "/shapes/*/shapeExpr/expression/expressions/*[inverse]").is_empty());
    // A value of more than one item is true. #UserShape's shape expression is a ShapeAnd, and
    // the other two shapes' expressions are single constraints.
    assert_eq!(
        selects(&doc, "/shapes/*/shapeExpr/expression[expressions/*]"),
        ["/shapes/0/shapeExpr/expression"]
    );
}

#[test]
fn the_number_0_is_false_and_any_other_number_true() {
    let doc = issue();
    // :processing is the only constraint with a cardinality: min 0, max 1.
    assert!(select(&doc, &format!("{ISSUE_CONSTRAINTS}[min]")).is_empty());
    assert_eq!(
        selects(&doc, &format!("{ISSUE_CONSTRAINTS}[max]")),
        [issue_constraint(3)]
    );
}

#[test]
fn a_boolean_is_its_own_value() {
    let doc = document("PREFIX ex: <http://ex.example/> ex:S CLOSED { ex:p . } ex:T { ex:p . }");
    assert_eq!(selects(&doc, "/shapes/*/shapeExpr[closed]"), ["/shapes/0/shapeExpr"]);
}

#[test]
fn a_path_compared_with_an_integer() {
    let doc = issue();
    assert_eq!(
        selects(&doc, &format!("{ISSUE_CONSTRAINTS}[min = 0]")),
        [issue_constraint(3)]
    );
    assert_eq!(
        selects(&doc, &format!("{ISSUE_CONSTRAINTS}[max = 1]")),
        [issue_constraint(3)]
    );
    assert!(select(&doc, &format!("{ISSUE_CONSTRAINTS}[predicate = 1]")).is_empty());
}

#[test]
fn a_term_type_is_a_keyword_not_an_iri() {
    let doc = issue();
    assert!(select(&doc, &format!("{ISSUE_CONSTRAINTS}[type = <TripleConstraint>]")).is_empty());
}

#[test]
fn less_than_and_greater_than_select_nothing() {
    let doc = issue();
    assert!(select(&doc, &format!("{ISSUE_CONSTRAINTS}[max > 0]")).is_empty());
    assert!(select(&doc, &format!("{ISSUE_CONSTRAINTS}[min < 1]")).is_empty());
    assert!(select(&doc, &format!("{ISSUE_CONSTRAINTS}[count() > 0]")).is_empty());
}
