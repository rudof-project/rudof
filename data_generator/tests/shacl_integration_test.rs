use data_generator::config::GeneratorConfig;
use data_generator::{DataGenerator, Result};
use tempfile::TempDir;
use std::path::PathBuf;

/// Helper function to create a temporary directory for test outputs
fn create_test_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp directory")
}

/// Helper function to create a simple SHACL schema file
fn create_simple_shacl_schema(temp_dir: &TempDir) -> PathBuf {
    let schema_path = temp_dir.path().join("test_shacl.ttl");
    std::fs::write(&schema_path, r#"
@prefix :       <http://example.org/> .
@prefix sh:     <http://www.w3.org/ns/shacl#> .
@prefix xsd:    <http://www.w3.org/2001/XMLSchema#> .
        
:Person a sh:NodeShape ;
   sh:closed true ;
   sh:property [                  
    sh:path     :name ; 
    sh:minCount 1; 
    sh:maxCount 1;
    sh:datatype xsd:string ;
  ] ;
  sh:property [                   
   sh:path     :age ; 
   sh:maxCount 1; 
   sh:datatype xsd:integer ;
  ] .
    "#).expect("Failed to write test SHACL schema");
    schema_path
}

#[tokio::test]
async fn test_shacl_schema_loading() -> Result<()> {
    let temp_dir = create_test_dir();
    let schema_path = create_simple_shacl_schema(&temp_dir);
    let output_path = temp_dir.path().join("output.ttl");
    
    let mut config = GeneratorConfig::default();
    config.output.path = output_path.clone();
    config.generation.entity_count = 5; // Small number for testing
    
    let mut generator = DataGenerator::new(config)?;
    
    // Test SHACL schema loading
    generator.load_shacl_schema(&schema_path).await?;
    
    println!("SHACL schema loaded successfully");
    Ok(())
}

#[tokio::test]
async fn test_shacl_data_generation() -> Result<()> {
    let temp_dir = create_test_dir();
    let schema_path = create_simple_shacl_schema(&temp_dir);
    let output_path = temp_dir.path().join("output.ttl");
    
    let mut config = GeneratorConfig::default();
    config.output.path = output_path.clone();
    config.generation.entity_count = 3; // Small number for testing
    
    let mut generator = DataGenerator::new(config)?;
    
    // Test complete SHACL pipeline: load schema and generate data
    generator.load_shacl_schema(&schema_path).await?;
    generator.generate().await?;
    
    // Verify output file exists and has content
    assert!(output_path.exists(), "Output file should exist");
    let content = std::fs::read_to_string(&output_path)?;
    assert!(!content.is_empty(), "Output file should not be empty");
    
    println!("Generated SHACL-based data:");
    println!("{content}");
    
    Ok(())
}

#[tokio::test]
async fn test_shacl_auto_detection() -> Result<()> {
    let temp_dir = create_test_dir();
    let schema_path = create_simple_shacl_schema(&temp_dir);
    let output_path = temp_dir.path().join("output.ttl");
    
    let mut config = GeneratorConfig::default();
    config.output.path = output_path.clone();
    config.generation.entity_count = 2;
    
    let mut generator = DataGenerator::new(config)?;
    
    // Test auto-detection of SHACL format based on .ttl extension
    generator.load_schema_auto(&schema_path).await?;
    generator.generate().await?;
    
    // Verify output file exists and has content
    assert!(output_path.exists(), "Output file should exist");
    let content = std::fs::read_to_string(&output_path)?;
    assert!(!content.is_empty(), "Output file should not be empty");
    
    println!("Generated data using auto-detection:");
    println!("{content}");
    
    Ok(())
}
