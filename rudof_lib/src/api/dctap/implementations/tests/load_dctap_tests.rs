use crate::{
    Rudof, RudofConfig,
    api::dctap::implementations::load_dctap::load_dctap,
    api::dctap::implementations::serialize_dctap::serialize_dctap,
    formats::{InputSpec, ResultDCTapFormat},
};
//use std::str::FromStr;

/// Helper: serialize current DCTap to string
fn serialize_to_string(rudof: &Rudof, format: Option<ResultDCTapFormat>) -> String {
    let mut buffer = Vec::new();

    serialize_dctap(rudof, format.as_ref(), &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_load_and_serialize_basic_dctap_csv() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let dctap_input = InputSpec::str(
        r#"shapeID,propertyID,mandatory,repeatable,valueDataType
:Person,rdf:type,true,false,
:Person,schema:name,true,false,xsd:string
:Person,schema:age,false,false,xsd:integer
:Person,schema:memberOf,false,true,:Organization
:Organization,rdf:type,true,false,
:Organization,schema:name,true,false,xsd:string
:Organization,schema:location,false,false,xsd:string"#,
    );

    load_dctap(&mut rudof, &dctap_input, None).unwrap();

    let serialized = serialize_to_string(&rudof, None);

    assert!(serialized.contains("Person"));
    assert!(serialized.contains("Organization"));
    assert!(serialized.contains("schema:name") || serialized.contains("name"));
    assert!(serialized.contains("schema:age") || serialized.contains("age"));

    println!(
        "\n===== test_load_and_serialize_basic_dctap_csv =====\n{}\n============================================",
        serialized
    );
}

#[test]
fn test_serialize_dctap_json_format() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let dctap_input = InputSpec::str(
        r#"shapeID,propertyID,mandatory,repeatable
:Book,dc:title,true,false
:Book,dc:creator,true,true
:Book,dc:date,false,false"#,
    );

    load_dctap(&mut rudof, &dctap_input, None).unwrap();

    let serialized = serialize_to_string(&rudof, Some(ResultDCTapFormat::Json));

    assert!(serialized.contains("Book"));
    assert!(serialized.contains("dc:title") || serialized.contains("title"));
    assert!(serialized.contains("dc:creator") || serialized.contains("creator"));

    // Verify it's valid JSON
    assert!(serde_json::from_str::<serde_json::Value>(&serialized).is_ok());

    println!(
        "\n===== test_serialize_dctap_json_format =====\n{}\n============================================",
        serialized
    );
}
