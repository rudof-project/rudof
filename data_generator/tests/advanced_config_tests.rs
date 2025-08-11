use data_generator::config::{Config, GeneratorConfig};
use data_generator::{DataGenerator, Result};
use tempfile::TempDir;
use std::path::PathBuf;
use serde_json::json;

/// Helper function to create a temporary directory for test outputs
fn create_test_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp directory")
}

/// Helper function to create a more complex test schema with various datatypes
fn create_complex_test_schema(temp_dir: &TempDir) -> PathBuf {
    let schema_path = temp_dir.path().join("complex_test.shex");
    std::fs::write(&schema_path, r#"
PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
PREFIX dc: <http://purl.org/dc/elements/1.1/>

ex:PersonShape {
    foaf:name xsd:string ;
    foaf:age xsd:integer {18,65} ;
    foaf:email xsd:string ? ;
    foaf:birthday xsd:date ? ;
    foaf:height xsd:decimal ? ;
    foaf:isActive xsd:boolean ;
    foaf:homepage xsd:anyURI ?
}

ex:ProductShape {
    dc:title xsd:string ;
    ex:price xsd:decimal {0.01,999.99} ;
    ex:inStock xsd:boolean ;
    ex:stockCount xsd:integer {0,1000} ;
    ex:releaseDate xsd:date ? ;
    ex:productUrl xsd:anyURI ?
}

ex:OrganizationShape {
    foaf:name xsd:string ;
    ex:foundedYear xsd:integer {1800,2024} ;
    ex:website xsd:anyURI ? ;
    ex:isActive xsd:boolean ;
    ex:revenue xsd:decimal ?
}
    "#).expect("Failed to write complex test schema");
    schema_path
}

/// Helper function to run generation with complex schema
fn run_complex_generation_test(config: Config, temp_dir: &TempDir) -> Result<String> {
    let schema_path = create_complex_test_schema(temp_dir);
    let output_path = temp_dir.path().join("output.ttl");
    
    let mut generator = DataGenerator::new(config)?;
    generator.load_schema(&schema_path)?;
    generator.generate()?;
    
    // Verify output file exists and has content
    assert!(output_path.exists(), "Output file should exist");
    let content = std::fs::read_to_string(&output_path)?;
    assert!(!content.is_empty(), "Output file should not be empty");
    
    Ok(content)
}

#[test]
fn test_comprehensive_field_generator_parameters() {
    let temp_dir = create_test_dir();
    
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("comprehensive_output.ttl");
    config.generation.entity_count = 30;
    config.generation.seed = Some(12345); // For reproducible results
    
    // Configure each datatype with specific parameters
    
    // String generator with locale
    config.field_generators.datatypes.insert(
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        GeneratorConfig {
            generator: Some("string".to_string()),
            parameters: json!({
                "locale": "en"
            }),
        }
    );
    
    // Integer generator with range
    config.field_generators.datatypes.insert(
        "http://www.w3.org/2001/XMLSchema#integer".to_string(),
        GeneratorConfig {
            generator: Some("integer".to_string()),
            parameters: json!({
                "min": 1,
                "max": 100
            }),
        }
    );
    
    // Decimal generator with precision
    config.field_generators.datatypes.insert(
        "http://www.w3.org/2001/XMLSchema#decimal".to_string(),
        GeneratorConfig {
            generator: Some("decimal".to_string()),
            parameters: json!({
                "min": 0.1,
                "max": 999.99,
                "precision": 2
            }),
        }
    );
    
    // Boolean generator with bias
    config.field_generators.datatypes.insert(
        "http://www.w3.org/2001/XMLSchema#boolean".to_string(),
        GeneratorConfig {
            generator: Some("boolean".to_string()),
            parameters: json!({
                "true_probability": 0.7
            }),
        }
    );
    
    // Date generator with range
    config.field_generators.datatypes.insert(
        "http://www.w3.org/2001/XMLSchema#date".to_string(),
        GeneratorConfig {
            generator: Some("date".to_string()),
            parameters: json!({
                "start_year": 1990,
                "end_year": 2024
            }),
        }
    );
    
    let content = run_complex_generation_test(config, &temp_dir)
        .expect("Comprehensive field generation should work");
    
    // Validate the generated content
    assert!(!content.is_empty());
    
    // Check that we have various datatypes represented
    assert!(content.contains("foaf:name"), "Should contain name properties");
    assert!(content.contains("foaf:age"), "Should contain age properties");
    assert!(content.contains("ex:price"), "Should contain price properties");
    assert!(content.contains("xsd:boolean"), "Should contain boolean values");
    assert!(content.contains("xsd:date"), "Should contain date values");
    
    // Verify integer values are in expected range
    let age_values: Vec<i32> = content.lines()
        .filter(|line| line.contains("foaf:age"))
        .filter_map(|line| {
            line.split_whitespace()
                .last()
                .and_then(|s| s.trim_end_matches(" .").parse().ok())
        })
        .collect();
    
    for age in age_values {
        assert!(age >= 1 && age <= 100, "Age {} should be in range 1-100", age);
    }
    
    // Verify decimal precision
    let price_pattern = regex::Regex::new(r#""(\d+\.\d{2})"\^\^<http://www\.w3\.org/2001/XMLSchema#decimal>"#).unwrap();
    let price_count = price_pattern.captures_iter(&content).count();
    assert!(price_count > 0, "Should have decimal values with 2 decimal places");
}

#[test]
fn test_property_specific_overrides() {
    let temp_dir = create_test_dir();
    
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("property_override_output.ttl");
    config.generation.entity_count = 20;
    
    // Set default locale
    config.field_generators.default.locale = "en".to_string();
    
    // Override specific properties
    config.field_generators.properties.insert(
        "foaf:name".to_string(),
        GeneratorConfig {
            generator: Some("string".to_string()),
            parameters: json!({
                "locale": "es"  // Spanish names
            }),
        }
    );
    
    config.field_generators.properties.insert(
        "foaf:age".to_string(),
        GeneratorConfig {
            generator: Some("integer".to_string()),
            parameters: json!({
                "min": 25,  // Override age range to be more specific
                "max": 45
            }),
        }
    );
    
    let content = run_complex_generation_test(config, &temp_dir)
        .expect("Property-specific configuration should work");
    
    // Verify age values are in the specific range
    let age_values: Vec<i32> = content.lines()
        .filter(|line| line.contains("foaf:age"))
        .filter_map(|line| {
            line.split_whitespace()
                .last()
                .and_then(|s| s.trim_end_matches(" .").parse().ok())
        })
        .collect();
    
    for age in age_values {
        assert!(age >= 25 && age <= 45, "Age {} should be in range 25-45", age);
    }
}

#[test]
fn test_mixed_configuration_hierarchy() {
    let temp_dir = create_test_dir();
    
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("hierarchy_output.ttl");
    config.generation.entity_count = 15;
    
    // Global default
    config.field_generators.default.locale = "en".to_string();
    
    // Datatype-specific configuration (affects all integers)
    config.field_generators.datatypes.insert(
        "http://www.w3.org/2001/XMLSchema#integer".to_string(),
        GeneratorConfig {
            generator: Some("integer".to_string()),
            parameters: json!({
                "min": 0,
                "max": 50
            }),
        }
    );
    
    // Property-specific configuration (should override datatype config for this property)
    config.field_generators.properties.insert(
        "ex:foundedYear".to_string(),
        GeneratorConfig {
            generator: Some("integer".to_string()),
            parameters: json!({
                "min": 1900,  // Should override the 0-50 range for founded years
                "max": 2024
            }),
        }
    );
    
    let content = run_complex_generation_test(config, &temp_dir)
        .expect("Configuration hierarchy should work");
    
    // Verify that foaf:age uses datatype config (0-50)
    let age_values: Vec<i32> = content.lines()
        .filter(|line| line.contains("foaf:age"))
        .filter_map(|line| {
            line.split_whitespace()
                .last()
                .and_then(|s| s.trim_end_matches(" .").parse().ok())
        })
        .collect();
    
    for age in age_values {
        assert!(age >= 0 && age <= 50, "Age {} should be in datatype range 0-50", age);
    }
    
    // Verify that ex:foundedYear uses property-specific config (1900-2024)
    let founded_values: Vec<i32> = content.lines()
        .filter(|line| line.contains("ex:foundedYear"))
        .filter_map(|line| {
            line.split_whitespace()
                .last()
                .and_then(|s| s.trim_end_matches(" .").parse().ok())
        })
        .collect();
    
    for year in founded_values {
        assert!(year >= 1900 && year <= 2024, 
               "Founded year {} should be in property range 1900-2024", year);
    }
}

#[test]
fn test_edge_case_configurations() {
    let temp_dir = create_test_dir();
    
    // Test with extreme values
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("edge_case_output.ttl");
    config.generation.entity_count = 5;
    
    // Test boolean with 100% true probability
    config.field_generators.datatypes.insert(
        "http://www.w3.org/2001/XMLSchema#boolean".to_string(),
        GeneratorConfig {
            generator: Some("boolean".to_string()),
            parameters: json!({
                "true_probability": 1.0
            }),
        }
    );
    
    // Test integer with narrow range
    config.field_generators.datatypes.insert(
        "http://www.w3.org/2001/XMLSchema#integer".to_string(),
        GeneratorConfig {
            generator: Some("integer".to_string()),
            parameters: json!({
                "min": 42,
                "max": 42  // Single value
            }),
        }
    );
    
    // Test decimal with high precision
    config.field_generators.datatypes.insert(
        "http://www.w3.org/2001/XMLSchema#decimal".to_string(),
        GeneratorConfig {
            generator: Some("decimal".to_string()),
            parameters: json!({
                "min": 0.0,
                "max": 1.0,
                "precision": 6
            }),
        }
    );
    
    let content = run_complex_generation_test(config, &temp_dir)
        .expect("Edge case configuration should work");
    
    // Verify all boolean values are true
    let false_count = content.matches("false").count();
    assert_eq!(false_count, 0, "With 100% true probability, should have no false values");
    
    // Verify all integers are 42
    let integer_values: Vec<i32> = content.lines()
        .filter(|line| line.contains("foaf:age") || line.contains("ex:stockCount"))
        .filter_map(|line| {
            line.split_whitespace()
                .last()
                .and_then(|s| s.trim_end_matches(" .").parse().ok())
        })
        .collect();
    
    for value in integer_values {
        assert_eq!(value, 42, "With min=max=42, all integers should be 42");
    }
    
    // Verify decimal precision
    let decimal_pattern = regex::Regex::new(r#""0\.\d{6}""#).unwrap();
    let high_precision_decimals = decimal_pattern.find_iter(&content).count();
    assert!(high_precision_decimals > 0, "Should have decimals with 6 decimal places");
}

#[test]
fn test_invalid_parameter_handling() {
    let temp_dir = create_test_dir();
    
    let mut config = Config::default();
    config.output.path = temp_dir.path().join("invalid_param_output.ttl");
    config.generation.entity_count = 5;
    
    // Test with invalid parameters (should fall back to defaults)
    config.field_generators.datatypes.insert(
        "http://www.w3.org/2001/XMLSchema#integer".to_string(),
        GeneratorConfig {
            generator: Some("integer".to_string()),
            parameters: json!({
                "min": "not_a_number",  // Invalid type
                "max": null,            // Null value
                "invalid_param": "should_be_ignored"
            }),
        }
    );
    
    // Should not crash and should fall back to defaults
    let content = run_complex_generation_test(config, &temp_dir)
        .expect("Invalid parameters should be handled gracefully");
    
    assert!(!content.is_empty());
    assert!(content.contains("foaf:age"), "Should still generate age values with defaults");
}

#[test]
fn test_performance_configuration_impact() {
    let temp_dir = create_test_dir();
    
    // Test with different parallel configurations
    let configs = vec![
        ("sequential", 1, 10, false, false),
        ("parallel_shapes", 4, 50, true, false),
        ("parallel_fields", 4, 50, false, true),
        ("fully_parallel", 4, 50, true, true),
    ];
    
    for (name, threads, batch_size, parallel_shapes, parallel_fields) in configs {
        let mut config = Config::default();
        config.output.path = temp_dir.path().join(format!("output_{}.ttl", name));
        config.generation.entity_count = 50;
        config.parallel.worker_threads = Some(threads);
        config.parallel.batch_size = batch_size;
        config.parallel.parallel_shapes = parallel_shapes;
        config.parallel.parallel_fields = parallel_fields;
        
        let start_time = std::time::Instant::now();
        let content = run_complex_generation_test(config, &temp_dir)
            .expect(&format!("Configuration {} should work", name));
        let duration = start_time.elapsed();
        
        assert!(!content.is_empty());
        println!("Configuration '{}' took {:?}", name, duration);
        
        // Verify content quality is not affected by parallelization
        let entity_count = content.lines()
            .filter(|line| line.contains(" a "))
            .count();
        assert!(entity_count > 40, 
               "Configuration {} should generate sufficient entities", name);
    }
}

#[test]
fn test_locale_specific_generation() {
    let temp_dir = create_test_dir();
    
    let locales = vec!["en", "es", "fr", "de"];
    
    for locale in locales {
        let mut config = Config::default();
        config.output.path = temp_dir.path().join(format!("output_{}.ttl", locale));
        config.generation.entity_count = 10;
        config.field_generators.default.locale = locale.to_string();
        
        let content = run_complex_generation_test(config, &temp_dir)
            .expect(&format!("Locale {} should work", locale));
        
        assert!(!content.is_empty());
        assert!(content.contains("foaf:name"), 
               "Locale {} should generate name properties", locale);
        
        // The content might differ based on locale, but should always be valid
        // We could add more specific locale validation here if needed
    }
}
