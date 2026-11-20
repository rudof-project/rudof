//! Evaluation of ShExPaths against the loaded ShEx schema.

use crate::{
    Rudof, RudofConfig,
    api::shex::implementations::{evaluate_shex_path::evaluate_shex_path, load_shex_schema::load_shex_schema},
    errors::{RudofError, ShExError},
    formats::{InputSpec, ShExFormat},
};
use serde_json::json;

/// Part of the example schema of the ShExPath spec.
const ISSUE_SCHEMA: &str = r#"
PREFIX : <http://ex.example/ns#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
BASE <http://ex.example/schema>

<#IssueShape> {
  :name xsd:string MINLENGTH 4 ;
  :category ["bug" "feature request"] ;
  $<#reportedBy> :postedBy @<#UserShape>
}

<#UserShape> IRI
"#;

fn rudof_with_schema() -> Rudof {
    let mut rudof = Rudof::new(RudofConfig::default());
    load_shex_schema(
        &mut rudof,
        &InputSpec::str(ISSUE_SCHEMA),
        Some(&ShExFormat::ShExC),
        None,
        None,
    )
    .expect("the schema is loaded");
    rudof
}

#[test]
fn test_evaluate_shex_path_selects_with_the_schema_prefixes_and_base() {
    let rudof = rudof_with_schema();
    let matches = evaluate_shex_path(&rudof, "@<#IssueShape>~:category").expect("the path is evaluated");
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].location, "/shapes/0/shapeExpr/expression/expressions/1");
    assert_eq!(matches[0].value["predicate"], json!("http://ex.example/ns#category"));
}

#[test]
fn test_evaluate_shex_path_keeps_the_order_and_the_duplicates() {
    let rudof = rudof_with_schema();
    let matches = evaluate_shex_path(&rudof, "@<#IssueShape>~:name , $<#reportedBy> , @<#IssueShape>~:name")
        .expect("the path is evaluated");
    let locations: Vec<&str> = matches.iter().map(|m| m.location.as_str()).collect();
    assert_eq!(
        locations,
        [
            "/shapes/0/shapeExpr/expression/expressions/0",
            "/shapes/0/shapeExpr/expression/expressions/2",
            "/shapes/0/shapeExpr/expression/expressions/0"
        ]
    );
}

#[test]
fn test_evaluate_shex_path_selects_atomic_values() {
    let rudof = rudof_with_schema();
    let matches = evaluate_shex_path(&rudof, "$<#reportedBy>/valueExpr").expect("the path is evaluated");
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].value, json!("http://ex.example/schema#UserShape"));
}

#[test]
fn test_evaluate_shex_path_selecting_nothing_is_not_an_error() {
    let rudof = rudof_with_schema();
    assert!(
        evaluate_shex_path(&rudof, "@<#Nope>")
            .expect("the path is evaluated")
            .is_empty()
    );
}

#[test]
fn test_evaluate_shex_path_without_a_schema_fails() {
    let rudof = Rudof::new(RudofConfig::default());
    let err = evaluate_shex_path(&rudof, "/shapes/*").unwrap_err();
    assert!(
        matches!(err, RudofError::ShEx(ShExError::NoShExSchemaLoaded)),
        "{err:?}"
    );
}

#[test]
fn test_evaluate_shex_path_with_a_malformed_path_fails() {
    let rudof = rudof_with_schema();
    let err = evaluate_shex_path(&rudof, "/shapes/asdf").unwrap_err();
    assert!(
        matches!(err, RudofError::ShEx(ShExError::FailedEvaluatingShExPath { .. })),
        "{err:?}"
    );
}

#[test]
fn test_evaluate_shex_path_with_a_failed_assertion_fails() {
    let rudof = rudof_with_schema();
    let err = evaluate_shex_path(&rudof, "@<#IssueShape>/expression/expressions/*[assert count() = 1]").unwrap_err();
    assert!(
        matches!(err, RudofError::ShEx(ShExError::FailedEvaluatingShExPath { .. })),
        "{err:?}"
    );
    assert!(err.to_string().contains("assertion"), "{err}");
}

#[test]
fn test_evaluate_shex_path_through_the_builder() {
    let rudof = rudof_with_schema();
    let matches = rudof
        .evaluate_shex_path("/shapes/*")
        .execute()
        .expect("the path is evaluated");
    assert_eq!(matches.len(), 2);
}
