use rudof_generate::{DataGenerator, GeneratorConfig};
use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
use rudof_rdf::rdf_impl::{InMemoryGraph, ReaderMode};
use std::collections::HashMap;
use std::io::Write;
use tempfile::NamedTempFile;

#[tokio::test]
async fn debug_shex_datatype_test() {
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
    config.generation.entity_count = 3;
    config.output.path = output_file.path().to_path_buf();

    // Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shex_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // Parse generated data
    let graph = InMemoryGraph::from_path(output_file.path(), &RDFFormat::Turtle, None, &ReaderMode::Strict)
        .expect("Failed to parse generated RDF");

    // Read the generated file content
    let content = std::fs::read_to_string(output_file.path()).unwrap();
    println!("Generated RDF content:\n{content}");

    // Verify that generated triples respect datatypes
    let mut datatype_counts = HashMap::new();

    for triple in graph.triples().unwrap() {
        if let oxrdf::Term::Literal(lit) = &triple.object {
            let datatype = lit.datatype().to_string();
            // Remove angle brackets if present
            let clean_datatype = datatype.trim_start_matches('<').trim_end_matches('>');
            println!("Found literal: {} with datatype: {}", lit.value(), clean_datatype);
            *datatype_counts.entry(clean_datatype.to_string()).or_insert(0) += 1;
        }
    }

    println!("Datatype counts: {datatype_counts:?}");

    // Should have string, integer, and boolean literals
    if !datatype_counts.contains_key("http://www.w3.org/2001/XMLSchema#string") {
        println!("Missing string datatype! Keys: {:?}", datatype_counts.keys());
    }
    if !datatype_counts.contains_key("http://www.w3.org/2001/XMLSchema#integer") {
        println!("Missing integer datatype! Keys: {:?}", datatype_counts.keys());
    }
    if !datatype_counts.contains_key("http://www.w3.org/2001/XMLSchema#boolean") {
        println!("Missing boolean datatype! Keys: {:?}", datatype_counts.keys());
    }
}
