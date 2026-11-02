#[cfg(not(target_family = "wasm"))]
use rudof_generate::config::{
    CardinalityStrategy, DataQuality, DatatypeConfig, EntityDistribution, GeneratorConfig, OutputFormat, PropertyConfig,
};
#[cfg(not(target_family = "wasm"))]
use std::collections::HashMap;
#[cfg(not(target_family = "wasm"))]
use std::path::PathBuf;
#[cfg(not(target_family = "wasm"))]
use tempfile::TempDir;

/// Helper function to create a temporary directory for test outputs
#[cfg(not(target_family = "wasm"))]
fn create_test_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp directory")
}

#[cfg(not(target_family = "wasm"))]
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

#[cfg(not(target_family = "wasm"))]
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

#[cfg(not(target_family = "wasm"))]
#[tokio::test]
async fn test_configuration_merge() {
    let mut config = GeneratorConfig::default();

    // Test merging CLI overrides
    config.merge_cli_overrides(Some(500), Some(PathBuf::from("custom_output.ttl")), Some(98765));

    assert_eq!(config.generation.entity_count, 500);
    assert_eq!(config.output.path.to_string_lossy(), "custom_output.ttl");
    assert_eq!(config.generation.seed, Some(98765));
}

#[cfg(not(target_family = "wasm"))]
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

#[cfg(not(target_family = "wasm"))]
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

#[cfg(not(target_family = "wasm"))]
#[tokio::test]
async fn a_config_without_version_is_rejected() {
    let err = GeneratorConfig::from_toml_str("[generation]\nentity_count = 10\n")
        .expect_err("the v1 format was removed")
        .to_string();
    assert!(err.contains("version = 2"), "{err}");
}

#[cfg(not(target_family = "wasm"))]
#[tokio::test]
async fn a_json_config_reads_like_the_toml_format() {
    let temp_dir = create_test_dir();
    let path = temp_dir.path().join("config.json");
    std::fs::write(
        &path,
        r#"{"version": 2, "run": {"seed": 7, "scale": {"entities": 50}}, "defaults": {"fill": 0.5}}"#,
    )
    .unwrap();
    let config = GeneratorConfig::from_json_file(&path).expect("a v2 JSON config should load");
    assert_eq!(config.generation.seed, Some(7));
    assert_eq!(config.generation.entity_count, 50);
    assert_eq!(config.generation.property_fill_probability, 0.5);

    std::fs::write(&path, r#"{"generation": {"entity_count": 10}}"#).unwrap();
    assert!(
        GeneratorConfig::from_json_file(&path).is_err(),
        "v1 JSON is rejected too"
    );
}
