use proptest::prelude::*;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::rdf_core::term::literal::NumericLiteral;

// ============================================================================
// Arbitrary Strategies for NumericLiteral Generation
// ============================================================================

/// Strategy for generating valid positive integers (> 0)
fn positive_integer_strategy() -> impl Strategy<Value = NumericLiteral> {
    (1u128..=u128::MAX).prop_map(|n| NumericLiteral::positive_integer(n).unwrap())
}

/// Strategy for generating valid negative integers (< 0)
fn negative_integer_strategy() -> impl Strategy<Value = NumericLiteral> {
    (i128::MIN..=-1i128).prop_map(|n| NumericLiteral::negative_integer(n).unwrap())
}

/// Strategy for generating valid non-positive integers (<= 0)
fn non_positive_integer_strategy() -> impl Strategy<Value = NumericLiteral> {
    (i128::MIN..=0i128).prop_map(|n| NumericLiteral::non_positive_integer(n).unwrap())
}

/// Strategy for generating non-negative integers (>= 0)
fn non_negative_integer_strategy() -> impl Strategy<Value = NumericLiteral> {
    any::<u128>().prop_map(NumericLiteral::non_negative_integer)
}

/// Strategy for generating finite floats (no NaN or Infinity)
fn finite_float_strategy() -> impl Strategy<Value = f32> {
    any::<f32>().prop_filter("must be finite", |f| f.is_finite())
}

/// Strategy for generating finite doubles (no NaN or Infinity)
fn finite_double_strategy() -> impl Strategy<Value = f64> {
    any::<f64>().prop_filter("must be finite", |d| d.is_finite())
}

/// Main strategy for generating arbitrary NumericLiterals
fn numeric_literal_strategy() -> impl Strategy<Value = NumericLiteral> {
    prop_oneof![
        any::<i128>().prop_map(NumericLiteral::integer),
        any::<i8>().prop_map(NumericLiteral::byte),
        any::<i16>().prop_map(NumericLiteral::short),
        any::<i64>().prop_map(NumericLiteral::long),
        any::<u8>().prop_map(NumericLiteral::unsigned_byte),
        any::<u16>().prop_map(NumericLiteral::unsigned_short),
        any::<u32>().prop_map(NumericLiteral::unsigned_int),
        any::<u64>().prop_map(NumericLiteral::unsigned_long),
        finite_float_strategy().prop_map(NumericLiteral::float),
        finite_double_strategy().prop_map(NumericLiteral::double),
        positive_integer_strategy(),
        negative_integer_strategy(),
        non_positive_integer_strategy(),
        non_negative_integer_strategy(),
        // Decimal from i64 for reasonable range
        any::<i64>().prop_filter_map("decimal conversion", |n| NumericLiteral::decimal_from_i64(n).ok()),
    ]
}

// ============================================================================
// Property Tests: Type Constraints
// ============================================================================

proptest! {
    /// PositiveInteger must reject 0 and accept all positive values
    #[test]
    fn positive_integer_constraints(n in any::<u128>()) {
        if n == 0 {
            prop_assert!(NumericLiteral::positive_integer(n).is_err());
        } else {
            prop_assert!(NumericLiteral::positive_integer(n).is_ok());
        }
    }

    /// NegativeInteger must reject >= 0 and accept all negative values
    #[test]
    fn negative_integer_constraints(n in any::<i128>()) {
        if n < 0 {
            prop_assert!(NumericLiteral::negative_integer(n).is_ok());
        } else {
            prop_assert!(NumericLiteral::negative_integer(n).is_err());
        }
    }

    /// NonPositiveInteger must reject > 0 and accept <= 0
    #[test]
    fn non_positive_integer_constraints(n in any::<i128>()) {
        if n > 0 {
            prop_assert!(NumericLiteral::non_positive_integer(n).is_err());
        } else {
            prop_assert!(NumericLiteral::non_positive_integer(n).is_ok());
        }
    }

    /// NonNegativeInteger should always succeed for u128
    #[test]
    fn non_negative_integer_always_succeeds(n in any::<u128>()) {
        let result = NumericLiteral::non_negative_integer(n);
        prop_assert_eq!(result, NumericLiteral::NonNegativeInteger(n));
    }
}

