use crate::{
    Rudof, RudofConfig,
    api::materialize::implementations::materialize::materialize,
    formats::{InputSpec, ResultDataFormat, ShExFormat},
};
use iri_s::{IriS, iri};
use shex_ast::Node;
use shex_ast::ir::map_state::MapState;

/// Helper: materialize the current rudof state to a string.
fn materialize_to_string(
    rudof: &Rudof,
    initial_node_iri: Option<&str>,
    result_format: Option<&ResultDataFormat>,
) -> String {
    let mut buffer = Vec::new();
    materialize(rudof, initial_node_iri, result_format, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

/// Helper: build a `Rudof` with a ShEx-JSON schema and a populated map state.
fn setup_rudof(schema_json: &str, map_entries: Vec<(&str, Node)>) -> Rudof {
    let mut rudof = Rudof::new(RudofConfig::default());

    let input = InputSpec::Str(schema_json.to_string());
    rudof
        .load_shex_schema(&input)
        .with_shex_schema_format(&ShExFormat::ShExJ)
        .execute()
        .expect("failed to load ShEx schema");

    let mut map_state = MapState::default();
    for (iri_str, node) in map_entries {
        map_state.insert(IriS::new_unchecked(iri_str), node);
    }
    rudof.map_state = Some(map_state);

    rudof
}

// -----------------------------------------------------------------------
// Test 1: materialize fails when no ShEx schema is loaded
// -----------------------------------------------------------------------

#[test]
fn test_materialize_fails_without_shex_schema() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let mut map_state = MapState::default();
    map_state.insert(
        IriS::new_unchecked("http://example.org/name"),
        Node::iri(iri!("http://example.org/Alice")),
    );
    rudof.map_state = Some(map_state);

    let mut buffer = Vec::new();
    let result = materialize(&rudof, None, None, &mut buffer);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("ShEx schema"));
}

// -----------------------------------------------------------------------
// Test 2: materialize fails when no map state is loaded
// -----------------------------------------------------------------------

#[test]
fn test_materialize_fails_without_map_state() {
    let schema_json = r#"{
      "@context": "http://www.w3.org/ns/shex.jsonld",
      "type": "Schema",
      "shapes": [{
        "type": "ShapeDecl",
        "id": "http://example.org/S",
        "shapeExpr": {
          "type": "Shape",
          "expression": {
            "type": "TripleConstraint",
            "predicate": "http://example.org/p",
            "semActs": [{
              "type": "SemAct",
              "name": "http://shex.io/extensions/Map/",
              "code": "<http://example.org/p>"
            }]
          }
        }
      }]
    }"#;

    let mut rudof = Rudof::new(RudofConfig::default());
    let input = InputSpec::Str(schema_json.to_string());
    rudof
        .load_shex_schema(&input)
        .with_shex_schema_format(&ShExFormat::ShExJ)
        .execute()
        .unwrap();
    // map_state intentionally left as None

    let mut buffer = Vec::new();
    let result = materialize(&rudof, None, None, &mut buffer);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("MapState"));
}

// -----------------------------------------------------------------------
// Test 3: single leaf property materializes to one triple
// -----------------------------------------------------------------------

#[test]
fn test_materialize_single_leaf_property() {
    let schema_json = r#"{
      "@context": "http://www.w3.org/ns/shex.jsonld",
      "type": "Schema",
      "shapes": [{
        "type": "ShapeDecl",
        "id": "http://example.org/PersonShape",
        "shapeExpr": {
          "type": "Shape",
          "expression": {
            "type": "TripleConstraint",
            "predicate": "http://example.org/name",
            "semActs": [{
              "type": "SemAct",
              "name": "http://shex.io/extensions/Map/",
              "code": "<http://example.org/name>"
            }]
          }
        }
      }]
    }"#;

    let rudof = setup_rudof(
        schema_json,
        vec![("http://example.org/name", Node::iri(iri!("http://example.org/Alice")))],
    );

    let output = materialize_to_string(&rudof, None, Some(&ResultDataFormat::NTriples));

    assert!(output.contains("<http://example.org/name>"), "missing predicate: {output}");
    assert!(output.contains("<http://example.org/Alice>"), "missing object: {output}");

    println!("\n===== test_materialize_single_leaf_property =====\n{output}\n=====");
}

// -----------------------------------------------------------------------
// Test 4: explicit initial IRI used as the root subject
// -----------------------------------------------------------------------

#[test]
fn test_materialize_with_explicit_initial_iri() {
    let schema_json = r#"{
      "@context": "http://www.w3.org/ns/shex.jsonld",
      "type": "Schema",
      "shapes": [{
        "type": "ShapeDecl",
        "id": "http://example.org/S",
        "shapeExpr": {
          "type": "Shape",
          "expression": {
            "type": "TripleConstraint",
            "predicate": "http://example.org/label",
            "semActs": [{
              "type": "SemAct",
              "name": "http://shex.io/extensions/Map/",
              "code": "<http://example.org/label>"
            }]
          }
        }
      }]
    }"#;

    let rudof = setup_rudof(
        schema_json,
        vec![("http://example.org/label", Node::iri(iri!("http://example.org/Thing")))],
    );

    let output = materialize_to_string(
        &rudof,
        Some("http://example.org/Bob"),
        Some(&ResultDataFormat::NTriples),
    );

    assert!(output.contains("<http://example.org/Bob>"), "missing subject: {output}");
    assert!(output.contains("<http://example.org/Thing>"), "missing object: {output}");

    println!("\n===== test_materialize_with_explicit_initial_iri =====\n{output}\n=====");
}

