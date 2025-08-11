use data_generator::config::{GeneratorConfig, GenerationConfig, FieldGeneratorConfig, OutputConfig, ParallelConfig};
use data_generator::config::{EntityDistribution, CardinalityStrategy, OutputFormat, DataQuality, DatatypeConfig, PropertyConfig};
use data_generator::{DataGenerator, Result};
use tempfile::TempDir;
use std::path::PathBuf;
use serde_json::json;
use std::collections::HashMap;

/// Helper function to create a temporary directory for test outputs
fn create_test_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp directory")
}

/// Helper function to create a minimal test schema file
fn create_test_schema(temp_dir: &TempDir) -> PathBuf {
    let schema_path = temp_dir.path().join("test.shex");
    std::fs::write(&schema_path, r#"
PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
PREFIX foaf: <http://xmlns.com/foaf/0.1/>

ex:PersonShape {
    foaf:name xsd:string ;
    foaf:age xsd:integer ;
    foaf:email xsd:string ?
}

ex:BookShape {
    ex:title xsd:string ;
    ex:price xsd:decimal ?;
    ex:inStock xsd:boolean
}
    "#).expect("Failed to write test schema");
    schema_path
}

/// Helper function to run generation and verify it produces output
async fn run_generation_test(config: GeneratorConfig, temp_dir: &TempDir) -> Result<String> {
    let schema_path = create_test_schema(temp_dir);
    let output_path = temp_dir.path().join("output.ttl");
    
    let mut generator = DataGenerator::new(config)?;
    generator.load_schema(&schema_path).await?;
    generator.generate().await?;
    
    // Verify output file exists and has content
    assert!(output_path.exists(), "Output file should exist");
    let content = std::fs::read_to_string(&output_path)?;
    assert!(!content.is_empty(), "Output file should not be empty");
    
    Ok(content)
}

#[tokio::test]
async fn test_entity_count_configuration() {
    let temp_dir = create_test_dir();
    
    // Test with small entity count
    let mut config = GeneratorConfig::default();
    config.output.path = temp_dir.path().join("output.ttl");
    config.generation.entity_count = 10;
    
    let content = run_generation_test(config, &temp_dir).await.expect("Generation should succeed");
    
    // Count entities in output (rough estimate by counting subjects)
    let entity_lines: Vec<&str> = content.lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('@'))
        .filter(|line| line.contains(" a "))
        .collect();
    
    // Should have around 10 entities (allowing for some variance due to relationships)
    assert!(entity_lines.len() >= 8 && entity_lines.len() <= 15, 
            "Should generate approximately 10 entities, got {}", entity_lines.len());
}

#[test]
fn test_seed_configuration() {
    let temp_dir1 = create_test_dir();
    let temp_dir2 = create_test_dir();
    
    // Generate with same seed twice
    let seed_value = Some(12345);
    
    let mut config1 = Config::default();
    config1.output.path = temp_dir1.path().join("output1.ttl");
    config1.generation.seed = seed_value;
    config1.generation.entity_count = 20;
    
    let mut config2 = Config::default();
    config2.output.path = temp_dir2.path().join("output2.ttl");
    config2.generation.seed = seed_value;
    config2.generation.entity_count = 20;
    
    let content1 = run_generation_test(config1, &temp_dir1).expect("First generation should succeed");
    let content2 = run_generation_test(config2, &temp_dir2).expect("Second generation should succeed");
    
    // With same seed, outputs should be identical
    assert_eq!(content1, content2, "Same seed should produce identical output");
}

#[test]
fn test_entity_distribution_configurations() {
    let temp_dir = create_test_dir();
    
    // Test Equal distribution
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("output_equal.ttl");
    config.generation.entity_distribution = EntityDistribution::Equal;
    config.generation.entity_count = 20;
    
    let content = run_generation_test(config, &temp_dir).expect("Equal distribution should work");
    assert!(!content.is_empty());
    
    // Test Weighted distribution
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("output_weighted.ttl");
    config.generation.entity_distribution = EntityDistribution::Weighted;
    config.generation.entity_count = 20;
    
    let content = run_generation_test(config, &temp_dir).expect("Weighted distribution should work");
    assert!(!content.is_empty());
}

#[test]
fn test_cardinality_strategy_configurations() {
    let temp_dir = create_test_dir();
    
    let strategies = vec![
        CardinalityStrategy::Minimum,
        CardinalityStrategy::Maximum,
        CardinalityStrategy::Random,
        CardinalityStrategy::Balanced,
    ];
    
    for strategy in strategies {
        let mut config = Config::default();
        config.output.path = temp_dir.path().join(format!("output_{:?}.ttl", strategy));
        config.generation.cardinality_strategy = strategy;
        config.generation.entity_count = 10;
        
        let content = run_generation_test(config, &temp_dir)
            .expect(&format!("Cardinality strategy {:?} should work", strategy));
        assert!(!content.is_empty());
    }
}

