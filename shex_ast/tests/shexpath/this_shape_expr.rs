//! The `thisShapeExpr::` axis.

use crate::common::*;

#[test]
fn the_node_constraint_half_of_user_shape() {
    assert_eq!(
        selects(&issue(), "@<#UserShape>/shapeExprs/*/thisShapeExpr::NodeConstraint"),
        ["/shapes/1/shapeExpr/shapeExprs/0"]
    );
}

#[test]
fn the_item_and_the_shape_expressions_a_conjunction_combines() {
    assert_eq!(
        selects(&issue(), "@<#UserShape>/thisShapeExpr::"),
        [
            "/shapes/1/shapeExpr",
            "/shapes/1/shapeExpr/shapeExprs/0",
            "/shapes/1/shapeExpr/shapeExprs/1"
        ]
    );
    assert_eq!(
        selects(&issue(), "@<#UserShape>/thisShapeExpr::Shape"),
        ["/shapes/1/shapeExpr/shapeExprs/1"]
    );
}

#[test]
fn transitively_and_through_negation() {
    let doc = document(
        "PREFIX : <http://a.example/>
         :S { :p . } OR NOT ( @:T AND { :q . } )
         :T { }",
    );
    assert_eq!(
        described(&select(&doc, "@:S/thisShapeExpr::")),
        ["ShapeOr", "Shape", "ShapeNot", "ShapeAnd", "Shape"]
    );
}

#[test]
fn it_stops_at_a_shape_reference() {
    let doc = document(
        "PREFIX : <http://a.example/>
         :S @:T AND { :p . }
         :T { }",
    );
    assert_eq!(described(&select(&doc, "@:S/thisShapeExpr::")), ["ShapeAnd", "Shape"]);
}

#[test]
fn it_yields_nothing_for_a_shape_external_or_a_shape_decl() {
    let doc = document("PREFIX : <http://a.example/> :S EXTERNAL");
    assert!(select(&doc, "/shapes/*/shapeExpr/thisShapeExpr::").is_empty());
    assert!(select(&issue(), "/shapes/*/thisShapeExpr::").is_empty());
}

#[test]
fn it_does_not_enter_a_shapes_expression() {
    assert_eq!(
        selects(&issue(), "@<#IssueShape>/thisShapeExpr::"),
        ["/shapes/0/shapeExpr"]
    );
}