// -----------------------------------------------------------------------
// Test 5: EachOf with two properties materializes to two triples
// -----------------------------------------------------------------------

#[test]
fn test_materialize_two_leaf_properties() {
    let schema_json = r#"{
      "@context": "http://www.w3.org/ns/shex.jsonld",
      "type": "Schema",
      "shapes": [{
        "type": "ShapeDecl",
        "id": "http://example.org/PersonShape",
        "shapeExpr": {
          "type": "Shape",
          "expression": {
            "type": "EachOf",
            "expressions": [
              {
                "type": "TripleConstraint",
                "predicate": "http://example.org/name",
                "semActs": [{
                  "type": "SemAct",
                  "name": "http://shex.io/extensions/Map/",
                  "code": "<http://example.org/name>"
                }]
              },
              {
                "type": "TripleConstraint",
                "predicate": "http://example.org/age",
                "semActs": [{
                  "type": "SemAct",
                  "name": "http://shex.io/extensions/Map/",
                  "code": "<http://example.org/age>"
                }]
              }
            ]
          }
        }
      }]
    }"#;

    let rudof = setup_rudof(
        schema_json,
        vec![
            ("http://example.org/name", Node::iri(iri!("http://example.org/Alice"))),
            ("http://example.org/age", Node::iri(iri!("http://example.org/age30"))),
        ],
    );

    let output = materialize_to_string(&rudof, None, Some(&ResultDataFormat::NTriples));

    assert!(output.contains("<http://example.org/name>"), "missing name: {output}");
    assert!(output.contains("<http://example.org/Alice>"), "missing Alice: {output}");
    assert!(output.contains("<http://example.org/age>"), "missing age: {output}");
    assert!(output.contains("<http://example.org/age30>"), "missing age30: {output}");

    println!("\n===== test_materialize_two_leaf_properties =====\n{output}\n=====");
}

// -----------------------------------------------------------------------
// Test 6: invalid IRI string returns an error
// -----------------------------------------------------------------------

#[test]
fn test_materialize_invalid_initial_iri_returns_error() {
    let schema_json = r#"{
      "@context": "http://www.w3.org/ns/shex.jsonld",
      "type": "Schema",
      "shapes": [{
        "type": "ShapeDecl",
        "id": "http://example.org/S",
        "shapeExpr": {
          "type": "Shape",
          "expression": {
            "type": "TripleConstraint",
            "predicate": "http://example.org/p",
            "semActs": [{
              "type": "SemAct",
              "name": "http://shex.io/extensions/Map/",
              "code": "<http://example.org/p>"
            }]
          }
        }
      }]
    }"#;

    let rudof = setup_rudof(
        schema_json,
        vec![("http://example.org/p", Node::iri(iri!("http://example.org/val")))],
    );

    let mut buffer = Vec::new();
    let result = materialize(&rudof, Some("not a valid IRI !!!"), None, &mut buffer);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("IRI"));
}

// -----------------------------------------------------------------------
// Test 7: default format (Turtle) is used when no format is specified
// -----------------------------------------------------------------------

#[test]
fn test_materialize_default_format_is_turtle() {
    let schema_json = r#"{
      "@context": "http://www.w3.org/ns/shex.jsonld",
      "type": "Schema",
      "shapes": [{
        "type": "ShapeDecl",
        "id": "http://example.org/S",
        "shapeExpr": {
          "type": "Shape",
          "expression": {
            "type": "TripleConstraint",
            "predicate": "http://example.org/label",
            "semActs": [{
              "type": "SemAct",
              "name": "http://shex.io/extensions/Map/",
              "code": "<http://example.org/label>"
            }]
          }
        }
      }]
    }"#;

    let rudof = setup_rudof(
        schema_json,
        vec![("http://example.org/label", Node::iri(iri!("http://example.org/Thing")))],
    );

    let output = materialize_to_string(&rudof, None, None);
    assert!(!output.is_empty(), "expected non-empty Turtle output");

    println!("\n===== test_materialize_default_format_is_turtle =====\n{output}\n=====");
}

// -----------------------------------------------------------------------
// Test 8: through the public builder API on Rudof
// -----------------------------------------------------------------------

#[test]
fn test_materialize_via_rudof_builder() {
    let schema_json = r#"{
      "@context": "http://www.w3.org/ns/shex.jsonld",
      "type": "Schema",
      "shapes": [{
        "type": "ShapeDecl",
        "id": "http://example.org/S",
        "shapeExpr": {
          "type": "Shape",
          "expression": {
            "type": "TripleConstraint",
            "predicate": "http://example.org/p",
            "semActs": [{
              "type": "SemAct",
              "name": "http://shex.io/extensions/Map/",
              "code": "<http://example.org/p>"
            }]
          }
        }
      }]
    }"#;

    let rudof = setup_rudof(
        schema_json,
        vec![("http://example.org/p", Node::iri(iri!("http://example.org/Value")))],
    );

    let mut buffer = Vec::new();
    rudof
        .materialize(&mut buffer)
        .with_result_format(&ResultDataFormat::NTriples)
        .execute()
        .expect("materialization should succeed");

    let output = String::from_utf8(buffer).unwrap();
    assert!(output.contains("<http://example.org/Value>"), "missing value: {output}");

    println!("\n===== test_materialize_via_rudof_builder =====\n{output}\n=====");
}