#[test]
fn test_field_generator_locale_configuration() {
    let temp_dir = create_test_dir();
    
    // Test with Spanish locale
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("output_es.ttl");
    config.field_generators.default.locale = "es".to_string();
    config.generation.entity_count = 5;
    
    let content = run_generation_test(config, &temp_dir).expect("Spanish locale should work");
    assert!(!content.is_empty());
    
    // Test with French locale
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("output_fr.ttl");
    config.field_generators.default.locale = "fr".to_string();
    config.generation.entity_count = 5;
    
    let content = run_generation_test(config, &temp_dir).expect("French locale should work");
    assert!(!content.is_empty());
}

#[test]
fn test_field_generator_quality_configuration() {
    let temp_dir = create_test_dir();
    
    let qualities = vec![Quality::Low, Quality::Medium, Quality::High];
    
    for quality in qualities {
        let mut config = Config::default();
        config.output.path = temp_dir.path().join(format!("output_{:?}.ttl", quality));
        config.field_generators.default.quality = quality;
        config.generation.entity_count = 5;
        
        let content = run_generation_test(config, &temp_dir)
            .expect(&format!("Quality {:?} should work", quality));
        assert!(!content.is_empty());
    }
}

#[test]
fn test_datatype_specific_configuration() {
    let temp_dir = create_test_dir();
    
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("output_datatypes.ttl");
    config.generation.entity_count = 10;
    
    // Configure integer generator with specific range
    config.field_generators.datatypes.insert(
        "http://www.w3.org/2001/XMLSchema#integer".to_string(),
        DatatypeConfig {
            generator: "integer".to_string(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("min".to_string(), json!(18));
                params.insert("max".to_string(), json!(65));
                params
            },
        }
    );
    
    // Configure decimal generator with specific precision
    config.field_generators.datatypes.insert(
        "http://www.w3.org/2001/XMLSchema#decimal".to_string(),
        DatatypeConfig {
            generator: "decimal".to_string(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("min".to_string(), json!(0.0));
                params.insert("max".to_string(), json!(100.0));
                params.insert("precision".to_string(), json!(3));
                params
            },
        }
    );
    
    let content = run_generation_test(config, &temp_dir).expect("Datatype configuration should work");
    assert!(!content.is_empty());
    
    // Verify integers are in expected range (18-65)
    let integer_values: Vec<i32> = content.lines()
        .filter(|line| line.contains("foaf:age"))
        .filter_map(|line| {
            line.split_whitespace()
                .last()
                .and_then(|s| s.trim_end_matches(" .").parse().ok())
        })
        .collect();
    
    for value in integer_values {
        assert!(value >= 18 && value <= 65, "Integer {} should be in range 18-65", value);
    }
}

#[test]
fn test_property_specific_configuration() {
    let temp_dir = create_test_dir();
    
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("output_properties.ttl");
    config.generation.entity_count = 10;
    
    // Configure foaf:name with specific locale
    config.field_generators.properties.insert(
        "foaf:name".to_string(),
        data_generator::config::GeneratorConfig {
            generator: Some("string".to_string()),
            parameters: json!({
                "locale": "es"
            }),
        }
    );
    
    let content = run_generation_test(config, &temp_dir).expect("Property configuration should work");
    assert!(!content.is_empty());
    assert!(content.contains("foaf:name"), "Output should contain foaf:name properties");
}

#[test]
fn test_output_format_configurations() {
    let temp_dir = create_test_dir();
    
    let formats = vec![
        (OutputFormat::Turtle, "output.ttl"),
        (OutputFormat::NTriples, "output.nt"),
        (OutputFormat::JsonLd, "output.jsonld"),
        (OutputFormat::RdfXml, "output.rdf"),
    ];
    
    for (format, filename) in formats {
        let mut config = Config::default();
        config.output.path = temp_dir.path().join(filename);
        config.output.format = format;
        config.generation.entity_count = 5;
        
        let content = run_generation_test(config, &temp_dir)
            .expect(&format!("Output format {:?} should work", format));
        assert!(!content.is_empty());
        
        // Basic format validation
        match format {
            OutputFormat::Turtle => {
                assert!(content.contains("@prefix") || content.contains("<"), 
                       "Turtle format should contain prefixes or URIs");
            },
            OutputFormat::NTriples => {
                assert!(content.lines().all(|line| 
                    line.trim().is_empty() || line.ends_with(" .")), 
                    "N-Triples should have dots at end of lines");
            },
            OutputFormat::JsonLd => {
                assert!(content.trim().starts_with("{{") || content.trim().starts_with("["), 
                       "JSON-LD should start with {{ or [");
            },
            OutputFormat::RdfXml => {
                assert!(content.contains("<?xml") || content.contains("<rdf:"), 
                       "RDF/XML should contain XML declarations");
            },
        }
    }
}

