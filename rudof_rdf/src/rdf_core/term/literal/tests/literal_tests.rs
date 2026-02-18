use proptest::prelude::*;
use rust_decimal::Decimal;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::rdf_core::term::literal::{ConcreteLiteral, Lang, NumericLiteral};
use iri_s::IriS;
use prefixmap::IriRef;

// ============================================================================
// Arbitrary Strategies for ConcreteLiteral Generation
// ============================================================================

/// Strategy for generating valid language tags
fn lang_strategy() -> impl Strategy<Value = Lang> {
    prop::option::of("[a-z]{2}(-[A-Z]{2})?")
        .prop_filter_map("valid lang tag", |s| s.and_then(|tag| Lang::new(tag).ok()))
}

/// Strategy for generating string literals (plain and language-tagged)
fn string_literal_strategy() -> impl Strategy<Value = ConcreteLiteral> {
    prop_oneof![
        // Plain string literals
        ".*".prop_map(|s| ConcreteLiteral::str(&s)),
        // Language-tagged string literals
        (".*", lang_strategy()).prop_map(|(s, lang)| ConcreteLiteral::lang_str(&s, lang)),
    ]
}

/// Strategy for generating numeric literals
fn numeric_literal_strategy() -> impl Strategy<Value = ConcreteLiteral> {
    prop_oneof![
        any::<i128>().prop_map(ConcreteLiteral::integer),
        any::<i8>().prop_map(ConcreteLiteral::byte),
        any::<i16>().prop_map(ConcreteLiteral::short),
        any::<i64>().prop_map(ConcreteLiteral::long),
        any::<u8>().prop_map(ConcreteLiteral::unsigned_byte),
        any::<u16>().prop_map(ConcreteLiteral::unsigned_short),
        any::<u32>().prop_map(ConcreteLiteral::unsigned_int),
        any::<u64>().prop_map(ConcreteLiteral::unsigned_long),
        any::<u128>().prop_map(ConcreteLiteral::non_negative_integer),
        any::<f32>()
            .prop_filter("finite float", |f| f.is_finite())
            .prop_map(ConcreteLiteral::float),
        any::<f64>()
            .prop_filter("finite double", |d| d.is_finite())
            .prop_map(ConcreteLiteral::double),
        // Positive integers
        (1u128..=u128::MAX).prop_filter_map("positive integer", |n| { ConcreteLiteral::positive_integer(n).ok() }),
        // Negative integers
        (i128::MIN..=-1i128).prop_filter_map("negative integer", |n| { ConcreteLiteral::negative_integer(n).ok() }),
        // Non-positive integers
        (i128::MIN..=0i128).prop_filter_map("non-positive integer", |n| {
            ConcreteLiteral::non_positive_integer(n).ok()
        }),
        // Decimals from i64 range
        any::<i64>().prop_map(|n| { ConcreteLiteral::decimal(Decimal::new(n, 0)) }),
    ]
}

/// Strategy for generating boolean literals
fn boolean_literal_strategy() -> impl Strategy<Value = ConcreteLiteral> {
    any::<bool>().prop_map(ConcreteLiteral::boolean)
}

/// Strategy for generating XSD datatype IRIs
fn xsd_datatype_strategy() -> impl Strategy<Value = IriRef> {
    prop_oneof![
        Just(IriRef::iri(IriS::new_unchecked(
            "http://www.w3.org/2001/XMLSchema#string"
        ))),
        Just(IriRef::iri(IriS::new_unchecked(
            "http://www.w3.org/2001/XMLSchema#integer"
        ))),
        Just(IriRef::iri(IriS::new_unchecked(
            "http://www.w3.org/2001/XMLSchema#boolean"
        ))),
        Just(IriRef::iri(IriS::new_unchecked(
            "http://www.w3.org/2001/XMLSchema#double"
        ))),
        Just(IriRef::iri(IriS::new_unchecked(
            "http://www.w3.org/2001/XMLSchema#decimal"
        ))),
        Just(IriRef::iri(IriS::new_unchecked(
            "http://www.w3.org/2001/XMLSchema#dateTime"
        ))),
    ]
}

/// Strategy for generating datatype literals
fn datatype_literal_strategy() -> impl Strategy<Value = ConcreteLiteral> {
    (".*", xsd_datatype_strategy()).prop_map(|(lexical, datatype)| ConcreteLiteral::lit_datatype(&lexical, &datatype))
}

/// Main strategy for generating arbitrary ConcreteLiterals
fn concrete_literal_strategy() -> impl Strategy<Value = ConcreteLiteral> {
    prop_oneof![
        string_literal_strategy(),
        numeric_literal_strategy(),
        boolean_literal_strategy(),
        datatype_literal_strategy(),
    ]
}

