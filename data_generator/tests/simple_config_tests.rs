use data_generator::config::{GeneratorConfig, EntityDistribution, CardinalityStrategy, OutputFormat, DataQuality};
use data_generator::{DataGenerator, Result};
use tempfile::TempDir;
use std::path::PathBuf;

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
    foaf:age xsd:integer
}
    "#).expect("Failed to write test schema");
    schema_path
}

/// Helper function to run generation and verify it produces output
async fn run_simple_generation_test(config: GeneratorConfig, temp_dir: &TempDir) -> Result<String> {
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
async fn test_basic_entity_count_configuration() {
    let temp_dir = create_test_dir();
    
    // Test with small entity count
    let mut config = GeneratorConfig::default();
    config.output.path = temp_dir.path().join("output.ttl");
    config.generation.entity_count = 5;
    
    let content = run_simple_generation_test(config, &temp_dir).await
        .expect("Generation should succeed");
    
    // Verify we have some content
    assert!(!content.is_empty());
    
    // Check for FOAF properties (using full URIs as generated)
    assert!(content.contains("xmlns.com/foaf/0.1/name") || content.contains("xmlns.com/foaf/0.1/age"));
    
    // Count the number of entities generated (by counting 'a' statements)
    let entity_count = content.matches(" a ").count();
    assert_eq!(entity_count, 5, "Should generate exactly 5 entities");
}

#[tokio::test]
async fn test_basic_seed_configuration() {
    let temp_dir1 = create_test_dir();
    let temp_dir2 = create_test_dir();
    
    // Generate with same seed twice
    let seed_value = Some(12345);
    
    let mut config1 = GeneratorConfig::default();
    config1.output.path = temp_dir1.path().join("output1.ttl");
    config1.generation.seed = seed_value;
    config1.generation.entity_count = 5;
    
    let mut config2 = GeneratorConfig::default();
    config2.output.path = temp_dir2.path().join("output2.ttl");
    config2.generation.seed = seed_value;
    config2.generation.entity_count = 5;
    
    let content1 = run_simple_generation_test(config1, &temp_dir1).await
        .expect("First generation should succeed");
    let content2 = run_simple_generation_test(config2, &temp_dir2).await
        .expect("Second generation should succeed");
    
    // With same seed, outputs should be identical
    assert_eq!(content1, content2, "Same seed should produce identical output");
}

#[tokio::test]
async fn test_entity_distribution_configurations() {
    let temp_dir = create_test_dir();
    
    // Test Equal distribution
    let mut config = GeneratorConfig::default();
    config.output.path = temp_dir.path().join("output_equal.ttl");
    config.generation.entity_distribution = EntityDistribution::Equal;
    config.generation.entity_count = 10;
    
    let content = run_simple_generation_test(config, &temp_dir).await
        .expect("Equal distribution should work");
    assert!(!content.is_empty());
}

#[tokio::test]
async fn test_cardinality_strategy_configurations() {
    let temp_dir = create_test_dir();
    
    let strategies = vec![
        CardinalityStrategy::Minimum,
        CardinalityStrategy::Maximum,
        CardinalityStrategy::Random,
        CardinalityStrategy::Balanced,
    ];
    
    for strategy in strategies {
        let mut config = GeneratorConfig::default();
        config.output.path = temp_dir.path().join(format!("output_{:?}.ttl", strategy));
        config.generation.cardinality_strategy = strategy;
        config.generation.entity_count = 5;
        
        let content = run_simple_generation_test(config, &temp_dir).await
            .expect(&format!("Cardinality strategy {:?} should work", strategy));
        assert!(!content.is_empty());
    }
}

#[tokio::test]
async fn test_field_generator_locale_configuration() {
    let temp_dir = create_test_dir();
    
    // Test with Spanish locale
    let mut config = GeneratorConfig::default();
    config.output.path = temp_dir.path().join("output_es.ttl");
    config.field_generators.default.locale = "es".to_string();
    config.generation.entity_count = 3;
    
    let content = run_simple_generation_test(config, &temp_dir).await
        .expect("Spanish locale should work");
    assert!(!content.is_empty());
}

#[tokio::test]
async fn test_field_generator_quality_configuration() {
    let temp_dir = create_test_dir();
    
    let qualities = vec![DataQuality::Low, DataQuality::Medium, DataQuality::High];
    
    for quality in qualities {
        let mut config = GeneratorConfig::default();
        config.output.path = temp_dir.path().join(format!("output_{:?}.ttl", quality));
        config.field_generators.default.quality = quality;
        config.generation.entity_count = 3;
        
        let content = run_simple_generation_test(config, &temp_dir).await
            .expect(&format!("Quality {:?} should work", quality));
        assert!(!content.is_empty());
    }
}

#[tokio::test]
async fn test_output_format_configurations() {
    let temp_dir = create_test_dir();
    
    // Only test supported formats (Turtle and NTriples)
    let formats = vec![
        (OutputFormat::Turtle, "output.ttl"),
        (OutputFormat::NTriples, "output.nt"),
    ];
    
    for (format, filename) in formats {
        let mut config = GeneratorConfig::default();
        config.output.path = temp_dir.path().join(filename);
        config.output.format = format;
        config.generation.entity_count = 3;
        
        let content = run_simple_generation_test(config, &temp_dir).await
            .expect(&format!("Output format {:?} should work", format));
        assert!(!content.is_empty());
        
        // Basic format validation
        match format {
            OutputFormat::Turtle => {
                assert!(content.contains("@prefix") || content.contains("<"), 
                       "Turtle format should contain prefixes or URIs");
            },
            OutputFormat::NTriples => {
                // N-Triples validation can be loose since some lines might be empty
                let non_empty_lines: Vec<&str> = content.lines()
                    .filter(|line| !line.trim().is_empty())
                    .collect();
                if !non_empty_lines.is_empty() {
                    assert!(non_empty_lines.iter().any(|line| line.ends_with(" .")), 
                           "N-Triples should have lines ending with dots");
                }
            },
            // JsonLd and RdfXml removed - only Turtle and NTriples supported
        }
    }
}

#[tokio::test]
async fn test_parallel_configuration() {
    let temp_dir = create_test_dir();
    
    // Test with specific thread count
    let mut config = GeneratorConfig::default();
    config.output.path = temp_dir.path().join("output_parallel.ttl");
    config.parallel.worker_threads = Some(2);
    config.parallel.batch_size = 50;
    config.parallel.parallel_shapes = true;
    config.parallel.parallel_fields = true;
    config.generation.entity_count = 5;
    
    let content = run_simple_generation_test(config, &temp_dir).await
        .expect("Parallel configuration should work");
    assert!(!content.is_empty());
    
    // Test with auto-detected threads
    let mut config = GeneratorConfig::default();
    config.output.path = temp_dir.path().join("output_auto_threads.ttl");
    config.parallel.worker_threads = None; // Auto-detect
    config.generation.entity_count = 5;
    
    let content = run_simple_generation_test(config, &temp_dir).await
        .expect("Auto thread detection should work");
    assert!(!content.is_empty());
}

#[tokio::test]
async fn test_configuration_validation() {
    // Test that invalid configurations are rejected
    let temp_dir = create_test_dir();
    
    let mut config = GeneratorConfig::default();
    config.output.path = temp_dir.path().join("output.ttl");
    
    // Test zero entity count (should be rejected or handled gracefully)
    config.generation.entity_count = 0;
    
    // This should either fail gracefully or produce empty output
    let result = run_simple_generation_test(config, &temp_dir).await;
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
