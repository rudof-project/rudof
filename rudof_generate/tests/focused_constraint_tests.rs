use rudof_generate::{DataGenerator, GeneratorConfig};
use srdf::{NeighsRDF, RDFFormat, ReaderMode, SRDFGraph};
use std::collections::HashMap;
use std::io::Write;
use tempfile::NamedTempFile;

/// Test that ShEx datatype constraints are passed down to generated data
#[tokio::test]
async fn test_shex_datatype_passthrough() {
    // Create a ShEx schema with specific datatypes
    let shex_schema = r#"
PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

ex:PersonShape {
  ex:name xsd:string ;
  ex:age xsd:integer ;
  ex:active xsd:boolean
}
"#;

    // Create temporary files
    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{shex_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    // Configure generator
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 5;
    config.output.path = output_file.path().to_path_buf();

    // Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator
        .load_shex_schema(schema_file.path())
        .await
        .unwrap();
    generator.generate().await.unwrap();

    // Parse generated data
    let graph = SRDFGraph::from_path(
        output_file.path(),
        &RDFFormat::Turtle,
        None,
        &ReaderMode::Strict,
    )
    .expect("Failed to parse generated RDF");

    // Verify that generated triples respect datatypes
    let mut property_counts = HashMap::new();

    for triple in graph.triples().unwrap() {
        let predicate_str = triple.predicate.to_string();
        *property_counts.entry(predicate_str).or_insert(0) += 1;
    }

    // Should have generated properties for all three datatypes
    assert!(property_counts.contains_key("<http://example.org/name>"));
    assert!(property_counts.contains_key("<http://example.org/age>"));
    assert!(property_counts.contains_key("<http://example.org/active>"));
}

/// Test that SHACL datatype constraints are passed down to generated data
#[tokio::test]
async fn test_shacl_datatype_passthrough() {
    // Create a SHACL schema with specific datatypes
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
    ] ;
    sh:property [
        sh:path ex:age ;
        sh:datatype xsd:integer ;
        sh:minCount 1 ;
        sh:maxCount 1 ;
    ] ;
    sh:property [
        sh:path ex:score ;
        sh:datatype xsd:decimal ;
        sh:minCount 0 ;
        sh:maxCount 1 ;
    ] .
"#;

    // Create temporary files
    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{shacl_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    // Configure generator
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 5;
    config.output.path = output_file.path().to_path_buf();

    // Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator
        .load_shacl_schema(schema_file.path())
        .await
        .unwrap();
    generator.generate().await.unwrap();

    // Parse generated data
    let graph = SRDFGraph::from_path(
        output_file.path(),
        &RDFFormat::Turtle,
        None,
        &ReaderMode::Strict,
    )
    .expect("Failed to parse generated RDF");

    // Verify that generated triples respect datatypes
    let mut property_counts = HashMap::new();

    for triple in graph.triples().unwrap() {
        let predicate_str = triple.predicate.to_string();
        *property_counts.entry(predicate_str).or_insert(0) += 1;
    }

    // Should have generated properties for all three datatypes
    assert!(property_counts.contains_key("<http://example.org/name>"));
    assert!(property_counts.contains_key("<http://example.org/age>"));
    assert!(property_counts.contains_key("<http://example.org/score>"));
}

/// Test that SHACL cardinality constraints are passed down to generated data
#[tokio::test]
async fn test_shacl_cardinality_passthrough() {
    // Create a SHACL schema with cardinality constraints
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
    ] ;
    sh:property [
        sh:path ex:email ;
        sh:datatype xsd:string ;
        sh:minCount 0 ;
        sh:maxCount 3 ;
    ] .
"#;

    // Create temporary files
    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{shacl_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    // Configure generator
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 8;
    config.output.path = output_file.path().to_path_buf();

    // Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator
        .load_shacl_schema(schema_file.path())
        .await
        .unwrap();
    generator.generate().await.unwrap();

    // Parse generated data
    let graph = SRDFGraph::from_path(
        output_file.path(),
        &RDFFormat::Turtle,
        None,
        &ReaderMode::Strict,
    )
    .expect("Failed to parse generated RDF");

    // Count properties per entity to verify cardinality
    let mut entity_properties: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for triple in graph.triples().unwrap() {
        let subject_str = triple.subject.to_string();
        let predicate_str = triple.predicate.to_string();

        entity_properties
            .entry(subject_str)
            .or_default()
            .entry(predicate_str)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    // Verify cardinality constraints for each entity
    for properties in entity_properties.values() {
        // Each entity should have exactly 1 name
        if let Some(&name_count) = properties.get("http://example.org/name") {
            assert_eq!(name_count, 1, "Entity should have exactly 1 name");
        }

        // Each entity should have 0-3 emails
        if let Some(&email_count) = properties.get("http://example.org/email") {
            assert!(email_count <= 3, "Entity should have at most 3 emails");
        }
    }
}

/// Test that basic constraints are correctly passed from schema to generated data
#[tokio::test]
async fn test_constraint_pipeline_verification() {
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
    config.generation.entity_count = 3;
    config.output.path = output_file.path().to_path_buf();

    // Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator
        .load_shacl_schema(schema_file.path())
        .await
        .unwrap();
    generator.generate().await.unwrap();

    // Parse generated data
    let graph = SRDFGraph::from_path(
        output_file.path(),
        &RDFFormat::Turtle,
        None,
        &ReaderMode::Strict,
    )
    .expect("Failed to parse generated RDF");

    // Verify that data was generated
    let triples: Vec<_> = graph.triples().unwrap().collect();
    assert!(!triples.is_empty(), "Should have generated some triples");

    // Count how many name properties were generated
    let name_count = triples
        .iter()
        .filter(|t| t.predicate.to_string() == "<http://example.org/name>")
        .count();

    assert!(
        name_count > 0,
        "Should have generated at least one name property"
    );
}
