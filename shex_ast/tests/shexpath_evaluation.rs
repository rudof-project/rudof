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
