//! Evaluation of ShExPaths over the ShExJ form of a schema.

use rudof_iri::IriS;
use serde_json::json;
use shex_ast::compact::ShExParser;
use shex_ast::shexpath::{Location, Segment, ShExJDocument, ShExPathItem};

fn document(src: &str) -> ShExJDocument {
    let schema = match ShExParser::parse(src, None, &IriS::new_unchecked("http://default/")) {
        Ok(schema) => schema,
        Err(e) => panic!("the schema is not parsed: {e}"),
    };
    match ShExJDocument::new(&schema) {
        Ok(doc) => doc,
        Err(e) => panic!("the schema is not written as ShExJ: {e}"),
    }
}

/// The example schema of the spec.
fn issue() -> ShExJDocument {
    document(include_str!("fixtures/shexpath/issue.shex"))
}

fn locations(items: &[ShExPathItem<'_>]) -> Vec<String> {
    items.iter().map(|item| item.location().to_string()).collect()
}

fn select<'a>(doc: &'a ShExJDocument, path: &str) -> Vec<ShExPathItem<'a>> {
    match doc.select(path) {
        Ok(items) => items,
        Err(e) => panic!("{path} is not evaluated: {e}"),
    }
}

fn selects(doc: &ShExJDocument, path: &str) -> Vec<String> {
    locations(&select(doc, path))
}

mod child_steps {
    use super::*;

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
        let path = shex_ast::compact::ShExPathParser::parse("expression/expressions/*", None, None)
            .expect("the path is parsed");
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
        let path = shex_ast::compact::ShExPathParser::parse("/shapes/*", None, None).expect("the path is parsed");
        assert!(
            doc.evaluate_from(&path, context)
                .expect("the path is evaluated")
                .is_empty()
        );
    }
}

mod locations {
    use super::*;

    #[test]
    fn the_schema_is_at_the_empty_pointer() {
        assert_eq!(Location::root().to_string(), "");
        assert_eq!(issue().root().location(), &Location::root());
    }

    #[test]
    fn a_location_is_a_json_pointer() {
        let location = Location::root().key("shapes").index(0).key("shapeExpr");
        assert_eq!(location.to_string(), "/shapes/0/shapeExpr");
        assert_eq!(
            location.segments(),
            [
                Segment::Key("shapes".to_string()),
                Segment::Index(0),
                Segment::Key("shapeExpr".to_string())
            ]
        );
        assert_eq!(Location::root().key("a/b~c").to_string(), "/a~1b~0c");
    }

    #[test]
    fn the_parent_location_drops_the_last_step() {
        let location = Location::root().key("shapes").index(0);
        assert_eq!(location.parent(), Some(Location::root().key("shapes")));
        assert_eq!(Location::root().parent(), None);
    }
}

/// `<S1> { <p1> . ; <p2> . ; <p3> . }` and `<S2> { <p4> . ; <p5> . }`, the schema of the
/// reference implementation's `filters-test.ts`.
fn two_shapes() -> ShExJDocument {
    document(
        "BASE <http://a.example/>
         <S1> { <p1> . ; <p2> . ; <p3> . }
         <S2> { <p4> . ; <p5> . }",
    )
}

/// The local names of the predicates of the triple constraints a path selects.
fn predicates(doc: &ShExJDocument, path: &str) -> Vec<String> {
    select(doc, path)
        .iter()
        .map(|item| match item.value()["predicate"].as_str() {
            Some(predicate) => predicate.rsplit(['/', '#']).next().unwrap_or(predicate).to_string(),
            None => panic!("{} has no predicate", item.value()),
        })
        .collect()
}

mod filters {
    use super::*;
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
        assert!(doc.select(&format!("{ISSUE_CONSTRAINTS}[assert count() = 5]")).is_err());
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
}

mod self_parent_and_ancestor {
    use super::*;

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
}