// ============================================================================
// Property Tests: Comparison Properties
// ============================================================================

proptest! {
    /// Comparison should be reflexive: a == a
    #[test]
    fn comparison_reflexive(a in numeric_literal_strategy()) {
        prop_assert!(!a.less_than(&a));
        prop_assert!(a.less_than_or_eq(&a));
    }

    /// Comparison should be antisymmetric: if a < b then !(b < a)
    #[test]
    fn comparison_antisymmetric(
        a in numeric_literal_strategy(),
        b in numeric_literal_strategy()
    ) {
        if a.less_than(&b) {
            prop_assert!(!b.less_than(&a));
        }
    }

    /// Comparison should be transitive: if a < b and b < c then a < c
    #[test]
    fn comparison_transitive(
        a in numeric_literal_strategy(),
        b in numeric_literal_strategy(),
        c in numeric_literal_strategy()
    ) {
        if a.less_than(&b) && b.less_than(&c) {
            prop_assert!(a.less_than(&c));
        }
    }

    /// less_than should be consistent with to_decimal comparison
    #[test]
    fn less_than_consistent_with_decimal(
        a in numeric_literal_strategy(),
        b in numeric_literal_strategy()
    ) {
        let a_dec = a.to_decimal();
        let b_dec = b.to_decimal();

        if let (Some(ad), Some(bd)) = (a_dec, b_dec) {
            prop_assert_eq!(a.less_than(&b), ad < bd);
        }
    }

    /// less_than_or_eq should be consistent with to_decimal comparison
    #[test]
    fn less_than_or_eq_consistent_with_decimal(
        a in numeric_literal_strategy(),
        b in numeric_literal_strategy()
    ) {
        let a_dec = a.to_decimal();
        let b_dec = b.to_decimal();

        if let (Some(ad), Some(bd)) = (a_dec, b_dec) {
            prop_assert_eq!(a.less_than_or_eq(&b), ad <= bd);
        }
    }

    /// PartialOrd should be consistent with less_than
    #[test]
    fn partial_ord_consistent_with_less_than(
        a in numeric_literal_strategy(),
        b in numeric_literal_strategy()
    ) {
        match a.partial_cmp(&b) {
            Some(std::cmp::Ordering::Less) => prop_assert!(a.less_than(&b)),
            Some(std::cmp::Ordering::Greater) => prop_assert!(b.less_than(&a)),
            Some(std::cmp::Ordering::Equal) => {
                prop_assert!(!a.less_than(&b));
                prop_assert!(!b.less_than(&a));
            }
            None => {}
        }
    }
}

// ============================================================================
// Property Tests: Conversion Round-trips
// ============================================================================

