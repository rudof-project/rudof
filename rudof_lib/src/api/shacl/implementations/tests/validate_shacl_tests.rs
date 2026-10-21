use crate::{
    Rudof, RudofConfig,
    api::data::implementations::load_data,
    api::shacl::implementations::load_shacl_schema::load_shacl_schema,
    api::shacl::implementations::serialize_shacl_validation_results::serialize_shacl_validation_results,
    api::shacl::implementations::validate_shacl::validate_shacl,
    formats::{
        DataFormat, InputSpec, ResultShaclValidationFormat, ShaclFormat, ShaclValidationMode, ShaclValidationSortByMode,
    },
};
use rudof_rdf::rdf_impl::EndpointStrategy;

/// Helper: serialize validation results to string
fn serialize_validation_to_string(
    rudof: &Rudof,
    sort_mode: Option<ShaclValidationSortByMode>,
    format: Option<ResultShaclValidationFormat>,
) -> String {
    let mut buffer = Vec::new();

    serialize_shacl_validation_results(rudof, sort_mode.as_ref(), format.as_ref(), &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_validate_shacl_conforming_data() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load SHACL schema
    let schema = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
            ] ;
            sh:property [
                sh:path ex:age ;
                sh:datatype xsd:integer ;
                sh:minCount 1 ;
            ] .
        "#,
    );

    load_shacl_schema(&mut rudof, Some(&schema), Some(&ShaclFormat::Turtle), None, None).unwrap();

    // Load conforming data
    let data = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:Alice
            a ex:Person ;
            ex:name "Alice Smith" ;
            ex:age 30 .

        ex:Bob
            a ex:Person ;
            ex:name "Bob Jones" ;
            ex:age 25 .
        "#,
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

    // Validate
    validate_shacl(&mut rudof, None).unwrap();

    let result = serialize_validation_to_string(&rudof, None, Some(ResultShaclValidationFormat::Minimal));

    assert!(result.contains("Conforms"));

    println!(
        "\n===== test_validate_shacl_conforming_data =====\n{}============================================",
        result
    );
}

#[test]
fn test_validate_shacl_non_conforming_data() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load SHACL schema
    let schema = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
            ] ;
            sh:property [
                sh:path ex:age ;
                sh:datatype xsd:integer ;
                sh:minCount 1 ;
            ] .
        "#,
    );

    load_shacl_schema(&mut rudof, Some(&schema), Some(&ShaclFormat::Turtle), None, None).unwrap();

    // Load non-conforming data (missing required properties)
    let data = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .

        ex:Alice
            a ex:Person ;
            ex:name "Alice Smith" .

        ex:Bob
            a ex:Person ;
            ex:age 25 .
        "#,
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

    // Validate
    validate_shacl(&mut rudof, None).unwrap();

    let result = serialize_validation_to_string(&rudof, None, Some(ResultShaclValidationFormat::Minimal));

    assert!(result.contains("Does not conform"));
    assert!(result.contains("violations"));

    println!(
        "\n===== test_validate_shacl_non_conforming_data =====\n{}============================================",
        result
    );
}

#[test]
fn test_validate_shacl_without_data() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load SHACL schema but no data
    let schema = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
            ] .
        "#,
    );

    load_shacl_schema(&mut rudof, Some(&schema), Some(&ShaclFormat::Turtle), None, None).unwrap();

    // Try to validate without data
    let result = validate_shacl(&mut rudof, None);

    assert!(result.is_err());
}

#[test]
fn test_validate_shacl_without_schema() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load data but no schema
    let data = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .

        ex:Alice
            a ex:Person ;
            ex:name "Alice Smith" .
        "#,
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
    let result = validate_shacl(&mut rudof, None);

    assert!(result.is_err());
}

#[test]
fn test_validate_shacl_datatype_violations() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:age ;
                sh:datatype xsd:integer ;
            ] .
        "#,
    );

    load_shacl_schema(&mut rudof, Some(&schema), Some(&ShaclFormat::Turtle), None, None).unwrap();

    // Data with wrong datatype
    let data = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .

        ex:Alice
            a ex:Person ;
            ex:age "thirty" .
        "#,
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

    validate_shacl(&mut rudof, None).unwrap();

    let result = serialize_validation_to_string(&rudof, None, Some(ResultShaclValidationFormat::Minimal));

    assert!(result.contains("Does not conform"));

    println!(
        "\n===== test_validate_shacl_datatype_violations =====\n{}============================================",
        result
    );
}

