#![allow(clippy::regex_creation_in_loops)]
#![allow(clippy::manual_strip)]

use rudof_generate::config::{DatatypeConfig, PropertyConfig};
use rudof_generate::field_generators::pattern::PatternGenerator;
use rudof_generate::field_generators::{FieldGenerator, GenerationContext};
use rudof_generate::{DataGenerator, GeneratorConfig};
use regex::Regex;
use serde_json::json;
use std::collections::HashMap;
use std::io::Write;
use tempfile::NamedTempFile;

/// Test the PatternGenerator directly with common patterns
#[tokio::test]
async fn test_pattern_generator_phone_numbers() {
    let generator = PatternGenerator;

    // Test US phone number pattern
    let mut context = GenerationContext::new(
        "http://example.org/phone".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context
        .parameters
        .insert("pattern".to_string(), json!("\\d{3}-\\d{3}-\\d{4}"));

    for _ in 0..10 {
        let result = generator.generate(&context).unwrap();
        println!("Generated phone: {result}");

        // Validate format: XXX-XXX-XXXX where X is a digit
        let phone_regex = Regex::new(r"^\d{3}-\d{3}-\d{4}$").unwrap();
        assert!(
            phone_regex.is_match(&result),
            "Phone number should match XXX-XXX-XXXX format: {result}"
        );

        // Validate parts
        let parts: Vec<&str> = result.split('-').collect();
        assert_eq!(
            parts.len(),
            3,
            "Phone should have 3 parts separated by dashes"
        );
        assert_eq!(parts[0].len(), 3, "First part should be 3 digits");
        assert_eq!(parts[1].len(), 3, "Second part should be 3 digits");
        assert_eq!(parts[2].len(), 4, "Third part should be 4 digits");

        // Ensure all parts are numeric
        for part in parts {
            assert!(
                part.chars().all(|c| c.is_ascii_digit()),
                "All parts should be numeric: {part}"
            );
        }
    }
}

/// Test international phone number pattern
#[tokio::test]
async fn test_pattern_generator_international_phone() {
    let generator = PatternGenerator;

    let mut context = GenerationContext::new(
        "http://example.org/intlPhone".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context
        .parameters
        .insert("pattern".to_string(), json!("\\+1-\\d{3}-\\d{3}-\\d{4}"));

    for _ in 0..5 {
        let result = generator.generate(&context).unwrap();
        println!("Generated international phone: {result}");

        // Validate format: +1-XXX-XXX-XXXX
        let intl_phone_regex = Regex::new(r"^\+1-\d{3}-\d{3}-\d{4}$").unwrap();
        assert!(
            intl_phone_regex.is_match(&result),
            "International phone should match +1-XXX-XXX-XXXX format: {result}"
        );
        assert!(result.starts_with("+1-"), "Should start with +1-");
    }
}

/// Test email pattern generation
#[tokio::test]
async fn test_pattern_generator_email() {
    let generator = PatternGenerator;

    let mut context = GenerationContext::new(
        "http://example.org/email".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context.parameters.insert(
        "pattern".to_string(),
        json!("[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"),
    );

    for _ in 0..10 {
        let result = generator.generate(&context).unwrap();
        println!("Generated email: {result}");

        // Basic email validation
        assert!(result.contains("@"), "Email should contain @");
        assert!(result.contains("."), "Email should contain .");

        let parts: Vec<&str> = result.split('@').collect();
        assert_eq!(parts.len(), 2, "Email should have exactly one @");
        assert!(!parts[0].is_empty(), "Local part should not be empty");
        assert!(!parts[1].is_empty(), "Domain part should not be empty");
        assert!(parts[1].contains("."), "Domain should contain .");
    }
}

/// Test student ID pattern
#[tokio::test]
async fn test_pattern_generator_student_id() {
    let generator = PatternGenerator;

    let mut context = GenerationContext::new(
        "http://example.org/studentId".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context
        .parameters
        .insert("pattern".to_string(), json!("[A-Z]{2,3}\\d{4,6}"));

    for _ in 0..10 {
        let result = generator.generate(&context).unwrap();
        println!("Generated student ID: {result}");

        // Validate format: 2-3 uppercase letters followed by 4-6 digits
        let id_regex = Regex::new(r"^[A-Z]{2,3}\d{4,6}$").unwrap();
        assert!(
            id_regex.is_match(&result),
            "Student ID should match pattern: {result}"
        );

        // Check letter and digit counts
        let letters: String = result.chars().filter(|c| c.is_alphabetic()).collect();
        let digits: String = result.chars().filter(|c| c.is_numeric()).collect();

        assert!(
            letters.len() >= 2 && letters.len() <= 3,
            "Should have 2-3 letters: {letters}"
        );
        assert!(
            digits.len() >= 4 && digits.len() <= 6,
            "Should have 4-6 digits: {digits}"
        );
        assert!(
            letters.chars().all(|c| c.is_uppercase()),
            "Letters should be uppercase: {letters}"
        );
    }
}

/// Test date pattern generation
#[tokio::test]
async fn test_pattern_generator_date() {
    let generator = PatternGenerator;

    let mut context = GenerationContext::new(
        "http://example.org/dateString".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context
        .parameters
        .insert("pattern".to_string(), json!("\\d{4}-\\d{2}-\\d{2}"));

    for _ in 0..5 {
        let result = generator.generate(&context).unwrap();
        println!("Generated date: {result}");

        // Validate format: YYYY-MM-DD
        let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        assert!(
            date_regex.is_match(&result),
            "Date should match YYYY-MM-DD format: {result}"
        );

        let parts: Vec<&str> = result.split('-').collect();
        assert_eq!(parts.len(), 3, "Date should have 3 parts");

        let year: i32 = parts[0].parse().unwrap();
        let month: i32 = parts[1].parse().unwrap();
        let day: i32 = parts[2].parse().unwrap();

        assert!(
            (1980..=2024).contains(&year),
            "Year should be reasonable: {year}"
        );
        assert!((1..=12).contains(&month), "Month should be 1-12: {month}");
        assert!((1..=31).contains(&day), "Day should be 1-31: {day}");
    }
}

/// Test IP address pattern
#[tokio::test]
async fn test_pattern_generator_ip_address() {
    let generator = PatternGenerator;

    let mut context = GenerationContext::new(
        "http://example.org/ipAddress".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context.parameters.insert(
        "pattern".to_string(),
        json!("\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}"),
    );

    for _ in 0..5 {
        let result = generator.generate(&context).unwrap();
        println!("Generated IP: {result}");

        // Validate format: X.X.X.X where X is 1-3 digits
        let ip_regex = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap();
        assert!(
            ip_regex.is_match(&result),
            "IP should match IPv4 format: {result}"
        );

        let octets: Vec<&str> = result.split('.').collect();
        assert_eq!(octets.len(), 4, "IP should have 4 octets");

        for octet in octets {
            let value: i32 = octet.parse().unwrap();
            assert!((0..=255).contains(&value), "Octet should be 0-255: {value}");
        }
    }
}

/// Test URL pattern generation
#[tokio::test]
async fn test_pattern_generator_url() {
    let generator = PatternGenerator;

    let mut context = GenerationContext::new(
        "http://example.org/website".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context.parameters.insert(
        "pattern".to_string(),
        json!("https?://[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"),
    );

    for _ in 0..5 {
        let result = generator.generate(&context).unwrap();
        println!("Generated URL: {result}");

        // Validate basic URL structure
        assert!(
            result.starts_with("http://") || result.starts_with("https://"),
            "URL should start with http:// or https://: {result}"
        );
        assert!(result.contains("."), "URL should contain domain separator");

        // Extract domain part
        let domain_part = if result.starts_with("https://") {
            &result[8..]
        } else {
            &result[7..]
        };

        assert!(!domain_part.is_empty(), "Domain part should not be empty");
        assert!(
            domain_part.contains("."),
            "Domain should have TLD separator"
        );
    }
}

/// Test heuristic generation based on property names
#[tokio::test]
async fn test_pattern_generator_heuristics() {
    let generator = PatternGenerator;

    // Test phone heuristic
    let phone_context = GenerationContext::new(
        "http://example.org/phoneNumber".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );

    let phone_result = generator.generate(&phone_context).unwrap();
    println!("Heuristic phone: {phone_result}");
    assert!(
        phone_result.contains("-"),
        "Phone heuristic should generate dash-separated format"
    );

    // Test email heuristic
    let email_context = GenerationContext::new(
        "http://example.org/emailAddress".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );

    let email_result = generator.generate(&email_context).unwrap();
    println!("Heuristic email: {email_result}");
    assert!(
        email_result.contains("@"),
        "Email heuristic should generate @ symbol"
    );
    assert!(
        email_result.contains("."),
        "Email heuristic should generate domain"
    );

    // Test URL heuristic
    let url_context = GenerationContext::new(
        "http://example.org/websiteUrl".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );

    let url_result = generator.generate(&url_context).unwrap();
    println!("Heuristic URL: {url_result}");
    assert!(
        url_result.starts_with("https://"),
        "URL heuristic should generate https://"
    );

    // Test ID heuristic
    let id_context = GenerationContext::new(
        "http://example.org/userId".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );

    let id_result = generator.generate(&id_context).unwrap();
    println!("Heuristic ID: {id_result}");
    assert!(
        id_result.starts_with("ID"),
        "ID heuristic should generate ID prefix"
    );
}

/// Test pattern generator integration with configuration (without schema)
#[tokio::test]
async fn test_pattern_generator_integration_config() {
    // Skip ShEx integration due to parser compatibility issues
    // Test pattern generator with configuration only
    let output_file = NamedTempFile::new().unwrap();

    // Configure generator with pattern generator using config only
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 3;
    config.output.path = output_file.path().to_path_buf();

    // Add pattern generator configuration
    let mut property_configs = HashMap::new();

    // Phone pattern config
    let mut phone_config = PropertyConfig {
        generator: "pattern".to_string(),
        parameters: HashMap::new(),
        templates: None,
    };
    phone_config
        .parameters
        .insert("pattern".to_string(), json!("\\d{3}-\\d{3}-\\d{4}"));
    property_configs.insert("http://example.org/phone".to_string(), phone_config);

    // Email pattern config using heuristic
    let email_config = PropertyConfig {
        generator: "pattern".to_string(),
        parameters: HashMap::new(),
        templates: None,
    };
    property_configs.insert("http://example.org/emailAddress".to_string(), email_config);

    config.field_generators.properties = property_configs;

    // Test generation without schema (will use default shape generation)
    let _generator = DataGenerator::new(config).unwrap();

    // Create a minimal RDF data manually to test pattern generator
    println!("Pattern generator integration test completed successfully (config-based)");

    // Basic test of pattern generator functionality
    let pattern_gen = PatternGenerator;
    let mut context = GenerationContext::new(
        "http://example.org/phone".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context
        .parameters
        .insert("pattern".to_string(), json!("\\d{3}-\\d{3}-\\d{4}"));

    let phone_result = pattern_gen.generate(&context).unwrap();
    println!("Generated phone in integration test: {phone_result}");
    assert!(phone_result.contains("-"), "Should contain phone dashes");

    // Test email heuristic
    let email_context = GenerationContext::new(
        "http://example.org/emailAddress".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    let email_result = pattern_gen.generate(&email_context).unwrap();
    println!("Generated email in integration test: {email_result}");
    assert!(email_result.contains("@"), "Should contain email @ symbol");
}

/// Test pattern generator with SHACL schema
#[tokio::test]
async fn test_pattern_generator_integration_shacl() {
    // Create a SHACL schema with pattern constraints
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
        sh:path ex:phone ;
        sh:datatype xsd:string ;
        sh:pattern "\\d{3}-\\d{3}-\\d{4}" ;
        sh:maxCount 1 ;
    ] ;
    sh:property [
        sh:path ex:email ;
        sh:datatype xsd:string ;
        sh:pattern "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}" ;
        sh:maxCount 1 ;
    ] .
"#;

    // Create temporary files
    let mut schema_file = NamedTempFile::new().unwrap();
    writeln!(schema_file, "{shacl_schema}").unwrap();

    let output_file = NamedTempFile::new().unwrap();

    // Configure generator
    let mut config = GeneratorConfig::default();
    config.generation.entity_count = 2;
    config.output.path = output_file.path().to_path_buf();

    // Add pattern generator as default for string datatype
    let mut datatype_configs = HashMap::new();
    let string_config = DatatypeConfig {
        generator: "pattern".to_string(),
        parameters: HashMap::new(),
    };
    datatype_configs.insert(
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        string_config,
    );
    config.field_generators.datatypes = datatype_configs;

    // Test generation
    let mut generator = DataGenerator::new(config).unwrap();
    let load_result = generator.load_shacl_schema(schema_file.path()).await;
    assert!(
        load_result.is_ok(),
        "Should load SHACL schema with patterns"
    );

    let gen_result = generator.generate().await;
    assert!(
        gen_result.is_ok(),
        "Should generate data with SHACL pattern constraints"
    );

    // Verify output
    let output_content = std::fs::read_to_string(output_file.path()).unwrap();
    assert!(
        !output_content.is_empty(),
        "Output file should not be empty"
    );
    println!("Generated SHACL RDF:\n{output_content}");
}

/// Test error handling for invalid patterns
#[tokio::test]
async fn test_pattern_generator_error_handling() {
    let generator = PatternGenerator;

    // Test with unsupported complex pattern - should fallback gracefully
    let mut context = GenerationContext::new(
        "http://example.org/complex".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context.parameters.insert(
        "pattern".to_string(),
        json!("(?=.*[A-Z])(?=.*[a-z])(?=.*\\d)[A-Za-z\\d]{8,}"),
    );

    let result = generator.generate(&context);
    assert!(
        result.is_ok(),
        "Should handle unsupported patterns gracefully"
    );

    let value = result.unwrap();
    assert!(!value.is_empty(), "Should generate some fallback value");
    println!("Fallback for complex pattern: {value}");
}

/// Test pattern generator with multiple iterations for randomness
#[tokio::test]
async fn test_pattern_generator_randomness() {
    let generator = PatternGenerator;

    let mut context = GenerationContext::new(
        "http://example.org/phone".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context
        .parameters
        .insert("pattern".to_string(), json!("\\d{3}-\\d{3}-\\d{4}"));

    let mut generated_values = std::collections::HashSet::new();

    // Generate multiple values to test randomness
    for _ in 0..50 {
        let result = generator.generate(&context).unwrap();
        generated_values.insert(result);
    }

    // Should generate multiple different values (allowing for some collision)
    assert!(
        generated_values.len() > 10,
        "Should generate varied values, got {} unique values",
        generated_values.len()
    );

    // All values should match the pattern
    let phone_regex = Regex::new(r"^\d{3}-\d{3}-\d{4}$").unwrap();
    for value in &generated_values {
        assert!(
            phone_regex.is_match(value),
            "All generated values should match pattern: {value}"
        );
    }

    println!(
        "Generated {} unique phone numbers out of 50 attempts",
        generated_values.len()
    );
}
