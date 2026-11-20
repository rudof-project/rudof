//! Every example of the [ShExPath specification](https://shexspec.github.io/spec/ShExPath#examples),
//! table by table, evaluated against the spec's example schemas (`fixtures/shexpath/`).
//!
//! The spec describes what each path selects in words; the locations below are those words
//! applied to the ShExJ form of the example schema.

use crate::common::*;
use prefixmap::PrefixMap;
use rudof_iri::IriS;
use serde_json::json;
use shex_ast::shexpath::{ShExJDocument, ShExPathError, ShExPathItem, ShExPathParser};

/// The schema of the Disambiguation section, with the example schema's prefixes and base.
fn bpobs() -> ShExJDocument {
    document(include_str!("../fixtures/shexpath/bpobs.shex"))
}

const ISSUE_SHAPE: &str = "/shapes/0/shapeExpr";

fn issue_constraint(n: usize) -> String {
    format!("{ISSUE_SHAPE}/expression/expressions/{n}")
}

mod simple_access {
    use super::*;

    #[test]
    fn at_issue_shape_is_the_shape_declared_as_issue_shape() {
        let doc = issue();
        let items = select(&doc, "@<#IssueShape>");
        assert_eq!(selects(&doc, "@<#IssueShape>"), [ISSUE_SHAPE]);
        assert_eq!(items[0].value()["type"], json!("Shape"));
    }

    #[test]
    fn the_category_constraint_in_it() {
        let doc = issue();
        let items = select(&doc, "@<#IssueShape>~:category");
        assert_eq!(selects(&doc, "@<#IssueShape>~:category"), [issue_constraint(1)]);
        assert_eq!(items[0].value()["predicate"], json!("http://ex.example/ns#category"));
    }

    #[test]
    fn the_posted_by_constraint_addressed_by_the_name_given_to_it() {
        let doc = issue();
        let items = select(&doc, "$<#reportedBy>");
        assert_eq!(selects(&doc, "$<#reportedBy>"), [issue_constraint(2)]);
        assert_eq!(items[0].value()["predicate"], json!("http://ex.example/ns#postedBy"));
    }

    #[test]
    fn the_reference_to_user_shape_an_iri_not_the_shape() {
        let doc = issue();
        let items = select(&doc, "$<#reportedBy>/valueExpr");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].value(), &json!("http://ex.example/schema#UserShape"));
    }

    #[test]
    fn the_mbox_constraint_reached_through_the_shape_and() {
        assert_eq!(
            selects(&issue(), "@<#UserShape>~foaf:mbox"),
            ["/shapes/1/shapeExpr/shapeExprs/1/expression/expressions/1"]
        );
    }

    #[test]
    fn the_two_constraints_of_the_nested_inline_shape() {
        assert_eq!(
            selects(
                &issue(),
                "@<#IssueShape>~:processing/valueExpr/expression/expressions/*"
            ),
            [
                format!("{}/valueExpr/expression/expressions/0", issue_constraint(3)),
                format!("{}/valueExpr/expression/expressions/1", issue_constraint(3))
            ]
        );
    }
}

mod long_forms {
    use super::*;

    #[test]
    fn the_long_form_of_at() {
        let doc = issue();
        assert_eq!(
            selects(&doc, "/shapes/*[id = <#IssueShape>][assert count() = 1]/shapeExpr"),
            selects(&doc, "@<#IssueShape>")
        );
    }

    #[test]
    fn the_long_form_of_tilde() {
        let doc = issue();
        assert_eq!(
            selects(
                &doc,
                "@<#IssueShape>/thisShapeExpr::Shape/expression\n  \
                 /thisTripleExpr::TripleConstraint[predicate = :category]"
            ),
            selects(&doc, "@<#IssueShape>~:category")
        );
    }

    #[test]
    fn the_four_shape_declarations() {
        let doc = issue();
        let items = select(&doc, "/shapes/*");
        assert_eq!(items.len(), 4);
        assert!(items.iter().all(|item| item.value()["type"] == json!("ShapeDecl")));
    }

    #[test]
    fn the_first_of_them_issue_shapes_declaration() {
        let doc = issue();
        let items = select(&doc, "/shapes/*[0]");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].value()["id"], json!("http://ex.example/schema#IssueShape"));
    }
}

mod filters_and_term_types {
    use super::*;

    fn four() -> Vec<String> {
        (0..4).map(issue_constraint).collect()
    }

    #[test]
    fn the_four_constraints_in_issue_shape() {
        assert_eq!(selects(&issue(), "@<#IssueShape>/expression/expressions/*"), four());
    }

    #[test]
    fn the_second_of_them() {
        assert_eq!(
            selects(&issue(), "@<#IssueShape>/expression/expressions/*[1]"),
            [issue_constraint(1)]
        );
    }

    #[test]
    fn the_same_written_out() {
        assert_eq!(
            selects(&issue(), "@<#IssueShape>/expression/expressions/*[index() = 1]"),
            [issue_constraint(1)]
        );
    }

    #[test]
    fn the_name_constraint() {
        assert_eq!(
            selects(&issue(), "@<#IssueShape>/expression/expressions/*[predicate = :name]"),
            [issue_constraint(0)]
        );
    }

    #[test]
    fn the_four_again_all_of_them_are_triple_constraints() {
        assert_eq!(
            selects(&issue(), "@<#IssueShape>/expression/expressions/TripleConstraint"),
            four()
        );
    }

