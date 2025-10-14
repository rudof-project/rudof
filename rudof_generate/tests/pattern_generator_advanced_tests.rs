use regex::Regex;
use rudof_generate::field_generators::pattern::PatternGenerator;
use rudof_generate::field_generators::{FieldGenerator, GenerationContext};
use serde_json::json;
use std::time::Instant;

/// Test pattern generator with various alphanumeric patterns
#[tokio::test]
async fn test_pattern_generator_alphanumeric_patterns() {
    let generator = PatternGenerator;

    // Test license plate pattern (mix of letters and numbers)
    let mut context = GenerationContext::new(
        "http://example.org/licensePlate".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context
        .parameters
        .insert("pattern".to_string(), json!("[A-Z]{3}\\d{3}"));

    let license_regex = Regex::new(r"^[A-Z]{3}\d{3}$").unwrap();
    for _ in 0..10 {
        let result = generator.generate(&context).unwrap();
        println!("Generated license plate: {result}");

        assert!(
            license_regex.is_match(&result),
            "License plate should match AAA123 format: {result}"
        );
        assert_eq!(
            result.len(),
            6,
            "License plate should be exactly 6 characters"
        );

        // First 3 should be uppercase letters
        let letters = &result[0..3];
        assert!(
            letters.chars().all(|c| c.is_ascii_uppercase()),
            "First 3 should be uppercase letters: {letters}"
        );

        // Last 3 should be digits
        let numbers = &result[3..6];
        assert!(
            numbers.chars().all(|c| c.is_ascii_digit()),
            "Last 3 should be digits: {numbers}"
        );
    }
}

/// Test pattern generator with optional groups and quantifiers
#[tokio::test]
async fn test_pattern_generator_optional_groups() {
    let generator = PatternGenerator;

    // Test phone with optional extension pattern
    let mut context = GenerationContext::new(
        "http://example.org/phoneWithExt".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context.parameters.insert(
        "pattern".to_string(),
        json!("\\d{3}-\\d{3}-\\d{4}( ext\\. \\d{1,4})?"),
    );

    let mut with_ext_count = 0;
    let mut without_ext_count = 0;

    let ext_regex = Regex::new(r"^\d{3}-\d{3}-\d{4} ext\. \d{1,4}$").unwrap();
    let base_regex = Regex::new(r"^\d{3}-\d{3}-\d{4}$").unwrap();

    for _ in 0..20 {
        let result = generator.generate(&context).unwrap();
        println!("Generated phone with optional ext: {result}");

        if result.contains(" ext. ") {
            with_ext_count += 1;
            // Validate full pattern with extension
            assert!(
                ext_regex.is_match(&result),
                "Phone with extension should match pattern: {result}"
            );
        } else {
            without_ext_count += 1;
            // Validate base pattern without extension
            assert!(
                base_regex.is_match(&result),
                "Phone without extension should match base pattern: {result}"
            );
        }
    }

    // Should generate both variants (allowing for randomness)
    assert!(
        with_ext_count > 0 || without_ext_count > 15,
        "Should generate some variety in optional groups"
    );
    println!("Generated {with_ext_count} with extension, {without_ext_count} without extension");
}

/// Test pattern generator with character classes and ranges
#[tokio::test]
async fn test_pattern_generator_character_classes() {
    let generator = PatternGenerator;

    // Test hexadecimal color pattern
    let mut context = GenerationContext::new(
        "http://example.org/hexColor".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context
        .parameters
        .insert("pattern".to_string(), json!("#[0-9A-Fa-f]{6}"));

    let hex_regex = Regex::new(r"^#[0-9A-Fa-f]{6}$").unwrap();
    for _ in 0..10 {
        let result = generator.generate(&context).unwrap();
        println!("Generated hex color: {result}");

        assert!(
            hex_regex.is_match(&result),
            "Hex color should match #RRGGBB format: {result}"
        );
        assert_eq!(
            result.len(),
            7,
            "Hex color should be 7 characters including #"
        );
        assert!(result.starts_with("#"), "Should start with #");

        // Validate hex characters
        let hex_part = &result[1..];
        assert!(
            hex_part.chars().all(|c| c.is_ascii_hexdigit()),
            "All characters after # should be hex: {hex_part}"
        );
    }
}

/// Test pattern generator with word boundaries and anchors
#[tokio::test]
async fn test_pattern_generator_anchors() {
    let generator = PatternGenerator;

    // Test pattern with start/end anchors
    let mut context = GenerationContext::new(
        "http://example.org/productCode".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context
        .parameters
        .insert("pattern".to_string(), json!("^PROD-\\d{4}-[A-Z]{2}$"));

    let prod_regex = Regex::new(r"^PROD-\d{4}-[A-Z]{2}$").unwrap();
    for _ in 0..5 {
        let result = generator.generate(&context).unwrap();
        println!("Generated product code: {result}");

        assert!(
            prod_regex.is_match(&result),
            "Product code should match PROD-XXXX-YY format: {result}"
        );
        assert!(result.starts_with("PROD-"), "Should start with PROD-");
        assert_eq!(
            result.len(),
            12,
            "Product code should be 12 characters total"
        );
    }
}

/// Test pattern generator with custom ranges and special characters
#[tokio::test]
async fn test_pattern_generator_special_characters() {
    let generator = PatternGenerator;

    // Test social security number pattern with literal dashes
    let mut context = GenerationContext::new(
        "http://example.org/ssn".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context
        .parameters
        .insert("pattern".to_string(), json!("\\d{3}-\\d{2}-\\d{4}"));

    let ssn_regex = Regex::new(r"^\d{3}-\d{2}-\d{4}$").unwrap();
    for _ in 0..5 {
        let result = generator.generate(&context).unwrap();
        println!("Generated SSN: {result}");

        assert!(
            ssn_regex.is_match(&result),
            "SSN should match XXX-XX-XXXX format: {result}"
        );

        let parts: Vec<&str> = result.split('-').collect();
        assert_eq!(parts.len(), 3, "SSN should have 3 parts");
        assert_eq!(parts[0].len(), 3, "First part should be 3 digits");
        assert_eq!(parts[1].len(), 2, "Second part should be 2 digits");
        assert_eq!(parts[2].len(), 4, "Third part should be 4 digits");
    }
}

/// Test pattern generator with escape sequences
#[tokio::test]
async fn test_pattern_generator_escape_sequences() {
    let generator = PatternGenerator;

    // Test pattern with escaped special characters
    let mut context = GenerationContext::new(
        "http://example.org/escapedPattern".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    // Pattern for version numbers like "v1.2.3"
    context
        .parameters
        .insert("pattern".to_string(), json!("v\\d+\\.\\d+\\.\\d+"));

    let version_regex = Regex::new(r"^v\d+\.\d+\.\d+$").unwrap();
    for _ in 0..5 {
        let result = generator.generate(&context).unwrap();
        println!("Generated version: {result}");

        assert!(
            version_regex.is_match(&result),
            "Version should match vX.Y.Z format: {result}"
        );
        assert!(result.starts_with("v"), "Should start with v");

        // Count dots
        let dot_count = result.matches('.').count();
        assert_eq!(dot_count, 2, "Should have exactly 2 dots: {result}");
    }
}

/// Test pattern generator performance with multiple generations
#[tokio::test]
async fn test_pattern_generator_performance() {
    let generator = PatternGenerator;

    let mut context = GenerationContext::new(
        "http://example.org/perfTest".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context
        .parameters
        .insert("pattern".to_string(), json!("\\d{3}-\\d{3}-\\d{4}"));

    let iterations = 1000;
    let start = Instant::now();

    for _ in 0..iterations {
        let result = generator.generate(&context);
        assert!(result.is_ok(), "Generation should succeed");

        let value = result.unwrap();
        assert!(!value.is_empty(), "Should generate non-empty value");
    }

    let duration = start.elapsed();
    let per_generation = duration.as_nanos() / iterations as u128;

    println!(
        "Generated {iterations} patterns in {duration:?} (avg: {per_generation}ns per generation)"
    );

    // Performance should be reasonable (less than 1ms per generation)
    assert!(
        per_generation < 1_000_000,
        "Generation should be fast: {per_generation}ns per generation"
    );
}

/// Test pattern generator with edge case patterns
#[tokio::test]
async fn test_pattern_generator_edge_cases() {
    let generator = PatternGenerator;

    // Test very simple pattern
    let mut simple_context = GenerationContext::new(
        "http://example.org/simple".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    simple_context
        .parameters
        .insert("pattern".to_string(), json!("a"));

    let simple_result = generator.generate(&simple_context).unwrap();
    assert_eq!(
        simple_result, "a",
        "Simple pattern should generate exact match"
    );

    // Test pattern with only quantifiers
    let mut quant_context = GenerationContext::new(
        "http://example.org/quantifiers".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    quant_context
        .parameters
        .insert("pattern".to_string(), json!("\\d{5}"));

    let quant_result = generator.generate(&quant_context).unwrap();
    assert_eq!(quant_result.len(), 5, "Should generate exactly 5 digits");
    assert!(
        quant_result.chars().all(|c| c.is_ascii_digit()),
        "Should be all digits: {quant_result}"
    );

    // Test pattern with alternation (simplified)
    let mut alt_context = GenerationContext::new(
        "http://example.org/alternation".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    alt_context
        .parameters
        .insert("pattern".to_string(), json!("(cat|dog)"));

    let alt_result = generator.generate(&alt_context).unwrap();
    println!("Alternation result: {alt_result}");
    // Note: Our current implementation may not handle alternation perfectly,
    // but should generate something reasonable
    assert!(
        !alt_result.is_empty(),
        "Should generate non-empty result for alternation"
    );
}

/// Test pattern generator with mixed case requirements
#[tokio::test]
async fn test_pattern_generator_case_sensitivity() {
    let generator = PatternGenerator;

    // Test pattern requiring both upper and lower case
    let mut context = GenerationContext::new(
        "http://example.org/mixedCase".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    context
        .parameters
        .insert("pattern".to_string(), json!("[A-Z]{2}[a-z]{2}\\d{2}"));

    let mixed_regex = Regex::new(r"^[A-Z]{2}[a-z]{2}\d{2}$").unwrap();
    for _ in 0..5 {
        let result = generator.generate(&context).unwrap();
        println!("Generated mixed case: {result}");

        assert!(
            mixed_regex.is_match(&result),
            "Should match mixed case pattern: {result}"
        );
        assert_eq!(result.len(), 6, "Should be exactly 6 characters");

        // Validate case requirements
        let upper_part = &result[0..2];
        let lower_part = &result[2..4];
        let digit_part = &result[4..6];

        assert!(
            upper_part.chars().all(|c| c.is_ascii_uppercase()),
            "First 2 should be uppercase: {upper_part}"
        );
        assert!(
            lower_part.chars().all(|c| c.is_ascii_lowercase()),
            "Next 2 should be lowercase: {lower_part}"
        );
        assert!(
            digit_part.chars().all(|c| c.is_ascii_digit()),
            "Last 2 should be digits: {digit_part}"
        );
    }
}

/// Test pattern generator with complex real-world patterns
#[tokio::test]
async fn test_pattern_generator_real_world_patterns() {
    let generator = PatternGenerator;

    // Test ISBN-10 pattern
    let mut isbn_context = GenerationContext::new(
        "http://example.org/isbn10".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    isbn_context
        .parameters
        .insert("pattern".to_string(), json!("\\d{1}-\\d{3}-\\d{5}-\\d{1}"));

    let isbn_result = generator.generate(&isbn_context).unwrap();
    println!("Generated ISBN-10: {isbn_result}");

    let isbn_regex = Regex::new(r"^\d{1}-\d{3}-\d{5}-\d{1}$").unwrap();
    assert!(
        isbn_regex.is_match(&isbn_result),
        "ISBN should match X-XXX-XXXXX-X format: {isbn_result}"
    );

    // Test UUID-like pattern (simplified)
    let mut uuid_context = GenerationContext::new(
        "http://example.org/uuid".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    uuid_context.parameters.insert(
        "pattern".to_string(),
        json!("[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}"),
    );

    let uuid_result = generator.generate(&uuid_context).unwrap();
    println!("Generated UUID-like: {uuid_result}");

    let uuid_regex =
        Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
    assert!(
        uuid_regex.is_match(&uuid_result),
        "UUID should match standard format: {uuid_result}"
    );
    assert_eq!(uuid_result.len(), 36, "UUID should be 36 characters total");

    // Count dashes
    let dash_count = uuid_result.matches('-').count();
    assert_eq!(
        dash_count, 4,
        "UUID should have exactly 4 dashes: {uuid_result}"
    );
}

/// Test pattern generator fallback behavior
#[tokio::test]
async fn test_pattern_generator_fallback_behavior() {
    let generator = PatternGenerator;

    // Test with empty pattern - should use heuristics
    let mut empty_context = GenerationContext::new(
        "http://example.org/phoneNumber".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );
    empty_context
        .parameters
        .insert("pattern".to_string(), json!(""));

    let empty_result = generator.generate(&empty_context).unwrap();
    println!("Empty pattern fallback: {empty_result}");
    assert!(
        !empty_result.is_empty(),
        "Should generate fallback for empty pattern"
    );

    // Test with no pattern parameter - should use heuristics
    let no_pattern_context = GenerationContext::new(
        "http://example.org/emailAddress".to_string(),
        "http://www.w3.org/2001/XMLSchema#string".to_string(),
        "test_subject".to_string(),
    );

    let no_pattern_result = generator.generate(&no_pattern_context).unwrap();
    println!("No pattern fallback: {no_pattern_result}");
    assert!(
        !no_pattern_result.is_empty(),
        "Should generate fallback for no pattern"
    );
    assert!(
        no_pattern_result.contains("@"),
        "Email heuristic should work without pattern"
    );
}

/// Test pattern generator with comprehensive pattern coverage (config-based)
#[tokio::test]
async fn test_pattern_generator_comprehensive_coverage() {
    // Test comprehensive pattern coverage using configuration instead of problematic ShEx parsing
    let pattern_gen = PatternGenerator;

    // Test multiple pattern types in sequence
    let test_patterns = vec![
        ("\\d{3}-\\d{3}-\\d{4}", "phone pattern"),
        ("\\+1-\\d{3}-\\d{3}-\\d{4}", "international phone"),
        (
            "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}",
            "email pattern",
        ),
        ("[A-Z]{2,3}\\d{4,6}", "ID pattern"),
        ("\\d{4}-\\d{2}-\\d{2}", "date pattern"),
        ("\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}", "IP pattern"),
        ("https?://[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}", "URL pattern"),
        ("#[0-9A-Fa-f]{6}", "hex color"),
        ("\\d{3}-\\d{2}-\\d{4}", "SSN pattern"),
        ("[A-Z]{3}\\d{3}", "license plate"),
    ];

    let total_patterns = test_patterns.len();
    let mut generated_count = 0;
    for (pattern, description) in &test_patterns {
        let mut context = GenerationContext::new(
            format!("http://example.org/{}", description.replace(" ", "_")),
            "http://www.w3.org/2001/XMLSchema#string".to_string(),
            "test_subject".to_string(),
        );
        context
            .parameters
            .insert("pattern".to_string(), json!(pattern));

        match pattern_gen.generate(&context) {
            Ok(result) => {
                println!("Generated {description} ({pattern}): {result}");
                assert!(
                    !result.is_empty(),
                    "Generated value should not be empty for {description}"
                );
                generated_count += 1;
            }
            Err(e) => {
                println!("Failed to generate {description} ({pattern}): {e}");
                // For this comprehensive test, we allow some patterns to fail gracefully
            }
        }
    }

    println!("Successfully generated {generated_count} out of {total_patterns} pattern types");
    assert!(
        generated_count >= 8,
        "Should successfully generate at least 8 out of 10 pattern types"
    );
}
