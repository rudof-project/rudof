//! The `self::`, `parent::` and `ancestor::` axes.

use crate::common::*;

#[test]
fn self_is_the_item_itself() {
    let doc = issue();
    assert_eq!(selects(&doc, "/shapes/*/self::"), selects(&doc, "/shapes/*"));
    assert_eq!(selects(&doc, "self::"), [""]);
}

#[test]
fn self_may_carry_a_filter() {
    assert_eq!(selects(&issue(), "/shapes/*/self::[1]"), ["/shapes/1"]);
}

#[test]
fn parent_is_the_item_that_holds_this_one() {
    let doc = issue();
    assert_eq!(selects(&doc, "/shapes/*[0]/parent::"), ["/shapes"]);
    assert_eq!(selects(&doc, "/shapes/*[0]/shapeExpr/parent::"), ["/shapes/0"]);
}

#[test]
fn each_item_has_its_parent_even_when_they_share_it() {
    assert_eq!(selects(&issue(), "/shapes/*/parent::"), ["/shapes"; 4]);
}

#[test]
fn an_atomic_value_has_a_parent() {
    assert_eq!(selects(&issue(), "/shapes/*[1]/id/parent::"), ["/shapes/1"]);
}

#[test]
fn the_schema_has_no_parent() {
    assert!(select(&issue(), "parent::").is_empty());
    assert!(select(&issue(), "ancestor::").is_empty());
}

#[test]
fn ancestor_is_the_parent_its_parent_and_so_on_up_to_the_schema() {
    assert_eq!(
        selects(&issue(), "/shapes/*[0]/shapeExpr/ancestor::"),
        ["/shapes/0", "/shapes", ""]
    );
}

#[test]
fn ancestor_with_a_term_type_finds_the_shape_a_constraint_is_in() {
    assert_eq!(
        selects(&issue(), "@<#IssueShape>/expression/expressions/*[1]/ancestor::Shape"),
        ["/shapes/0/shapeExpr"]
    );
}

#[test]
fn the_ancestors_of_every_item_are_concatenated() {
    // From junctions-test.ts: two declarations, each with `shapes` and the schema above it.
    assert_eq!(
        selects(&two_shapes(), "/shapes/*/ancestor::"),
        ["/shapes", "", "/shapes", ""]
    );
}