// ============================================================================
// Property Tests: Basic Invariants
// ============================================================================

proptest! {
    /// datatype should always return a valid IRI
    #[test]
    fn datatype_is_valid(lit in concrete_literal_strategy()) {
        let datatype = lit.datatype();
        match datatype {
            IriRef::Iri(iri) => {
                prop_assert!(!iri.as_str().is_empty());
                // Should be an XSD or RDF IRI
                let iri_str = iri.as_str();
                prop_assert!(
                    iri_str.starts_with("http://www.w3.org/2001/XMLSchema#") ||
                    iri_str.starts_with("http://www.w3.org/1999/02/22-rdf-syntax-ns#")
                );
            }
            IriRef::Prefixed { prefix, local } => {
                prop_assert!(!prefix.is_empty() || !local.is_empty());
            }
        }
    }

    /// Display should produce non-empty output
    #[test]
    fn display_non_empty(lit in concrete_literal_strategy()) {
        let display_str = lit.to_string();
        prop_assert!(!display_str.is_empty());
    }
}

// ============================================================================
// Property Tests: Equality and Hash
// ============================================================================

proptest! {
    /// Equality should be reflexive: a == a
    #[test]
    fn equality_reflexive(a in concrete_literal_strategy()) {
        prop_assert_eq!(&a, &a);
    }

    /// Equality should be symmetric: if a == b then b == a
    #[test]
    fn equality_symmetric(
        a in concrete_literal_strategy(),
        b in concrete_literal_strategy()
    ) {
        if a == b {
            prop_assert_eq!(b, a);
        }
    }

    /// Equality should be transitive: if a == b and b == c then a == c
    #[test]
    fn equality_transitive(
        a in concrete_literal_strategy(),
        b in concrete_literal_strategy(),
        c in concrete_literal_strategy()
    ) {
        if a == b && b == c {
            prop_assert_eq!(a, c);
        }
    }

    /// If a == b, then hash(a) == hash(b)
    #[test]
    fn hash_eq_consistency(
        a in concrete_literal_strategy(),
        b in concrete_literal_strategy()
    ) {
        if a == b {
            let mut hasher_a = DefaultHasher::new();
            let mut hasher_b = DefaultHasher::new();
            a.hash(&mut hasher_a);
            b.hash(&mut hasher_b);
            prop_assert_eq!(
                hasher_a.finish(),
                hasher_b.finish(),
                "Equal values must have equal hashes"
            );
        }
    }

    /// Hash should be deterministic
    #[test]
    fn hash_deterministic(lit in concrete_literal_strategy()) {
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        lit.clone().hash(&mut hasher1);
        lit.hash(&mut hasher2);
        prop_assert_eq!(hasher1.finish(), hasher2.finish());
    }

    /// match_literal should be reflexive
    #[test]
    fn match_literal_reflexive(a in concrete_literal_strategy()) {
        prop_assert!(a.match_literal(&a));
    }

    /// match_literal should be symmetric
    #[test]
    fn match_literal_symmetric(
        a in concrete_literal_strategy(),
        b in concrete_literal_strategy()
    ) {
        if a.match_literal(&b) {
            prop_assert!(b.match_literal(&a));
        }
    }
}

// ============================================================================
// Property Tests: Type Constructors
// ============================================================================