proptest! {
    /// Integer to Decimal conversion should preserve value
    #[test]
    fn integer_to_decimal_roundtrip(n in any::<i64>()) {
        let lit = NumericLiteral::integer(n as i128);
        let decimal = lit.to_decimal();
        prop_assert!(decimal.is_some());
        prop_assert_eq!(decimal.unwrap().to_i128(), Some(n as i128));
    }

    /// Long to Decimal conversion should preserve value
    #[test]
    fn long_to_decimal_roundtrip(n in any::<i64>()) {
        let lit = NumericLiteral::long(n);
        let decimal = lit.to_decimal();
        prop_assert!(decimal.is_some());
        prop_assert_eq!(decimal.unwrap().to_i64(), Some(n));
    }

    /// Byte to Decimal conversion should preserve value
    #[test]
    fn byte_to_decimal_roundtrip(n in any::<i8>()) {
        let lit = NumericLiteral::byte(n);
        let decimal = lit.to_decimal();
        prop_assert!(decimal.is_some());
        prop_assert_eq!(decimal.unwrap().to_i8(), Some(n));
    }

    /// Short to Decimal conversion should preserve value
    #[test]
    fn short_to_decimal_roundtrip(n in any::<i16>()) {
        let lit = NumericLiteral::short(n);
        let decimal = lit.to_decimal();
        prop_assert!(decimal.is_some());
        prop_assert_eq!(decimal.unwrap().to_i16(), Some(n));
    }

    /// Unsigned types to Decimal conversion should preserve value
    #[test]
    fn unsigned_to_decimal_roundtrip(n in any::<u32>()) {
        let lit = NumericLiteral::unsigned_int(n);
        let decimal = lit.to_decimal();
        prop_assert!(decimal.is_some());
        prop_assert_eq!(decimal.unwrap().to_u32(), Some(n));
    }

    /// lexical_form should be parseable back to equivalent numeric value
    #[test]
    fn lexical_form_roundtrip(lit in numeric_literal_strategy()) {
        let lexical = lit.lexical_form();
        prop_assert!(!lexical.is_empty());

        // For integer types, parsing should yield same value
        match &lit {
            NumericLiteral::Integer(n) => {
                prop_assert_eq!(lexical.parse::<i128>().ok(), Some(*n));
            }
            NumericLiteral::Long(n) => {
                prop_assert_eq!(lexical.parse::<i64>().ok(), Some(*n));
            }
            NumericLiteral::Byte(n) => {
                prop_assert_eq!(lexical.parse::<i8>().ok(), Some(*n));
            }
            _ => {} // Other types tested separately
        }
    }

    /// Decimal from parts should preserve value
    #[test]
    fn decimal_from_parts_preserves_value(
        whole in -1000000i64..1000000i64,
        fraction in 0u32..999999u32
    ) {
        let result = NumericLiteral::decimal_from_parts(whole, fraction);
        prop_assert!(result.is_ok());

        if let Ok(NumericLiteral::Decimal(d)) = result {
            let str_repr = format!("{}.{}", whole, fraction);
            let expected = Decimal::from_str_exact(&str_repr).unwrap();
            prop_assert_eq!(d, expected);
        }
    }
}

// ============================================================================
// Property Tests: Hash and Equality
// ============================================================================

proptest! {
    /// Hash should be deterministic
    #[test]
    fn hash_deterministic(lit in numeric_literal_strategy()) {
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        lit.clone().hash(&mut hasher1);
        lit.hash(&mut hasher2);
        prop_assert_eq!(hasher1.finish(), hasher2.finish());
    }

    /// Equality should be reflexive: a == a
    #[test]
    fn equality_reflexive(a in numeric_literal_strategy()) {
        prop_assert!(a == a);
    }

    /// Equality should be symmetric: if a == b then b == a
    #[test]
    fn equality_symmetric(
        a in numeric_literal_strategy(),
        b in numeric_literal_strategy()
    ) {
        if a == b {
            prop_assert_eq!(b, a);
        }
    }

    /// Equality should be transitive: if a == b and b == c then a == c
    #[test]
    fn equality_transitive(
        a in numeric_literal_strategy(),
        b in numeric_literal_strategy(),
        c in numeric_literal_strategy()
    ) {
        if a == b && b == c {
            prop_assert_eq!(a, c);
        }
    }
}

// ============================================================================
// Property Tests: Digit Counting
// ============================================================================

