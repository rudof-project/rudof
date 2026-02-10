use rudof_generate::config::OutputFormat;
use rudof_generate::{DataGenerator, GeneratorConfig};
use std::io::Write;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_property_fill_probability() {
    // 1. Create a SHACL schema with many optional properties
    // We use minCount 0 to ensure they are optional by default
    let shacl_schema = r#"
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <http://example.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:PersonShape a sh:NodeShape ;
    sh:targetClass ex:Person ;
    sh:property [ sh:path ex:p1 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p2 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p3 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p4 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p5 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p6 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p7 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p8 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p9 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;
    sh:property [ sh:path ex:p10 ; sh:datatype xsd:integer ; sh:minCount 0 ; ] .
"#;

    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{}", shacl_schema).unwrap();
    let output_file = NamedTempFile::new().unwrap();

    // 2. Configure generator with property_fill_probability = 0.5
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 100;
    config.generation.property_fill_probability = 0.5;
    config.generation.cardinality_strategy = rudof_generate::config::CardinalityStrategy::Maximum;
    config.output.path = output_file.path().to_path_buf();
    config.output.format = OutputFormat::NTriples; // Use NTriples for easy counting
    
    // 3. Generate data
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shacl_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // 4. Verify output
    let output_content = std::fs::read_to_string(output_file.path()).unwrap();
    
    // Count total property occurrences (excluding rdf:type)
    let total_triples = output_content.lines().count();
    let type_triples = output_content.lines().filter(|l| l.contains("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")).count();
    let property_triples = total_triples - type_triples;
    
    // Expected: 100 entities * 10 properties * 0.5 = 500 properties
    // Allow reasonable variance (e.g., +/- 10% -> 450 to 550)
    // Actually standard deviation for binomial n=1000, p=0.5 is sqrt(1000*0.25) = 15.8
    // So 3-sigma is +/- 48. Range [452, 548]. 
    // Let's be generous: [400, 600].
    
    println!("Total property triples generated: {}", property_triples);
    
    assert!(property_triples >= 400, "Too few properties generated: {}", property_triples);
    assert!(property_triples <= 600, "Too many properties generated: {}", property_triples);
}

#[tokio::test]
async fn test_ignore_min_cardinality() {
    // 1. Create a SHACL schema with REQUIRED properties (minCount 1)
    let shacl_schema = r#"
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <http://example.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:RequiredShape a sh:NodeShape ;
    sh:targetClass ex:Required ;
    sh:property [ sh:path ex:req1 ; sh:datatype xsd:integer ; sh:minCount 1 ; ] ;
    sh:property [ sh:path ex:req2 ; sh:datatype xsd:integer ; sh:minCount 1 ; ] .
"#;

    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{}", shacl_schema).unwrap();
    let output_file = NamedTempFile::new().unwrap();

    // 2. Configure with ignore_min_cardinality = true AND low probability
    // If it works, we should see MISSING properties despite minCount 1
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 100;
    config.generation.property_fill_probability = 0.5;
    config.generation.ignore_min_cardinality = true;
    config.generation.cardinality_strategy = rudof_generate::config::CardinalityStrategy::Maximum;
    config.output.path = output_file.path().to_path_buf();
    config.output.format = OutputFormat::NTriples;

    // 3. Generate
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shacl_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // 4. Verify
    let output_content = std::fs::read_to_string(output_file.path()).unwrap();
    let total_triples = output_content.lines().count();
    let type_triples = output_content.lines().filter(|l| l.contains("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")).count();
    let property_triples = total_triples - type_triples;

    // If minCount was respected, we'd have exactly 200 properties (100 * 2).
    // With ignore_min_cardinality=true and prob=0.5, we expect ~100 properties.
    println!("Total property triples (ignore_min=true): {}", property_triples);

    assert!(property_triples < 180, "Should have skipped many required properties. Got: {}", property_triples);
}

#[tokio::test]
async fn test_max_properties_per_instance() {
    // 1. Create a SHACL schema with MANY optional properties
    // 20 properties
    let mut shacl_schema = String::from(r#"
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <http://example.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

ex:ManyPropsShape a sh:NodeShape ;
    sh:targetClass ex:ManyProps ;
"#);

    for i in 1..=20 {
        shacl_schema.push_str(&format!(
            "    sh:property [ sh:path ex:p{} ; sh:datatype xsd:integer ; sh:minCount 0 ; ] ;\n", 
            i
        ));
    }
    shacl_schema.push_str("    .\n");

    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{}", shacl_schema).unwrap();
    let output_file = NamedTempFile::new().unwrap();

    // 2. Configure with max_properties_per_instance = 5
    // Even though probability is 1.0 (default), we should get exactly 5 properties per instance
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 10;
    config.generation.max_properties_per_instance = 5;
    config.generation.property_fill_probability = 1.0; 
    config.generation.cardinality_strategy = rudof_generate::config::CardinalityStrategy::Maximum;
    config.output.path = output_file.path().to_path_buf();
    config.output.format = OutputFormat::NTriples;

    // 3. Generate
    let mut generator = DataGenerator::new(config).unwrap();
    generator.load_shacl_schema(schema_file.path()).await.unwrap();
    generator.generate().await.unwrap();

    // 4. Verify
    let output_content = std::fs::read_to_string(output_file.path()).unwrap();
    let total_triples = output_content.lines().count();
    let type_triples = output_content.lines().filter(|l| l.contains("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")).count();
    let property_triples = total_triples - type_triples;

    println!("Total property triples (max=5): {}", property_triples);

    // 10 entities * 5 properties = 50 properties
    assert_eq!(property_triples, 50, "Should have exactly 50 properties (5 per instance)");
}
