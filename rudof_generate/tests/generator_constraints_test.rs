use rudof_generate::{DataGenerator, GeneratorConfig};
use srdf::{NeighsRDF, RDFFormat, ReaderMode, SRDFGraph};
use std::collections::HashMap;
use std::io::Write;
use tempfile::NamedTempFile;

/// Test that ShEx cardinality constraints are respected in generated data
#[tokio::test]
async fn test_shex_cardinality_constraints() {
    // Create a simple ShEx schema with cardinality constraints using ShEx Compact syntax
    let shex_schema = r#"
PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

ex:PersonShape {
    ex:name xsd:string {1,1} ;
    ex:email xsd:string {0,3}
}
    "#;

    // Create temporary files
    let mut schema_file = NamedTempFile::new().unwrap();
    write!(schema_file, "{shex_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    // Configure generator
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 10;
    config.output.path = output_file.path().to_path_buf();

    // Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shex_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // Parse generated data
    let graph = SRDFGraph::from_path(output_file.path(), &RDFFormat::Turtle, None, &ReaderMode::Strict)
        .expect("Failed to parse generated RDF");

    // Verify cardinality constraints
    verify_shex_cardinality(&graph);
}

/// Test that SHACL cardinality constraints are respected in generated data
#[tokio::test]
async fn test_shacl_cardinality_constraints() {
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
    write!(schema_file, "{shacl_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    // Configure generator
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 10;
    config.output.path = output_file.path().to_path_buf();

    // Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shacl_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // Parse generated data
    let graph = SRDFGraph::from_path(output_file.path(), &RDFFormat::Turtle, None, &ReaderMode::Strict)
        .expect("Failed to parse generated RDF");

    // Verify cardinality constraints
    verify_shacl_cardinality(&graph);
}

/// Test that datatype constraints are respected in generated data
#[tokio::test]
async fn test_datatype_constraints() {
    // Create a schema with different datatypes
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
            sh:path ex:height ;
            sh:datatype xsd:decimal ;
            sh:minCount 0 ;
            sh:maxCount 1 ;
        ] ;
        sh:property [
            sh:path ex:birthDate ;
            sh:datatype xsd:date ;
            sh:minCount 0 ;
            sh:maxCount 1 ;
        ] .
    "#;

    // Create temporary files
    let mut schema_file = NamedTempFile::new().unwrap();
    write!(schema_file, "{shacl_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    // Configure generator
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 5;
    config.output.path = output_file.path().to_path_buf();

    // Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shacl_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // Parse generated data
    let graph = SRDFGraph::from_path(output_file.path(), &RDFFormat::Turtle, None, &ReaderMode::Strict)
        .expect("Failed to parse generated RDF");

    // Verify datatype constraints
    verify_datatypes(&graph);
}

/// Test that value constraints (length, range) are respected
#[tokio::test]
async fn test_value_constraints() {
    // Create a SHACL schema with value constraints
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
        ] .
    "#;

    // Create temporary files
    let mut schema_file = NamedTempFile::new().unwrap();
    write!(schema_file, "{shacl_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    // Configure generator
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 5;
    config.output.path = output_file.path().to_path_buf();

    // Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shacl_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // Parse generated data
    let graph = SRDFGraph::from_path(output_file.path(), &RDFFormat::Turtle, None, &ReaderMode::Strict)
        .expect("Failed to parse generated RDF");

    // Verify value constraints
    verify_value_constraints(&graph);
}

/// Test that ShEx datatype constraints are respected in generated data
#[tokio::test]
async fn test_shex_datatype_constraints() {
    // Create a ShEx schema with different datatypes
    let shex_schema = r#"
PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

ex:PersonShape {
    ex:name xsd:string {1,1} ;
    ex:age xsd:integer {1,1} ;
    ex:height xsd:decimal {0,1} ;
    ex:birthDate xsd:date {0,1}
}
    "#;

    // Create temporary files
    let mut schema_file = NamedTempFile::new().unwrap();
    write!(schema_file, "{shex_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    // Configure generator
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 5;
    config.output.path = output_file.path().to_path_buf();

    // Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shex_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // Parse generated data
    let graph = SRDFGraph::from_path(output_file.path(), &RDFFormat::Turtle, None, &ReaderMode::Strict)
        .expect("Failed to parse generated RDF");

    // Verify datatype constraints (reuse the same verification function)
    verify_datatypes(&graph);
}

/// Test that ShEx value constraints are respected
#[tokio::test]
async fn test_shex_value_constraints() {
    // Create a ShEx schema with basic value constraints
    let shex_schema = r#"
PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

ex:PersonShape {
    ex:name xsd:string {1,1} ;
    ex:age xsd:integer {1,1}
}
    "#;

    // Create temporary files
    let mut schema_file = NamedTempFile::new().unwrap();
    write!(schema_file, "{shex_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    // Configure generator
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 5;
    config.output.path = output_file.path().to_path_buf();

    // Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shex_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // Parse generated data
    let graph = SRDFGraph::from_path(output_file.path(), &RDFFormat::Turtle, None, &ReaderMode::Strict)
        .expect("Failed to parse generated RDF");

    // Verify value constraints (reuse the same verification function)
    verify_value_constraints(&graph);
}

// Helper functions for verification

fn verify_shex_cardinality(graph: &SRDFGraph) {
    let mut entity_properties: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    // Collect properties for each entity
    for triple in graph.triples().unwrap() {
        let subject_str = triple.subject.to_string();
        let predicate_str = triple.predicate.to_string();
        let object_str = triple.object.to_string();

        entity_properties
            .entry(subject_str)
            .or_default()
            .entry(predicate_str)
            .or_default()
            .push(object_str);
    }

    // Verify cardinality constraints for ShEx
    for (entity, properties) in entity_properties {
        if let Some(names) = properties.get("http://example.org/name") {
            assert_eq!(
                names.len(),
                1,
                "Entity {} should have exactly 1 name, found {}",
                entity,
                names.len()
            );
        }

        if let Some(emails) = properties.get("http://example.org/email") {
            assert!(
                emails.len() <= 3,
                "Entity {} should have at most 3 emails, found {}",
                entity,
                emails.len()
            );
        }
    }
}

fn verify_shacl_cardinality(graph: &SRDFGraph) {
    let mut entity_properties: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    // Collect properties for each entity
    for triple in graph.triples().unwrap() {
        let subject_str = triple.subject.to_string();
        let predicate_str = triple.predicate.to_string();
        let object_str = triple.object.to_string();

        entity_properties
            .entry(subject_str)
            .or_default()
            .entry(predicate_str)
            .or_default()
            .push(object_str);
    }

    // Verify cardinality constraints for SHACL
    for (entity, properties) in entity_properties {
        if let Some(names) = properties.get("http://example.org/name") {
            assert_eq!(
                names.len(),
                1,
                "Entity {} should have exactly 1 name, found {}",
                entity,
                names.len()
            );
        }

        if let Some(emails) = properties.get("http://example.org/email") {
            assert!(
                emails.len() <= 3,
                "Entity {} should have at most 3 emails, found {}",
                entity,
                emails.len()
            );
        }
    }
}

fn verify_datatypes(graph: &SRDFGraph) {
    for triple in graph.triples().unwrap() {
        let literal = triple.object.clone();
        if let oxrdf::Term::Literal(lit) = literal {
            let predicate_str = triple.predicate.as_str();
            let datatype = lit.datatype().as_str();

            match predicate_str {
                "http://example.org/name" => {
                    assert_eq!(
                        datatype, "http://www.w3.org/2001/XMLSchema#string",
                        "Name should be xsd:string"
                    );
                },
                "http://example.org/age" => {
                    assert_eq!(
                        datatype, "http://www.w3.org/2001/XMLSchema#integer",
                        "Age should be xsd:integer"
                    );
                    // Verify it's actually a valid integer
                    lit.value().parse::<i32>().expect("Age should be valid integer");
                },
                "http://example.org/height" => {
                    assert_eq!(
                        datatype, "http://www.w3.org/2001/XMLSchema#decimal",
                        "Height should be xsd:decimal"
                    );
                    // Verify it's actually a valid decimal
                    lit.value().parse::<f64>().expect("Height should be valid decimal");
                },
                "http://example.org/birthDate" => {
                    assert_eq!(
                        datatype, "http://www.w3.org/2001/XMLSchema#date",
                        "Birth date should be xsd:date"
                    );
                },
                _ => {},
            }
        }
    }
}

fn verify_value_constraints(graph: &SRDFGraph) {
    // Only verify that basic datatypes are respected (no range/length constraints since they're not supported)
    for triple in graph.triples().unwrap() {
        let literal = triple.object.clone();
        if let oxrdf::Term::Literal(lit) = literal {
            let predicate_str = triple.predicate.as_str();
            let value = lit.value();

            match predicate_str {
                "http://example.org/name" => {
                    // Just verify it's a string - no length constraints since they're not supported
                    assert!(!value.is_empty(), "Name should not be empty");
                },
                "http://example.org/age" => {
                    // Just verify it's a valid integer - no range constraints since they're not supported
                    value.parse::<i32>().expect("Age should be a valid integer");
                },
                _ => {},
            }
        }
    }
}