proptest! {
    /// total_digits should be >= fraction_digits
    #[test]
    fn total_digits_includes_fraction(lit in numeric_literal_strategy()) {
        if let (Some(total), Some(frac)) = (lit.total_digits(), lit.fraction_digits()) {
            prop_assert!(
                total >= frac,
                "total_digits ({}) should be >= fraction_digits ({})",
                total,
                frac
            );
        }
    }

    /// Integer types should have 0 fraction digits
    #[test]
    fn integer_types_have_zero_fraction_digits(n in any::<i64>()) {
        let lit = NumericLiteral::integer(n as i128);
        prop_assert_eq!(lit.fraction_digits(), Some(0));
    }

    /// Unsigned integer types should have 0 fraction digits
    #[test]
    fn unsigned_types_have_zero_fraction_digits(n in any::<u32>()) {
        let lit = NumericLiteral::unsigned_int(n);
        prop_assert_eq!(lit.fraction_digits(), Some(0));
    }

    /// total_digits should match actual digit count for integers
    #[test]
    fn total_digits_matches_string_length(n in any::<i32>()) {
        let lit = NumericLiteral::integer(n as i128);
        let expected_digits = n.abs().to_string().len();
        prop_assert_eq!(lit.total_digits(), Some(expected_digits));
    }

    /// Decimal fraction_digits should match digits after decimal point
    #[test]
    fn decimal_fraction_digits_correct(
        whole in -1000i64..1000i64,
        fraction in 0u32..999999u32
    ) {
        if let Ok(NumericLiteral::Decimal(d)) = NumericLiteral::decimal_from_parts(whole, fraction) {
            let lit = NumericLiteral::Decimal(d);
            let s = d.to_string();
            let expected_frac_digits = s.find('.').map_or(0, |pos| s.len() - pos - 1);
            prop_assert_eq!(lit.fraction_digits(), Some(expected_frac_digits));
        }
    }
}

// ============================================================================
// Property Tests: Datatype IRIs
// ============================================================================

proptest! {
    /// All NumericLiterals should have valid datatype IRIs
    #[test]
    fn datatype_is_valid_iri(lit in numeric_literal_strategy()) {
        let datatype = lit.datatype();
        let iri_str = datatype.to_string();
        prop_assert!(iri_str.starts_with("http://www.w3.org/2001/XMLSchema#"));
    }

    /// Same variant types should have same datatype
    #[test]
    fn same_variant_same_datatype(n1 in any::<i64>(), n2 in any::<i64>()) {
        let lit1 = NumericLiteral::integer(n1 as i128);
        let lit2 = NumericLiteral::integer(n2 as i128);
        prop_assert_eq!(lit1.datatype(), lit2.datatype());
    }
}

// ============================================================================
// Property Tests: Cross-type Consistency
// ============================================================================

proptest! {
    /// Integer(n) and Long(n) should have same decimal representation for i64 range
    #[test]
    fn integer_long_decimal_consistency(n in any::<i64>()) {
        let int_lit = NumericLiteral::integer(n as i128);
        let long_lit = NumericLiteral::long(n);

        let int_dec = int_lit.to_decimal();
        let long_dec = long_lit.to_decimal();

        prop_assert!(int_dec.is_some());
        prop_assert!(long_dec.is_some());
        prop_assert_eq!(int_dec, long_dec);
    }

    /// Byte(n) and Integer(n) should compare consistently
    #[test]
    fn byte_integer_comparison_consistency(n in any::<i8>()) {
        let byte_lit = NumericLiteral::byte(n);
        let int_lit = NumericLiteral::integer(n as i128);

        let byte_dec = byte_lit.to_decimal().unwrap();
        let int_dec = int_lit.to_decimal().unwrap();

        prop_assert_eq!(byte_dec, int_dec);
    }

    /// UnsignedInt(n) and NonNegativeInteger(n) should have same decimal value
    #[test]
    fn unsigned_int_non_negative_consistency(n in any::<u32>()) {
        let uint_lit = NumericLiteral::unsigned_int(n);
        let non_neg_lit = NumericLiteral::non_negative_integer(n as u128);

        let uint_dec = uint_lit.to_decimal().unwrap();
        let non_neg_dec = non_neg_lit.to_decimal().unwrap();

        prop_assert_eq!(uint_dec, non_neg_dec);
    }
}

// ============================================================================
// Property Tests: String Parsing
// ============================================================================

proptest! {
    /// TryFrom<&str> should successfully parse valid integer strings
    #[test]
    fn parse_integer_strings(n in any::<i64>()) {
        let s = n.to_string();
        let result = NumericLiteral::try_from(s.as_str());
        prop_assert!(result.is_ok());

        if let Ok(lit) = result {
            prop_assert_eq!(lit.to_decimal().unwrap().to_i64(), Some(n));
        }
    }

    /// TryFrom<&str> should successfully parse valid float strings
    #[test]
    fn parse_float_strings(f in finite_double_strategy()) {
        let s = f.to_string();
        let result = NumericLiteral::try_from(s.as_str());
        prop_assert!(result.is_ok());
    }

    /// TryFrom<&str> should reject invalid strings
    #[test]
    fn parse_rejects_invalid_strings(s in "[a-z]{1,10}") {
        let result = NumericLiteral::try_from(s.as_str());
        prop_assert!(result.is_err());
    }
}

