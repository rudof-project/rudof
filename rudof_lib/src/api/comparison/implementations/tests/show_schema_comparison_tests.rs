use crate::{
    Rudof, RudofConfig,
    api::comparison::implementations::show_schema_comparison::show_schema_comparison,
    formats::{ComparisonFormat, ComparisonMode, InputSpec, ResultComparisonFormat},
};
use std::str::FromStr;

/// Helper: serialize comparison to string with shape labels
fn serialize_comparison_to_string(
    rudof: &mut Rudof,
    schema1: &InputSpec,
    schema2: &InputSpec,
    mode1: &ComparisonMode,
    mode2: &ComparisonMode,
    format1: &ComparisonFormat,
    format2: &ComparisonFormat,
    shape1: Option<&str>,
    shape2: Option<&str>,
    result_format: &ResultComparisonFormat,
) -> String {
    let mut buffer = Vec::new();

    show_schema_comparison(
        rudof,
        schema1,
        schema2,
        None,
        None,
        None,
        format1,
        format2,
        mode1,
        mode2,
        shape1,
        shape2,
        None,
        Some(result_format),
        &mut buffer,
    )
    .unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_compare_identical_shex_schemas() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema = InputSpec::from_str(
        r#"
PREFIX : <http://example.org/>
PREFIX schema: <http://schema.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
    schema:name xsd:string ;
    schema:age xsd:integer ?
}
        "#,
    )
    .unwrap();

    let serialized = serialize_comparison_to_string(
        &mut rudof,
        &schema,
        &schema,
        &ComparisonMode::ShEx,
        &ComparisonMode::ShEx,
        &ComparisonFormat::ShExC,
        &ComparisonFormat::ShExC,
        Some("http://example.org/Person"),
        Some("http://example.org/Person"),
        &ResultComparisonFormat::Internal,
    );

    // Identical schemas should show some form of equality or empty differences
    assert!(!serialized.is_empty());

    println!(
        "\n===== test_compare_identical_shex_schemas =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_compare_different_shex_schemas() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema1 = InputSpec::from_str(
        r#"
PREFIX : <http://example.org/>
PREFIX schema: <http://schema.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
    schema:name xsd:string ;
    schema:age xsd:integer ?
}
        "#,
    )
    .unwrap();

    let schema2 = InputSpec::from_str(
        r#"
PREFIX : <http://example.org/>
PREFIX schema: <http://schema.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
    schema:name xsd:string ;
    schema:email xsd:string
}
        "#,
    )
    .unwrap();

    let serialized = serialize_comparison_to_string(
        &mut rudof,
        &schema1,
        &schema2,
        &ComparisonMode::ShEx,
        &ComparisonMode::ShEx,
        &ComparisonFormat::ShExC,
        &ComparisonFormat::ShExC,
        Some("http://example.org/Person"),
        Some("http://example.org/Person"),
        &ResultComparisonFormat::Internal,
    );

    // Different schemas should show differences
    assert!(!serialized.is_empty());

    println!(
        "\n===== test_compare_different_shex_schemas =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_compare_shex_schemas_json_output() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema1 = InputSpec::from_str(
        r#"
PREFIX : <http://example.org/>
PREFIX schema: <http://schema.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
    schema:name xsd:string
}
        "#,
    )
    .unwrap();

    let schema2 = InputSpec::from_str(
        r#"
PREFIX : <http://example.org/>
PREFIX schema: <http://schema.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
    schema:name xsd:string ;
    schema:age xsd:integer
}
        "#,
    )
    .unwrap();

    let serialized = serialize_comparison_to_string(
        &mut rudof,
        &schema1,
        &schema2,
        &ComparisonMode::ShEx,
        &ComparisonMode::ShEx,
        &ComparisonFormat::ShExC,
        &ComparisonFormat::ShExC,
        Some("http://example.org/Person"),
        Some("http://example.org/Person"),
        &ResultComparisonFormat::Json,
    );

    // JSON output should be valid JSON
    assert!(serialized.contains("{") && serialized.contains("}"));

    println!(
        "\n===== test_compare_shex_schemas_json_output =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_compare_complex_shex_schemas() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema1 = InputSpec::from_str(
        r#"
PREFIX : <http://example.org/>
PREFIX schema: <http://schema.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
    schema:name xsd:string ;
    schema:email xsd:string * ;
    schema:knows @:Person *
}

:Organization {
    schema:name xsd:string ;
    schema:employee @:Person +
}
        "#,
    )
    .unwrap();

    let schema2 = InputSpec::from_str(
        r#"
PREFIX : <http://example.org/>
PREFIX schema: <http://schema.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
    schema:name xsd:string ;
    schema:email xsd:string + ;
    schema:worksWith @:Person *
}

:Organization {
    schema:legalName xsd:string ;
    schema:member @:Person +
}
        "#,
    )
    .unwrap();

    let serialized = serialize_comparison_to_string(
        &mut rudof,
        &schema1,
        &schema2,
        &ComparisonMode::ShEx,
        &ComparisonMode::ShEx,
        &ComparisonFormat::ShExC,
        &ComparisonFormat::ShExC,
        Some("http://example.org/Person"),
        Some("http://example.org/Person"),
        &ResultComparisonFormat::Json,
    );

    // Complex comparison with multiple shapes and properties
    assert!(serialized.contains("{") && serialized.contains("}"));

    println!(
        "\n===== test_compare_complex_shex_schemas =====\n{}\n============================================",
        serialized
    );
}
