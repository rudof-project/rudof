use rudof_generate::GeneratorConfig;
use std::io::Write;
use tempfile::NamedTempFile;

/// Test that the basic DataGenerator can be created and configured
#[tokio::test]
async fn test_basic_generator_creation() {
    // Test that we can create a config and generator
    let output_file = NamedTempFile::new().unwrap();

    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 3;
    config.output.path = output_file.path().to_path_buf();

    let generator = rudof_generate::DataGenerator::new(config);
    assert!(
        generator.is_ok(),
        "Should be able to create a DataGenerator"
    );
}

/// Test that ShEx schemas can be loaded
#[tokio::test]
async fn test_shex_schema_loading() {
    // Create a simple ShEx schema
    let shex_schema = r#"
PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

ex:PersonShape {
  ex:name xsd:string
}
"#;

    // Create temporary files
    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{shex_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    // Configure generator
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 2;
    config.output.path = output_file.path().to_path_buf();

    // Test schema loading
    let mut generator = rudof_generate::DataGenerator::new(config).unwrap();
    let result = generator.load_shex_schema(schema_file.path()).await;

    assert!(result.is_ok(), "Should be able to load ShEx schema");
}

/// Test that SHACL schemas can be loaded
#[tokio::test]
async fn test_shacl_schema_loading() {
    // Create a simple SHACL schema
    let shacl_schema = r#"
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <http://example.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:PersonShape a sh:NodeShape ;
    sh:targetClass ex:Person ;
    sh:property [
        sh:path ex:name ;
        sh:datatype xsd:string ;
        sh:minCount 1 ;
        sh:maxCount 1 ;
    ] .
"#;

    // Create temporary files
    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{shacl_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    // Configure generator
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 2;
    config.output.path = output_file.path().to_path_buf();

    // Test schema loading
    let mut generator = rudof_generate::DataGenerator::new(config).unwrap();
    let result = generator.load_shacl_schema(schema_file.path()).await;

    assert!(result.is_ok(), "Should be able to load SHACL schema");
}

/// Test that data generation produces output files
#[tokio::test]
async fn test_data_generation_produces_output() {
    // Create a simple SHACL schema
    let shacl_schema = r#"
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <http://example.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:PersonShape a sh:NodeShape ;
    sh:targetClass ex:Person ;
    sh:property [
        sh:path ex:name ;
        sh:datatype xsd:string ;
        sh:minCount 1 ;
        sh:maxCount 1 ;
    ] .
"#;

    // Create temporary files
    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{shacl_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    // Configure generator
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 2;
    config.output.path = output_file.path().to_path_buf();

    // Generate data
    let mut generator = rudof_generate::DataGenerator::new(config).unwrap();
    generator.load_shacl_schema(schema_file.path()).await.unwrap();
    let result = generator.generate().await;

    assert!(result.is_ok(), "Should be able to generate data");

    // Verify output file exists and has content
    assert!(output_file.path().exists(), "Output file should exist");
    let content = std::fs::read_to_string(output_file.path()).unwrap();
    assert!(!content.is_empty(), "Output file should have content");
}