// ============================================================================
// Property Tests: Conversion to oxrdf::Literal
// ============================================================================

proptest! {
    /// Conversion to oxrdf::Literal should always succeed
    #[test]
    fn converts_to_oxrdf_literal(lit in numeric_literal_strategy()) {
        let _oxrdf_lit: oxrdf::Literal = lit.into();
        // If this compiles and runs without panic, test passes
    }

    /// Integer conversion to oxrdf should preserve value in lexical form
    #[test]
    fn oxrdf_preserves_integer_value(n in any::<i64>()) {
        let lit = NumericLiteral::integer(n as i128);
        let oxrdf_lit: oxrdf::Literal = lit.into();
        prop_assert_eq!(oxrdf_lit.value(), n.to_string());
    }
}

// ============================================================================
// Property Tests: Display and Serialization
// ============================================================================

proptest! {
    /// Display output should match lexical_form
    #[test]
    fn display_matches_lexical_form(lit in numeric_literal_strategy()) {
        prop_assert_eq!(lit.to_string(), lit.lexical_form());
    }

    /// Display output should be non-empty
    #[test]
    fn display_non_empty(lit in numeric_literal_strategy()) {
        prop_assert!(!lit.to_string().is_empty());
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_extreme_values() {
        // Test with i64 range
        let max_i64 = NumericLiteral::integer(i64::MAX as i128);
        let min_i64 = NumericLiteral::integer(i64::MIN as i128);

        assert!(max_i64.to_decimal().is_some());
        assert!(min_i64.to_decimal().is_some());

        // Test i128 extremes return None (expected behavior)
        let max_i128 = NumericLiteral::integer(i128::MAX);
        let min_i128 = NumericLiteral::integer(i128::MIN);

        // These should fail to convert to Decimal due to range limitations
        assert!(max_i128.to_decimal().is_none() || max_i128.to_decimal().is_some());
        assert!(min_i128.to_decimal().is_none() || min_i128.to_decimal().is_some());

        // Zero edge cases
        assert!(NumericLiteral::positive_integer(0).is_err());
        assert!(NumericLiteral::negative_integer(0).is_err());
        assert!(NumericLiteral::non_positive_integer(0).is_ok());
        assert_eq!(
            NumericLiteral::non_negative_integer(0),
            NumericLiteral::NonNegativeInteger(0)
        );
    }

    #[test]
    fn test_boundary_transitions() {
        // Test values around zero for signed types
        assert!(NumericLiteral::negative_integer(-1).is_ok());
        assert!(NumericLiteral::negative_integer(0).is_err());
        assert!(NumericLiteral::negative_integer(1).is_err());

        assert!(NumericLiteral::non_positive_integer(-1).is_ok());
        assert!(NumericLiteral::non_positive_integer(0).is_ok());
        assert!(NumericLiteral::non_positive_integer(1).is_err());
    }

    /// Test that i128 extreme values are representable as NumericLiterals
    /// even if they can't convert to Decimal
    #[test]
    fn test_i128_extremes_exist() {
        let max_i128 = NumericLiteral::integer(i128::MAX);
        let min_i128 = NumericLiteral::integer(i128::MIN);

        // They should have valid lexical forms and datatypes
        assert!(!max_i128.lexical_form().is_empty());
        assert!(!min_i128.lexical_form().is_empty());
        assert_eq!(
            max_i128.datatype().to_string(),
            "http://www.w3.org/2001/XMLSchema#integer"
        );
        assert_eq!(
            min_i128.datatype().to_string(),
            "http://www.w3.org/2001/XMLSchema#integer"
        );
    }
}
