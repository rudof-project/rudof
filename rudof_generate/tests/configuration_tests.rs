use rudof_generate::config::{
    CardinalityStrategy, DataQuality, DatatypeConfig, EntityDistribution, GeneratorConfig, OutputFormat, PropertyConfig,
};
use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper function to create a temporary directory for test outputs
fn create_test_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp directory")
}

#[tokio::test]
async fn test_configuration_loading_toml() {
    let temp_dir = create_test_dir();

    // Create a comprehensive TOML config file
    let config_content = r#"
[generation]
entity_count = 100
seed = 12345
entity_distribution = "Equal"
cardinality_strategy = "Balanced"

[field_generators.default]
locale = "es"
quality = "High"

[field_generators.datatypes]

[field_generators.properties]

[output]
path = "test_output.ttl"
format = "Turtle"
compress = false
write_stats = true
parallel_writing = false
parallel_file_count = 1

[parallel]
worker_threads = 4
batch_size = 50
parallel_shapes = true
parallel_fields = true
"#;

    let config_path = temp_dir.path().join("test_config.toml");
    std::fs::write(&config_path, config_content).expect("Failed to write config file");

    // Load and validate the configuration
    let config = GeneratorConfig::from_toml_file(&config_path).expect("Should load TOML config successfully");

    // Verify generation settings
    assert_eq!(config.generation.entity_count, 100);
    assert_eq!(config.generation.seed, Some(12345));
    assert_eq!(config.generation.entity_distribution, EntityDistribution::Equal);
    assert_eq!(config.generation.cardinality_strategy, CardinalityStrategy::Balanced);

    // Verify field generator settings
    assert_eq!(config.field_generators.default.locale, "es");
    assert_eq!(config.field_generators.default.quality, DataQuality::High);

    // Verify output settings
    assert_eq!(config.output.path.to_string_lossy(), "test_output.ttl");
    assert_eq!(config.output.format, OutputFormat::Turtle);
    assert!(!config.output.compress);
    assert!(config.output.write_stats);

    // Verify parallel settings
    assert_eq!(config.parallel.worker_threads, Some(4));
    assert_eq!(config.parallel.batch_size, 50);
    assert!(config.parallel.parallel_shapes);
    assert!(config.parallel.parallel_fields);
}

#[tokio::test]
async fn test_configuration_loading_json() {
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
    "datatypes": {},
    "properties": {}
  },
  "output": {
    "path": "json_output.ttl",
    "format": "NTriples",
    "compress": true,
    "write_stats": false,
    "parallel_writing": false,
    "parallel_file_count": 1
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
    let config = GeneratorConfig::from_json_file(&config_path).expect("Should load JSON config successfully");

    // Verify generation settings
    assert_eq!(config.generation.entity_count, 75);
    assert_eq!(config.generation.seed, Some(54321));
    assert_eq!(config.generation.entity_distribution, EntityDistribution::Equal);
    assert_eq!(config.generation.cardinality_strategy, CardinalityStrategy::Random);

    // Verify field generator settings
    assert_eq!(config.field_generators.default.locale, "en");
    assert_eq!(config.field_generators.default.quality, DataQuality::Medium);

    // Verify output settings
    assert_eq!(config.output.format, OutputFormat::NTriples);
    assert!(config.output.compress);
    assert!(!config.output.write_stats);

    // Verify parallel settings
    assert_eq!(config.parallel.worker_threads, None); // Auto-detect
    assert_eq!(config.parallel.batch_size, 25);
    assert!(!config.parallel.parallel_shapes);
    assert!(config.parallel.parallel_fields);
}

#[tokio::test]
async fn test_default_configuration() {
    let config = GeneratorConfig::default();

    // Verify sensible defaults
    assert_eq!(config.generation.entity_count, 1000);
    assert_eq!(config.generation.seed, None);
    assert_eq!(config.generation.entity_distribution, EntityDistribution::Equal);
    assert_eq!(config.generation.cardinality_strategy, CardinalityStrategy::Balanced);

    assert_eq!(config.field_generators.default.locale, "en");
    assert_eq!(config.field_generators.default.quality, DataQuality::Medium);

    assert_eq!(config.output.path.to_string_lossy(), "output.ttl");
    assert_eq!(config.output.format, OutputFormat::Turtle);
    assert!(!config.output.compress);
    assert!(config.output.write_stats);

    assert_eq!(config.parallel.worker_threads, None);
    assert_eq!(config.parallel.batch_size, 100);
    assert!(config.parallel.parallel_shapes);
    assert!(config.parallel.parallel_fields);
}

#[tokio::test]
async fn test_configuration_validation() {
    // Test valid configuration
    let mut config = GeneratorConfig::default();
    assert!(config.validate().is_ok(), "Default config should be valid");

    // Test invalid entity count
    config.generation.entity_count = 0;
    assert!(config.validate().is_err(), "Zero entity count should be invalid");

    // Reset and test invalid batch size
    config = GeneratorConfig::default();
    config.parallel.batch_size = 0;
    assert!(config.validate().is_err(), "Zero batch size should be invalid");
}

#[tokio::test]
async fn test_configuration_merge() {
    let mut config = GeneratorConfig::default();

    // Test merging CLI overrides
    config.merge_cli_overrides(Some(500), Some(PathBuf::from("custom_output.ttl")), Some(98765));

    assert_eq!(config.generation.entity_count, 500);
    assert_eq!(config.output.path.to_string_lossy(), "custom_output.ttl");
    assert_eq!(config.generation.seed, Some(98765));
}

