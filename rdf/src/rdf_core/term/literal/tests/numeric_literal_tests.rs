use crate::rdf_core::{
    term::literal::NumericLiteral,
};
use rust_decimal::Decimal;
use std::collections::HashSet;

// ========================================================================
// Constructor Tests
// ========================================================================

#[test]
fn test_integer_constructor() {
    let lit = NumericLiteral::integer(42);
    assert_eq!(lit, NumericLiteral::Integer(42));
    assert_eq!(lit.lexical_form(), "42");
}

#[test]
fn test_integer_i128_bounds() {
    let max = NumericLiteral::integer(i128::MAX);
    let min = NumericLiteral::integer(i128::MIN);
    assert_eq!(max.lexical_form(), i128::MAX.to_string());
    assert_eq!(min.lexical_form(), i128::MIN.to_string());
}

#[test]
fn test_long_constructor() {
    let lit = NumericLiteral::long(9223372036854775807);
    assert_eq!(lit, NumericLiteral::Long(i64::MAX));
    assert_eq!(lit.lexical_form(), "9223372036854775807");
}

#[test]
fn test_byte_constructor() {
    let lit = NumericLiteral::byte(-128);
    assert_eq!(lit, NumericLiteral::Byte(i8::MIN));
    let lit2 = NumericLiteral::byte(127);
    assert_eq!(lit2, NumericLiteral::Byte(i8::MAX));
}

#[test]
fn test_short_constructor() {
    let lit = NumericLiteral::short(-32768);
    assert_eq!(lit, NumericLiteral::Short(i16::MIN));
    let lit2 = NumericLiteral::short(32767);
    assert_eq!(lit2, NumericLiteral::Short(i16::MAX));
}

#[test]
fn test_float_constructor() {
    let lit = NumericLiteral::float(3.14f32);
    assert!(matches!(lit, NumericLiteral::Float(_)));
}

#[test]
fn test_double_constructor() {
    let lit = NumericLiteral::double(3.14159265359);
    assert!(matches!(lit, NumericLiteral::Double(_)));
}

#[test]
fn test_decimal_constructor() {
    let dec = Decimal::new(314, 2); // 3.14
    let lit = NumericLiteral::decimal(dec);
    assert_eq!(lit, NumericLiteral::Decimal(dec));
}

// ========================================================================
// Validation Constructor Tests
// ========================================================================

#[test]
fn test_positive_integer_valid() {
    let lit = NumericLiteral::positive_integer(1).unwrap();
    assert_eq!(lit, NumericLiteral::PositiveInteger(1));

    let lit2 = NumericLiteral::positive_integer(u128::MAX).unwrap();
    assert!(matches!(lit2, NumericLiteral::PositiveInteger(_)));
}

#[test]
fn test_positive_integer_rejects_zero() {
    let result = NumericLiteral::positive_integer(0);
    assert!(result.is_err());
}

#[test]
fn test_negative_integer_valid() {
    let lit = NumericLiteral::negative_integer(-1).unwrap();
    assert_eq!(lit, NumericLiteral::NegativeInteger(-1));

    let lit2 = NumericLiteral::negative_integer(i128::MIN).unwrap();
    assert!(matches!(lit2, NumericLiteral::NegativeInteger(_)));
}

#[test]
fn test_negative_integer_rejects_zero() {
    let result = NumericLiteral::negative_integer(0);
    assert!(result.is_err());
}

#[test]
fn test_negative_integer_rejects_positive() {
    let result = NumericLiteral::negative_integer(1);
    assert!(result.is_err());
}

#[test]
fn test_non_positive_integer_valid() {
    let lit1 = NumericLiteral::non_positive_integer(0).unwrap();
    assert_eq!(lit1, NumericLiteral::NonPositiveInteger(0));

    let lit2 = NumericLiteral::non_positive_integer(-100).unwrap();
    assert_eq!(lit2, NumericLiteral::NonPositiveInteger(-100));
}

#[test]
fn test_non_positive_integer_rejects_positive() {
    let result = NumericLiteral::non_positive_integer(1);
    assert!(result.is_err());
}