#[test]
fn test_validate_shacl_min_max_violations() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:age ;
                sh:datatype xsd:integer ;
                sh:minInclusive 0 ;
                sh:maxInclusive 150 ;
            ] .
        "#,
    );

    load_shacl_schema(&mut rudof, Some(&schema), Some(&ShaclFormat::Turtle), None, None).unwrap();

    // Data violating min/max constraints
    let data = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .

        ex:Alice
            a ex:Person ;
            ex:age 200 .

        ex:Bob
            a ex:Person ;
            ex:age -5 .
        "#,
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

    validate_shacl(&mut rudof, None).unwrap();

    let result = serialize_validation_to_string(&rudof, None, Some(ResultShaclValidationFormat::Minimal));

    assert!(result.contains("Does not conform"));
    assert!(result.contains("violations"));

    println!(
        "\n===== test_validate_shacl_min_max_violations =====\n{}============================================",
        result
    );
}

#[test]
fn test_validate_shacl_pattern_violations() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:email ;
                sh:datatype xsd:string ;
                sh:pattern "^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$" ;
            ] .
        "#,
    );

    load_shacl_schema(&mut rudof, Some(&schema), Some(&ShaclFormat::Turtle), None, None).unwrap();

    // Data with invalid email
    let data = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .

        ex:Alice
            a ex:Person ;
            ex:email "not-an-email" .

        ex:Bob
            a ex:Person ;
            ex:email "bob@example.com" .
        "#,
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

    validate_shacl(&mut rudof, None).unwrap();

    let result = serialize_validation_to_string(&rudof, None, Some(ResultShaclValidationFormat::Minimal));

    assert!(result.contains("Does not conform"));

    println!(
        "\n===== test_validate_shacl_pattern_violations =====\n{}============================================",
        result
    );
}

#[test]
fn test_validate_shacl_node_shape_violations() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:AddressShape
            a sh:NodeShape ;
            sh:targetClass ex:Address ;
            sh:property [
                sh:path ex:street ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
            ] .

        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:address ;
                sh:node ex:AddressShape ;
            ] .
        "#,
    );

    load_shacl_schema(&mut rudof, Some(&schema), Some(&ShaclFormat::Turtle), None, None).unwrap();

    // Data with address missing required street
    let data = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .

        ex:addr1
            a ex:Address ;
            ex:city "Springfield" .

        ex:Alice
            a ex:Person ;
            ex:address ex:addr1 .
        "#,
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

    validate_shacl(&mut rudof, None).unwrap();

    let result = serialize_validation_to_string(&rudof, None, Some(ResultShaclValidationFormat::Minimal));

    assert!(result.contains("Does not conform"));

    println!(
        "\n===== test_validate_shacl_node_shape_violations =====\n{}============================================",
        result
    );
}

#[test]
fn test_serialize_validation_results_compact() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
            ] .
        "#,
    );

    load_shacl_schema(&mut rudof, Some(&schema), Some(&ShaclFormat::Turtle), None, None).unwrap();

    let data = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .

        ex:Alice
            a ex:Person .
        "#,
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

    validate_shacl(&mut rudof, None).unwrap();

    let result = serialize_validation_to_string(&rudof, None, Some(ResultShaclValidationFormat::Compact));

    // Compact format should produce table output
    assert!(!result.is_empty());

    println!(
        "\n===== test_serialize_validation_results_compact =====\n{}============================================",
        result
    );
}

#[test]
fn test_serialize_validation_results_details() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
            ] .
        "#,
    );

    load_shacl_schema(&mut rudof, Some(&schema), Some(&ShaclFormat::Turtle), None, None).unwrap();

    let data = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .

        ex:Alice
            a ex:Person .
        "#,
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

    validate_shacl(&mut rudof, None).unwrap();

    let result = serialize_validation_to_string(&rudof, None, Some(ResultShaclValidationFormat::Details));

    // Details format should produce detailed table output
    assert!(!result.is_empty());

    println!(
        "\n===== test_serialize_validation_results_details =====\n{}============================================",
        result
    );
}

