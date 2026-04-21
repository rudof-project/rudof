use crate::{
    Rudof, RudofConfig, api::query::implementations::load_query::load_query,
    api::query::implementations::serialize_query::serialize_query, formats::InputSpec,
};
use std::str::FromStr;

/// Helper: serialize current query to string
fn serialize_to_string(rudof: &Rudof) -> String {
    let mut buffer = Vec::new();

    serialize_query(rudof, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_load_and_serialize_basic_select_query() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let query_input = InputSpec::from_str(
        r#"
PREFIX ex: <http://example.org/>
PREFIX schema: <http://schema.org/>

SELECT ?person ?name
WHERE {
    ?person a ex:Person .
    ?person schema:name ?name .
}
        "#,
    )
    .unwrap();

    load_query(&mut rudof, &query_input, None).unwrap();

    let serialized = serialize_to_string(&rudof);

    assert!(serialized.contains("SELECT") || serialized.to_uppercase().contains("SELECT"));
    assert!(serialized.contains("?person"));
    assert!(serialized.contains("?name"));
    assert!(serialized.contains("WHERE") || serialized.to_uppercase().contains("WHERE"));

    println!(
        "\n===== test_load_and_serialize_basic_select_query =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_load_and_serialize_construct_query() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let query_input = InputSpec::from_str(
        r#"
PREFIX ex: <http://example.org/>
PREFIX foaf: <http://xmlns.com/foaf/0.1/>

CONSTRUCT {
    ?person foaf:name ?name .
    ?person foaf:age ?age .
}
WHERE {
    ?person a ex:Person .
    ?person ex:name ?name .
    ?person ex:age ?age .
}
        "#,
    )
    .unwrap();

    load_query(&mut rudof, &query_input, None).unwrap();

    let serialized = serialize_to_string(&rudof);

    assert!(serialized.contains("CONSTRUCT") || serialized.to_uppercase().contains("CONSTRUCT"));
    assert!(serialized.contains("?person"));
    assert!(serialized.contains("foaf:name") || serialized.contains("name"));

    println!(
        "\n===== test_load_and_serialize_construct_query =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_load_and_serialize_query_with_filter() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let query_input = InputSpec::from_str(
        r#"
PREFIX ex: <http://example.org/>
PREFIX schema: <http://schema.org/>

SELECT ?person ?age
WHERE {
    ?person a ex:Person .
    ?person schema:age ?age .
    FILTER (?age > 18)
}
        "#,
    )
    .unwrap();

    load_query(&mut rudof, &query_input, None).unwrap();

    let serialized = serialize_to_string(&rudof);

    assert!(serialized.contains("SELECT") || serialized.to_uppercase().contains("SELECT"));
    assert!(serialized.contains("?age"));
    assert!(serialized.contains("FILTER") || serialized.to_uppercase().contains("FILTER"));

    println!(
        "\n===== test_load_and_serialize_query_with_filter =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_load_and_serialize_query_with_optional() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let query_input = InputSpec::from_str(
        r#"
PREFIX ex: <http://example.org/>
PREFIX schema: <http://schema.org/>

SELECT ?person ?name ?email
WHERE {
    ?person a ex:Person .
    ?person schema:name ?name .
    OPTIONAL {
        ?person schema:email ?email .
    }
}
        "#,
    )
    .unwrap();

    load_query(&mut rudof, &query_input, None).unwrap();

    let serialized = serialize_to_string(&rudof);

    assert!(serialized.contains("SELECT") || serialized.to_uppercase().contains("SELECT"));
    assert!(serialized.contains("?person"));
    assert!(serialized.contains("OPTIONAL") || serialized.to_uppercase().contains("OPTIONAL"));

    println!(
        "\n===== test_load_and_serialize_query_with_optional =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_load_and_serialize_ask_query() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let query_input = InputSpec::from_str(
        r#"
PREFIX ex: <http://example.org/>
PREFIX schema: <http://schema.org/>

ASK {
    ?person a ex:Person .
    ?person schema:name "Alice" .
}
        "#,
    )
    .unwrap();

    load_query(&mut rudof, &query_input, None).unwrap();

    let serialized = serialize_to_string(&rudof);

    assert!(serialized.contains("ASK") || serialized.to_uppercase().contains("ASK"));
    assert!(serialized.contains("Person") || serialized.contains("ex:Person"));

    println!(
        "\n===== test_load_and_serialize_ask_query =====\n{}\n============================================",
        serialized
    );
}
