use crate::{
    Rudof, RudofConfig,
    api::data::implementations::load_data,
    api::pgschema::implementations::load_pgschema::load_pgschema,
    api::pgschema::implementations::validate_pgschema::validate_pgschema,
    api::pgschema::implementations::serialize_pgschema_validation_results::serialize_pgschema_validation_results,
    api::pgschema::implementations::load_typemap::load_typemap,
    formats::{InputSpec, DataFormat, ResultPgSchemaValidationFormat},
};
use std::str::FromStr;

/// Helper: serialize validation results to string
fn serialize_validation_to_string(
    rudof: &Rudof, 
    format: Option<ResultPgSchemaValidationFormat>,
    show_colors: Option<bool>,
) -> String {
    let mut buffer = Vec::new();

    serialize_pgschema_validation_results(
        rudof,
        format.as_ref(),
        show_colors,
        &mut buffer,
    )
    .unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_validate_and_serialize_pgschema() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load PG data
    let pg_data = InputSpec::from_str(
        r#"
(n1 {"Student"}["name": "Alice", "age": 23])
(n2_wrong {"Student"}["name": "Bob", "age": 12])
(n3_wrong {"Student"}["name": "Carol", "age": "unknown"])
        "#,
    )
    .unwrap();

    load_data(&mut rudof, Some(&[pg_data]), Some(&DataFormat::Pg), None, None, None, None).unwrap();

    // Load PG schema
    let pg_schema = InputSpec::from_str(
        r#"
CREATE NODE TYPE ( AdultStudentType: Student {
    name: STRING ,
    age: INTEGER CHECK > 18
})
        "#,
    )
    .unwrap();

    load_pgschema(&mut rudof, &pg_schema, None).unwrap();

    // Load typemap
    let typemap = InputSpec::from_str(
        r#"
n1: AdultStudentType,
n2_wrong: AdultStudentType,
n3_wrong: AdultStudentType
        "#,
    )
    .unwrap();

    load_typemap(&mut rudof, &typemap).unwrap();

    // Validate
    validate_pgschema(&mut rudof).unwrap();

    // Serialize results in different formats
    let compact = serialize_validation_to_string(&rudof, Some(ResultPgSchemaValidationFormat::Compact), None);
    println!("\n===== Compact Format =====\n{}\n", compact);

    let json = serialize_validation_to_string(&rudof, Some(ResultPgSchemaValidationFormat::Json), None);
    println!("===== JSON Format =====\n{}\n", json);

    let csv = serialize_validation_to_string(&rudof, Some(ResultPgSchemaValidationFormat::Csv), Some(true));
    println!("===== CSV Format =====\n{}\n", csv);

    // Assertions - should find validation errors
    // n1 should pass (age 23 > 18)
    // n2_wrong should fail (age 12 <= 18)
    // n3_wrong should fail (age is "unknown", not INTEGER)
    assert!(compact.contains("n2_wrong") || compact.contains("n3_wrong") || compact.contains("error") || compact.contains("fail"));
    assert!(json.contains("n2_wrong") || json.contains("n3_wrong"));
    assert!(csv.contains("n2_wrong") || csv.contains("n3_wrong"));
    println!("===== test_validate_and_serialize_pgschema =====\nValidation completed successfully");
}