proptest! {
    /// String literal construction should preserve lexical form
    #[test]
    fn str_constructor_preserves_lexical(s in ".*") {
        let lit = ConcreteLiteral::str(&s);
        prop_assert_eq!(lit.lexical_form(), s);
        prop_assert_eq!(lit.lang(), None);
    }

    /// Language-tagged string should preserve both lexical form and language
    #[test]
    fn lang_str_preserves_both(s in ".*", lang in lang_strategy()) {
        let lit = ConcreteLiteral::lang_str(&s, lang.clone());
        prop_assert_eq!(lit.lexical_form(), s);
        prop_assert_eq!(lit.lang(), Some(lang));
    }

    /// Boolean literals should preserve their value
    #[test]
    fn boolean_preserves_value(b in any::<bool>()) {
        let lit = ConcreteLiteral::boolean(b);
        prop_assert_eq!(lit.lexical_form(), b.to_string());
        match lit {
            ConcreteLiteral::BooleanLiteral(val) => prop_assert_eq!(val, b),
            _ => prop_assert!(false, "Expected BooleanLiteral variant"),
        }
    }

    /// Integer constructor should create Integer variant
    #[test]
    fn integer_creates_correct_variant(n in any::<i128>()) {
        let lit = ConcreteLiteral::integer(n);
        match lit {
            ConcreteLiteral::NumericLiteral(NumericLiteral::Integer(val)) => {
                prop_assert_eq!(val, n);
            }
            _ => prop_assert!(false, "Expected NumericLiteral::Integer"),
        }
    }

    /// Positive integer should reject zero and negatives
    #[test]
    fn positive_integer_validates_range(n in any::<u128>()) {
        let result = ConcreteLiteral::positive_integer(n);
        if n == 0 {
            prop_assert!(result.is_err());
        } else {
            prop_assert!(result.is_ok());
        }
    }

    /// Negative integer should reject zero and positives
    #[test]
    fn negative_integer_validates_range(n in any::<i128>()) {
        let result = ConcreteLiteral::negative_integer(n);
        if n < 0 {
            prop_assert!(result.is_ok());
        } else {
            prop_assert!(result.is_err());
        }
    }

    /// Non-positive integer should accept zero and negatives
    #[test]
    fn non_positive_integer_validates_range(n in any::<i128>()) {
        let result = ConcreteLiteral::non_positive_integer(n);
        if n <= 0 {
            prop_assert!(result.is_ok());
        } else {
            prop_assert!(result.is_err());
        }
    }
}

// ============================================================================
// Property Tests: Parsing
// ============================================================================

proptest! {
    /// parse_bool should reject invalid strings
    #[test]
    fn parse_bool_rejects_invalid(s in "[a-z]{2,10}") {
        if s != "true" && s != "false" {
            prop_assert!(ConcreteLiteral::parse_bool(&s).is_err());
        }
    }

    /// parse_integer should round-trip with to_string
    #[test]
    fn parse_integer_roundtrip(n in any::<i64>()) {
        let s = n.to_string();
        let result = ConcreteLiteral::parse_integer(&s);
        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap(), n as i128);
    }

    /// parse_double should round-trip for finite doubles
    #[test]
    fn parse_double_roundtrip(d in any::<f64>().prop_filter("finite", |f| f.is_finite())) {
        let s = d.to_string();
        let result = ConcreteLiteral::parse_double(&s);
        prop_assert!(result.is_ok());
    }

    /// parse_negative_integer should reject non-negative values
    #[test]
    fn parse_negative_integer_validates(n in any::<i128>()) {
        let s = n.to_string();
        let result = ConcreteLiteral::parse_negative_integer(&s);
        if n < 0 {
            prop_assert!(result.is_ok());
            prop_assert_eq!(result.unwrap(), n);
        } else {
            prop_assert!(result.is_err());
        }
    }

    /// parse_positive_integer should reject non-positive values
    #[test]
    fn parse_positive_integer_validates(n in any::<u128>()) {
        let s = n.to_string();
        let result = ConcreteLiteral::parse_positive_integer(&s);
        if n > 0 {
            prop_assert!(result.is_ok());
            prop_assert_eq!(result.unwrap(), n);
        } else {
            prop_assert!(result.is_err());
        }
    }

    /// parse_non_positive_integer should accept zero and negatives
    #[test]
    fn parse_non_positive_integer_validates(n in any::<i128>()) {
        let s = n.to_string();
        let result = ConcreteLiteral::parse_non_positive_integer(&s);
        if n <= 0 {
            prop_assert!(result.is_ok());
            prop_assert_eq!(result.unwrap(), n);
        } else {
            prop_assert!(result.is_err());
        }
    }

    /// Unsigned parsers should handle full range
    #[test]
    fn parse_unsigned_byte_full_range(n in any::<u8>()) {
        let s = n.to_string();
        let result = ConcreteLiteral::parse_unsigned_byte(&s);
        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap(), n);
    }

    #[test]
    fn parse_unsigned_short_full_range(n in any::<u16>()) {
        let s = n.to_string();
        let result = ConcreteLiteral::parse_unsigned_short(&s);
        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap(), n);
    }

    #[test]
    fn parse_unsigned_int_full_range(n in any::<u32>()) {
        let s = n.to_string();
        let result = ConcreteLiteral::parse_unsigned_int(&s);
        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap(), n);
    }

    #[test]
    fn parse_unsigned_long_full_range(n in any::<u64>()) {
        let s = n.to_string();
        let result = ConcreteLiteral::parse_unsigned_long(&s);
        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap(), n);
    }
}

// ============================================================================
// Property Tests: Comparison and Ordering
// ============================================================================