    #[test]
    fn the_four_and_an_error_if_a_fifth_is_ever_added() {
        assert_eq!(
            selects(&issue(), "@<#IssueShape>/expression/expressions/*[assert count() = 4]"),
            four()
        );
        let five = document(&include_str!("../fixtures/shexpath/issue.shex").replace(
            ":name xsd:string MINLENGTH 4 ;",
            ":name xsd:string MINLENGTH 4 ; :fifth . ;",
        ));
        assert!(matches!(
            five.select("@<#IssueShape>/expression/expressions/*[assert count() = 4]"),
            Err(ShExPathError::AssertionFailed { .. })
        ));
    }

    #[test]
    fn the_node_constraint_half_of_user_shape() {
        let doc = issue();
        let items = select(&doc, "@<#UserShape>/shapeExprs/*/thisShapeExpr::NodeConstraint");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].value()["pattern"], json!("User\\?id=[0-9]+"));
    }

    #[test]
    fn the_shape_that_constraint_is_in() {
        assert_eq!(
            selects(&issue(), "@<#IssueShape>~:category/ancestor::Shape"),
            [ISSUE_SHAPE]
        );
    }
}

mod junctions {
    use super::*;

    #[test]
    fn two_constraints() {
        assert_eq!(
            selects(&issue(), "@<#IssueShape>~:name , @<#IssueShape>~:category"),
            [issue_constraint(0), issue_constraint(1)]
        );
    }

    #[test]
    fn one_constraint_union_drops_the_repeat() {
        assert_eq!(
            selects(&issue(), "@<#IssueShape>~:name union @<#IssueShape>~:name"),
            [issue_constraint(0)]
        );
    }

    #[test]
    fn the_same_constraint_twice_comma_keeps_it() {
        assert_eq!(
            selects(&issue(), "@<#IssueShape>~:name , @<#IssueShape>~:name"),
            [issue_constraint(0), issue_constraint(0)]
        );
    }
}

mod extended_shape_expressions {
    use super::*;

    #[test]
    fn the_list_of_shape_expressions_extended_by_employee_shape() {
        let doc = issue();
        let items = select(&doc, "@<#EmployeeShape>/extends");
        assert_eq!(items.len(), 1);
        assert!(items[0].value().is_array());
    }

    #[test]
    fn the_two_references_in_it() {
        let doc = issue();
        let items = select(&doc, "@<#EmployeeShape>/extends/*");
        assert_eq!(
            items.iter().map(ShExPathItem::value).collect::<Vec<_>>(),
            [
                &json!("http://ex.example/schema#UserShape"),
                &json!("http://ex.example/schema#AgentShape")
            ]
        );
    }

    #[test]
    fn the_first_extended_shape_expression_the_reference_to_user_shape() {
        let doc = issue();
        let items = select(&doc, "@<#EmployeeShape>/extends/*[0]");
        assert_eq!(
            items.iter().map(ShExPathItem::value).collect::<Vec<_>>(),
            [&json!("http://ex.example/schema#UserShape")]
        );
    }
}

mod disambiguation {
    use super::*;

    #[test]
    fn all_three_component_constraints() {
        assert_eq!(
            selects(&bpobs(), "@<#BPObs>~:component"),
            (0..3)
                .map(|n| format!("/shapes/0/shapeExpr/expression/expressions/{n}"))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn the_third_of_them_the_one_expecting_a_code_of_posture() {
        let doc = bpobs();
        let items = select(&doc, "@<#BPObs>~:component/self::[2]");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].value()["valueExpr"]["expression"]["expressions"][0]["valueExpr"]["values"][0]["value"],
            json!("posture")
        );
    }

    #[test]
    fn an_error_there_are_three() {
        assert!(matches!(
            bpobs().select("@<#BPObs>~:component/self::[assert count() = 1]"),
            Err(ShExPathError::AssertionFailed { .. })
        ));
    }
}

mod relative_shexpaths {
    use super::*;

    fn example_prefixmap() -> PrefixMap {
        let mut prefixmap = PrefixMap::new();
        prefixmap.add_prefix("", IriS::new_unchecked("http://ex.example/ns#"));
        prefixmap
    }

    fn from_issue_shape(path: &str) -> Vec<String> {
        let doc = issue();
        let context = select(&doc, "@<#IssueShape>");
        let prefixmap = example_prefixmap();
        let path = match ShExPathParser::parse(path, None, Some(&prefixmap)) {
            Ok(path) => path,
            Err(e) => panic!("{path} is not parsed: {e}"),
        };
        match doc.evaluate_from(&path, context) {
            Ok(items) => items.iter().map(|item| item.location().to_string()).collect(),
            Err(e) => panic!("the path is not evaluated: {e}"),
        }
    }

    #[test]
    fn the_category_constraint_in_the_issue_shape_shape() {
        assert_eq!(from_issue_shape("~:category"), [issue_constraint(1)]);
    }

    #[test]
    fn the_same_constraint() {
        assert_eq!(from_issue_shape("expression/expressions/*[1]"), [issue_constraint(1)]);
    }

    #[test]
    fn a_relative_path_concatenated_to_its_context_path() {
        let doc = issue();
        assert_eq!(
            selects(&doc, "@<#IssueShape>/expression/expressions/*[1]"),
            from_issue_shape("expression/expressions/*[1]")
        );
        assert_eq!(
            selects(&doc, "@<#IssueShape>~:category"),
            from_issue_shape("~:category")
        );
    }
}
