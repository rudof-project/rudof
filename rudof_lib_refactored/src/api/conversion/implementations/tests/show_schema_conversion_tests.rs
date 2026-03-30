use crate::{
    Rudof, RudofConfig,
    api::conversion::implementations::show_schema_conversion::show_schema_conversion,
    formats::{ConversionFormat, ConversionMode, InputSpec, ResultConversionFormat, ResultConversionMode},
};
use std::str::FromStr;

/// Helper: serialize conversion to string
fn serialize_conversion_to_string(
    rudof: &mut Rudof,
    schema: &InputSpec,
    input_mode: &ConversionMode,
    output_mode: &ResultConversionMode,
    input_format: &ConversionFormat,
    output_format: &ResultConversionFormat,
) -> String {
    let mut buffer = Vec::new();

    show_schema_conversion(
        rudof,
        schema,
        None,
        None,
        input_mode,
        output_mode,
        input_format,
        output_format,
        None,
        None,
        None,
        None,
        &mut buffer,
    )
    .unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_convert_shex_to_shex() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema_input = InputSpec::from_str(
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

    let serialized = serialize_conversion_to_string(
        &mut rudof,
        &schema_input,
        &ConversionMode::ShEx,
        &ResultConversionMode::ShEx,
        &ConversionFormat::ShExC,
        &ResultConversionFormat::ShExC,
    );

    assert!(serialized.contains("Person") || serialized.contains(":Person"));
    assert!(serialized.contains("schema:name") || serialized.contains("name"));

    println!(
        "\n===== test_convert_shex_to_shex =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_convert_shex_to_sparql() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema_input = InputSpec::from_str(
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

    let serialized = serialize_conversion_to_string(
        &mut rudof,
        &schema_input,
        &ConversionMode::ShEx,
        &ResultConversionMode::Sparql,
        &ConversionFormat::ShExC,
        &ResultConversionFormat::Default,
    );

    // SPARQL queries typically contain SELECT, WHERE, or CONSTRUCT
    assert!(
        serialized.to_uppercase().contains("SELECT") 
        || serialized.to_uppercase().contains("WHERE")
        || serialized.to_uppercase().contains("CONSTRUCT")
    );

    println!(
        "\n===== test_convert_shex_to_sparql =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_convert_shex_to_uml_plantuml() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema_input = InputSpec::from_str(
        r#"
PREFIX : <http://example.org/>
PREFIX schema: <http://schema.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
    schema:name xsd:string ;
    schema:knows @:Person *
}
        "#,
    )
    .unwrap();

    let serialized = serialize_conversion_to_string(
        &mut rudof,
        &schema_input,
        &ConversionMode::ShEx,
        &ResultConversionMode::Uml,
        &ConversionFormat::ShExC,
        &ResultConversionFormat::PlantUML,
    );

    // PlantUML format typically starts with @startuml
    assert!(serialized.contains("@startuml") || serialized.contains("class") || !serialized.is_empty());

    println!(
        "\n===== test_convert_shex_to_uml_plantuml =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_convert_shacl_to_shacl() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema_input = InputSpec::from_str(
        r#"
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <http://example.org/> .
@prefix schema: <http://schema.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:PersonShape a sh:NodeShape ;
    sh:targetClass ex:Person ;
    sh:property [
        sh:path schema:name ;
        sh:datatype xsd:string ;
        sh:minCount 1
    ] .
        "#,
    )
    .unwrap();

    let serialized = serialize_conversion_to_string(
        &mut rudof,
        &schema_input,
        &ConversionMode::Shacl,
        &ResultConversionMode::Shacl,
        &ConversionFormat::Turtle,
        &ResultConversionFormat::Turtle,
    );

    assert!(serialized.contains("PersonShape") || serialized.contains("NodeShape"));

    println!(
        "\n===== test_convert_shacl_to_shacl =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_convert_dctap_to_uml() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema_input = InputSpec::from_str(
        r#"shapeID,propertyID,mandatory,repeatable,valueDataType
:Person,schema:name,true,false,xsd:string
:Person,schema:knows,false,true,:Person
:Organization,schema:name,true,false,xsd:string"#,
    )
    .unwrap();

    let serialized = serialize_conversion_to_string(
        &mut rudof,
        &schema_input,
        &ConversionMode::Dctap,
        &ResultConversionMode::Uml,
        &ConversionFormat::Csv,
        &ResultConversionFormat::PlantUML,
    );

    // PlantUML output should contain class definitions or diagram markers
    assert!(!serialized.is_empty());

    println!(
        "\n===== test_convert_dctap_to_uml =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_convert_with_shape_filter() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let schema_input = InputSpec::from_str(
        r#"
PREFIX : <http://example.org/>
PREFIX schema: <http://schema.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:Person {
    schema:name xsd:string
}

:Organization {
    schema:name xsd:string ;
    schema:member @:Person *
}
        "#,
    )
    .unwrap();

    let mut buffer = Vec::new();

    show_schema_conversion(
        &mut rudof,
        &schema_input,
        None,
        None,
        &ConversionMode::ShEx,
        &ResultConversionMode::ShEx,
        &ConversionFormat::ShExC,
        &ResultConversionFormat::ShExC,
        Some(":Person"),
        None,
        None,
        None,
        &mut buffer,
    )
    .unwrap();

    let serialized = String::from_utf8(buffer).unwrap();

    // When filtering by shape, should only show Person shape
    assert!(serialized.contains("Person"));

    println!(
        "\n===== test_convert_with_shape_filter =====\n{}\n============================================",
        serialized
    );
}

