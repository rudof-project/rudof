use crate::{
    Rudof, RudofConfig,
    api::data::implementations::load_data,
    api::shacl::implementations::load_shacl_schema::load_shacl_schema,
    api::shacl::implementations::serialize_shacl_schema::serialize_shacl_schema,
    formats::{DataFormat, InputSpec, ShaclFormat},
};

/// Helper: serialize current SHACL schema to string
fn serialize_to_string(rudof: &Rudof, format: Option<ShaclFormat>) -> String {
    let mut buffer = Vec::new();

    serialize_shacl_schema(rudof, format.as_ref(), &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_extract_shacl_shapes_from_loaded_data() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load RDF data that contains SHACL shapes
    let data_with_shapes = InputSpec::str(
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
            ] .

        ex:CompanyShape
            a sh:NodeShape ;
            sh:targetClass ex:Company ;
            sh:property [
                sh:path ex:companyName ;
                sh:datatype xsd:string ;
            ] .
        "#,
    );

    // Load data into rudof
    load_data(
        &mut rudof,
        Some(&[data_with_shapes]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();

    // Extract SHACL shapes from loaded data (schema parameter is None)
    load_shacl_schema(&mut rudof, None, None, None, None).unwrap();

    let serialized = serialize_to_string(&rudof, Some(ShaclFormat::Turtle));

    assert!(serialized.contains("PersonShape") || serialized.contains("person"));
    assert!(serialized.contains("CompanyShape") || serialized.contains("company"));
    assert!(serialized.contains("name") || serialized.contains("age"));

    println!(
        "\n===== test_extract_shacl_shapes_from_loaded_data =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_extract_shacl_shapes_mixed_data() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load RDF data that contains both SHACL shapes and instance data
    let mixed_data = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        # SHACL Shape
        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
            ] .

        # Instance data
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
        Some(&[mixed_data]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();

    load_shacl_schema(&mut rudof, None, None, None, None).unwrap();

    let serialized = serialize_to_string(&rudof, Some(ShaclFormat::Turtle));

    // Should contain the shape
    assert!(serialized.contains("PersonShape") || serialized.contains("person"));
    assert!(serialized.contains("name"));

    println!(
        "\n===== test_extract_shacl_shapes_mixed_data =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_extract_shacl_shapes_from_data_no_shapes() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Load RDF data without SHACL shapes (just instance data)
    let data_no_shapes = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .

        ex:Alice
            a ex:Person ;
            ex:name "Alice Smith" ;
            ex:age 30 .
        "#,
    );

    load_data(
        &mut rudof,
        Some(&[data_no_shapes]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None
    )
    .unwrap();

    // This should succeed but extract an empty or minimal schema
    let result = load_shacl_schema(&mut rudof, None, None, None, None);

    // The result depends on implementation - it might succeed with empty schema
    // or might succeed with no shapes found
    assert!(result.is_ok());

    println!("\n===== test_extract_shacl_shapes_from_data_no_shapes =====");
}

#[test]
fn test_extract_shacl_shapes_no_data_loaded() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // Try to extract shapes without loading data first
    let result = load_shacl_schema(&mut rudof, None, None, None, None);

    // Should fail because no data is loaded
    assert!(result.is_err());
}

#[test]
fn test_extract_and_serialize_complex_shapes() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let complex_data = InputSpec::str(
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
            ] ;
            sh:property [
                sh:path ex:city ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
            ] ;
            sh:property [
                sh:path ex:postalCode ;
                sh:datatype xsd:string ;
                sh:pattern "^[0-9]{5}$" ;
            ] .

        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
                sh:maxCount 1 ;
            ] ;
            sh:property [
                sh:path ex:age ;
                sh:datatype xsd:integer ;
                sh:minInclusive 0 ;
                sh:maxInclusive 150 ;
            ] ;
            sh:property [
                sh:path ex:address ;
                sh:node ex:AddressShape ;
                sh:minCount 1 ;
            ] ;
            sh:property [
                sh:path ex:email ;
                sh:datatype xsd:string ;
                sh:pattern "^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$" ;
            ] .
        "#,
    );

    load_data(
        &mut rudof,
        Some(&[complex_data]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();

    load_shacl_schema(&mut rudof, None, None, None, None).unwrap();

    let serialized = serialize_to_string(&rudof, Some(ShaclFormat::Turtle));

    assert!(serialized.contains("PersonShape") || serialized.contains("person"));
    assert!(serialized.contains("AddressShape") || serialized.contains("address"));
    assert!(serialized.contains("email") || serialized.contains("age"));

    println!(
        "\n===== test_extract_and_serialize_complex_shapes =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_extract_shapes_then_load_separate_schema() {
    let mut rudof = Rudof::new(RudofConfig::default());

    // First load data with shapes
    let data_with_shapes = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:Shape1
            a sh:NodeShape ;
            sh:targetClass ex:Class1 ;
            sh:property [
                sh:path ex:prop1 ;
                sh:datatype xsd:string ;
            ] .
        "#,
    );

    load_data(
        &mut rudof,
        Some(&[data_with_shapes]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();

    load_shacl_schema(&mut rudof, None, None, None, None).unwrap();

    let serialized1 = serialize_to_string(&rudof, Some(ShaclFormat::Turtle));
    assert!(serialized1.contains("Shape1") || serialized1.contains("Class1"));

    // Now load a separate schema - should replace the extracted one
    let separate_schema = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:Shape2
            a sh:NodeShape ;
            sh:targetClass ex:Class2 ;
            sh:property [
                sh:path ex:prop2 ;
                sh:datatype xsd:integer ;
            ] .
        "#,
    );

    load_shacl_schema(
        &mut rudof,
        Some(&separate_schema),
        Some(&ShaclFormat::Turtle),
        None,
        None,
    )
    .unwrap();

    let serialized2 = serialize_to_string(&rudof, Some(ShaclFormat::Turtle));

    assert!(!serialized2.contains("Shape1") && !serialized2.contains("Class1"));
    assert!(serialized2.contains("Shape2") || serialized2.contains("Class2"));

    println!(
        "\n===== test_extract_shapes_then_load_separate_schema =====\n{}============================================",
        serialized2
    );
}

#[test]
fn test_extract_shacl_shapes_with_lists() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let data_with_lists = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:PersonShape
            a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:status ;
                sh:in ( "active" "inactive" "pending" ) ;
            ] ;
            sh:property [
                sh:path ex:role ;
                sh:in ( "admin" "user" "guest" ) ;
            ] .
        "#,
    );

    load_data(
        &mut rudof,
        Some(&[data_with_lists]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();

    load_shacl_schema(&mut rudof, None, None, None, None).unwrap();

    let serialized = serialize_to_string(&rudof, Some(ShaclFormat::Turtle));

    assert!(serialized.contains("PersonShape") || serialized.contains("person"));
    assert!(serialized.contains("status") || serialized.contains("role"));

    println!(
        "\n===== test_extract_shacl_shapes_with_lists =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_extract_shapes_multiple_data_sources() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let data_source1 = InputSpec::str(
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

    let data_source2 = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:CompanyShape
            a sh:NodeShape ;
            sh:targetClass ex:Company ;
            sh:property [
                sh:path ex:companyName ;
                sh:datatype xsd:string ;
            ] .
        "#,
    );

    // Load multiple data sources
    load_data(
        &mut rudof,
        Some(&[data_source1, data_source2]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        Some(true),
        None
    )
    .unwrap();

    load_shacl_schema(&mut rudof, None, None, None, None).unwrap();

    let serialized = serialize_to_string(&rudof, Some(ShaclFormat::Turtle));

    // Should contain shapes from both sources
    assert!(serialized.contains("PersonShape") || serialized.contains("person"));
    assert!(serialized.contains("CompanyShape") || serialized.contains("company"));

    println!(
        "\n===== test_extract_shapes_multiple_data_sources =====\n{}============================================",
        serialized
    );
}

#[test]
fn test_extract_shapes_with_merge() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let data_source1 = InputSpec::str(
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

    // Load first data source
    load_data(
        &mut rudof,
        Some(&[data_source1]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        None,
        None
    )
    .unwrap();

    let data_source2 = InputSpec::str(
        r#"
        @prefix ex: <http://example.org/> .
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:CompanyShape
            a sh:NodeShape ;
            sh:targetClass ex:Company ;
            sh:property [
                sh:path ex:companyName ;
                sh:datatype xsd:string ;
            ] .
        "#,
    );

    // Load second data source with merge=true
    load_data(
        &mut rudof,
        Some(&[data_source2]),
        Some(&DataFormat::Turtle),
        None,
        None,
        None,
        Some(true),
        None
    )
    .unwrap();

    load_shacl_schema(&mut rudof, None, None, None, None).unwrap();

    let serialized = serialize_to_string(&rudof, Some(ShaclFormat::Turtle));

    // Should contain shapes from both sources when merged
    assert!(serialized.contains("PersonShape") || serialized.contains("person"));
    assert!(serialized.contains("CompanyShape") || serialized.contains("company"));

    println!(
        "\n===== test_extract_shapes_with_merge =====\n{}============================================",
        serialized
    );
}
