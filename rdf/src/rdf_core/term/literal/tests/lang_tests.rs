use crate::rdf_core::term::literal::Lang;
use std::collections::HashSet;

// ----------------------------------
// Constructor tests
// ----------------------------------

#[test]
fn test_new_valid_simple_language() {
    let lang = Lang::new("en").unwrap();
    assert_eq!(lang.as_str(), "en");
}

#[test]
fn test_new_valid_language_with_region() {
    let lang = Lang::new("en-US").unwrap();
    assert_eq!(lang.as_str(), "en-US");
}

#[test]
fn test_new_valid_language_with_script() {
    let lang = Lang::new("zh-Hans").unwrap();
    assert_eq!(lang.as_str(), "zh-Hans");
}

#[test]
fn test_new_valid_complex_tag() {
    let lang = Lang::new("zh-Hans-CN").unwrap();
    assert_eq!(lang.as_str(), "zh-Hans-CN");
}

#[test]
fn test_new_invalid_empty_string() {
    let result = Lang::new("");
    assert!(result.is_err());
}

#[test]
fn test_new_invalid_malformed() {
    let result = Lang::new("");
    assert!(result.is_err());
}

#[test]
fn test_new_invalid_special_characters() {
    assert!(Lang::new("en_US").is_err()); // Underscore not allowed
    assert!(Lang::new("en@US").is_err()); // @ not allowed
}

#[test]
fn test_new_accepts_string() {
    let s = String::from("fr");
    let lang = Lang::new(s).unwrap();
    assert_eq!(lang.as_str(), "fr");
}

#[test]
fn test_new_accepts_str_slice() {
    let lang = Lang::new("de").unwrap();
    assert_eq!(lang.as_str(), "de");
}

// ----------------------------------
// Normalization tests
// ----------------------------------

#[test]
fn test_normalization_lowercase_language() {
    let lang = Lang::new("EN").unwrap();
    assert_eq!(lang.as_str(), "en"); // Should be lowercased
}

#[test]
fn test_normalization_uppercase_region() {
    let lang = Lang::new("en-us").unwrap();
    assert_eq!(lang.as_str(), "en-US"); // Region should be uppercased
}

#[test]
fn test_normalization_titlecase_script() {
    let lang = Lang::new("zh-hans").unwrap();
    assert_eq!(lang.as_str(), "zh-Hans"); // Script should be title-cased
}

#[test]
fn test_normalization_complex_tag() {
    let lang = Lang::new("ZH-HANS-CN").unwrap();
    assert_eq!(lang.as_str(), "zh-Hans-CN");
}

#[test]
fn test_normalization_mixed_case() {
    let lang = Lang::new("Es-Latn-MX").unwrap();  
    // Language lowercase, script titlecase, region uppercase
    assert_eq!(lang.as_str(), "es-Latn-MX");
}

// ----------------------------------
// Display trait tests
// ----------------------------------

#[test]
fn test_display_simple() {
    let lang = Lang::new("en").unwrap();
    assert_eq!(format!("{}", lang), "en");
}

#[test]
fn test_display_with_region() {
    let lang = Lang::new("en-US").unwrap();
    assert_eq!(format!("{}", lang), "en-US");
}

#[test]
fn test_display_complex() {
    let lang = Lang::new("zh-Hans-CN").unwrap();
    assert_eq!(format!("{}", lang), "zh-Hans-CN");
}

#[test]
fn test_display_normalized() {
    let lang = Lang::new("EN-us").unwrap();
    assert_eq!(format!("{}", lang), "en-US");
}

#[test]
fn test_to_string() {
    let lang = Lang::new("fr-FR").unwrap();
    let s = lang.to_string();
    assert_eq!(s, "fr-FR");
}

// ----------------------------------
// as_str method tests
// ----------------------------------

#[test]
fn test_as_str_returns_correct_value() {
    let lang = Lang::new("ja").unwrap();
    assert_eq!(lang.as_str(), "ja");
}

