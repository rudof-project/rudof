//! The ShExJ document a ShExPath is evaluated over.

use rudof_iri::IriS;
use serde_json::{Value, json};
use shex_ast::compact::ShExParser;
use shex_ast::shexpath::shexj;

fn parse(src: &str) -> shex_ast::Schema {
    match ShExParser::parse(src, None, &IriS::new_unchecked("http://default/")) {
        Ok(schema) => schema,
        Err(e) => panic!("the schema is not parsed: {e}"),
    }
}

fn issue_schema() -> shex_ast::Schema {
    parse(include_str!("../fixtures/shexpath/issue.shex"))
}

#[test]
fn the_document_has_no_keys_that_shexj_does_not_define() {
    let doc = shexj(&issue_schema()).expect("the schema is written as ShExJ");
    let mut keys: Vec<&str> = doc
        .as_object()
        .expect("a schema is an object")
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(keys, ["@context", "shapes", "type"]);
}

#[test]
fn the_rest_of_the_document_is_the_schema_as_rudof_writes_it() {
    let schema = issue_schema();
    let mut expected = serde_json::to_value(&schema).expect("the schema is serialized");
    let map = expected.as_object_mut().expect("a schema is an object");
    map.remove("prefixmap");
    map.remove("base");
    assert_eq!(shexj(&schema).expect("the schema is written as ShExJ"), expected);
}

#[test]
fn semantic_actions_have_their_shexj_type() {
    let schema = parse(
        r#"PREFIX ex: <http://ex.example/>
           %ex:start{ start code %}
           ex:S {
             ex:p . %ex:onConstraint{ constraint code %}
           } %ex:onShape{ shape code %}
        "#,
    );
    let doc = shexj(&schema).expect("the schema is written as ShExJ");
    let start_act = &doc["startActs"][0];
    let shape_act = &doc["shapes"][0]["shapeExpr"]["semActs"][0];
    let constraint_act = &doc["shapes"][0]["shapeExpr"]["expression"]["semActs"][0];
    for (act, name) in [
        (start_act, "http://ex.example/start"),
        (shape_act, "http://ex.example/onShape"),
        (constraint_act, "http://ex.example/onConstraint"),
    ] {
        assert_eq!(act["type"], json!("SemAct"), "{act}");
        assert_eq!(act["name"], json!(name), "{act}");
    }
}

#[test]
fn annotations_keep_their_shexj_type() {
    let schema = parse(
        r#"PREFIX ex: <http://ex.example/>
           ex:S { ex:p . // ex:comment "a comment" }
        "#,
    );
    let doc = shexj(&schema).expect("the schema is written as ShExJ");
    let annotation: &Value = &doc["shapes"][0]["shapeExpr"]["expression"]["annotations"][0];
    assert_eq!(annotation["type"], json!("Annotation"), "{annotation}");
}
