use crate::{
    Rudof, RudofConfig,
    api::data::implementations::load_data,
    api::shex::implementations::load_shapemap::load_shapemap,
    api::shex::implementations::load_shex_schema::load_shex_schema,
    api::shex::implementations::serialize_shex_validation_results::serialize_shex_validation_results,
    api::shex::implementations::validate_shex::validate_shex,
    formats::{
        DataFormat, InputSpec, ResultShExValidationFormat, ShExFormat, ShExValidationSortByMode, ShapeMapFormat,
    },
};
use regex::Regex;
use rudof_rdf::rdf_impl::EndpointStrategy;
//use std::str::FromStr;

/// Helper: serialize validation results to string
fn serialize_validation_results_to_string(
    rudof: &mut Rudof,
    sort_order: Option<ShExValidationSortByMode>,
    format: Option<ResultShExValidationFormat>,
) -> String {
    let mut buffer = Vec::new();

    serialize_shex_validation_results(rudof, sort_order.as_ref(), format.as_ref(), &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_validate_shex_success() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load RDF data
    let data = InputSpec::str(
        r#"
            <alice> <name> "Alice" ;
                <age> 30 .
        "#,
    );

    load_data(
        &mut rudof,
        Some(&[data]),
        Some(&DataFormat::Turtle),
        Some("http://example.org/"),
        None,
        None,
        None,
        None,
        EndpointStrategy::default(),
    )
    .unwrap();

    // Load ShEx schema
    let schema = InputSpec::str(
        r#"PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           <PersonShape> {
             <name> xsd:string ;
             <age> xsd:integer
           }"#,
    );

    load_shex_schema(
        &mut rudof,
        &schema,
        Some(&ShExFormat::ShExC),
        Some("http://example.org/"),
        None,
    )
    .unwrap();

    // Load ShapeMap
    let shapemap = InputSpec::str(r#"<alice>@<PersonShape>"#);

    load_shapemap(
        &mut rudof,
        &shapemap,
        Some(&ShapeMapFormat::Compact),
        Some("http://example.org/"),
        Some("http://example.org/"),
    )
    .unwrap();

    // Validate
    validate_shex(&mut rudof).unwrap();

    assert!(rudof.shex_validation_results.is_some());

    let serialized =
        serialize_validation_results_to_string(&mut rudof, None, Some(ResultShExValidationFormat::Compact));

    let re = Regex::new(r"(?m)^│\s+Node\s+│\s+Shape\s+│\s+Status\s+│$").unwrap();

    assert!(re.is_match(&serialized));

    println!(
        "\n===== test_validate_shex_success =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_validate_shex_no_data_error() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Try to validate without data
    let result = validate_shex(&mut rudof);

    assert!(result.is_err());
}

#[test]
fn test_validate_shex_no_schema_error() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load data but no schema
    let data = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ."#,
    );

    load_data(
        &mut rudof,
        Some(&[data]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None,
        EndpointStrategy::default(),
    )
    .unwrap();

    // Try to validate without schema
    let result = validate_shex(&mut rudof);

    assert!(result.is_err());
}

#[test]
fn test_validate_shex_no_shapemap_error() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load data and schema but no shapemap
    let data = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ."#,
    );

    load_data(
        &mut rudof,
        Some(&[data]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None,
        EndpointStrategy::default(),
    )
    .unwrap();

    let schema = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape { ex:name xsd:string }"#,
    );

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    // Try to validate without shapemap
    let result = validate_shex(&mut rudof);

    assert!(result.is_err());
}