#[test]
fn test_as_str_lifetime() {
    let lang = Lang::new("ko").unwrap();
    let s1 = lang.as_str();
    let s2 = lang.as_str();
    assert_eq!(s1, s2);
}

// ----------------------------------
// Equality tests
// ----------------------------------

#[test]
fn test_equality_same_tag() {
    let lang1 = Lang::new("en").unwrap();
    let lang2 = Lang::new("en").unwrap();
    assert_eq!(lang1, lang2);
}

#[test]
fn test_equality_different_tags() {
    let lang1 = Lang::new("en").unwrap();
    let lang2 = Lang::new("fr").unwrap();
    assert_ne!(lang1, lang2);
}

#[test]
fn test_equality_normalized_vs_unnormalized() {
    let lang1 = Lang::new("en-US").unwrap();
    let lang2 = Lang::new("en-us").unwrap();
    // Both should normalize to the same value
    assert_eq!(lang1, lang2);
}

#[test]
fn test_equality_complex_tags() {
    let lang1 = Lang::new("zh-Hans-CN").unwrap();
    let lang2 = Lang::new("zh-Hans-CN").unwrap();
    assert_eq!(lang1, lang2);
}

// ----------------------------------
// Ordering tests
// ----------------------------------

#[test]
fn test_ordering_alphabetical() {
    let en = Lang::new("en").unwrap();
    let fr = Lang::new("fr").unwrap();
    assert!(en < fr);
    assert!(fr > en);
}

#[test]
fn test_ordering_with_regions() {
    let en_gb = Lang::new("en-GB").unwrap();
    let en_us = Lang::new("en-US").unwrap();
    assert!(en_gb < en_us); // GB comes before US alphabetically
}

#[test]
fn test_ordering_simple_vs_complex() {
    let en = Lang::new("en").unwrap();
    let en_us = Lang::new("en-US").unwrap();
    assert!(en < en_us);
}

#[test]
fn test_partial_cmp() {
    let lang1 = Lang::new("de").unwrap();
    let lang2 = Lang::new("es").unwrap();
    assert_eq!(lang1.partial_cmp(&lang2), Some(std::cmp::Ordering::Less));
}

#[test]
fn test_cmp() {
    let lang1 = Lang::new("it").unwrap();
    let lang2 = Lang::new("ja").unwrap();
    assert_eq!(lang1.cmp(&lang2), std::cmp::Ordering::Less);
}

// ----------------------------------
// Hash tests
// ----------------------------------

#[test]
fn test_hash_consistency() {
    let lang1 = Lang::new("en").unwrap();
    let lang2 = Lang::new("en").unwrap();

    let mut set = HashSet::new();
    set.insert(lang1);
    assert!(set.contains(&lang2));
}

#[test]
fn test_hash_normalized_tags() {
    let lang1 = Lang::new("en-US").unwrap();
    let lang2 = Lang::new("en-us").unwrap();

    let mut set = HashSet::new();
    set.insert(lang1);
    // Normalized versions should be considered equal
    assert!(set.contains(&lang2));
}

#[test]
fn test_hash_different_tags() {
    let lang1 = Lang::new("en").unwrap();
    let lang2 = Lang::new("fr").unwrap();

    let mut set = HashSet::new();
    set.insert(lang1);
    assert!(!set.contains(&lang2));
}

// ----------------------------------
// Clone tests
// ----------------------------------

#[test]
fn test_clone() {
    let lang1 = Lang::new("pt-BR").unwrap();
    let lang2 = lang1.clone();

    assert_eq!(lang1, lang2);
    assert_eq!(lang1.as_str(), lang2.as_str());
}

#[test]
fn test_clone_independence() {
    let lang1 = Lang::new("ru").unwrap();
    let lang2 = lang1.clone();

    drop(lang1);
    // lang2 should still be valid
    assert_eq!(lang2.as_str(), "ru");
}

// ----------------------------------
// Serialization tests
// ----------------------------------

#[test]
fn test_serialize_simple() {
    let lang = Lang::new("en").unwrap();
    let json = serde_json::to_string(&lang).unwrap();
    assert_eq!(json, "\"en\"");
}