#[test]
fn test_serialize_validation_results_turtle() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
            ] .
        "#,
    );

    load_shacl_schema(&mut rudof, Some(&schema), Some(&ShaclFormat::Turtle), None, None).unwrap();

    let data = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .

        ex:Alice
            a ex:Person ;
            ex:name "Alice" .
        "#,
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

    validate_shacl(&mut rudof, None).unwrap();

    let result = serialize_validation_to_string(&rudof, None, Some(ResultShaclValidationFormat::Turtle));

    // Should produce RDF Turtle output
    assert!(!result.is_empty());
    assert!(result.contains("@prefix") || result.contains("sh:") || result.contains("ValidationReport"));

    println!(
        "\n===== test_serialize_validation_results_turtle =====\n{}============================================",
        result
    );
}

#[test]
fn test_serialize_validation_results_without_validation() {
    let rudof = Rudof::new(RudofConfig::default());

    let mut buffer = Vec::new();
    let result =
        serialize_shacl_validation_results(&rudof, None, Some(&ResultShaclValidationFormat::Minimal), &mut buffer);

    assert!(result.is_err());
}

#[test]
fn test_validate_shacl_with_validation_mode() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
            ] .
        "#,
    );

    load_shacl_schema(&mut rudof, Some(&schema), Some(&ShaclFormat::Turtle), None, None).unwrap();

    let data = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .

        ex:Alice
            a ex:Person ;
            ex:name "Alice" .
        "#,
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

    // Validate with default mode
    validate_shacl(&mut rudof, Some(&ShaclValidationMode::default())).unwrap();

    let result = serialize_validation_to_string(&rudof, None, Some(ResultShaclValidationFormat::Minimal));

    assert!(result.contains("Conforms"));

    println!(
        "\n===== test_validate_shacl_with_validation_mode =====\n{}============================================",
        result
    );
}

#[test]
fn test_validate_multiple_violations() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
            ] ;
            sh:property [
                sh:path ex:age ;
                sh:datatype xsd:integer ;
                sh:minCount 1 ;
                sh:minInclusive 0 ;
            ] .
        "#,
    );

    load_shacl_schema(&mut rudof, Some(&schema), Some(&ShaclFormat::Turtle), None, None).unwrap();

    // Multiple persons with different violations
    let data = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .

        ex:Alice
            a ex:Person .

        ex:Bob
            a ex:Person ;
            ex:name "Bob" .

        ex:Charlie
            a ex:Person ;
            ex:name "Charlie" ;
            ex:age -5 .
        "#,
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

    validate_shacl(&mut rudof, None).unwrap();

    let result = serialize_validation_to_string(&rudof, None, Some(ResultShaclValidationFormat::Minimal));

    assert!(result.contains("Does not conform"));
    assert!(result.contains("violations"));

    println!(
        "\n===== test_validate_multiple_violations =====\n{}============================================",
        result
    );
}

/// Loads a fixed schema/data pair (one conforming node, one violating node)
/// under the given `ShaclConfig`, returning the resulting report.
fn validate_with_shacl_config(config: shacl::validator::ShaclConfig) -> shacl::validator::report::ValidationReport {
    let mut rudof = Rudof::new(RudofConfig::default().with_shacl(config));

    let schema = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
            ] .
        "#,
    );
    load_shacl_schema(&mut rudof, Some(&schema), Some(&ShaclFormat::Turtle), None, None).unwrap();

    let data = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .

        ex:Alice a ex:Person ; ex:name "Alice" .
        ex:Bob a ex:Person .
        "#,
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

    validate_shacl(&mut rudof, None).unwrap();
    rudof.shacl_validation_results.unwrap()
}

#[test]
fn table_shows_conforming_evidence_rows_in_green_alongside_violations() {
    use crate::display::Table;

    let config = shacl::validator::ShaclConfig::default().with_store_evidences(true);
    let report = validate_with_shacl_config(config);

    // :Alice conforms (an evidence row) and :Bob doesn't (a violation row) —
    // both should show up in the same table.
    let mut colored = Vec::new();
    report.table(&mut colored, Some(true), Some(true), Some(120)).unwrap();
    let colored = String::from_utf8(colored).unwrap();
    assert!(colored.contains("Conforms"), "{colored}");
    assert!(colored.contains("Alice"), "{colored}");
    assert!(colored.contains("Bob"), "{colored}");

    // Same assertions with color turned off, to check the plain text
    // content isn't accidentally hidden inside escape codes only.
    let mut plain = Vec::new();
    report.table(&mut plain, Some(true), Some(false), Some(120)).unwrap();
    let plain = String::from_utf8(plain).unwrap();
    assert!(plain.contains("Conforms"), "{plain}");
    assert!(plain.contains("satisfied"), "{plain}");
}

