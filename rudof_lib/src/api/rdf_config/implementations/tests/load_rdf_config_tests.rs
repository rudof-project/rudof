use crate::{
    Rudof, RudofConfig,
    api::rdf_config::implementations::load_rdf_config::load_rdf_config,
    api::rdf_config::implementations::serialize_rdf_config::serialize_rdf_config,
    formats::{InputSpec, ResultRdfConfigFormat},
};

/// Helper: serialize current RDF config to string
fn serialize_to_string(rudof: &Rudof, format: Option<ResultRdfConfigFormat>) -> String {
    let mut buffer = Vec::new();

    serialize_rdf_config(rudof, format.as_ref(), &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

#[test]
fn test_load_and_serialize_basic_rdf_config() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let rdf_config_input = InputSpec::str(
        r#"
- Person ex:person1 ex:person2:
  - a: ex:Person
  - rdfs:label:
    - name: "Alice"
  - ex:age?:
    - age_value: 32
  - ex:memberOf:
    - organization: Organization
- Organization ex:org1:
  - a: ex:Organization
  - rdfs:label:
    - org_name: "Example Org"
  - ex:location:
    - city: "Oviedo"
        "#,
    );

    load_rdf_config(&mut rudof, &rdf_config_input, None).unwrap();

    let serialized = serialize_to_string(&rudof, None);

    assert!(serialized.contains("Person"));
    assert!(serialized.contains("Organization"));
    assert!(serialized.contains("Alice") || serialized.contains("name"));
    assert!(serialized.contains("Oviedo") || serialized.contains("city"));

    println!(
        "\n===== test_load_and_serialize_basic_rdf_config =====\n{}\n============================================",
        serialized
    );
}
