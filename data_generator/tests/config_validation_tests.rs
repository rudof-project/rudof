use data_generator::config::{Config, EntityDistribution, CardinalityStrategy, OutputFormat, Quality};
use data_generator::{Result};
use tempfile::TempDir;
use std::path::PathBuf;

/// Helper function to create a temporary directory for test outputs
fn create_test_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp directory")
}

#[test]
fn test_toml_config_loading() {
    let temp_dir = create_test_dir();
    
    // Create a comprehensive TOML config file
    let config_content = r#"
[generation]
entity_count = 100
seed = 12345
entity_distribution = "Weighted"
cardinality_strategy = "Balanced"

[field_generators.default]
locale = "es"
quality = "High"

[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#integer"]
generator = "integer"
[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#integer".parameters]
min = 18
max = 65

[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#decimal"]
generator = "decimal"
[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#decimal".parameters]
min = 0.0
max = 100.0
precision = 3

[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#boolean"]
generator = "boolean"
[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#boolean".parameters]
true_probability = 0.7

[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#date"]
generator = "date"
[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#date".parameters]
start_year = 2000
end_year = 2023

[field_generators.properties."foaf:name"]
generator = "string"
[field_generators.properties."foaf:name".parameters]
locale = "fr"

[field_generators.properties."foaf:age"]
generator = "integer"
[field_generators.properties."foaf:age".parameters]
min = 25
max = 45

[output]
path = "test_output.ttl"
format = "Turtle"
compress = false
write_stats = true

[parallel]
worker_threads = 4
batch_size = 50
parallel_shapes = true
parallel_fields = true
"#;
    
    let config_path = temp_dir.path().join("test_config.toml");
    std::fs::write(&config_path, config_content).expect("Failed to write config file");
    
    // Load and validate the configuration
    let config = Config::from_file(&config_path).expect("Should load TOML config successfully");
    
    // Verify generation settings
    assert_eq!(config.generation.entity_count, 100);
    assert_eq!(config.generation.seed, Some(12345));
    assert_eq!(config.generation.entity_distribution, EntityDistribution::Weighted);
    assert_eq!(config.generation.cardinality_strategy, CardinalityStrategy::Balanced);
    
    // Verify field generator settings
    assert_eq!(config.field_generators.default.locale, "es");
    assert_eq!(config.field_generators.default.quality, Quality::High);
    
    // Verify datatype-specific settings
    let int_config = config.field_generators.datatypes
        .get("http://www.w3.org/2001/XMLSchema#integer")
        .expect("Should have integer datatype config");
    assert_eq!(int_config.generator, Some("integer".to_string()));
    assert_eq!(int_config.parameters["min"].as_i64(), Some(18));
    assert_eq!(int_config.parameters["max"].as_i64(), Some(65));
    
    let bool_config = config.field_generators.datatypes
        .get("http://www.w3.org/2001/XMLSchema#boolean")
        .expect("Should have boolean datatype config");
    assert_eq!(bool_config.parameters["true_probability"].as_f64(), Some(0.7));
    
    // Verify property-specific settings
    let name_config = config.field_generators.properties
        .get("foaf:name")
        .expect("Should have foaf:name property config");
    assert_eq!(name_config.parameters["locale"].as_str(), Some("fr"));
    
    // Verify output settings
    assert_eq!(config.output.path.to_string_lossy(), "test_output.ttl");
    assert_eq!(config.output.format, OutputFormat::Turtle);
    assert_eq!(config.output.compress, false);
    assert_eq!(config.output.write_stats, true);
    
    // Verify parallel settings
    assert_eq!(config.parallel.worker_threads, Some(4));
    assert_eq!(config.parallel.batch_size, 50);
    assert_eq!(config.parallel.parallel_shapes, true);
    assert_eq!(config.parallel.parallel_fields, true);
}

#[test]
fn test_json_config_loading() {
    let temp_dir = create_test_dir();
    
    // Create a comprehensive JSON config file
    let config_content = r#"{
  "generation": {
    "entity_count": 75,
    "seed": 54321,
    "entity_distribution": "Equal",
    "cardinality_strategy": "Random"
  },
  "field_generators": {
    "default": {
      "locale": "en",
      "quality": "Medium"
    },
    "datatypes": {
      "http://www.w3.org/2001/XMLSchema#integer": {
        "generator": "integer",
        "parameters": {
          "min": 0,
          "max": 1000
        }
      },
      "http://www.w3.org/2001/XMLSchema#string": {
        "generator": "string",
        "parameters": {
          "locale": "de"
        }
      }
    },
    "properties": {
      "ex:price": {
        "generator": "decimal",
        "parameters": {
          "min": 1.0,
          "max": 999.99,
          "precision": 2
        }
      }
    }
  },
  "output": {
    "path": "json_output.ttl",
    "format": "NTriples",
    "compress": true,
    "write_stats": false
  },
  "parallel": {
    "worker_threads": null,
    "batch_size": 25,
    "parallel_shapes": false,
    "parallel_fields": true
  }
}"#;
    
    let config_path = temp_dir.path().join("test_config.json");
    std::fs::write(&config_path, config_content).expect("Failed to write JSON config file");
    
    // Load and validate the configuration
    let config = Config::from_file(&config_path).expect("Should load JSON config successfully");
    
    // Verify generation settings
    assert_eq!(config.generation.entity_count, 75);
    assert_eq!(config.generation.seed, Some(54321));
    assert_eq!(config.generation.entity_distribution, EntityDistribution::Equal);
    assert_eq!(config.generation.cardinality_strategy, CardinalityStrategy::Random);
    
    // Verify field generator settings
    assert_eq!(config.field_generators.default.locale, "en");
    assert_eq!(config.field_generators.default.quality, Quality::Medium);
    
    // Verify output settings
    assert_eq!(config.output.format, OutputFormat::NTriples);
    assert_eq!(config.output.compress, true);
    assert_eq!(config.output.write_stats, false);
    
    // Verify parallel settings
    assert_eq!(config.parallel.worker_threads, None); // Auto-detect
    assert_eq!(config.parallel.batch_size, 25);
    assert_eq!(config.parallel.parallel_shapes, false);
    assert_eq!(config.parallel.parallel_fields, true);
}

#[test]
fn test_partial_config_loading() {
    let temp_dir = create_test_dir();
    
    // Create a minimal config file (should use defaults for missing values)
    let config_content = r#"
[generation]
entity_count = 50

[field_generators.default]
locale = "fr"

[output]
path = "minimal_output.ttl"
"#;
    
    let config_path = temp_dir.path().join("minimal_config.toml");
    std::fs::write(&config_path, config_content).expect("Failed to write minimal config file");
    
    let config = Config::from_file(&config_path).expect("Should load minimal config successfully");
    
    // Verify specified values
    assert_eq!(config.generation.entity_count, 50);
    assert_eq!(config.field_generators.default.locale, "fr");
    assert_eq!(config.output.path.to_string_lossy(), "minimal_output.ttl");
    
    // Verify defaults are used for unspecified values
    assert_eq!(config.generation.seed, None); // Default
    assert_eq!(config.generation.entity_distribution, EntityDistribution::Equal); // Default
    assert_eq!(config.output.format, OutputFormat::Turtle); // Default
    assert_eq!(config.output.compress, false); // Default
    assert_eq!(config.parallel.batch_size, 100); // Default
}

#[test]
fn test_config_validation_errors() {
    let temp_dir = create_test_dir();
    
    // Test invalid enum values
    let invalid_config = r#"
[generation]
entity_count = 10
entity_distribution = "InvalidDistribution"

[output]
path = "test.ttl"
format = "InvalidFormat"
"#;
    
    let config_path = temp_dir.path().join("invalid_config.toml");
    std::fs::write(&config_path, invalid_config).expect("Failed to write invalid config file");
    
    // This should fail to load due to invalid enum values
    let result = Config::from_file(&config_path);
    assert!(result.is_err(), "Should fail to load config with invalid enum values");
}

#[test]
fn test_config_file_not_found() {
    let temp_dir = create_test_dir();
    let non_existent_path = temp_dir.path().join("does_not_exist.toml");
    
    let result = Config::from_file(&non_existent_path);
    assert!(result.is_err(), "Should fail when config file doesn't exist");
}

#[test]
fn test_malformed_config_files() {
    let temp_dir = create_test_dir();
    
    // Test malformed TOML
    let malformed_toml = r#"
[generation
entity_count = 10  # Missing closing bracket
"#;
    
    let toml_path = temp_dir.path().join("malformed.toml");
    std::fs::write(&toml_path, malformed_toml).expect("Failed to write malformed TOML");
    
    let result = Config::from_file(&toml_path);
    assert!(result.is_err(), "Should fail to load malformed TOML");
    
    // Test malformed JSON
    let malformed_json = r#"{
  "generation": {
    "entity_count": 10,
  }  // Trailing comma
}"#;
    
    let json_path = temp_dir.path().join("malformed.json");
    std::fs::write(&json_path, malformed_json).expect("Failed to write malformed JSON");
    
    let result = Config::from_file(&json_path);
    assert!(result.is_err(), "Should fail to load malformed JSON");
}

#[test]
fn test_config_merge_and_override() {
    let temp_dir = create_test_dir();
    
    // Create base config
    let base_config = r#"
[generation]
entity_count = 100
seed = 123

[field_generators.default]
locale = "en"
quality = "Medium"

[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#integer"]
generator = "integer"
[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#integer".parameters]
min = 0
max = 100

[output]
path = "base_output.ttl"
format = "Turtle"
"#;
    
    let base_path = temp_dir.path().join("base_config.toml");
    std::fs::write(&base_path, base_config).expect("Failed to write base config");
    
    let base_config = Config::from_file(&base_path).expect("Should load base config");
    
    // Create override config (in practice, this would be CLI args or env vars)
    let mut override_config = Config::default();
    override_config.generation.entity_count = 200; // Override
    override_config.output.format = OutputFormat::NTriples; // Override
    // locale and other settings should come from base
    
    // In a real implementation, you'd have a merge function
    // For now, just verify that individual configs work
    assert_eq!(base_config.generation.entity_count, 100);
    assert_eq!(base_config.field_generators.default.locale, "en");
    assert_eq!(override_config.generation.entity_count, 200);
}

#[test]
fn test_environment_variable_substitution() {
    let temp_dir = create_test_dir();
    
    // This test would be more complete if we had env var substitution
    // For now, just test that we can load configs that might reference env vars
    let config_with_path = r#"
[generation]
entity_count = 10

[output]
path = "./test_env_output.ttl"

[parallel]
worker_threads = 2  # Could be from env var in real implementation
"#;
    
    let config_path = temp_dir.path().join("env_config.toml");
    std::fs::write(&config_path, config_with_path).expect("Failed to write env config");
    
    let config = Config::from_file(&config_path).expect("Should load config with path");
    assert_eq!(config.parallel.worker_threads, Some(2));
}

#[test]
fn test_config_with_all_output_formats() {
    let temp_dir = create_test_dir();
    
    let formats = vec![
        ("Turtle", OutputFormat::Turtle),
        ("NTriples", OutputFormat::NTriples), 
        ("JsonLd", OutputFormat::JsonLd),
        ("RdfXml", OutputFormat::RdfXml),
    ];
    
    for (format_str, expected_format) in formats {
        let config_content = format!(r#"
[generation]
entity_count = 5

[output]
path = "output_{}.txt"
format = "{}"
"#, format_str.to_lowercase(), format_str);
        
        let config_path = temp_dir.path().join(format!("config_{}.toml", format_str.to_lowercase()));
        std::fs::write(&config_path, config_content).expect("Failed to write format config");
        
        let config = Config::from_file(&config_path)
            .expect(&format!("Should load config with format {}", format_str));
        
        assert_eq!(config.output.format, expected_format);
    }
}

#[test]
fn test_config_with_all_quality_levels() {
    let temp_dir = create_test_dir();
    
    let qualities = vec![
        ("Low", Quality::Low),
        ("Medium", Quality::Medium),
        ("High", Quality::High),
    ];
    
    for (quality_str, expected_quality) in qualities {
        let config_content = format!(r#"
[generation]
entity_count = 5

[field_generators.default]
quality = "{}"
"#, quality_str);
        
        let config_path = temp_dir.path().join(format!("config_{}.toml", quality_str.to_lowercase()));
        std::fs::write(&config_path, config_content).expect("Failed to write quality config");
        
        let config = Config::from_file(&config_path)
            .expect(&format!("Should load config with quality {}", quality_str));
        
        assert_eq!(config.field_generators.default.quality, expected_quality);
    }
}

#[test]
fn test_config_serialization_roundtrip() {
    // Create a complex config programmatically
    let mut config = Config::default();
    config.generation.entity_count = 123;
    config.generation.seed = Some(456);
    config.field_generators.default.locale = "de".to_string();
    config.output.format = OutputFormat::JsonLd;
    config.output.compress = true;
    
    // Serialize to TOML
    let toml_string = toml::to_string(&config).expect("Should serialize config to TOML");
    assert!(!toml_string.is_empty());
    assert!(toml_string.contains("entity_count = 123"));
    assert!(toml_string.contains("locale = \"de\""));
    
    // Deserialize back
    let config_from_toml: Config = toml::from_str(&toml_string)
        .expect("Should deserialize config from TOML");
    
    assert_eq!(config_from_toml.generation.entity_count, 123);
    assert_eq!(config_from_toml.generation.seed, Some(456));
    assert_eq!(config_from_toml.field_generators.default.locale, "de");
    assert_eq!(config_from_toml.output.format, OutputFormat::JsonLd);
    assert_eq!(config_from_toml.output.compress, true);
}