#[test]
fn test_compression_configuration() {
    let temp_dir = create_test_dir();
    
    // Test without compression
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("output_uncompressed.ttl");
    config.output.compress = false;
    config.generation.entity_count = 10;
    
    let content_uncompressed = run_generation_test(config, &temp_dir)
        .expect("Uncompressed output should work");
    
    // Test with compression
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("output_compressed.ttl.gz");
    config.output.compress = true;
    config.generation.entity_count = 10;
    
    // Note: This will create a .gz file
    run_generation_test(config, &temp_dir).expect("Compressed output should work");
    
    // Verify compressed file exists
    let compressed_path = temp_dir.path().join("output_compressed.ttl.gz");
    assert!(compressed_path.exists(), "Compressed file should exist");
}

#[test]
fn test_statistics_configuration() {
    let temp_dir = create_test_dir();
    
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("output_with_stats.ttl");
    config.output.write_stats = true;
    config.generation.entity_count = 10;
    
    run_generation_test(config, &temp_dir).expect("Statistics generation should work");
    
    // Check if stats file exists
    let stats_path = temp_dir.path().join("output_with_stats.ttl.stats.json");
    assert!(stats_path.exists(), "Statistics file should exist");
    
    // Verify stats file has content
    let stats_content = std::fs::read_to_string(&stats_path)
        .expect("Should be able to read stats file");
    assert!(!stats_content.is_empty(), "Stats file should not be empty");
    
    // Verify it's valid JSON
    let _: serde_json::Value = serde_json::from_str(&stats_content)
        .expect("Stats file should contain valid JSON");
}

#[test]
fn test_parallel_configuration() {
    let temp_dir = create_test_dir();
    
    // Test with specific thread count
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("output_parallel.ttl");
    config.parallel.worker_threads = Some(2);
    config.parallel.batch_size = 50;
    config.parallel.parallel_shapes = true;
    config.parallel.parallel_fields = true;
    config.generation.entity_count = 20;
    
    let content = run_generation_test(config, &temp_dir).expect("Parallel configuration should work");
    assert!(!content.is_empty());
    
    // Test with auto-detected threads
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("output_auto_threads.ttl");
    config.parallel.worker_threads = None; // Auto-detect
    config.generation.entity_count = 20;
    
    let content = run_generation_test(config, &temp_dir).expect("Auto thread detection should work");
    assert!(!content.is_empty());
}

#[test]
fn test_boolean_generator_configuration() {
    let temp_dir = create_test_dir();
    
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("output_boolean.ttl");
    config.generation.entity_count = 20;
    
    // Configure boolean generator with high true probability
    config.field_generators.datatypes.insert(
        "http://www.w3.org/2001/XMLSchema#boolean".to_string(),
        data_generator::config::GeneratorConfig {
            generator: Some("boolean".to_string()),
            parameters: json!({
                "true_probability": 0.8
            }),
        }
    );
    
    let content = run_generation_test(config, &temp_dir).expect("Boolean configuration should work");
    assert!(!content.is_empty());
    
    // Count true vs false values
    let true_count = content.matches("true").count();
    let false_count = content.matches("false").count();
    let total_booleans = true_count + false_count;
    
    if total_booleans > 0 {
        let true_ratio = true_count as f64 / total_booleans as f64;
        // With 80% probability, we should have more trues than falses
        // Allow some variance due to randomness
        assert!(true_ratio > 0.6, 
               "With 80% true probability, should have more trues. Got {:.2}% true", 
               true_ratio * 100.0);
    }
}

#[test]
fn test_date_generator_configuration() {
    let temp_dir = create_test_dir();
    
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("output_dates.ttl");
    config.generation.entity_count = 10;
    
    // Configure date generator with specific year range
    config.field_generators.datatypes.insert(
        "http://www.w3.org/2001/XMLSchema#date".to_string(),
        data_generator::config::GeneratorConfig {
            generator: Some("date".to_string()),
            parameters: json!({
                "start_year": 2020,
                "end_year": 2023
            }),
        }
    );
    
    let content = run_generation_test(config, &temp_dir).expect("Date configuration should work");
    assert!(!content.is_empty());
    
    // Extract date values and verify they're in the expected range
    let date_pattern = regex::Regex::new(r"(\d{4})-\d{2}-\d{2}").unwrap();
    for captures in date_pattern.captures_iter(&content) {
        if let Some(year_str) = captures.get(1) {
            let year: i32 = year_str.as_str().parse().expect("Should be valid year");
            assert!(year >= 2020 && year <= 2023, 
                   "Year {} should be in range 2020-2023", year);
        }
    }
}

#[test]
fn test_configuration_validation() {
    // Test that invalid configurations are rejected
    let temp_dir = create_test_dir();
    
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("output.ttl");
    
    // Test zero entity count (should be rejected or handled gracefully)
    config.generation.entity_count = 0;
    
    // This should either fail gracefully or produce empty output
    let result = run_generation_test(config, &temp_dir);
    match result {
        Ok(content) => {
            // If it succeeds, content should be minimal
            assert!(content.lines().filter(|l| !l.trim().is_empty()).count() <= 5);
        },
        Err(_) => {
            // It's also acceptable for this to fail
        }
    }
}