#[test]
fn test_validate_shex_validation_failure() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load RDF data that doesn't conform to schema
    let data = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ;
                    ex:age "not an integer" ."#,
    );

    load_data(
        &mut rudof,
        Some(&[data]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None,
        EndpointStrategy::default(),
    )
    .unwrap();

    // Load ShEx schema expecting integer
    let schema = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape {
             ex:name xsd:string ;
             ex:age xsd:integer
           }"#,
    );

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    // Load ShapeMap
    let shapemap = InputSpec::str(r#"ex:alice@ex:PersonShape"#);

    load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    // Validate - should succeed but with validation failure result
    validate_shex(&mut rudof).unwrap();

    assert!(rudof.shex_validation_results.is_some());

    let serialized =
        serialize_validation_results_to_string(&mut rudof, None, Some(ResultShExValidationFormat::Compact));

    println!(
        "\n===== test_validate_shex_validation_failure =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_serialize_validation_results_compact() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Setup and validate
    let data = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ;
                    ex:age 30 ."#,
    );

    load_data(
        &mut rudof,
        Some(&[data]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None,
        EndpointStrategy::default(),
    )
    .unwrap();

    let schema = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape {
             ex:name xsd:string ;
             ex:age xsd:integer
           }"#,
    );

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    let shapemap = InputSpec::str(r#"ex:alice@ex:PersonShape"#);

    load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    validate_shex(&mut rudof).unwrap();

    // Test compact format
    let serialized =
        serialize_validation_results_to_string(&mut rudof, None, Some(ResultShExValidationFormat::Compact));

    let re = Regex::new(r"(?m)^│\s+Node\s+│\s+Shape\s+│\s+Status\s+│$").unwrap();

    assert!(re.is_match(&serialized));

    println!(
        "\n===== test_serialize_validation_results_compact =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_serialize_validation_results_json() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Setup and validate
    let data = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ;
                    ex:age 30 ."#,
    );

    load_data(
        &mut rudof,
        Some(&[data]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None,
        EndpointStrategy::default(),
    )
    .unwrap();

    let schema = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape {
             ex:name xsd:string ;
             ex:age xsd:integer
           }"#,
    );

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    let shapemap = InputSpec::str(r#"ex:alice@ex:PersonShape"#);

    load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    validate_shex(&mut rudof).unwrap();

    // Test JSON format
    let serialized = serialize_validation_results_to_string(&mut rudof, None, Some(ResultShExValidationFormat::Json));

    assert!(serialized.contains("{"));
    assert!(serialized.contains("}"));

    println!(
        "\n===== test_serialize_validation_results_json =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_serialize_validation_results_csv() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Setup and validate
    let data = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ;
                    ex:age 30 ."#,
    );

    load_data(
        &mut rudof,
        Some(&[data]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None,
        EndpointStrategy::default(),
    )
    .unwrap();

    let schema = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape {
             ex:name xsd:string ;
             ex:age xsd:integer
           }"#,
    );

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    let shapemap = InputSpec::str(r#"ex:alice@ex:PersonShape"#);

    load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    validate_shex(&mut rudof).unwrap();

    // Test CSV format
    let serialized = serialize_validation_results_to_string(&mut rudof, None, Some(ResultShExValidationFormat::Csv));

    // CSV should have comma-separated values
    assert!(serialized.contains(","));

    println!(
        "\n===== test_serialize_validation_results_csv =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_serialize_validation_results_details() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Setup and validate
    let data = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ;
                    ex:age 30 ."#,
    );

    load_data(
        &mut rudof,
        Some(&[data]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None,
        EndpointStrategy::default(),
    )
    .unwrap();

    let schema = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape {
             ex:name xsd:string ;
             ex:age xsd:integer
           }"#,
    );

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    let shapemap = InputSpec::str(r#"ex:alice@ex:PersonShape"#);

    load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    validate_shex(&mut rudof).unwrap();

    // Test details format
    let serialized =
        serialize_validation_results_to_string(&mut rudof, None, Some(ResultShExValidationFormat::Details));

    let re = Regex::new(r"(?m)^│\s+Node\s+│\s+Shape\s+│\s+Status\s+│\s+Details\s+│$").unwrap();

    assert!(re.is_match(&serialized));

    println!(
        "\n===== test_serialize_validation_results_details =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_serialize_validation_results_no_results_error() {
    let rudof = Rudof::new(RudofConfig::default());

    let mut buffer = Vec::new();

    let result =
        serialize_shex_validation_results(&rudof, None, Some(&ResultShExValidationFormat::Compact), &mut buffer);

    assert!(result.is_err());
}

#[test]
fn test_validate_shex_multiple_nodes() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load RDF data with multiple nodes
    let data = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ;
                    ex:age 30 .
           ex:bob ex:name "Bob" ;
                  ex:age 25 ."#,
    );

    load_data(
        &mut rudof,
        Some(&[data]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None,
        EndpointStrategy::default(),
    )
    .unwrap();

    // Load ShEx schema
    let schema = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape {
             ex:name xsd:string ;
             ex:age xsd:integer
           }"#,
    );

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    // Load ShapeMap with multiple associations
    let shapemap = InputSpec::str(
        r#"ex:alice@ex:PersonShape,
           ex:bob@ex:PersonShape"#,
    );

    load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    // Validate
    validate_shex(&mut rudof).unwrap();

    assert!(rudof.shex_validation_results.is_some());

    let serialized =
        serialize_validation_results_to_string(&mut rudof, None, Some(ResultShExValidationFormat::Compact));

    let re = Regex::new(r"(?m)^│\s+Node\s+│\s+Shape\s+│\s+Status\s+│$").unwrap();

    assert!(re.is_match(&serialized));

    println!(
        "\n===== test_validate_shex_multiple_nodes =====\n{}============================================",
        serialized
    );
}

/// Regression test for the IRI-pattern bug triggered by `--base-schema`/`--base-data`.
#[test]
fn test_validate_shex_iri_pattern_with_base_schema() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let data = InputSpec::str(
        r#"PREFIX ex: <http://a.example/>
           ex:n1 ex:p1 "ab"^^ex:dt1 ."#,
    );
    load_data(
        &mut rudof,
        Some(&[data]),
        Some(&DataFormat::Turtle),
        Some("http://a.example/"),
        None,
        None,
        None,
        None,
        EndpointStrategy::default(),
    )
    .unwrap();

    let schema = InputSpec::str(
        r#"<http://a.example/S1> IRI /^https?:\/\// {
             <http://a.example/p1> <http://a.example/dt1>
           }"#,
    );
    load_shex_schema(
        &mut rudof,
        &schema,
        Some(&ShExFormat::ShExC),
        Some("http://a.example/"),
        None,
    )
    .unwrap();

    let shapemap = InputSpec::str(r#"<http://a.example/n1>@<http://a.example/S1>"#);
    load_shapemap(
        &mut rudof,
        &shapemap,
        Some(&ShapeMapFormat::Compact),
        Some("http://a.example/"),
        Some("http://a.example/"),
    )
    .unwrap();

    validate_shex(&mut rudof).unwrap();
    let serialized =
        serialize_validation_results_to_string(&mut rudof, None, Some(ResultShExValidationFormat::Compact));
    assert!(
        serialized.contains("OK"),
        "Expected OK status (pattern ^https?:// must match full IRI even when base is set). Got:\n{serialized}",
    );
    assert!(!serialized.contains("FAIL"), "Got unexpected FAIL:\n{serialized}");
}

/// Regression test for the ShapeMap parser rejecting `_:label` as a focus node.
#[test]
fn test_validate_shex_bnode_focus_in_shapemap() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let data = InputSpec::str(r#"_:abcd <http://a.example/p1> <http://a.example/o1> ."#);
    load_data(
        &mut rudof,
        Some(&[data]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None,
        EndpointStrategy::default(),
    )
    .unwrap();

    let schema = InputSpec::str(
        r#"<http://a.example/S1> IRI {
             <http://a.example/p1> .
           }"#,
    );
    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    let shapemap = InputSpec::str(r#"_:abcd@<http://a.example/S1>"#);
    load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    validate_shex(&mut rudof).unwrap();
    let serialized =
        serialize_validation_results_to_string(&mut rudof, None, Some(ResultShExValidationFormat::Compact));
    assert!(
        serialized.contains("_:abcd"),
        "Expected focus _:abcd to appear in results:\n{serialized}",
    );
    assert!(
        serialized.contains("FAIL"),
        "Expected FAIL (BNode focus against IRI shape):\n{serialized}",
    );
}

/// A SPARQL endpoint on a local port answering every query with
/// `<http://example.org/nI> <http://example.org/p> "I"` for each node `nI`
/// (`I` < 100) the query mentions. Returns its URL.
#[cfg(not(target_family = "wasm"))]
fn serve_mock_endpoint() -> String {
    use std::io::{BufRead, BufReader, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut request_line = String::new();
            reader.read_line(&mut request_line).unwrap();
            let mut line = String::new();
            while reader.read_line(&mut line).unwrap() > 0 && line != "\r\n" {
                line.clear();
            }
            let target = request_line.split_whitespace().nth(1).unwrap();
            let url = url::Url::parse(&format!("http://localhost{target}")).unwrap();
            let query: String = url
                .query_pairs()
                .find(|(k, _)| k == "query")
                .map(|(_, v)| v.into_owned())
                .unwrap();
            let bindings: Vec<String> = (0..100)
                .filter(|i| query.contains(&format!("<http://example.org/n{i}>")))
                .map(|i| {
                    format!(
                        r#"{{"s":{{"type":"uri","value":"http://example.org/n{i}"}},"p":{{"type":"uri","value":"http://example.org/p"}},"o":{{"type":"literal","value":"{i}"}}}}"#
                    )
                })
                .collect();
            let body = format!(
                r#"{{"head":{{"vars":["s","p","o"]}},"results":{{"bindings":[{}]}}}}"#,
                bindings.join(",")
            );
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/sparql-results+json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = stream.write_all(response.as_bytes());
        }
    });
    format!("http://{addr}/sparql")
}

/// Validates `n0`..`n4` (conformant) and `n200` (no `p`, so non-conformant)
/// against a mock endpoint, returning the number of requests it took.
#[cfg(not(target_family = "wasm"))]
fn validate_shapemap_against_mock_endpoint(limits: rudof_rdf::rdf_core::EndpointLimits) -> u64 {
    let url = serve_mock_endpoint();
    let mut rudof = Rudof::new(RudofConfig::default());
    rudof
        .load_data()
        .with_endpoint(&url)
        .with_endpoint_limits(limits)
        .execute()
        .unwrap();
    let schema = InputSpec::str("<S> { <p> . }");
    load_shex_schema(
        &mut rudof,
        &schema,
        Some(&ShExFormat::ShExC),
        Some("http://example.org/"),
        None,
    )
    .unwrap();
    let shapemap = InputSpec::str("<n0>@<S>, <n1>@<S>, <n2>@<S>, <n3>@<S>, <n4>@<S>, <n200>@<S>");
    load_shapemap(
        &mut rudof,
        &shapemap,
        Some(&ShapeMapFormat::Compact),
        Some("http://example.org/"),
        Some("http://example.org/"),
    )
    .unwrap();

    validate_shex(&mut rudof).unwrap();

    let results = rudof.shex_validation_results.as_ref().unwrap();
    let mut seen = 0;
    for (node, _, status) in results.iter() {
        seen += 1;
        if node.to_string().contains("n200") {
            assert!(status.is_non_conformant(), "{node}: {status:?}");
        } else {
            assert!(status.is_conformant(), "{node}: {status:?}");
        }
    }
    assert_eq!(seen, 6);
    rudof.endpoint_request_count()
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn test_validate_shex_endpoint_batches_shapemap_nodes() {
    let requests = validate_shapemap_against_mock_endpoint(Default::default());
    assert_eq!(requests, 1);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn test_validate_shex_endpoint_without_batching() {
    let requests = validate_shapemap_against_mock_endpoint(
        rudof_rdf::rdf_core::EndpointLimits::default().with_max_nodes_per_query(Some(1)),
    );
    assert_eq!(requests, 6);
}
