use crate::{
    Rudof, RudofConfig,
    api::data::implementations::load_data,
    api::shex::implementations::load_shapemap::load_shapemap,
    api::shex::implementations::load_shex_schema::load_shex_schema,
    api::shex::implementations::serialize_shapemap::serialize_shapemap,
    formats::{DataFormat, InputSpec, ShExFormat, ShapeMapFormat},
};
//use std::str::FromStr;

/// Helper: serialize current ShapeMap to string
fn serialize_shapemap_to_string(rudof: &mut Rudof, format: Option<ShapeMapFormat>) -> String {
    let mut buffer = Vec::new();

    serialize_shapemap(rudof, format.as_ref(), Some(true), &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_load_shapemap_compact_success() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load RDF data
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
        None
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

    // Load ShapeMap
    let shapemap = InputSpec::str(r#"ex:alice@ex:PersonShape"#);

    load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    let serialized = serialize_shapemap_to_string(&mut rudof, Some(ShapeMapFormat::Compact));

    assert!(serialized.contains("alice"));
    assert!(serialized.contains("PersonShape"));

    println!(
        "\n===== test_load_shapemap_compact_success =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_load_shapemap_replace() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load RDF data
    let data = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           ex:alice ex:name "Alice" .
           ex:bob ex:name "Bob" ."#,
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

    // Load ShEx schema
    let schema = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape { ex:name xsd:string }
           ex:EmployeeShape { ex:name xsd:string }"#,
    );

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    // Load first shapemap
    let shapemap1 = InputSpec::str(r#"ex:alice@ex:PersonShape"#);

    load_shapemap(&mut rudof, &shapemap1, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    // Load second shapemap (should replace)
    let shapemap2 = InputSpec::str(r#"ex:bob@ex:EmployeeShape"#);

    load_shapemap(&mut rudof, &shapemap2, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    let serialized = serialize_shapemap_to_string(&mut rudof, Some(ShapeMapFormat::Compact));

    assert!(!serialized.contains("alice"));
    assert!(serialized.contains("bob"));
    assert!(serialized.contains("EmployeeShape"));

    println!(
        "\n===== test_load_shapemap_replace =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_load_shapemap_no_data_error() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load schema but no data
    let schema = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape { ex:name xsd:string }"#,
    );

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    // Try to load shapemap without data
    let shapemap = InputSpec::str(r#"ex:alice@ex:PersonShape"#);

    let result = load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None);

    assert!(result.is_err());
}

#[test]
fn test_load_shapemap_no_schema_error() {
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
        None
    )
    .unwrap();

    // Try to load shapemap without schema
    let shapemap = InputSpec::str(r#"ex:alice@ex:PersonShape"#);

    let result = load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None);

    assert!(result.is_err());
}

#[test]
fn test_load_shapemap_invalid_syntax() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load data and schema
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
        None
    )
    .unwrap();

    let schema = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape { ex:name xsd:string }"#,
    );

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    // Invalid shapemap syntax
    let invalid_shapemap = InputSpec::str("not valid shapemap @@@");

    let result = load_shapemap(
        &mut rudof,
        &invalid_shapemap,
        Some(&ShapeMapFormat::Compact),
        None,
        None,
    );

    assert!(result.is_err());
}

#[test]
fn test_load_shapemap_with_base_nodes() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load data
    let data = InputSpec::str(r#"<alice> <http://example.org/name> "Alice" ."#);

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

    // Load schema
    let schema = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape { ex:name xsd:string }"#,
    );

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    // Load shapemap with base for nodes
    let shapemap = InputSpec::str(r#"<alice>@ex:PersonShape"#);

    load_shapemap(
        &mut rudof,
        &shapemap,
        Some(&ShapeMapFormat::Compact),
        Some("http://example.org/"),
        None,
    )
    .unwrap();

    let serialized = serialize_shapemap_to_string(&mut rudof, Some(ShapeMapFormat::Compact));

    assert!(serialized.contains("alice"));

    println!(
        "\n===== test_load_shapemap_with_base_nodes =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_serialize_shapemap_json() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load data
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
        None
    )
    .unwrap();

    // Load schema
    let schema = InputSpec::str(
        r#"PREFIX ex: <http://example.org/>
           PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
           ex:PersonShape { ex:name xsd:string }"#,
    );

    load_shex_schema(&mut rudof, &schema, Some(&ShExFormat::ShExC), None, None).unwrap();

    // Load shapemap
    let shapemap = InputSpec::str(r#"ex:alice@ex:PersonShape"#);

    load_shapemap(&mut rudof, &shapemap, Some(&ShapeMapFormat::Compact), None, None).unwrap();

    let serialized = serialize_shapemap_to_string(&mut rudof, Some(ShapeMapFormat::Json));

    assert!(serialized.contains("alice"));
    assert!(serialized.contains("PersonShape"));
    // JSON format check
    assert!(serialized.contains("{") && serialized.contains("}"));

    println!(
        "\n===== test_serialize_shapemap_json =====\n{}\n============================================",
        serialized
    );
}