#[test]
fn test_non_negative_integer() {
    let lit1 = NumericLiteral::non_negative_integer(0);
    assert_eq!(lit1, NumericLiteral::NonNegativeInteger(0));

    let lit2 = NumericLiteral::non_negative_integer(100);
    assert_eq!(lit2, NumericLiteral::NonNegativeInteger(100));
}

#[test]
fn test_unsigned_types() {
    let byte = NumericLiteral::unsigned_byte(255);
    assert_eq!(byte, NumericLiteral::UnsignedByte(u8::MAX));

    let short = NumericLiteral::unsigned_short(65535);
    assert_eq!(short, NumericLiteral::UnsignedShort(u16::MAX));

    let int = NumericLiteral::unsigned_int(u32::MAX);
    assert_eq!(int, NumericLiteral::UnsignedInt(u32::MAX));

    let long = NumericLiteral::unsigned_long(u64::MAX);
    assert_eq!(long, NumericLiteral::UnsignedLong(u64::MAX));
}

// ========================================================================
// Decimal Conversion Tests
// ========================================================================

#[test]
fn test_decimal_from_i128() {
    let lit = NumericLiteral::decimal_from_i128(42).unwrap();
    assert!(matches!(lit, NumericLiteral::Decimal(_)));
    assert_eq!(lit.lexical_form(), "42");
}

#[test]
fn test_decimal_from_f64() {
    let lit = NumericLiteral::decimal_from_f64(3.14).unwrap();
    assert!(matches!(lit, NumericLiteral::Decimal(_)));
}

#[test]
fn test_decimal_from_f64_nan() {
    let result = NumericLiteral::decimal_from_f64(f64::NAN);
    assert!(result.is_err());
}

#[test]
fn test_decimal_from_f64_infinity() {
    let result = NumericLiteral::decimal_from_f64(f64::INFINITY);
    assert!(result.is_err());
}

#[test]
fn test_decimal_from_parts() {
    let lit = NumericLiteral::decimal_from_parts(3, 14).unwrap();
    assert_eq!(lit.lexical_form(), "3.14");
}

#[test]
fn test_integer_from_i64() {
    let lit = NumericLiteral::integer_from_i64(i64::MAX);
    assert_eq!(lit, NumericLiteral::Integer(i64::MAX as i128));
}

// ========================================================================
// to_decimal Tests
// ========================================================================

#[test]
fn test_to_decimal_integer() {
    let lit = NumericLiteral::integer(42);
    let dec = lit.to_decimal();
    assert_eq!(dec.unwrap(), Decimal::new(42, 0));
}

#[test]
fn test_to_decimal_long() {
    let lit = NumericLiteral::long(100);
    let dec = lit.to_decimal();
    assert_eq!(dec.unwrap(), Decimal::new(100, 0));
}

#[test]
fn test_to_decimal_byte() {
    let lit = NumericLiteral::byte(127);
    let dec = lit.to_decimal();
    assert_eq!(dec.unwrap(), Decimal::new(127, 0));
}

#[test]
fn test_to_decimal_decimal() {
    let original = Decimal::new(314, 2);
    let lit = NumericLiteral::decimal(original);
    let dec = lit.to_decimal();
    assert_eq!(dec.unwrap(), original);
}

// ========================================================================
// Comparison Tests
// ========================================================================

#[test]
fn test_less_than_same_type() {
    let a = NumericLiteral::integer(5);
    let b = NumericLiteral::integer(10);
    assert!(a.less_than(&b));
    assert!(!b.less_than(&a));
}

#[test]
fn test_less_than_mixed_types() {
    let int = NumericLiteral::integer(5);
    let long = NumericLiteral::long(10);
    assert!(int.less_than(&long));
}

#[test]
fn test_less_than_or_eq() {
    let a = NumericLiteral::integer(5);
    let b = NumericLiteral::integer(5);
    let c = NumericLiteral::integer(10);

    assert!(a.less_than_or_eq(&b));
    assert!(a.less_than_or_eq(&c));
    assert!(!c.less_than_or_eq(&a));
}

#[test]
fn test_partial_ord() {
    let a = NumericLiteral::integer(5);
    let b = NumericLiteral::integer(10);

    assert!(a < b);
    assert!(b > a);
    assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Less));
}