#[test]
fn errors_only_mode_is_the_default_behavior() {
    let report = validate_with_shacl_config(shacl::validator::ShaclConfig::default());
    assert!(!report.conforms());
    assert_eq!(report.results().len(), 1);
    assert!(report.evidences().is_empty());
}

#[test]
fn boolean_only_mode_keeps_neither_errors_nor_evidences() {
    let config = shacl::validator::ShaclConfig::default().with_store_errors(false);
    let report = validate_with_shacl_config(config);
    assert!(!report.conforms());
    assert!(report.results().is_empty());
    assert!(report.evidences().is_empty());
}

#[test]
fn evidences_only_mode_keeps_evidences_but_not_errors() {
    let config = shacl::validator::ShaclConfig::default()
        .with_store_errors(false)
        .with_store_evidences(true);
    let report = validate_with_shacl_config(config);
    assert!(!report.conforms());
    assert!(report.results().is_empty());
    assert!(!report.evidences().is_empty());
}

#[test]
fn errors_and_evidences_mode_keeps_both() {
    let config = shacl::validator::ShaclConfig::default().with_store_evidences(true);
    let report = validate_with_shacl_config(config);
    assert!(!report.conforms());
    assert_eq!(report.results().len(), 1);
    assert!(!report.evidences().is_empty());
}

/// Regression test: a `sh:reifierShape` violation (reported against the
/// reifier node, not the outer focus node) used to get silently dropped
/// while bucketing evidence/violations per focus node, making the outer
/// node's own property shape — and therefore the enclosing node shape —
/// wrongly look like it conformed in the evidence report, even though the
/// plain (non-evidence) violations list correctly included the failure.
#[test]
fn reifier_shape_violation_is_not_masked_by_a_spurious_conforms_evidence() {
    use rudof_iri::IriS;
    use rudof_rdf::rdf_core::term::Object;
    use shacl::validator::ShaclConfig;
    use shacl::validator::report::EvidenceKind;

    let config = ShaclConfig::default().with_store_evidences(true);
    let mut rudof = Rudof::new(RudofConfig::default().with_shacl(config));

    let schema = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:UserShape a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:knows ;
                sh:class ex:Person ;
                sh:reificationRequired true ;
                sh:reifierShape ex:ReifierShape ;
            ] .

        ex:ReifierShape a sh:NodeShape ;
            sh:property [
                sh:path ex:since ;
                sh:datatype xsd:integer ;
                sh:minCount 1 ;
                sh:maxCount 1 ;
            ] .
        "#,
    );
    load_shacl_schema(&mut rudof, Some(&schema), Some(&ShaclFormat::Turtle), None, None).unwrap();

    let data = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .

        ex:b a ex:Person ; ex:name "Robert" .
        ex:c a ex:Person ;
             ex:name "Alice" ;
             ex:knows ex:b {| ex:since "Unknown" |} .
        "#,
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

    validate_shacl(&mut rudof, None).unwrap();
    let report = rudof.shacl_validation_results.unwrap();

    // `ex:since "Unknown"` on the reifier fails `sh:datatype xsd:integer`,
    // so this must not conform overall...
    assert!(!report.conforms());

    // ...and `ex:c` must not get a "shape satisfied" evidence for
    // `ex:UserShape` — that's exactly the bug: the reifier violation was
    // dropped from `ex:c`'s bucket, making it look like it had none.
    let c = Object::iri(IriS::new_unchecked("http://example.org/c"));
    let user_shape = Object::iri(IriS::new_unchecked("http://example.org/UserShape"));
    let spuriously_conforms = report
        .evidences()
        .iter()
        .any(|e| e.kind() == EvidenceKind::Shape && e.focus_node() == &c && e.constraint_component() == &user_shape);
    assert!(!spuriously_conforms, "{:#?}", report.evidences());
}

