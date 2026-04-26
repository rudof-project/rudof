use crate::{Rudof, RudofConfig, api::shex::implementations::check_shex_schema::check_shex_schema, formats::InputSpec};
use std::io::Cursor;

/// Helper function to create a valid ShEx schema
fn create_valid_schema() -> &'static str {
    r#"
PREFIX ex: <http://example.org/>
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

ex:PersonShape {
    foaf:name xsd:string ;
    foaf:age xsd:integer ?
}
        "#
}

/// Helper function to create a malformed ShEx schema (syntax error)
fn create_malformed_schema() -> &'static str {
    r#"
PREFIX ex: <http://example.org/>
PREFIX foaf: <http://xmlns.com/foaf/0.1/>

ex:PersonShape {
    foaf:name xsd:string ;
    foaf:age xsd:integer ?
}
        "#
}

/// Helper function to create a schema with negative cycles
fn create_schema_with_neg_cycle() -> &'static str {
    r#"
PREFIX ex: <http://example.org/>

ex:Shape1 {
    ex:prop1 @ex:Shape2
}

ex:Shape2 {
    ex:prop2 NOT @ex:Shape1
}
        "#
}

/// Helper function to create a schema with multiple negative cycles
fn create_schema_with_multiple_neg_cycles() -> &'static str {
    r#"
PREFIX ex: <http://example.org/>

ex:Shape1 {
    ex:prop1 @ex:Shape2
}

ex:Shape2 {
    ex:prop2 NOT @ex:Shape1
}

ex:Shape3 {
    ex:prop3 @ex:Shape4
}

ex:Shape4 {
    ex:prop4 @ex:Shape5
}

ex:Shape5 {
    ex:prop5 @ex:Shape6
}

ex:Shape6 {
    ex:prop6 NOT @ex:Shape3
}
        "#
}

/// Helper function to create a complex valid schema
fn create_complex_valid_schema() -> &'static str {
    r#"
PREFIX ex: <http://example.org/>
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

ex:PersonShape {
    foaf:name xsd:string ;
    foaf:age xsd:integer ? ;
    foaf:knows @ex:PersonShape *
}

ex:OrganizationShape {
    foaf:name xsd:string ;
    ex:employee @ex:PersonShape +
}
        "#
}

#[test]
fn test_check_valid_schema() {
    let rudof = Rudof::new(RudofConfig::default());
    let schema_str = create_valid_schema();
    let schema = InputSpec::str(schema_str);
    let mut output = Cursor::new(Vec::new());

    let result = check_shex_schema(&rudof, &schema, None, Some("http://example.org/"), &mut output);
    let output_str = String::from_utf8(output.into_inner()).unwrap();

    assert!(result.is_ok());
    let is_valid = result.unwrap();
    assert!(is_valid, "Expected schema to be valid");

    assert!(output_str.contains("Schema is valid"));
    assert!(output_str.contains("well-formed"));
    assert!(output_str.contains("no negative cycles"));

    println!(
        "\n===== test_check_valid_schema =====\n{}\n====================================",
        output_str
    );
}

#[test]
fn test_check_malformed_schema() {
    let rudof = Rudof::new(RudofConfig::default());
    let schema_str = create_malformed_schema();
    let schema = InputSpec::str(schema_str);
    let mut output = Cursor::new(Vec::new());

    let result = check_shex_schema(&rudof, &schema, None, Some("http://example.org/"), &mut output);

    assert!(result.is_ok());
    let is_valid = result.unwrap();
    assert!(!is_valid, "Expected schema to be invalid (malformed)");

    let output_str = String::from_utf8(output.into_inner()).unwrap();
    assert!(output_str.contains("malformed"));
    assert!(output_str.contains("Error during parsing"));

    println!(
        "\n===== test_check_malformed_schema =====\n{}\n========================================",
        output_str
    );
}

#[test]
fn test_check_schema_with_neg_cycle() {
    let rudof = Rudof::new(RudofConfig::default());
    let schema_str = create_schema_with_neg_cycle();
    let schema = InputSpec::str(schema_str);
    let mut output = Cursor::new(Vec::new());

    let result = check_shex_schema(&rudof, &schema, None, Some("http://example.org/"), &mut output);

    assert!(result.is_ok());
    let is_valid = result.unwrap();
    assert!(!is_valid, "Expected schema to be invalid (negative cycle)");

    let output_str = String::from_utf8(output.into_inner()).unwrap();
    assert!(output_str.contains("negative cycles"));
    assert!(output_str.contains("dependency graph"));
    assert!(output_str.contains("Negative cycle"));

    println!(
        "\n===== test_check_schema_with_neg_cycle =====\n{}\n=============================================",
        output_str
    );
}

#[test]
fn test_check_schema_with_multiple_neg_cycles() {
    let rudof = Rudof::new(RudofConfig::default());
    let schema_str = create_schema_with_multiple_neg_cycles();
    let schema = InputSpec::str(schema_str);
    let mut output = Cursor::new(Vec::new());

    let result = check_shex_schema(&rudof, &schema, None, Some("http://example.org/"), &mut output);

    assert!(result.is_ok());
    let is_valid = result.unwrap();
    assert!(!is_valid, "Expected schema to be invalid (multiple negative cycles)");

    let output_str = String::from_utf8(output.into_inner()).unwrap();

    assert!(output_str.contains("negative cycles"));
    assert!(output_str.contains("Negative cycle #1"));

    println!(
        "\n===== test_check_schema_with_multiple_neg_cycles =====\n{}\n========================================================",
        output_str
    );
}

#[test]
fn test_check_complex_valid_schema() {
    let rudof = Rudof::new(RudofConfig::default());
    let schema_str = create_complex_valid_schema();
    let schema = InputSpec::str(schema_str);
    let mut output = Cursor::new(Vec::new());

    let result = check_shex_schema(&rudof, &schema, None, Some("http://example.org/"), &mut output);

    assert!(result.is_ok());
    let is_valid = result.unwrap();
    assert!(is_valid, "Expected complex schema to be valid");

    let output_str = String::from_utf8(output.into_inner()).unwrap();
    assert!(output_str.contains("Schema is valid"));

    println!(
        "\n===== test_check_complex_valid_schema =====\n{}\n============================================",
        output_str
    );
}