#[test]
fn test_partial_ord_nan() {
    let nan = NumericLiteral::double(f64::NAN);
    let num = NumericLiteral::integer(5);

    assert_eq!(nan.partial_cmp(&num), None);
}

// ========================================================================
// Digit Counting Tests
// ========================================================================

#[test]
fn test_total_digits_integer() {
    let lit = NumericLiteral::integer(12345);
    assert_eq!(lit.total_digits(), Some(5));

    let neg = NumericLiteral::integer(-9876);
    assert_eq!(neg.total_digits(), Some(4)); // Sign excluded
}

#[test]
fn test_total_digits_decimal() {
    let dec = Decimal::new(12345, 2); // 123.45
    let lit = NumericLiteral::decimal(dec);
    assert_eq!(lit.total_digits(), Some(5));
}

#[test]
fn test_total_digits_float() {
    let lit = NumericLiteral::float(3.14f32);
    assert_eq!(lit.total_digits(), None);
}

#[test]
fn test_fraction_digits_integer() {
    let lit = NumericLiteral::integer(42);
    assert_eq!(lit.fraction_digits(), Some(0));
}

#[test]
fn test_fraction_digits_decimal() {
    let dec = Decimal::new(12345, 2); // 123.45
    let lit = NumericLiteral::decimal(dec);
    assert_eq!(lit.fraction_digits(), Some(2));
}

#[test]
fn test_fraction_digits_float() {
    let lit = NumericLiteral::float(3.14f32);
    assert_eq!(lit.fraction_digits(), None);
}

// ========================================================================
// Datatype IRI Tests
// ========================================================================

#[test]
fn test_datatype_integer() {
    let lit = NumericLiteral::integer(42);
    assert_eq!(lit.datatype().get_iri().unwrap().as_str(), "http://www.w3.org/2001/XMLSchema#integer");
}

#[test]
fn test_datatype_long() {
    let lit = NumericLiteral::long(42);
    assert_eq!(lit.datatype().get_iri().unwrap().as_str(), "http://www.w3.org/2001/XMLSchema#long");
}

#[test]
fn test_datatype_byte() {
    let lit = NumericLiteral::byte(42);
    assert_eq!(lit.datatype().get_iri().unwrap().as_str(), "http://www.w3.org/2001/XMLSchema#byte");
}

#[test]
fn test_datatype_short() {
    let lit = NumericLiteral::short(42);
    assert_eq!(lit.datatype().get_iri().unwrap().as_str(), "http://www.w3.org/2001/XMLSchema#short");
}

#[test]
fn test_datatype_float() {
    let lit = NumericLiteral::float(3.14f32);
    assert_eq!(lit.datatype().get_iri().unwrap().as_str(), "http://www.w3.org/2001/XMLSchema#float");
}

#[test]
fn test_datatype_double() {
    let lit = NumericLiteral::double(3.14);
    assert_eq!(lit.datatype().get_iri().unwrap().as_str(), "http://www.w3.org/2001/XMLSchema#double");
}

#[test]
fn test_datatype_decimal() {
    let lit = NumericLiteral::decimal(Decimal::new(314, 2));
    assert_eq!(lit.datatype().get_iri().unwrap().as_str(), "http://www.w3.org/2001/XMLSchema#decimal");
}

#[test]
fn test_datatype_positive_integer() {
    let lit = NumericLiteral::positive_integer(1).unwrap();
    assert_eq!(lit.datatype().get_iri().unwrap().as_str(), "http://www.w3.org/2001/XMLSchema#positiveInteger");
}

#[test]
fn test_datatype_negative_integer() {
    let lit = NumericLiteral::negative_integer(-1).unwrap();
    assert_eq!(lit.datatype().get_iri().unwrap().as_str(), "http://www.w3.org/2001/XMLSchema#negativeInteger");
}

// ========================================================================
// Hash and Eq Tests
// ========================================================================

#[test]
fn test_eq_same_variant() {
    let a = NumericLiteral::integer(42);
    let b = NumericLiteral::integer(42);
    assert_eq!(a, b);
}

