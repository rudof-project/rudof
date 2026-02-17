use rudof_generate::config::OutputFormat;
use rudof_generate::{DataGenerator, GeneratorConfig};
use rdf::rdf_core::{NeighsRDF, RDFFormat};
use rdf::rdf_impl::{InMemoryGraph, ReaderMode};
use std::collections::HashMap;
use std::io::Write;
use tempfile::NamedTempFile;

/// Debug test to see what datatypes are actually being generated
#[tokio::test]
async fn debug_shex_datatype_generation() {
    // Create a simple ShEx schema with datatypes
    let shex_schema = r#"
    PREFIX : <http://example.org/>
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

    :PersonShape {
        :name xsd:string ;
        :age xsd:integer ;
        :active xsd:boolean ;
    }
    "#;

    let mut schema_file = NamedTempFile::new().unwrap();
    write!(schema_file, "{shex_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 3;
    config.output.path = output_file.path().to_path_buf();
    config.output.format = OutputFormat::Turtle;

    let mut generator = DataGenerator::new(config).unwrap();

    // Try to load the schema
    match generator.load_shex_schema(schema_file.path()).await {
        Ok(_) => println!("ShEx schema loaded successfully"),
        Err(e) => {
            println!("ShEx schema loading failed: {e:?}");
            return;
        },
    }

    // Try to generate data
    match generator.generate().await {
        Ok(_) => println!("Data generation completed"),
        Err(e) => {
            println!("Data generation failed: {e:?}");
            return;
        },
    }

    // Read the generated content
    let content = std::fs::read_to_string(output_file.path()).unwrap();
    println!("Generated content:");
    println!("{content}");

    // Parse and analyze the generated RDF
    let graph = InMemoryGraph::from_path(
        output_file.path(),
        &RDFFormat::Turtle,
        None,
        &ReaderMode::Strict,
    )
    .expect("Failed to parse generated RDF");

    let mut datatype_counts = HashMap::new();
    let mut all_triples = Vec::new();

    for triple in graph.triples().unwrap() {
        all_triples.push(format!("{} {} {}", triple.subject, triple.predicate, triple.object));
        match &triple.object {
            oxrdf::Term::Literal(lit) => {
                let datatype = lit.datatype().to_string();
                *datatype_counts.entry(datatype).or_insert(0) += 1;
                println!("Found literal: {} with datatype: {}", lit.value(), lit.datatype());
            },
            oxrdf::Term::NamedNode(node) => {
                println!("Found named node: {node}");
            },
            oxrdf::Term::BlankNode(blank) => {
                println!("Found blank node: {blank}");
            },
            #[allow(unreachable_patterns)]
            _ => {
                println!("Found other term type: {:?}", triple.object);
            },
        }
    }

    println!("All triples generated:");
    for triple in all_triples {
        println!("  {triple}");
    }

    println!("Datatype counts: {datatype_counts:?}");
}

/// Debug test for SHACL datatype generation
#[tokio::test]
async fn debug_shacl_datatype_generation() {
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
            sh:path ex:active ;
            sh:datatype xsd:boolean ;
            sh:minCount 1 ;
            sh:maxCount 1 ;
        ] .
    "#;

    let mut schema_file = NamedTempFile::new().unwrap();
    write!(schema_file, "{shacl_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 3;
    config.output.path = output_file.path().to_path_buf();
    config.output.format = OutputFormat::Turtle;

    let mut generator = DataGenerator::new(config).unwrap();

    // Try to load the schema
    match generator.load_shacl_schema(schema_file.path()).await {
        Ok(_) => println!("SHACL schema loaded successfully"),
        Err(e) => {
            println!("SHACL schema loading failed: {e:?}");
            return;
        },
    }

    // Try to generate data
    match generator.generate().await {
        Ok(_) => println!("Data generation completed"),
        Err(e) => {
            println!("Data generation failed: {e:?}");
            return;
        },
    }

    // Read the generated content
    let content = std::fs::read_to_string(output_file.path()).unwrap();
    println!("Generated SHACL content:");
    println!("{content}");

    // Parse and analyze the generated RDF
    let graph = InMemoryGraph::from_path(
        output_file.path(),
        &RDFFormat::Turtle,
        None,
        &ReaderMode::Strict,
    )
    .expect("Failed to parse generated RDF");

    let mut datatype_counts = HashMap::new();
    let mut all_triples = Vec::new();

    for triple in graph.triples().unwrap() {
        all_triples.push(format!("{} {} {}", triple.subject, triple.predicate, triple.object));

        match &triple.object {
            oxrdf::Term::Literal(lit) => {
                let datatype = lit.datatype().to_string();
                *datatype_counts.entry(datatype).or_insert(0) += 1;
                println!("Found literal: {} with datatype: {}", lit.value(), lit.datatype());
            },
            oxrdf::Term::NamedNode(node) => {
                println!("Found named node: {node}");
            },
            oxrdf::Term::BlankNode(blank) => {
                println!("Found blank node: {blank}");
            },
            #[allow(unreachable_patterns)]
            _ => {
                println!("Found other term type: {:?}", triple.object);
            },
        }
    }

    println!("All triples generated:");
    for triple in all_triples {
        println!("  {triple}");
    }

    println!("Datatype counts: {datatype_counts:?}");
}
