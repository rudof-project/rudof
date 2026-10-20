#[cfg(feature = "qlever")]
use crate::errors::DataError;
use crate::{
    Rudof, RudofConfig,
    api::data::implementations::load_data::load_data,
    api::data::implementations::serialize_data::serialize_data,
    formats::{DataFormat, DataReaderMode, InputSpec, ResultDataFormat},
};
use rudof_rdf::rdf_impl::EndpointStrategy;

/// Helper: serialize current data to string
fn serialize_to_string(rudof: &mut Rudof, format: Option<ResultDataFormat>) -> String {
    let mut buffer = Vec::new();

    serialize_data(rudof, format.as_ref(), None, &mut buffer).unwrap();

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
        None,
        EndpointStrategy::default(),
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
        None,
        EndpointStrategy::default(),
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
        None,
        EndpointStrategy::default(),
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
        None,
        EndpointStrategy::default(),
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
        None,
        EndpointStrategy::default(),
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
        None,
        EndpointStrategy::default(),
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
        None,
        EndpointStrategy::default(),
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
        None,
        EndpointStrategy::default(),
    );

    assert!(result.is_err());
}

#[cfg(feature = "pgschema")]
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
        None,
        EndpointStrategy::default(),
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

#[cfg(feature = "pgschema")]
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
        None,
        EndpointStrategy::default(),
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
        None,
        EndpointStrategy::default(),
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

#[cfg(feature = "pgschema")]
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
        None,
        EndpointStrategy::default(),
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
        None,
        EndpointStrategy::default(),
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

#[cfg(feature = "pgschema")]
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
        None,
        EndpointStrategy::default(),
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
        None,
        EndpointStrategy::default(),
    )
    .unwrap();

    assert!(rudof.data.is_some());
    assert!(rudof.data.as_ref().unwrap().is_rdf());
}

// SPARQL endpoints are only available natively.
#[cfg(not(target_family = "wasm"))]
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
        None,
        EndpointStrategy::default(),
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

/// The minimum request interval of the active endpoint registered under `name`.
#[cfg(not(target_family = "wasm"))]
fn active_endpoint_interval(rudof: &mut Rudof, name: &str) -> u64 {
    rudof
        .data
        .as_mut()
        .unwrap()
        .unwrap_rdf_mut()
        .use_endpoints()
        .get(name)
        .expect("endpoint should be active")
        .min_request_interval_ms()
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn test_load_data_endpoint_min_request_interval_defaults() {
    let mut rudof = Rudof::new(RudofConfig::default());
    rudof
        .load_data()
        .with_endpoint("https://example.org/sparql")
        .execute()
        .unwrap();
    assert_eq!(active_endpoint_interval(&mut rudof, "https://example.org/sparql"), 1100);

    rudof
        .load_data()
        .with_endpoint("http://localhost:7878/query")
        .execute()
        .unwrap();
    assert_eq!(active_endpoint_interval(&mut rudof, "http://localhost:7878/query"), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn test_load_data_endpoint_min_request_interval_from_config() {
    let config = RudofConfig::default();
    let rdf_data = config
        .rdf_data()
        .clone()
        .with_endpoint_limits(rudof_rdf::rdf_core::EndpointLimits::default().with_min_request_interval_ms(Some(5)));
    let mut rudof = Rudof::new(config.with_rdf_data(rdf_data));

    // An endpoint given by URL takes the config-wide default...
    rudof
        .load_data()
        .with_endpoint("https://example.org/sparql")
        .execute()
        .unwrap();
    assert_eq!(active_endpoint_interval(&mut rudof, "https://example.org/sparql"), 5);

    // ...but a built-in public endpoint keeps its own interval.
    rudof
        .load_data()
        .with_endpoint("https://dbpedia.org/sparql")
        .execute()
        .unwrap();
    assert_eq!(active_endpoint_interval(&mut rudof, "https://dbpedia.org/sparql"), 1100);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn test_load_data_endpoint_min_request_interval_override() {
    let mut rudof = Rudof::new(RudofConfig::default());
    rudof
        .load_data()
        .with_endpoint("wikidata")
        .with_endpoint_min_request_interval_ms(Some(0))
        .execute()
        .unwrap();
    assert_eq!(active_endpoint_interval(&mut rudof, "wikidata"), 0);
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
        None,
        EndpointStrategy::default(),
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
        None,
        EndpointStrategy::default(),
    );

    assert!(result.is_err());
}

#[cfg(feature = "qlever")]
#[test]
fn test_load_data_via_qlever_rejects_pg_format() {
    use crate::api::data::implementations::load_data::load_data_via_qlever;
    use crate::errors::RudofError;
    use std::path::PathBuf;

    let mut rudof = Rudof::new(RudofConfig::default());
    let path = InputSpec::Path(PathBuf::from("examples/whatever.pg"));

    let err = load_data_via_qlever(&mut rudof, Some(&[path]), Some(&DataFormat::Pg), None, None)
        .expect_err("Pg should be rejected up front");
    match err {
        RudofError::Data(boxed) => assert!(
            matches!(*boxed, DataError::NonRdfFormat { ref format } if format == "pg"),
            "expected NonRdfFormat(pg), got {boxed:?}"
        ),
        other => panic!("expected RudofError::Data, got {other:?}"),
    }
}

#[cfg(feature = "qlever")]
#[test]
fn test_load_data_via_qlever_rejects_non_path_inputs() {
    use crate::api::data::implementations::load_data::load_data_via_qlever;
    use crate::errors::RudofError;

    let mut rudof = Rudof::new(RudofConfig::default());
    // A string input — not a filesystem path.
    let input = InputSpec::str("@prefix ex: <http://example.org/> . ex:a a ex:B .");

    let err = load_data_via_qlever(&mut rudof, Some(&[input]), Some(&DataFormat::Turtle), None, None)
        .expect_err("non-Path inputs should be rejected");
    match err {
        RudofError::Data(boxed) => assert!(
            matches!(*boxed, DataError::DataSourceSpec { ref message } if message.contains("file-system inputs")),
            "expected DataSourceSpec mentioning file-system inputs, got {boxed:?}"
        ),
        other => panic!("expected RudofError::Data, got {other:?}"),
    }
}