#[tokio::test]
async fn test_configuration_serialization() {
    let config = GeneratorConfig::default();
    let temp_dir = create_test_dir();

    // Test TOML serialization
    let toml_path = temp_dir.path().join("test_output.toml");
    config.to_toml_file(&toml_path).expect("Should serialize to TOML");

    // Verify file exists and has content
    assert!(toml_path.exists());
    let toml_content = std::fs::read_to_string(&toml_path).expect("Should read TOML file");
    assert!(!toml_content.is_empty());
    assert!(toml_content.contains("entity_count"));
    assert!(toml_content.contains("locale"));

    // Test round-trip
    let loaded_config = GeneratorConfig::from_toml_file(&toml_path).expect("Should load serialized TOML");
    assert_eq!(loaded_config.generation.entity_count, config.generation.entity_count);
    assert_eq!(
        loaded_config.field_generators.default.locale,
        config.field_generators.default.locale
    );
}

#[tokio::test]
async fn test_datatype_configuration() {
    let mut config = GeneratorConfig::default();

    // Add datatype-specific configuration
    let mut datatype_config = DatatypeConfig {
        generator: "integer".to_string(),
        parameters: HashMap::new(),
    };
    datatype_config
        .parameters
        .insert("min".to_string(), serde_json::json!(18));
    datatype_config
        .parameters
        .insert("max".to_string(), serde_json::json!(65));

    config
        .field_generators
        .datatypes
        .insert("http://www.w3.org/2001/XMLSchema#integer".to_string(), datatype_config);

    // Verify configuration is stored correctly
    let int_config = config
        .field_generators
        .datatypes
        .get("http://www.w3.org/2001/XMLSchema#integer")
        .expect("Should have integer datatype config");

    assert_eq!(int_config.generator, "integer");
    assert_eq!(int_config.parameters["min"].as_i64(), Some(18));
    assert_eq!(int_config.parameters["max"].as_i64(), Some(65));
}

#[tokio::test]
async fn test_property_configuration() {
    let mut config = GeneratorConfig::default();

    // Add property-specific configuration
    let mut property_config = PropertyConfig {
        generator: "string".to_string(),
        parameters: HashMap::new(),
        templates: Some(vec!["{{name}}".to_string()]),
    };
    property_config
        .parameters
        .insert("locale".to_string(), serde_json::json!("fr"));

    config
        .field_generators
        .properties
        .insert("foaf:name".to_string(), property_config);

    // Verify configuration is stored correctly
    let name_config = config
        .field_generators
        .properties
        .get("foaf:name")
        .expect("Should have foaf:name property config");

    assert_eq!(name_config.generator, "string");
    assert_eq!(name_config.parameters["locale"].as_str(), Some("fr"));
    assert!(name_config.templates.is_some());
    assert_eq!(name_config.templates.as_ref().unwrap()[0], "{{name}}");
}

#[tokio::test]
async fn test_complex_configuration_toml() {
    let temp_dir = create_test_dir();

    // Create TOML with datatype and property configurations
    let config_content = r#"
[generation]
entity_count = 50
seed = 999
entity_distribution = "Equal"
cardinality_strategy = "Balanced"

[field_generators.default]
locale = "de"
quality = "Low"

[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#integer"]
generator = "integer"

[field_generators.datatypes."http://www.w3.org/2001/XMLSchema#integer".parameters]
min = 1
max = 100

[field_generators.properties."foaf:name"]
generator = "string"
templates = ["John Doe", "Jane Smith"]

[field_generators.properties."foaf:name".parameters]
locale = "en"

[output]
path = "complex_output.ttl"
format = "Turtle"
compress = false
write_stats = true
parallel_writing = false
parallel_file_count = 1

[parallel]
worker_threads = 1
batch_size = 100
parallel_shapes = true
parallel_fields = true
"#;

    let config_path = temp_dir.path().join("complex_config.toml");
    std::fs::write(&config_path, config_content).expect("Failed to write complex config");

    let config = GeneratorConfig::from_toml_file(&config_path).expect("Should load complex TOML config");

    // Verify complex configuration
    assert_eq!(config.generation.entity_count, 50);
    assert_eq!(config.field_generators.default.quality, DataQuality::Low);
    assert_eq!(config.output.format, OutputFormat::Turtle);

    // Verify datatype config
    let int_config = config
        .field_generators
        .datatypes
        .get("http://www.w3.org/2001/XMLSchema#integer")
        .expect("Should have integer config");
    assert_eq!(int_config.generator, "integer");
    assert_eq!(int_config.parameters["min"].as_i64(), Some(1));
    assert_eq!(int_config.parameters["max"].as_i64(), Some(100));

    // Verify property config
    let name_config = config
        .field_generators
        .properties
        .get("foaf:name")
        .expect("Should have name config");
    assert_eq!(name_config.generator, "string");
    assert_eq!(name_config.parameters["locale"].as_str(), Some("en"));
    assert!(name_config.templates.is_some());
    let templates = name_config.templates.as_ref().unwrap();
    assert_eq!(templates.len(), 2);
    assert!(templates.contains(&"John Doe".to_string()));
    assert!(templates.contains(&"Jane Smith".to_string()));
}
