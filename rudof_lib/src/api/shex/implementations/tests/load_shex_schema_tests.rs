use crate::{
    Rudof, RudofConfig,
    api::shex::implementations::load_shex_schema::load_shex_schema,
    api::shex::implementations::serialize_shex_schema::serialize_shex_schema,
    formats::{InputSpec, ShExFormat},
};
use std::str::FromStr;

/// Helper: serialize current ShEx schema to string
fn serialize_to_string(rudof: &mut Rudof, format: Option<ShExFormat>) -> String {
    let mut buffer = Vec::new();

    serialize_shex_schema(
        rudof,
        None,
        Some(true),
        Some(true),
        Some(true),
        Some(true),
        None,
        format.as_ref(),
        &mut buffer,
    )
    .unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_load_shex_schema_shexc_success() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema = InputSpec::from_str(
        r#"
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           <PersonShape> {
             <name> xsd:string ;
             <age> xsd:integer
           }

           <EmployeeShape> {
             <employeeId> xsd:string ;
             <worksFor> <PersonShape>
           }
        "#,
    )
    .unwrap();

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), Some(&"http://example.org/"), None).unwrap();

    let serialized = serialize_to_string(&mut rudof, Some(ShExFormat::ShExC));

    assert!(serialized.contains("PersonShape"));
    assert!(serialized.contains("name"));
    assert!(serialized.contains("age"));

    println!(
        "\n===== test_load_shex_schema_shexc_success =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_load_shex_schema_replace() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema1 = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:Shape1 { ex:prop1 xsd:string }"#,
    )
    .unwrap();

    let schema2 = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:Shape2 { ex:prop2 xsd:integer }"#,
    )
    .unwrap();

    load_shex_schema(&mut rudof, &schema1, Some(&ShExFormat::ShExC), None, None).unwrap();

    load_shex_schema(&mut rudof, &schema2, Some(&ShExFormat::ShExC), None, None).unwrap();

    let serialized = serialize_to_string(&mut rudof, Some(ShExFormat::ShExC));

    assert!(!serialized.contains("Shape1"));
    assert!(serialized.contains("Shape2"));

    println!(
        "\n===== test_load_shex_schema_replace =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_load_shex_schema_invalid_shexc() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let invalid = InputSpec::from_str("not valid shexc syntax {{{").unwrap();

    let result = load_shex_schema(&mut rudof, &invalid, Some(&ShExFormat::ShExC), None, None);

    assert!(result.is_err());
}

#[test]
fn test_load_shex_schema_with_base() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema = InputSpec::from_str(
        r#"
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
        <PersonShape> {
             <name> xsd:string
           }"#,
    )
    .unwrap();

    load_shex_schema(
        &mut rudof,
        &schema,
        Some(&ShExFormat::ShExC),
        Some("http://example.org/"),
        None,
    )
    .unwrap();

    let serialized = serialize_to_string(&mut rudof, Some(ShExFormat::ShExC));

    assert!(serialized.contains("PersonShape"));

    println!(
        "\n===== test_load_shex_schema_with_base =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_load_shex_schema_negative_cycles() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Schema con ciclos negativos
    let schema = InputSpec::from_str(
        r#"PREFIX ex: <http://example.org/>
           ex:Shape1 NOT @ex:Shape2
           ex:Shape2 NOT @ex:Shape1"#,
    )
    .unwrap();

    let result = load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None);

    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("negative cycle"));
    }
}
