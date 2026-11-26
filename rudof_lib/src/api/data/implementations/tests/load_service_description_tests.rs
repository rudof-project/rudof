use crate::{
    Rudof, RudofConfig,
    api::data::implementations::{
        load_service_description::load_service_description,
        serialize_service_description::serialize_service_description,
    },
    formats::{DataFormat, DataReaderMode, InputSpec, ResultServiceFormat},
};

#[test]
fn test_load_service_description() {
    let mut rudof = Rudof::new(RudofConfig::default());

    let service_desc_ttl = r#"
        @prefix sd: <http://www.w3.org/ns/sparql-service-description#> .
        @prefix ent: <http://www.w3.org/ns/entailment/> .
        
        <http://example.org/sparql> a sd:Service ;
            sd:endpoint <http://example.org/sparql> ;
            sd:supportedLanguage sd:SPARQL11Query ;
            sd:defaultEntailmentRegime ent:Simple .
    "#;

    let input_spec = InputSpec::str(service_desc_ttl);

    // Load
    load_service_description(
        &mut rudof,
        &input_spec,
        Some(&DataFormat::Turtle),
        Some(&DataReaderMode::Strict),
        Some("http://example.org/"),
    )
    .expect("Failed to load service description");

    // Serialize
    let mut output = Vec::new();
    serialize_service_description(&rudof, Some(&ResultServiceFormat::Json), &mut output)
        .expect("Failed to serialize service description");

    // Check
    let result = String::from_utf8(output).unwrap();
    assert!(result.contains("http://www.w3.org/ns/sparql-service-description"));
    assert!(result.contains("http://www.w3.org/ns/entailment/"));
    assert!(result.contains("SPARQL11Query"));

    println!(
        "\n===== test_load_service_description =====\n{}============================================",
        result
    );
}