#[test]
fn test_serialize_complex() {
    let lang = Lang::new("zh-Hans-CN").unwrap();
    let json = serde_json::to_string(&lang).unwrap();
    assert_eq!(json, "\"zh-Hans-CN\"");
}

#[test]
fn test_deserialize_simple() {
    let json = "\"fr\"";
    let lang: Lang = serde_json::from_str(json).unwrap();
    assert_eq!(lang.as_str(), "fr");
}

#[test]
fn test_deserialize_complex() {
    let json = "\"es-ES\"";
    let lang: Lang = serde_json::from_str(json).unwrap();
    assert_eq!(lang.as_str(), "es-ES");
}

#[test]
fn test_deserialize_invalid() {
    let json = "\"\""; 
    let result: Result<Lang, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_roundtrip_serialization() {
    let original = Lang::new("de-DE").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Lang = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_serialize_normalized() {
    let lang = Lang::new("EN-us").unwrap(); // Unnormalized input
    let json = serde_json::to_string(&lang).unwrap();
    assert_eq!(json, "\"en-US\""); // Should serialize normalized
}

#[test]
fn test_transparent_serialization_structure() {
    // Test that it serializes as a plain string, not {"lang":"en"}
    let lang = Lang::new("ja").unwrap();
    let json = serde_json::to_string(&lang).unwrap();
    assert!(!json.contains("lang"));
    assert!(!json.contains("{"));
}

// ----------------------------------
// Error tests
// ----------------------------------

#[test]
fn test_error_display() {
    let result = Lang::new("invalid!!!");
    assert!(result.is_err());

    let err = result.unwrap_err();
    let error_msg = format!("{}", err);
    assert!(error_msg.contains("Invalid language tag"));
}

#[test]
fn test_error_debug() {
    let result = Lang::new("invalid!!!");
    let err = result.unwrap_err();
    let debug_msg = format!("{:?}", err);
    assert!(!debug_msg.is_empty());
}

// ----------------------------------
// Various valid language tag formats
// ----------------------------------

#[test]
fn test_various_valid_tags() {
    let valid_tags = vec![
        "en", "en-GB", "en-US", "es-419", // Spanish for Latin America
        "zh-Hans", "zh-Hant", "sr-Cyrl", "sr-Latn", "pt-BR", "fr-CA", "de-CH", "nb-NO",
    ];

    for tag in valid_tags {
        let result = Lang::new(tag);
        assert!(result.is_ok(), "Tag '{}' should be valid", tag);
    }
}

#[test]
fn test_various_invalid_tags() {
    let invalid_tags = vec![
        "",
        " ",
        "en_US",      // Underscore not allowed
        "en-",        // Trailing hyphen
        "-en",        // Leading hyphen
        "123",        // Numbers only
        "en--US",     // Double hyphen
        "toolongtag", // Invalid format
    ];

    for tag in invalid_tags {
        let result = Lang::new(tag);
        assert!(result.is_err(), "Tag '{}' should be invalid", tag);
    }
}

// ----------------------------------
// Edge cases
// ----------------------------------

#[test]
fn test_debug_format() {
    let lang = Lang::new("ar").unwrap();
    let debug_str = format!("{:?}", lang);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_multiple_operations() {
    let lang = Lang::new("it-IT").unwrap();

    // Multiple as_str calls
    assert_eq!(lang.as_str(), "it-IT");
    assert_eq!(lang.as_str(), "it-IT");

    // Display
    assert_eq!(lang.to_string(), "it-IT");

    // Clone and compare
    let lang2 = lang.clone();
    assert_eq!(lang, lang2);
}

#[test]
fn test_case_insensitive_comparison_via_normalization() {
    let lang1 = Lang::new("EN-gb").unwrap();
    let lang2 = Lang::new("en-GB").unwrap();
    let lang3 = Lang::new("en-Gb").unwrap();

    // All should normalize to the same value
    assert_eq!(lang1, lang2);
    assert_eq!(lang2, lang3);
    assert_eq!(lang1, lang3);
}
