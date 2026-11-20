//! The `thisTripleExpr::` axis and inclusions, among them the reference implementation's `inclusions-test.ts`.

use crate::common::*;

#[test]
fn the_item_and_the_triple_expressions_it_groups_transitively() {
    assert_eq!(
        described(&select(
            &issue(),
            "@<#UserShape>/thisShapeExpr::Shape/expression/thisTripleExpr::"
        ))
        .into_iter()
        .map(|d| d.replace("http://xmlns.com/foaf/0.1/", "foaf:"))
        .collect::<Vec<_>>(),
        [
            "EachOf",
            "OneOf",
            "TripleConstraint(foaf:name)",
            "EachOf",
            "TripleConstraint(foaf:givenName)",
            "TripleConstraint(foaf:familyName)",
            "TripleConstraint(foaf:mbox)"
        ]
    );
}

#[test]
fn it_does_not_enter_a_constraints_value_expression() {
    let all: Vec<String> = (0..4)
        .map(|n| format!("/shapes/0/shapeExpr/expression/expressions/{n}"))
        .collect();
    assert_eq!(
        selects(&issue(), "@<#IssueShape>/expression/thisTripleExpr::TripleConstraint"),
        all
    );
}

#[test]
fn it_reaches_a_constraint_the_shape_includes_by_reference() {
    assert_eq!(
        described(&select(&inclusions_schema(), "@<S1>~:p1")),
        ["TripleConstraint(:p1)"]
    );
    assert_eq!(
        selects(&inclusions_schema(), "@<S1>~:p1"),
        ["/shapes/1/shapeExpr/expression/expressions/0"]
    );
}

#[test]
fn it_still_reaches_the_constraints_written_in_the_shape() {
    assert_eq!(
        described(&select(&inclusions_schema(), "@<S1>~:p9")),
        ["TripleConstraint(:p9)"]
    );
}

#[test]
fn the_axis_itself_walks_into_the_included_expression() {
    assert_eq!(
        described(&select(
            &inclusions_schema(),
            "@<S1>/expression/thisTripleExpr::TripleConstraint"
        )),
        [
            "TripleConstraint(:p1)",
            "TripleConstraint(:p2)",
            "TripleConstraint(:p9)"
        ]
    );
}

#[test]
fn it_gives_the_included_group_as_well_as_its_constraints() {
    assert_eq!(
        described(&select(&inclusions_schema(), "@<S1>/expression/thisTripleExpr::")),
        [
            "EachOf",
            "EachOf(:grp)",
            "TripleConstraint(:p1)",
            "TripleConstraint(:p2)",
            "TripleConstraint(:p9)"
        ]
    );
}

#[test]
fn it_follows_a_label_once_so_a_cycle_terminates() {
    assert_eq!(
        described(&select(
            &inclusions_schema(),
            "@<Cycle>/expression/thisTripleExpr::TripleConstraint"
        )),
        ["TripleConstraint(:p3)"]
    );
}

#[test]
fn an_expression_included_twice_is_reported_once() {
    let doc = document(
        "PREFIX : <http://a.example/>
         BASE <http://a.example/>
         <Twice> { &<grp> ; &<grp> }
         <Holder> { $<grp> ( :p1 . ; :p2 . ) }",
    );
    assert_eq!(
        described(&select(&doc, "@<Twice>/expression/thisTripleExpr::TripleConstraint")),
        ["TripleConstraint(:p1)", "TripleConstraint(:p2)"]
    );
}

#[test]
fn an_inclusion_of_a_label_nothing_declares_selects_nothing() {
    let doc = document(
        "PREFIX : <http://a.example/>
         BASE <http://a.example/>
         <S1> { &<nope> ; :p9 . }",
    );
    assert_eq!(
        described(&select(&doc, "@<S1>/expression/thisTripleExpr::TripleConstraint")),
        ["TripleConstraint(:p9)"]
    );
}
