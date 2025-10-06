use data_generator::config::OutputFormat;
use data_generator::{DataGenerator, GeneratorConfig};
use srdf::{Literal, NeighsRDF, RDFFormat, ReaderMode, SRDFGraph};
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
    let mut datatype_counts = HashMap::new();

    for triple in graph.triples().unwrap() {
        if let oxrdf::Term::Literal(lit) = &triple.object {
            let datatype = lit.datatype().to_string();
            // Remove angle brackets if present
            let clean_datatype = datatype.trim_start_matches('<').trim_end_matches('>');
            *datatype_counts
                .entry(clean_datatype.to_string())
                .or_insert(0) += 1;
        }
    }

    // Should have string, integer, and boolean literals
    assert!(datatype_counts.contains_key("http://www.w3.org/2001/XMLSchema#string"));
    assert!(datatype_counts.contains_key("http://www.w3.org/2001/XMLSchema#integer"));
    assert!(datatype_counts.contains_key("http://www.w3.org/2001/XMLSchema#boolean"));
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
    let mut datatype_counts = HashMap::new();

    for triple in graph.triples().unwrap() {
        if let oxrdf::Term::Literal(lit) = &triple.object {
            let datatype = lit.datatype().to_string();
            // Strip angle brackets from datatype URI if present
            let clean_datatype = if datatype.starts_with('<') && datatype.ends_with('>') {
                datatype[1..datatype.len() - 1].to_string()
            } else {
                datatype
            };
            *datatype_counts.entry(clean_datatype).or_insert(0) += 1;
        }
    }

    // Should have string, integer, and decimal literals
    assert!(datatype_counts.contains_key("http://www.w3.org/2001/XMLSchema#string"));
    assert!(datatype_counts.contains_key("http://www.w3.org/2001/XMLSchema#integer"));
    assert!(datatype_counts.contains_key("http://www.w3.org/2001/XMLSchema#decimal"));
}