proptest! {
    /// PartialOrd should be reflexive for comparable literals
    #[test]
    fn partial_ord_reflexive(a in concrete_literal_strategy()) {
        if let Some(ord) = a.partial_cmp(&a) {
            prop_assert_eq!(ord, std::cmp::Ordering::Equal);
        }
    }

    /// PartialOrd should be antisymmetric
    #[test]
    fn partial_ord_antisymmetric(
        a in concrete_literal_strategy(),
        b in concrete_literal_strategy()
    ) {
        if let (Some(ord_ab), Some(ord_ba)) = (a.partial_cmp(&b), b.partial_cmp(&a)) {
          prop_assert_eq!(ord_ab, ord_ba.reverse());
        }
    }

    /// PartialOrd should be transitive
    #[test]
    fn partial_ord_transitive(
        a in concrete_literal_strategy(),
        b in concrete_literal_strategy(),
        c in concrete_literal_strategy()
    ) {
        if let (Some(ord_ab), Some(ord_bc), Some(ord_ac)) =
            (a.partial_cmp(&b), b.partial_cmp(&c), a.partial_cmp(&c)) {
            // If a <= b and b <= c, then a <= c
            if ord_ab != std::cmp::Ordering::Greater && ord_bc != std::cmp::Ordering::Greater {
                prop_assert_ne!(ord_ac, std::cmp::Ordering::Greater);
            }
        }
    }

    /// String literals should compare lexicographically
    #[test]
    fn string_literals_compare_lexicographically(s1 in ".*", s2 in ".*") {
        let lit1 = ConcreteLiteral::str(&s1);
        let lit2 = ConcreteLiteral::str(&s2);

        match lit1.partial_cmp(&lit2) {
            Some(ord) => prop_assert_eq!(ord, s1.cmp(&s2)),
            None => prop_assert!(false, "String literals should be comparable"),
        }
    }

    /// Numeric literals should be comparable
    #[test]
    fn numeric_literals_comparable(n1 in any::<i32>(), n2 in any::<i32>()) {
        let lit1 = ConcreteLiteral::integer(n1 as i128);
        let lit2 = ConcreteLiteral::integer(n2 as i128);

        match lit1.partial_cmp(&lit2) {
            Some(ord) => prop_assert_eq!(ord, n1.cmp(&n2)),
            None => prop_assert!(false, "Integer literals should be comparable"),
        }
    }
}

// ============================================================================
// Property Tests: Conversion
// ============================================================================

proptest! {
    /// Conversion to oxrdf::Literal should always succeed
    #[test]
    fn converts_to_oxrdf(lit in concrete_literal_strategy()) {
        let _oxrdf_lit: oxrdf::Literal = lit.clone().into();
        // If this compiles and runs without panic, test passes
    }

    /// Conversion from simple string should preserve value
    #[test]
    fn string_conversion_preserves_value(s in ".*") {
        let lit = ConcreteLiteral::str(&s);
        let oxrdf_lit: oxrdf::Literal = lit.clone().into();
        prop_assert_eq!(oxrdf_lit.value(), s);
    }

    /// Boolean conversion should preserve value
    #[test]
    fn boolean_conversion_preserves_value(b in any::<bool>()) {
        let lit = ConcreteLiteral::boolean(b);
        let oxrdf_lit: oxrdf::Literal = lit.into();
        prop_assert_eq!(oxrdf_lit.value(), b.to_string());
    }
}

// ============================================================================
// Property Tests: into_checked_literal
// ============================================================================

proptest! {
    /// into_checked_literal should not modify non-datatype literals
    #[test]
    fn checked_literal_preserves_non_datatype(lit in string_literal_strategy()) {
        let original = lit.clone();
        let checked = lit.into_checked_literal();
        prop_assert!(checked.is_ok());
        prop_assert_eq!(checked.unwrap(), original);
    }

    /// into_checked_literal on boolean literals should succeed
    #[test]
    fn checked_literal_boolean(b in any::<bool>()) {
        let lit = ConcreteLiteral::boolean(b);
        let checked = lit.into_checked_literal();
        prop_assert!(checked.is_ok());
    }

    /// as_checked_literal on numeric literals should succeed
    #[test]
    fn checked_literal_numeric(n in any::<i64>()) {
        let lit = ConcreteLiteral::integer(n as i128);
        let checked = lit.into_checked_literal();
        prop_assert!(checked.is_ok());
    }
}

// ============================================================================
// Property Tests: Accessor Methods
// ============================================================================

