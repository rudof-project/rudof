use crate::{
    Rudof, RudofConfig,
    api::shex::implementations::load_shex_schema::load_shex_schema,
    api::shex::implementations::load_shapemap::load_shapemap,
    api::shex::implementations::validate_shex::validate_shex,
    api::shex::implementations::serialize_shex_validation_results::serialize_shex_validation_results,
    api::data::implementations::load_data,
    formats::{InputSpec, ShExFormat, ShapeMapFormat, DataFormat, ResultShExValidationFormat, ShExValidationSortByMode},
};
use std::str::FromStr;

/// Helper: serialize validation results to string
fn serialize_validation_results_to_string(
    rudof: &mut Rudof,
    sort_order: Option<ShExValidationSortByMode>,
    format: Option<ResultShExValidationFormat>,
) -> String {
    let mut buffer = Vec::new();

    serialize_shex_validation_results(
        rudof,
        sort_order.as_ref(),
        format.as_ref(),
        &mut buffer,
    )
    .unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_validate_shex_success() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load RDF data
    let data = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ;
                    ex:age 30 ."#,
    )
    .unwrap();

    load_data(&mut rudof, Some(&[data]), Some(&DataFormat::Turtle), None, None, None, None).unwrap();

    // Load ShEx schema
    let schema = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape {
             ex:name xsd:string ;
             ex:age xsd:integer
           }"#,
    )
    .unwrap();

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    // Load ShapeMap
    let shapemap = InputSpec::from_str(
        r#"ex:alice@ex:PersonShape"#,
    )
    .unwrap();

    load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    // Validate
    validate_shex(&mut rudof).unwrap();

    assert!(rudof.shex_validation_results.is_some());

    let serialized = serialize_validation_results_to_string(&mut rudof, None, Some(ResultShExValidationFormat::Compact));

    assert!(serialized.contains("Results"));

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
    let data = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ."#,
    )
    .unwrap();

    load_data(&mut rudof, Some(&[data]), Some(&DataFormat::Turtle), None, None, None, None).unwrap();

    // Try to validate without schema
    let result = validate_shex(&mut rudof);

    assert!(result.is_err());
}

#[test]
fn test_validate_shex_no_shapemap_error() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load data and schema but no shapemap
    let data = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ."#,
    )
    .unwrap();

    load_data(&mut rudof, Some(&[data]), Some(&DataFormat::Turtle), None, None, None, None).unwrap();

    let schema = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape { ex:name xsd:string }"#,
    )
    .unwrap();

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    // Try to validate without shapemap
    let result = validate_shex(&mut rudof);

    assert!(result.is_err());
}