/// A SPARQL endpoint on a local port, answering SELECT and ASK queries over
/// `turtle` with Oxigraph. Returns its URL and the queries it received.
#[cfg(not(target_family = "wasm"))]
fn serve_sparql(turtle: &str) -> (String, std::sync::Arc<std::sync::Mutex<Vec<String>>>) {
    use oxigraph::io::RdfFormat;
    use oxigraph::sparql::{QueryResults, SparqlEvaluator};
    use oxigraph::store::Store;
    use sparesults::{QueryResultsFormat, QueryResultsSerializer};
    use std::io::{BufRead, BufReader, Write};
    use std::sync::{Arc, Mutex};

    let store = Store::new().unwrap();
    store.load_from_reader(RdfFormat::Turtle, turtle.as_bytes()).unwrap();
    let queries = Arc::new(Mutex::new(Vec::new()));
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    {
        let queries = Arc::clone(&queries);
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let store = store.clone();
                let queries = Arc::clone(&queries);
                std::thread::spawn(move || {
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
                    queries.lock().unwrap().push(query.clone());
                    let results = SparqlEvaluator::new()
                        .parse_query(&query)
                        .unwrap()
                        .on_store(&store)
                        .execute()
                        .unwrap();
                    let serializer = QueryResultsSerializer::from_format(QueryResultsFormat::Json);
                    let mut body = Vec::new();
                    match results {
                        QueryResults::Solutions(solutions) => {
                            let mut writer = serializer
                                .serialize_solutions_to_writer(&mut body, solutions.variables().to_vec())
                                .unwrap();
                            for solution in solutions {
                                let solution = solution.unwrap();
                                writer
                                    .serialize(solution.iter().map(|(v, t)| (v.as_ref(), t.as_ref())))
                                    .unwrap();
                            }
                            writer.finish().unwrap();
                        },
                        QueryResults::Boolean(value) => {
                            serializer.serialize_boolean_to_writer(&mut body, value).unwrap();
                        },
                        QueryResults::Graph(_) => panic!("CONSTRUCT is not supported by the mock endpoint"),
                    }
                    let head = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/sparql-results+json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len()
                    );
                    let _ = stream.write_all(head.as_bytes());
                    let _ = stream.write_all(&body);
                });
            }
        });
    }
    (format!("http://{addr}/sparql"), queries)
}

/// Validates `shapes` against `data` served by a mock SPARQL endpoint, with
/// `strategy`. Returns the report and the queries the endpoint received.
#[cfg(not(target_family = "wasm"))]
fn validate_shacl_against_endpoint(
    data: &str,
    shapes: &str,
    strategy: shacl::validator::FetchStrategy,
) -> (shacl::validator::report::ValidationReport, Vec<String>) {
    validate_shacl_against_endpoint_in_mode(data, shapes, strategy, ShaclValidationMode::Native)
}

#[cfg(not(target_family = "wasm"))]
fn validate_shacl_against_endpoint_in_mode(
    data: &str,
    shapes: &str,
    strategy: shacl::validator::FetchStrategy,
    mode: ShaclValidationMode,
) -> (shacl::validator::report::ValidationReport, Vec<String>) {
    let (url, queries) = serve_sparql(data);
    let config = RudofConfig::default();
    let shacl_config = config.shacl().clone().with_fetch_strategy(strategy);
    let mut rudof = Rudof::new(config.with_shacl(shacl_config));
    rudof.load_data().with_endpoint(&url).execute().unwrap();
    load_shacl_schema(
        &mut rudof,
        Some(&InputSpec::str(shapes)),
        Some(&ShaclFormat::Turtle),
        None,
        None,
    )
    .unwrap();
    validate_shacl(&mut rudof, Some(&mode)).unwrap();
    let report = rudof.shacl_validation_results.clone().unwrap();
    let queries = queries.lock().unwrap().clone();
    (report, queries)
}

