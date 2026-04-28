use crate::{
    Rudof, RudofConfig,
    api::data::implementations::load_data::load_data,
    api::data::implementations::serialize_data::serialize_data,
    formats::{DataFormat, DataReaderMode, InputSpec, ResultDataFormat},
};

/// Helper: serialize current data to string
fn serialize_to_string(rudof: &mut Rudof, format: Option<ResultDataFormat>) -> String {
    let mut buffer = Vec::new();

    serialize_data(rudof, format.as_ref(), &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_load_data_rdf_success() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let rdf = InputSpec::str(r#"@prefix ex: <http://example.org/> . ex:alice a ex:Person ."#);

    load_data(
        &mut rudof,
        Some(&[rdf]),
        Some(&DataFormat::Turtle),
        None,
        None,
        Some(&DataReaderMode::Strict),
        Some(false),
    )
    .unwrap();

    let serialized = serialize_to_string(&mut rudof, Some(ResultDataFormat::Turtle));

    assert!(serialized.contains("ex:alice"));
    assert!(serialized.contains("ex:Person"));

    println!(
        "\n===== test_load_data_rdf_success =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_serialize_data_rdf_jsonld() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let rdf = InputSpec::str(r#"@prefix ex: <http://example.org/> . ex:alice a ex:Person ."#);

    load_data(
        &mut rudof,
        Some(&[rdf]),
        Some(&DataFormat::Turtle),
        None,
        None,
        Some(&DataReaderMode::Strict),
        Some(false),
    )
    .unwrap();

    let serialized = serialize_to_string(&mut rudof, Some(ResultDataFormat::JsonLd));

    let parsed = serde_json::from_str::<serde_json::Value>(&serialized).expect("jsonld output must be valid JSON");

    assert!(parsed.is_object() || parsed.is_array());
    assert!(serialized.contains("http://example.org/alice"));
}

#[test]
fn test_serialize_data_rdf_json_alias() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let rdf = InputSpec::str(r#"@prefix ex: <http://example.org/> . ex:alice a ex:Person ."#);

    load_data(
        &mut rudof,
        Some(&[rdf]),
        Some(&DataFormat::Turtle),
        None,
        None,
        Some(&DataReaderMode::Strict),
        Some(false),
    )
    .unwrap();

    let serialized = serialize_to_string(&mut rudof, Some(ResultDataFormat::Json));

    let parsed = serde_json::from_str::<serde_json::Value>(&serialized).expect("json output must be valid JSON");

    assert!(parsed.is_object() || parsed.is_array());
    assert!(serialized.contains("http://example.org/alice"));
}

#[test]
fn test_load_data_rdf_merge() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let data1 = InputSpec::str(r#"@prefix ex: <http://example.org/> . ex:a a ex:Person ."#);

    let data2 = InputSpec::str(r#"@prefix ex: <http://example.org/> . ex:b a ex:Person ."#);

    load_data(
        &mut rudof,
        Some(&[data1]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        Some(false),
    )
    .unwrap();

    load_data(
        &mut rudof,
        Some(&[data2]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        Some(true),
    )
    .unwrap();

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

    let data1 = InputSpec::str(r#"@prefix ex: <http://example.org/> . ex:a a ex:Person ."#);

    let data2 = InputSpec::str(r#"@prefix ex: <http://example.org/> . ex:b a ex:Person ."#);

    load_data(
        &mut rudof,
        Some(&[data1]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        Some(false),
    )
    .unwrap();

    load_data(
        &mut rudof,
        Some(&[data2]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        Some(false),
    )
    .unwrap();

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

    let invalid = InputSpec::str("not valid rdf");

    let result = load_data(
        &mut rudof,
        Some(&[invalid]),
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

    let pg_data = InputSpec::str(
        r#"
        (alice {Person} [ name: "Alice", age: 23, aliases: "Ally" ])
        (bob   {Person} [ name: "Robert", aliases: ["Bob", "Bobby"] ])
        "#,
    );

    load_data(
        &mut rudof,
        Some(&[pg_data]),
        Some(&DataFormat::Pg),
        None,
        None,
        None,
        Some(false),
    )
    .unwrap();

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

    let pg1 = InputSpec::str(r#"(alice {Person} [ name: "Alice" ])"#);

    let pg2 = InputSpec::str(r#"(bob {Person} [ name: "Bob" ])"#);

    load_data(
        &mut rudof,
        Some(&[pg1]),
        Some(&DataFormat::Pg),
        None,
        None,
        None,
        Some(true),
    )
    .unwrap();

    load_data(
        &mut rudof,
        Some(&[pg2]),
        Some(&DataFormat::Pg),
        None,
        None,
        None,
        Some(true),
    )
    .unwrap();

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

    let pg1 = InputSpec::str(r#"(alice {Person} [ name: "Alice" ])"#);

    let pg2 = InputSpec::str(r#"(bob {Person} [ name: "Bob" ])"#);

    load_data(
        &mut rudof,
        Some(&[pg1]),
        Some(&DataFormat::Pg),
        None,
        None,
        None,
        Some(false),
    )
    .unwrap();

    load_data(
        &mut rudof,
        Some(&[pg2]),
        Some(&DataFormat::Pg),
        None,
        None,
        None,
        Some(false),
    )
    .unwrap();

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

    let invalid_pg = InputSpec::str(r#"(alice {Person} name: "missing brackets")"#);

    let result = load_data(
        &mut rudof,
        Some(&[invalid_pg]),
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
    )
    .unwrap();

    assert!(rudof.data.is_some());
    assert!(rudof.data.as_ref().unwrap().is_rdf());
}

#[test]
fn test_load_data_endpoint_uses_configured_prefixmap_for_known_endpoint() {
    let mut rudof = Rudof::new(RudofConfig::default());

    load_data(
        &mut rudof,
        None,
        None,
        None,
        Some("https://dbpedia.org/sparql"),
        None,
        None,
    )
    .unwrap();

    let endpoint = rudof
        .data
        .as_mut()
        .unwrap()
        .unwrap_rdf_mut()
        .use_endpoints()
        .get("https://dbpedia.org/sparql")
        .expect("Configured DBpedia endpoint should be active");

    assert!(endpoint.prefixmap().resolve_prefix_local("dbr", "Oviedo").is_ok());
    assert!(endpoint.prefixmap().resolve_prefix_local("foaf", "depiction").is_ok());
}

#[test]
fn test_load_data_conflicting_sources() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let rdf = InputSpec::str(r#"@prefix ex: <http://example.org/> . ex:a a ex:Person ."#);

    let result = load_data(
        &mut rudof,
        Some(&[rdf]),
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

    let result = load_data(&mut rudof, None, None, None, None, None, None);

    assert!(result.is_err());
}