#[test]
fn test_validate_shex_validation_failure() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load RDF data that doesn't conform to schema
    let data = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ;
                    ex:age "not an integer" ."#,
    )
    .unwrap();

    load_data(&mut rudof, Some(&[data]), Some(&DataFormat::Turtle), None, None, None, None).unwrap();

    // Load ShEx schema expecting integer
    let schema = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape {
             ex:name xsd:string ;
             ex:age xsd:integer
           }"#,
    )
    .unwrap();

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    // Load ShapeMap
    let shapemap = InputSpec::from_str(
        r#"ex:alice@ex:PersonShape"#,
    )
    .unwrap();

    load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    // Validate - should succeed but with validation failure result
    validate_shex(&mut rudof).unwrap();

    assert!(rudof.shex_validation_results.is_some());

    let serialized = serialize_validation_results_to_string(&mut rudof, None, Some(ResultShExValidationFormat::Compact));

    println!(
        "\n===== test_validate_shex_validation_failure =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_serialize_validation_results_compact() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Setup and validate
    let data = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ;
                    ex:age 30 ."#,
    )
    .unwrap();

    load_data(&mut rudof, Some(&[data]), Some(&DataFormat::Turtle), None, None, None, None).unwrap();

    let schema = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape {
             ex:name xsd:string ;
             ex:age xsd:integer
           }"#,
    )
    .unwrap();

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    let shapemap = InputSpec::from_str(
        r#"ex:alice@ex:PersonShape"#,
    )
    .unwrap();

    load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    validate_shex(&mut rudof).unwrap();

    // Test compact format
    let serialized = serialize_validation_results_to_string(&mut rudof, None, Some(ResultShExValidationFormat::Compact));

    assert!(serialized.contains("Results"));

    println!(
        "\n===== test_serialize_validation_results_compact =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_serialize_validation_results_json() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Setup and validate
    let data = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ;
                    ex:age 30 ."#,
    )
    .unwrap();

    load_data(&mut rudof, Some(&[data]), Some(&DataFormat::Turtle), None, None, None, None).unwrap();

    let schema = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape {
             ex:name xsd:string ;
             ex:age xsd:integer
           }"#,
    )
    .unwrap();

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    let shapemap = InputSpec::from_str(
        r#"ex:alice@ex:PersonShape"#,
    )
    .unwrap();

    load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    validate_shex(&mut rudof).unwrap();

    // Test JSON format
    let serialized = serialize_validation_results_to_string(&mut rudof, None, Some(ResultShExValidationFormat::Json));

    assert!(serialized.contains("Results"));
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
    let data = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ;
                    ex:age 30 ."#,
    )
    .unwrap();

    load_data(&mut rudof, Some(&[data]), Some(&DataFormat::Turtle), None, None, None, None).unwrap();

    let schema = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape {
             ex:name xsd:string ;
             ex:age xsd:integer
           }"#,
    )
    .unwrap();

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    let shapemap = InputSpec::from_str(
        r#"ex:alice@ex:PersonShape"#,
    )
    .unwrap();

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
    let data = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ;
                    ex:age 30 ."#,
    )
    .unwrap();

    load_data(&mut rudof, Some(&[data]), Some(&DataFormat::Turtle), None, None, None, None).unwrap();

    let schema = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape {
             ex:name xsd:string ;
             ex:age xsd:integer
           }"#,
    )
    .unwrap();

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    let shapemap = InputSpec::from_str(
        r#"ex:alice@ex:PersonShape"#,
    )
    .unwrap();

    load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    validate_shex(&mut rudof).unwrap();

    // Test details format
    let serialized = serialize_validation_results_to_string(&mut rudof, None, Some(ResultShExValidationFormat::Details));

    assert!(serialized.contains("Results"));

    println!(
        "\n===== test_serialize_validation_results_details =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_serialize_validation_results_no_results_error() {
    let rudof = Rudof::new(RudofConfig::default());

    let mut buffer = Vec::new();

    let result = serialize_shex_validation_results(
        &rudof,
        None,
        Some(&ResultShExValidationFormat::Compact),
        &mut buffer,
    );

    assert!(result.is_err());
}

#[test]
fn test_validate_shex_multiple_nodes() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load RDF data with multiple nodes
    let data = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" ;
                    ex:age 30 .
           ex:bob ex:name "Bob" ;
                  ex:age 25 ."#,
    )
    .unwrap();

    load_data(&mut rudof, Some(&[data]), Some(&DataFormat::Turtle), None, None, None, None).unwrap();

    // Load ShEx schema
    let schema = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape {
             ex:name xsd:string ;
             ex:age xsd:integer
           }"#,
    )
    .unwrap();

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    // Load ShapeMap with multiple associations
    let shapemap = InputSpec::from_str(
        r#"ex:alice@ex:PersonShape,
           ex:bob@ex:PersonShape"#,
    )
    .unwrap();

    load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    // Validate
    validate_shex(&mut rudof).unwrap();

    assert!(rudof.shex_validation_results.is_some());

    let serialized = serialize_validation_results_to_string(&mut rudof, None, Some(ResultShExValidationFormat::Compact));

    assert!(serialized.contains("Results"));

    println!(
        "\n===== test_validate_shex_multiple_nodes =====\n{}============================================",
        serialized
    );
}