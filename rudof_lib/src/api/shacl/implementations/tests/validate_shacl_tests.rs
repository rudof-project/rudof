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
        None
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
        None
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
        None
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
        None
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
        None
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
        None
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
        None
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
        None
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
        None
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
        None
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
        None
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
        None
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
