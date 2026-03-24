use crate::{
    Rudof, RudofConfig,
    formats::{InputSpec, DataFormat, DataReaderMode, ResultDataFormat},
    api::data::implementations::load_data::load_data,
    api::data::implementations::serialize_data::serialize_data,
};
use std::str::FromStr;

/// Helper: serialize current data to string
fn serialize_to_string(rudof: &mut Rudof, format: Option<ResultDataFormat>) -> String {
    let mut buffer = Vec::new();

    serialize_data(
        rudof,
        format.as_ref(),
        &mut buffer,
    ).unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_load_data_rdf_success() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let rdf = InputSpec::from_str(
        r#"@prefix ex: <http://example.org/> . ex:alice a ex:Person ."#
    ).unwrap();

    load_data(
        &mut rudof,
        Some(&vec![rdf]),
        Some(&DataFormat::Turtle),
        None,
        None,
        Some(&DataReaderMode::Strict),
        Some(false),
    ).unwrap();

    let serialized = serialize_to_string(&mut rudof, Some(ResultDataFormat::Turtle));

    assert!(serialized.contains("ex:alice"));
    assert!(serialized.contains("ex:Person"));

    println!(
        "\n===== test_load_data_rdf_success =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_load_data_rdf_merge() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let data1 = InputSpec::from_str(
        r#"@prefix ex: <http://example.org/> . ex:a a ex:Person ."#
    ).unwrap();

    let data2 = InputSpec::from_str(
        r#"@prefix ex: <http://example.org/> . ex:b a ex:Person ."#
    ).unwrap();

    load_data(
        &mut rudof,
        Some(&vec![data1]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        Some(false),
    ).unwrap();

    load_data(
        &mut rudof,
        Some(&vec![data2]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        Some(true),
    ).unwrap();

    let serialized = serialize_to_string(&mut rudof, Some(ResultDataFormat::Turtle));

    assert!(serialized.contains("ex:a"));
    assert!(serialized.contains("ex:b"));

    println!(
        "\n===== test_load_data_rdf_merge =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_load_data_rdf_replace() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let data1 = InputSpec::from_str(
        r#"@prefix ex: <http://example.org/> . ex:a a ex:Person ."#
    ).unwrap();

    let data2 = InputSpec::from_str(
        r#"@prefix ex: <http://example.org/> . ex:b a ex:Person ."#
    ).unwrap();

    load_data(
        &mut rudof,
        Some(&vec![data1]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        Some(false),
    ).unwrap();

    load_data(
        &mut rudof,
        Some(&vec![data2]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        Some(false),
    ).unwrap();

    let serialized = serialize_to_string(&mut rudof, Some(ResultDataFormat::Turtle));

    assert!(!serialized.contains("ex:a"));
    assert!(serialized.contains("ex:b"));

    println!(
        "\n===== test_load_data_rdf_replace =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_load_data_invalid_rdf() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let invalid = InputSpec::from_str("not valid rdf").unwrap();

    let result = load_data(
        &mut rudof,
        Some(&vec![invalid]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
    );

    assert!(result.is_err());
}

#[test]
fn test_load_data_pg_success() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let pg_data = InputSpec::from_str(
        r#"
        (alice {Person} [ name: "Alice", age: 23, aliases: "Ally" ])
        (bob   {Person} [ name: "Robert", aliases: ["Bob", "Bobby"] ])
        "#
    ).unwrap();

    load_data(
        &mut rudof,
        Some(&vec![pg_data]),
        Some(&DataFormat::Pg),
        None,
        None,
        None,
        Some(false),
    ).unwrap();

    let serialized = serialize_to_string(&mut rudof, None);

    assert!(serialized.contains("alice"));
    assert!(serialized.contains("bob"));
    assert!(serialized.contains("Person"));
    assert!(serialized.contains("Alice"));
    assert!(serialized.contains("Robert"));

    println!(
        "\n===== test_load_data_pg_success =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_load_data_pg_merge() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let pg1 = InputSpec::from_str(
        r#"(alice {Person} [ name: "Alice" ])"#
    ).unwrap();

    let pg2 = InputSpec::from_str(
        r#"(bob {Person} [ name: "Bob" ])"#
    ).unwrap();

    load_data(
        &mut rudof,
        Some(&vec![pg1]),
        Some(&DataFormat::Pg),
        None,
        None,
        None,
        Some(true),
    ).unwrap();

    load_data(
        &mut rudof,
        Some(&vec![pg2]),
        Some(&DataFormat::Pg),
        None,
        None,
        None,
        Some(true),
    ).unwrap();

    let serialized = serialize_to_string(&mut rudof, None);

    assert!(serialized.contains("alice"));
    assert!(serialized.contains("bob"));

    println!(
        "\n===== test_load_data_pg_merge =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_load_data_pg_replace() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let pg1 = InputSpec::from_str(
        r#"(alice {Person} [ name: "Alice" ])"#
    ).unwrap();

    let pg2 = InputSpec::from_str(
        r#"(bob {Person} [ name: "Bob" ])"#
    ).unwrap();

    load_data(
        &mut rudof,
        Some(&vec![pg1]),
        Some(&DataFormat::Pg),
        None,
        None,
        None,
        Some(false),
    ).unwrap();

    load_data(
        &mut rudof,
        Some(&vec![pg2]),
        Some(&DataFormat::Pg),
        None,
        None,
        None,
        Some(false),
    ).unwrap();

    let serialized = serialize_to_string(&mut rudof, None);

    assert!(!serialized.contains("alice"));
    assert!(serialized.contains("bob"));

    println!(
        "\n===== test_load_data_pg_replace =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_load_data_pg_invalid_syntax() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let invalid_pg = InputSpec::from_str(
        r#"(alice {Person} name: "missing brackets")"#
    ).unwrap();

    let result = load_data(
        &mut rudof,
        Some(&vec![invalid_pg]),
        Some(&DataFormat::Pg),
        None,
        None,
        None,
        None,
    );

    assert!(result.is_err());
}

#[test]
fn test_load_data_endpoint_success() {
    let mut rudof = Rudof::new(RudofConfig::default());

    load_data(
        &mut rudof,
        None,
        None,
        None,
        Some("http://example.org/sparql"),
        None,
        None,
    ).unwrap();

    assert!(rudof.data.is_some());
    assert!(rudof.data.as_ref().unwrap().is_rdf());
}

#[test]
fn test_load_data_conflicting_sources() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let rdf = InputSpec::from_str(
        r#"@prefix ex: <http://example.org/> . ex:a a ex:Person ."#
    ).unwrap();

    let result = load_data(
        &mut rudof,
        Some(&vec![rdf]),
        Some(&DataFormat::Turtle),
        None,
        Some("http://example.org/sparql"),
        None,
        None,
    );

    assert!(result.is_err());
}

#[test]
fn test_load_data_no_source() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let result = load_data(
        &mut rudof,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    assert!(result.is_err());
}