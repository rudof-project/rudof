//! The JSON Schema of a term has to describe what serde actually writes: the two are
//! derived separately, and the leaf types whose `Serialize` is hand-written are pinned
//! to a shape with `#[schemars(with = ...)]` that nothing else enforces.

use crate::rdf_core::term::{IriOrBlankNode, Object, literal::ConcreteLiteral};
use rudof_iri::IriS;
use serde_json::{Value, json};

/// The `$defs` entry, or the root schema, describing `value`'s variant.
fn variant_schema(schema: &Value, value: &Value) -> Value {
    let tag = value
        .as_object()
        .expect("a term serializes as an externally tagged object")
        .keys()
        .next()
        .expect("the tag names the variant")
        .clone();
    schema
        .get("oneOf")
        .and_then(Value::as_array)
        .expect("an externally tagged enum schemas as a oneOf")
        .iter()
        .find(|branch| branch.pointer(&format!("/properties/{tag}")).is_some())
        .unwrap_or_else(|| panic!("the schema should describe the '{tag}' variant"))
        .clone()
}

#[test]
fn every_object_variant_is_in_the_schema() {
    let schema = serde_json::to_value(schemars::schema_for!(Object)).unwrap();

    let objects = [
        Object::iri(IriS::new_unchecked("http://example.org/a")),
        Object::bnode("b0".to_string()),
        Object::str("Alice"),
        Object::Triple {
            subject: Box::new(IriOrBlankNode::Iri(IriS::new_unchecked("http://example.org/s"))),
            predicate: IriS::new_unchecked("http://example.org/p"),
            object: Box::new(Object::str("o")),
        },
    ];

    for object in objects {
        let value = serde_json::to_value(&object).unwrap();
        // Panics when the variant serde writes is missing from the schema.
        variant_schema(&schema, &value);
    }
}

#[test]
fn an_iri_is_a_plain_string_in_both() {
    let object = Object::iri(IriS::new_unchecked("http://example.org/a"));

    assert_eq!(
        serde_json::to_value(&object).unwrap(),
        json!({"Iri": "http://example.org/a"})
    );

    let schema = serde_json::to_value(schemars::schema_for!(Object)).unwrap();
    let iri = variant_schema(&schema, &serde_json::to_value(&object).unwrap());
    assert_eq!(iri.pointer("/properties/Iri/type").unwrap(), "string");
}

#[test]
fn a_boolean_literal_is_a_string_in_both() {
    // `serialize_boolean_literal` writes "true"/"false", not a JSON boolean, so the
    // schema says string where the Rust variant holds a `bool`.
    let literal = ConcreteLiteral::boolean(true);

    assert_eq!(
        serde_json::to_value(&literal).unwrap(),
        json!({"BooleanLiteral": "true"})
    );

    let schema = serde_json::to_value(schemars::schema_for!(ConcreteLiteral)).unwrap();
    let boolean = variant_schema(&schema, &serde_json::to_value(&literal).unwrap());
    assert_eq!(boolean.pointer("/properties/BooleanLiteral/type").unwrap(), "string");
}