#[cfg(not(target_family = "wasm"))]
fn is_full_scan(query: &str) -> bool {
    regex::Regex::new(r"(?i)WHERE\s*\{\s*\?s\s+\?p\s+\?o\s*\.?\s*\}")
        .unwrap()
        .is_match(query)
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn test_validate_shacl_endpoint_never_fetches_every_triple() {
    // The example from the report: a target node and a value on a path.
    let shapes = r#"
        prefix : <http://example.org/>
        prefix sh: <http://www.w3.org/ns/shacl#>
        :Person a sh:NodeShape ;
           sh:targetNode :q42 ;
           sh:property [ sh:path :p31 ; sh:hasValue :q5 ] .
    "#;
    let data = "prefix : <http://example.org/> :q42 :p31 :q5 . :q1 :p31 :q2 .";
    for mode in [ShaclValidationMode::Native, ShaclValidationMode::Sparql] {
        for strategy in [
            shacl::validator::FetchStrategy::OnDemand,
            shacl::validator::FetchStrategy::ByShape,
        ] {
            let (report, queries) = validate_shacl_against_endpoint_in_mode(data, shapes, strategy, mode);
            assert!(report.conforms(), "{mode} {strategy}: {report:?}");
            assert!(!queries.is_empty());
            assert!(
                !queries.iter().any(|q| is_full_scan(q)),
                "{mode} {strategy}: a query asked for every triple: {queries:#?}"
            );
        }
    }
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn test_validate_shacl_endpoint_by_shape_matches_on_demand_with_fewer_queries() {
    // 40 people who must have a string name, work for an organisation (an
    // instance of a subclass of :Org), and know only people that conform too
    // (a recursive shape). Each :pI knows :pI+1, and :p7 has no name.
    let mut data = String::from(
        "prefix : <http://example.org/>
         prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#>
         :Company rdfs:subClassOf :Org .
         :acme a :Company .\n",
    );
    for i in 0..40 {
        data.push_str(&format!(":p{i} a :Person ; :worksFor :acme"));
        if i != 7 {
            data.push_str(&format!(" ; :name \"Person {i}\""));
        }
        if i < 39 {
            data.push_str(&format!(" ; :knows :p{}", i + 1));
        }
        data.push_str(" .\n");
    }
    let shapes = r#"
        prefix : <http://example.org/>
        prefix sh: <http://www.w3.org/ns/shacl#>
        prefix xsd: <http://www.w3.org/2001/XMLSchema#>
        :PersonShape a sh:NodeShape ;
           sh:targetClass :Person ;
           sh:property [ sh:path :name ; sh:minCount 1 ; sh:datatype xsd:string ] ;
           sh:property [ sh:path :worksFor ; sh:minCount 1 ; sh:class :Org ] ;
           sh:property [ sh:path :knows ; sh:node :PersonShape ] .
    "#;

    let (on_demand, on_demand_queries) =
        validate_shacl_against_endpoint(&data, shapes, shacl::validator::FetchStrategy::OnDemand);
    let (by_shape, by_shape_queries) =
        validate_shacl_against_endpoint(&data, shapes, shacl::validator::FetchStrategy::ByShape);

    let failing = |report: &shacl::validator::report::ValidationReport| {
        let mut nodes: Vec<String> = report.results().iter().map(|r| r.focus_node().to_string()).collect();
        nodes.sort();
        nodes.dedup();
        nodes
    };
    // :p7 has no name; :p0 to :p6 know someone who fails, down to :p7.
    let expected: Vec<String> = {
        let mut nodes: Vec<String> = (0..=7).map(|i| format!("http://example.org/p{i}")).collect();
        nodes.sort();
        nodes
    };
    assert!(!on_demand.conforms());
    assert_eq!(failing(&on_demand), expected);
    assert_eq!(failing(&by_shape), failing(&on_demand));
    assert_eq!(by_shape.results().len(), on_demand.results().len());

    assert!(!by_shape_queries.iter().any(|q| is_full_scan(q)));
    assert!(!on_demand_queries.iter().any(|q| is_full_scan(q)));
    // On demand: a few queries per person (124). By shape, whatever the
    // number of people (up to 50 per query): the targets (2 queries), the
    // three paths of the people together (1), and the class of their
    // employer with its superclasses (3).
    assert!(on_demand_queries.len() > 100, "{} queries", on_demand_queries.len());
    assert_eq!(by_shape_queries.len(), 6, "{by_shape_queries:#?}");
}

/// The (focus node, constraint component, value) of each result, sorted.
#[cfg(not(target_family = "wasm"))]
fn result_keys(report: &shacl::validator::report::ValidationReport) -> Vec<String> {
    let mut keys: Vec<String> = report
        .results()
        .iter()
        .map(|r| format!("{} {} {:?}", r.focus_node(), r.constraint_component(), r.value()))
        .collect();
    keys.sort();
    keys
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn test_validate_shacl_endpoint_strategies_agree_with_in_memory() {
    // Every kind of path, and the components that read data beyond the
    // values of a path: sh:class (with subclasses), property pairs, closed
    // shapes, qualified values, and the logical components.
    let data = r#"
        prefix : <http://example.org/>
        prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#>
        :Sub rdfs:subClassOf :Super . :Super rdfs:subClassOf :Top .
        :a a :Item ; :p :b ; :q :c ; :r "x"@en, "y"@fr ; :start 1 ; :end 2 ; :same 1 .
        :b a :Item, :Sub ; :p :c ; :q :a ; :start 3 ; :end 1 ; :same 2 ; :extra 1 .
        :c a :Item ; :p :d ; :r "z"@en, "w"@en ; :start 1 ; :end 5 ; :same 1 .
        :d a :Item, :Other ; :q :a .
        :e :p :a .
    "#;
    let shapes = r#"
        prefix : <http://example.org/>
        prefix sh: <http://www.w3.org/ns/shacl#>
        :ItemShape a sh:NodeShape ;
            sh:targetClass :Item ;
            sh:property [ sh:path ( :p :p ) ; sh:maxCount 1 ] ;
            sh:property [ sh:path [ sh:inversePath :p ] ; sh:minCount 1 ] ;
            sh:property [ sh:path [ sh:alternativePath ( :p :q ) ] ; sh:class :Top ] ;
            sh:property [ sh:path [ sh:zeroOrMorePath :p ] ; sh:minCount 4 ] ;
            sh:property [ sh:path [ sh:oneOrMorePath :q ] ; sh:maxCount 2 ] ;
            sh:property [ sh:path :r ; sh:uniqueLang true ; sh:languageIn ( "en" ) ] ;
            sh:property [ sh:path :start ; sh:lessThan :end ; sh:equals :same ] ;
            sh:property [ sh:path :p ;
                sh:qualifiedValueShape [ sh:class :Other ] ; sh:qualifiedMaxCount 0 ] ;
            sh:property [ sh:path :q ; sh:node :ItemShape ] .
        :ClosedShape a sh:NodeShape ;
            sh:targetNode :a, :b ;
            sh:closed true ;
            sh:ignoredProperties ( <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> ) ;
            sh:property [ sh:path :p ] ; sh:property [ sh:path :q ] ; sh:property [ sh:path :r ] ;
            sh:property [ sh:path :start ] ; sh:property [ sh:path :end ] ; sh:property [ sh:path :same ] .
        :LogicShape a sh:NodeShape ;
            sh:targetSubjectsOf :p ;
            sh:or ( [ sh:class :Sub ] [ sh:property [ sh:path :q ; sh:minCount 1 ] ] ) ;
            sh:xone ( [ sh:class :Item ] [ sh:class :Other ] ) ;
            sh:not [ sh:property [ sh:path :extra ; sh:minCount 1 ] ] .
    "#;

    // Ground truth: the same data in memory.
    let mut rudof = Rudof::new(RudofConfig::default());
    load_shacl_schema(
        &mut rudof,
        Some(&InputSpec::str(shapes)),
        Some(&ShaclFormat::Turtle),
        None,
        None,
    )
    .unwrap();
    rudof
        .load_data()
        .with_data(&[InputSpec::str(data)])
        .with_data_format(&DataFormat::Turtle)
        .execute()
        .unwrap();
    validate_shacl(&mut rudof, Some(&ShaclValidationMode::Native)).unwrap();
    let in_memory = rudof.shacl_validation_results.clone().unwrap();
    assert!(!in_memory.conforms());
    let expected = result_keys(&in_memory);
    assert!(expected.len() > 5, "{expected:#?}");

    for strategy in [
        shacl::validator::FetchStrategy::OnDemand,
        shacl::validator::FetchStrategy::ByShape,
    ] {
        let (report, _) = validate_shacl_against_endpoint(data, shapes, strategy);
        assert_eq!(result_keys(&report), expected, "{strategy}");
    }
}