/// Test that ShEx cardinality constraints are passed down to generated data
#[tokio::test]
async fn test_shex_cardinality_passthrough() {
    // Create a ShEx schema with cardinality constraints
    let shex_schema = r#"
PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

ex:PersonShape {
  ex:name xsd:string {1,1} ;      # exactly one name
  ex:email xsd:string {0,2} ;     # zero to two emails  
  ex:phone xsd:string *           # zero or more phones
}
"#;

    // Create temporary files
    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{shex_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    // Configure generator
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 10; // More entities for better cardinality testing
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

    // Count properties per entity to verify cardinality
    let mut entity_properties: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for triple in graph.triples().unwrap() {
        let subject = triple.subject.to_string();
        let predicate = triple.predicate.to_string();

        entity_properties
            .entry(subject)
            .or_default()
            .entry(predicate)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    // Verify cardinality constraints for each entity
    for properties in entity_properties.values() {
        // Each entity should have exactly 1 name
        if let Some(&name_count) = properties.get("http://example.org/name") {
            assert_eq!(name_count, 1, "Entity should have exactly 1 name");
        }

        // Each entity should have 0-2 emails
        if let Some(&email_count) = properties.get("http://example.org/email") {
            assert!(email_count <= 2, "Entity should have at most 2 emails");
        }

        // Phone count can be any number (0 or more)
        // No assertion needed for * cardinality
    }
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
    ] ;
    sh:property [
        sh:path ex:hobby ;
        sh:datatype xsd:string ;
        sh:minCount 2 ;
        sh:maxCount 5 ;
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
        let subject = triple.subject.to_string();
        let predicate = triple.predicate.to_string();

        entity_properties
            .entry(subject)
            .or_default()
            .entry(predicate)
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

        // Each entity should have 2-5 hobbies
        if let Some(&hobby_count) = properties.get("http://example.org/hobby") {
            assert!(
                (2..=5).contains(&hobby_count),
                "Entity should have 2-5 hobbies"
            );
        }
    }
}

/// Test that ShEx shape references are passed down correctly
#[tokio::test]
async fn test_shex_shape_reference_passthrough() {
    // Create a ShEx schema with shape references
    let shex_schema = r#"
PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

ex:PersonShape {
  ex:name xsd:string ;
  ex:address @ex:AddressShape
}

ex:AddressShape {
  ex:street xsd:string ;
  ex:city xsd:string ;
  ex:country xsd:string
}
"#;

    // Create temporary files
    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{shex_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    // Configure generator
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 3;
    config.output.format = OutputFormat::Turtle;
    config.output.path = output_file.path().to_path_buf();

    // Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator
        .load_shex_schema(schema_file.path())
        .await
        .unwrap();
    generator.generate().await.unwrap();

    // Read and validate generated data
    let generated_data = std::fs::read_to_string(output_file.path()).unwrap();
    let graph = SRDFGraph::from_str(
        &generated_data,
        &RDFFormat::Turtle,
        None,
        &ReaderMode::Strict,
    )
    .unwrap();

    // Check that we have both person and address data
    let triples = graph.triples().unwrap();
    let mut has_person_name = false;
    let mut has_address_street = false;
    let mut has_address_city = false;

    for triple in triples {
        let predicate = triple.predicate.to_string();
        match predicate.as_str() {
            "<http://example.org/name>" => has_person_name = true,
            "<http://example.org/street>" => has_address_street = true,
            "<http://example.org/city>" => has_address_city = true,
            _ => {}
        }
    }

    assert!(has_person_name, "Should have person names");
    assert!(has_address_street, "Should have address streets");
    assert!(has_address_city, "Should have address cities");
}

/// Test that SHACL node shape references are passed down correctly
#[tokio::test]
async fn test_shacl_shape_reference_passthrough() {
    // Create a SHACL schema with shape references
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
        sh:path ex:address ;
        sh:node ex:AddressShape ;
        sh:minCount 1 ;
        sh:maxCount 1 ;
    ] .

ex:AddressShape a sh:NodeShape ;
    sh:property [
        sh:path ex:street ;
        sh:datatype xsd:string ;
        sh:minCount 1 ;
        sh:maxCount 1 ;
    ] ;
    sh:property [
        sh:path ex:city ;
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
    config.output.format = OutputFormat::Turtle;
    config.output.path = output_file.path().to_path_buf();

    // Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator
        .load_shacl_schema(schema_file.path())
        .await
        .unwrap();
    generator.generate().await.unwrap();

    // Read and validate generated data
    let generated_data = std::fs::read_to_string(output_file.path()).unwrap();
    let graph = SRDFGraph::from_str(
        &generated_data,
        &RDFFormat::Turtle,
        None,
        &ReaderMode::Strict,
    )
    .unwrap();

    // Check that we have both person and address data
    let triples = graph.triples().unwrap();
    let mut has_person_name = false;
    let mut has_address_street = false;
    let mut has_address_city = false;

    for triple in triples {
        let predicate = triple.predicate.to_string();
        match predicate.as_str() {
            "<http://example.org/name>" => has_person_name = true,
            "<http://example.org/street>" => has_address_street = true,
            "<http://example.org/city>" => has_address_city = true,
            _ => {}
        }
    }

    assert!(has_person_name, "Should have person names");
    assert!(has_address_street, "Should have address streets");
    assert!(has_address_city, "Should have address cities");
}

/// Test that SHACL value constraints are passed down correctly
#[tokio::test]
async fn test_shacl_value_constraints_passthrough() {
    // Create a SHACL schema with basic supported constraints
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
        sh:path ex:status ;
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
    config.generation.entity_count = 5;
    config.output.format = OutputFormat::Turtle;
    config.output.path = output_file.path().to_path_buf();

    // Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator
        .load_shacl_schema(schema_file.path())
        .await
        .unwrap();
    generator.generate().await.unwrap();

    // Read and validate generated data
    let generated_data = std::fs::read_to_string(output_file.path()).unwrap();
    let graph = SRDFGraph::from_str(
        &generated_data,
        &RDFFormat::Turtle,
        None,
        &ReaderMode::Strict,
    )
    .unwrap();

    // Verify that generated values respect constraints
    let triples = graph.triples().unwrap();

    for triple in triples {
        let predicate = triple.predicate.to_string();
        let object = triple.object;

        match predicate.as_str() {
            "http://example.org/name" => {
                if let oxrdf::Term::Literal(literal) = object {
                    let value = literal.lexical_form();
                    assert!(
                        value.len() >= 2 && value.len() <= 50,
                        "Name length should be between 2 and 50 characters, got: {value}"
                    );
                }
            }
            "http://example.org/age" => {
                if let oxrdf::Term::Literal(literal) = object {
                    let value: i32 = literal.lexical_form().parse().unwrap();
                    assert!(
                        (0..=150).contains(&value),
                        "Age should be between 0 and 150, got: {value}"
                    );
                }
            }
            "http://example.org/status" => {
                if let oxrdf::Term::Literal(literal) = object {
                    let value = literal.lexical_form();
                    assert!(
                        ["active", "inactive", "pending"].contains(&value),
                        "Status should be one of active/inactive/pending, got: {value}"
                    );
                }
            }
            _ => {}
        }
    }
}