#[test]
fn test_not_eq_different_variants() {
    let int = NumericLiteral::integer(42);
    let long = NumericLiteral::long(42);
    assert_ne!(int, long); // Different variants, even with same value
}

#[test]
fn test_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a = NumericLiteral::integer(42);
    let b = NumericLiteral::integer(42);

    let mut hasher_a = DefaultHasher::new();
    let mut hasher_b = DefaultHasher::new();

    a.hash(&mut hasher_a);
    b.hash(&mut hasher_b);

    assert_eq!(hasher_a.finish(), hasher_b.finish());
}

#[test]
fn test_hash_set_usage() {
    let mut set = HashSet::new();
    set.insert(NumericLiteral::integer(42));
    set.insert(NumericLiteral::integer(42)); // Duplicate
    set.insert(NumericLiteral::integer(43));

    assert_eq!(set.len(), 2);
}

// ========================================================================
// Display Tests
// ========================================================================

#[test]
fn test_display_integer() {
    let lit = NumericLiteral::integer(42);
    assert_eq!(format!("{}", lit), "42");
}

#[test]
fn test_display_negative() {
    let lit = NumericLiteral::integer(-42);
    assert_eq!(format!("{}", lit), "-42");
}

#[test]
fn test_display_decimal() {
    let dec = Decimal::new(314, 2);
    let lit = NumericLiteral::decimal(dec);
    assert_eq!(format!("{}", lit), "3.14");
}

#[test]
fn test_lexical_form() {
    let lit = NumericLiteral::integer(42);
    assert_eq!(lit.lexical_form(), "42");
}

// ========================================================================
// TryFrom String Tests
// ========================================================================

#[test]
fn test_try_from_integer_string() {
    let lit = NumericLiteral::try_from("42").unwrap();
    assert_eq!(lit, NumericLiteral::Integer(42));
}

#[test]
fn test_try_from_negative_string() {
    let lit = NumericLiteral::try_from("-42").unwrap();
    assert_eq!(lit, NumericLiteral::Integer(-42));
}

#[test]
fn test_try_from_float_string() {
    let lit = NumericLiteral::try_from("3.14").unwrap();
    assert!(matches!(lit, NumericLiteral::Double(_))); 
}

#[test]
fn test_try_from_decimal_string() {
    let lit = NumericLiteral::try_from("3.141592653589793238462643383279").unwrap();
    assert!(matches!(lit, NumericLiteral::Double(_)));
}

#[test]
fn test_try_from_invalid_string() {
    let result = NumericLiteral::try_from("not_a_number");
    assert!(result.is_err());
}

#[test]
fn test_try_from_large_integer() {
    let lit = NumericLiteral::try_from("170141183460469231731687303715884105727").unwrap();
    assert_eq!(lit, NumericLiteral::Integer(i128::MAX));
}

// ========================================================================
// Edge Cases
// ========================================================================

#[test]
fn test_zero_values() {
    let int_zero = NumericLiteral::integer(0);
    let long_zero = NumericLiteral::long(0);
    let byte_zero = NumericLiteral::byte(0);

    assert_eq!(int_zero.lexical_form(), "0");
    assert_eq!(long_zero.lexical_form(), "0");
    assert_eq!(byte_zero.lexical_form(), "0");
}

#[test]
fn test_boundary_values() {
    // Test i8 boundaries
    let byte_min = NumericLiteral::byte(i8::MIN);
    let byte_max = NumericLiteral::byte(i8::MAX);
    assert_eq!(byte_min.lexical_form(), "-128");
    assert_eq!(byte_max.lexical_form(), "127");

    // Test i16 boundaries
    let short_min = NumericLiteral::short(i16::MIN);
    let short_max = NumericLiteral::short(i16::MAX);
    assert_eq!(short_min.lexical_form(), "-32768");
    assert_eq!(short_max.lexical_form(), "32767");
}

#[test]
fn test_clone() {
    let lit = NumericLiteral::integer(42);
    let cloned = lit.clone();
    assert_eq!(lit, cloned);
}

#[test]
fn test_debug() {
    let lit = NumericLiteral::integer(42);
    let debug_str = format!("{:?}", lit);
    assert!(debug_str.contains("Integer"));
    assert!(debug_str.contains("42"));
}