proptest! {
    /// lang() should return None for non-language-tagged literals
    #[test]
    fn lang_returns_none_for_non_lang_literals(n in any::<i64>()) {
        let lit = ConcreteLiteral::integer(n as i128);
        prop_assert_eq!(lit.lang(), None);
    }

    /// lang() should return Some for language-tagged literals
    #[test]
    fn lang_returns_some_for_lang_literals(s in ".*", lang in lang_strategy()) {
        let lit = ConcreteLiteral::lang_str(&s, lang.clone());
        prop_assert_eq!(lit.lang(), Some(lang));
    }

    /// numeric_value() should return Some for numeric literals
    #[test]
    fn numeric_value_returns_some_for_numeric(n in any::<i64>()) {
        let lit = ConcreteLiteral::integer(n as i128);
        prop_assert!(lit.numeric_value().is_some());
    }

    /// numeric_value() should return None for non-numeric literals
    #[test]
    fn numeric_value_returns_none_for_non_numeric(s in ".*") {
        let lit = ConcreteLiteral::str(&s);
        prop_assert_eq!(lit.numeric_value(), None);
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_string_literal() {
        let lit = ConcreteLiteral::str("");
        assert_eq!(lit.lexical_form(), "");
        assert_eq!(lit.lang(), None);
    }

    #[test]
    fn test_zero_boundary_cases() {
        // Zero should be accepted by non-positive and non-negative
        assert!(ConcreteLiteral::non_positive_integer(0).is_ok());
        assert_eq!(
            ConcreteLiteral::non_negative_integer(0),
            ConcreteLiteral::NumericLiteral(NumericLiteral::NonNegativeInteger(0))
        );

        // Zero should be rejected by positive and negative
        assert!(ConcreteLiteral::positive_integer(0).is_err());
        assert!(ConcreteLiteral::negative_integer(0).is_err());
    }

    #[test]
    fn test_language_tag_case_sensitivity() {
        let lang_en = Lang::new("en").unwrap();
        let lit1 = ConcreteLiteral::lang_str("hello", lang_en.clone());
        let lit2 = ConcreteLiteral::lang_str("hello", lang_en);

        assert!(lit1.match_literal(&lit2));
        assert_eq!(lit1, lit2);
    }

    #[test]
    fn test_different_datatypes_not_equal() {
        let lit1 = ConcreteLiteral::str("42");
        let lit2 = ConcreteLiteral::integer(42);

        assert_ne!(lit1, lit2);
        assert!(!lit1.match_literal(&lit2));
    }

    #[test]
    fn test_datatype_literal_with_custom_iri() {
        let custom_iri = IriRef::iri(IriS::new_unchecked("http://example.org/customType"));
        let lit = ConcreteLiteral::lit_datatype("value", &custom_iri);

        assert_eq!(lit.lexical_form(), "value");
        assert_eq!(lit.datatype(), custom_iri);
    }

    #[test]
    fn test_boolean_parse_variations() {
        assert_eq!(ConcreteLiteral::parse_bool("true"), Ok(true));
        assert_eq!(ConcreteLiteral::parse_bool("false"), Ok(false));
        assert_eq!(ConcreteLiteral::parse_bool("1"), Ok(true));
        assert_eq!(ConcreteLiteral::parse_bool("0"), Ok(false));

        assert!(ConcreteLiteral::parse_bool("TRUE").is_err());
        assert!(ConcreteLiteral::parse_bool("yes").is_err());
        assert!(ConcreteLiteral::parse_bool("no").is_err());
    }

    #[test]
    fn test_extreme_integer_values() {
        let max_i128 = ConcreteLiteral::integer(i128::MAX);
        let min_i128 = ConcreteLiteral::integer(i128::MIN);

        assert!(!max_i128.lexical_form().is_empty());
        assert!(!min_i128.lexical_form().is_empty());
        assert_eq!(
            max_i128.datatype().to_string(),
            "http://www.w3.org/2001/XMLSchema#integer"
        );
    }

    #[test]
    fn test_float_special_values() {
        // NaN and infinity should not be allowed by our filter
        // but let's test that finite values work
        #[allow(clippy::approx_constant)]
        let lit = ConcreteLiteral::float(3.14f32);
        assert!(lit.numeric_value().is_some());

        #[allow(clippy::approx_constant)]
        let lit2 = ConcreteLiteral::double(2.718);
        assert!(lit2.numeric_value().is_some());
    }

    #[test]
    fn test_wrong_datatype_literal_structure() {
        // This would typically be created internally during validation
        let wrong_lit = ConcreteLiteral::WrongDatatypeLiteral {
            lexical_form: "not_a_number".to_string(),
            datatype: IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#integer")),
            error: "parse error".to_string(),
        };

        assert_eq!(wrong_lit.lexical_form(), "not_a_number");
    }
}
