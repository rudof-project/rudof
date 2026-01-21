use crate::data::literal::{ConcreteLiteral, Lang};
use prefixmap::{IriRef, PrefixMap, Deref};
use iri_s::IriS;


// ----------------------------------
// Validation and Conversion tests
// ----------------------------------


#[test]
fn test_as_checked_literal_valid_integer() {
    let xsd_integer = IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"));
    let lit = ConcreteLiteral::lit_datatype("42", &xsd_integer);
    let checked = lit.as_checked_literal().unwrap();
    
    assert_eq!(checked.lexical_form(), "42");
    assert!(checked.numeric_value().is_some());
}


#[test]
fn test_as_checked_literal_invalid_creates_wrong_datatype() {
    let xsd_integer = IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"));
    let lit = ConcreteLiteral::lit_datatype("not_a_number", &xsd_integer);
    let checked = lit.as_checked_literal().unwrap();
    
    // Should create a WrongDatatypeLiteral variant
    assert_eq!(checked.lexical_form(), "not_a_number");
    assert!(checked.numeric_value().is_none());
}


#[test]
fn test_as_checked_literal_non_datatype_unchanged() {
    let lit = ConcreteLiteral::str("hello");
    let checked = lit.as_checked_literal().unwrap();
    
    assert_eq!(checked.lexical_form(), "hello");
}


#[test]
fn test_as_checked_literal_boolean_valid() {
    let xsd_boolean = IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#boolean"));
    let lit = ConcreteLiteral::lit_datatype("true", &xsd_boolean);
    let checked = lit.as_checked_literal().unwrap();
    
    assert_eq!(checked.lexical_form(), "true");
}


#[test]
fn test_as_checked_literal_boolean_invalid() {
    let xsd_boolean = IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#boolean"));
    let lit = ConcreteLiteral::lit_datatype("maybe", &xsd_boolean);
    let checked = lit.as_checked_literal().unwrap();
    
    assert_eq!(checked.lexical_form(), "maybe");
}


#[test]
fn test_as_checked_literal_double_valid() {
    let xsd_double = IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#double"));
    let lit = ConcreteLiteral::lit_datatype("3.14159", &xsd_double);
    let checked = lit.as_checked_literal().unwrap();
    
    assert!(checked.numeric_value().is_some());
}


#[test]
fn test_as_checked_literal_unsigned_types() {
    let xsd_unsigned_byte = IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#unsignedByte"));
    let xsd_unsigned_short = IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#unsignedShort"));
    let xsd_unsigned_int = IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#unsignedInt"));
    let xsd_unsigned_long = IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#unsignedLong"));
    
    assert!(ConcreteLiteral::lit_datatype("200", &xsd_unsigned_byte).as_checked_literal().unwrap().numeric_value().is_some());
    assert!(ConcreteLiteral::lit_datatype("50000", &xsd_unsigned_short).as_checked_literal().unwrap().numeric_value().is_some());
    assert!(ConcreteLiteral::lit_datatype("1000000", &xsd_unsigned_int).as_checked_literal().unwrap().numeric_value().is_some());
    assert!(ConcreteLiteral::lit_datatype("10000000000", &xsd_unsigned_long).as_checked_literal().unwrap().numeric_value().is_some());
}


#[test]
fn test_as_checked_literal_custom_datatype_passthrough() {
    let custom_iri = IriRef::iri(IriS::new_unchecked("http://example.org/customType"));
    let lit = ConcreteLiteral::lit_datatype("custom_value", &custom_iri);
    let checked = lit.as_checked_literal().unwrap();
    
    // Custom datatypes are not validated, just passed through
    assert_eq!(checked.lexical_form(), "custom_value");
}


// ----------------------------------
// Display and Formatting tests
// ----------------------------------


#[test]
fn test_display_string_literal() {
    let lit = ConcreteLiteral::str("hello");
    assert_eq!(lit.to_string(), "\"hello\"");
}


#[test]
fn test_display_lang_string() {
    let lit = ConcreteLiteral::lang_str("hello", Lang::new("en").unwrap());
    let display = lit.to_string();
    assert!(display.contains("\"hello\""));
    assert!(display.contains("en"));
}


#[test]
fn test_display_numeric_literals() {
    assert_eq!(ConcreteLiteral::integer(42).to_string(), "42");
    assert_eq!(ConcreteLiteral::double(3.14).to_string(), "3.14");
    assert_eq!(ConcreteLiteral::boolean(true).to_string(), "true");
}


#[test]
fn test_show_qualified_with_basic_prefixmap() {
    let prefixmap = PrefixMap::basic();
    let xsd_integer = IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"));
    let lit = ConcreteLiteral::lit_datatype("42", &xsd_integer);
    
    let qualified = lit.show_qualified(&prefixmap);
    assert!(qualified.contains("\"42\""));
    assert!(qualified.contains("^^"));
}


#[test]
fn test_show_qualified_string_literal() {
    let prefixmap = PrefixMap::basic();
    let lit = ConcreteLiteral::str("test value");
    
    let qualified = lit.show_qualified(&prefixmap);
    assert_eq!(qualified, "\"test value\"");
}


#[test]
fn test_show_qualified_boolean() {
    let prefixmap = PrefixMap::basic();
    let lit = ConcreteLiteral::boolean(false);
    
    let qualified = lit.show_qualified(&prefixmap);
    assert_eq!(qualified, "false");
}


#[test]
fn test_display_empty_string() {
    let lit = ConcreteLiteral::str("");
    assert_eq!(lit.to_string(), "\"\"");
}


// ----------------------------------
// Ordering and Comparison tests
// ----------------------------------


#[test]
fn test_partial_ord_string_literals() {
    let lit1 = ConcreteLiteral::str("apple");
    let lit2 = ConcreteLiteral::str("banana");
    let lit3 = ConcreteLiteral::str("apple");
    
    assert!(lit1 < lit2);
    assert!(lit2 > lit1);
    assert_eq!(lit1.partial_cmp(&lit3), Some(std::cmp::Ordering::Equal));
}


#[test]
fn test_partial_ord_numeric_literals() {
    let lit1 = ConcreteLiteral::integer(10);
    let lit2 = ConcreteLiteral::integer(20);
    let lit3 = ConcreteLiteral::integer(10);
    
    assert!(lit1 < lit2);
    assert!(lit2 > lit1);
    assert_eq!(lit1.partial_cmp(&lit3), Some(std::cmp::Ordering::Equal));
}


#[test]
fn test_partial_ord_boolean_literals() {
    let lit_false = ConcreteLiteral::boolean(false);
    let lit_true = ConcreteLiteral::boolean(true);
    
    assert!(lit_false < lit_true);
    assert_eq!(lit_false.partial_cmp(&lit_false), Some(std::cmp::Ordering::Equal));
}


#[test]
fn test_partial_ord_incomparable_types() {
    let str_lit = ConcreteLiteral::str("hello");
    let int_lit = ConcreteLiteral::integer(42);
    let bool_lit = ConcreteLiteral::boolean(true);
    
    assert_eq!(str_lit.partial_cmp(&int_lit), None);
    assert_eq!(int_lit.partial_cmp(&bool_lit), None);
    assert_eq!(str_lit.partial_cmp(&bool_lit), None);
}


#[test]
fn test_partial_ord_different_datatypes_incomparable() {
    let xsd_integer = IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"));
    let xsd_string = IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#string"));
    
    let lit1 = ConcreteLiteral::lit_datatype("42", &xsd_integer);
    let lit2 = ConcreteLiteral::lit_datatype("42", &xsd_string);
    
    assert_eq!(lit1.as_checked_literal().unwrap().partial_cmp(&lit2.as_checked_literal().unwrap()), None);
}


#[test]
fn test_partial_ord_same_datatype_comparable() {
    let xsd_string = IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#string"));
    
    let lit1 = ConcreteLiteral::lit_datatype("alpha", &xsd_string);
    let lit2 = ConcreteLiteral::lit_datatype("beta", &xsd_string);
    
    assert!(lit1 < lit2);
}


#[test]
fn test_ord_comparable_types() {
    let lit1 = ConcreteLiteral::integer(5);
    let lit2 = ConcreteLiteral::integer(10);
    
    assert_eq!(lit1.cmp(&lit2), std::cmp::Ordering::Less);
}


#[test]
#[should_panic(expected = "Cannot compare literals")]
fn test_ord_panics_on_incomparable() {
    let str_lit = ConcreteLiteral::str("hello");
    let int_lit = ConcreteLiteral::integer(42);
    
    // This should panic
    let _ = str_lit.cmp(&int_lit);
}


#[test]
#[should_panic]
fn test_partial_ord_floats_with_nan() {
    let lit1 = ConcreteLiteral::double(3.14);
    let lit_nan = ConcreteLiteral::double(f64::NAN);
    
    // NaN comparisons return None
    assert_eq!(lit1.partial_cmp(&lit_nan), None);
    assert_eq!(lit_nan.partial_cmp(&lit_nan), None);
}


// ----------------------------------
// Type Conversion tests (oxrdf)
// ----------------------------------


#[test]
fn test_try_from_oxrdf_plain_string() {
    let oxrdf_lit = oxrdf::Literal::from("hello");
    let concrete: ConcreteLiteral = oxrdf_lit.try_into().unwrap();
    
    assert_eq!(concrete.lexical_form(), "hello");
    assert_eq!(concrete.lang(), None);
}


#[test]
fn test_try_from_oxrdf_lang_string() {
    let oxrdf_lit = oxrdf::Literal::new_language_tagged_literal_unchecked("hello", "en");
    let concrete: ConcreteLiteral = oxrdf_lit.try_into().unwrap();
    
    assert_eq!(concrete.lexical_form(), "hello");
    assert_eq!(concrete.lang(), Some(Lang::new("en").unwrap()));
}


#[test]
fn test_try_from_oxrdf_integer() {
    let oxrdf_lit = oxrdf::Literal::from(42);
    let concrete: ConcreteLiteral = oxrdf_lit.try_into().unwrap();
    
    assert!(concrete.numeric_value().is_some());
}


#[test]
fn test_try_from_oxrdf_boolean() {
    let oxrdf_lit = oxrdf::Literal::from(true);
    let concrete: ConcreteLiteral = oxrdf_lit.try_into().unwrap();
    
    assert_eq!(concrete.lexical_form(), "true");
}


#[test]
fn test_into_oxrdf_from_string() {
    let concrete = ConcreteLiteral::str("test");
    let oxrdf_lit: oxrdf::Literal = concrete.into();
    
    assert_eq!(oxrdf_lit.value(), "test");
}


#[test]
fn test_into_oxrdf_from_lang_string() {
    let concrete = ConcreteLiteral::lang_str("test", Lang::new("fr").unwrap());
    let oxrdf_lit: oxrdf::Literal = concrete.into();
    
    assert_eq!(oxrdf_lit.value(), "test");
    assert_eq!(oxrdf_lit.language(), Some("fr"));
}


#[test]
fn test_into_oxrdf_from_integer() {
    let concrete = ConcreteLiteral::integer(123);
    let oxrdf_lit: oxrdf::Literal = concrete.into();
    
    assert_eq!(oxrdf_lit.value(), "123");
}


#[test]
fn test_into_oxrdf_from_boolean() {
    let concrete = ConcreteLiteral::boolean(false);
    let oxrdf_lit: oxrdf::Literal = concrete.into();
    
    assert_eq!(oxrdf_lit.value(), "false");
}


#[test]
fn test_roundtrip_oxrdf_conversion() {
    let original = ConcreteLiteral::integer(99);
    let oxrdf_lit: oxrdf::Literal = original.clone().into();
    let converted: ConcreteLiteral = oxrdf_lit.try_into().unwrap();
    
    assert!(original.match_literal(&converted));
}


// ----------------------------------
// Deref trait tests
// ----------------------------------


#[test]
fn test_deref_numeric_literal() {
    let lit = ConcreteLiteral::integer(42);
    let derefed = lit.deref(&None, &None).unwrap();
    
    assert_eq!(derefed.lexical_form(), "42");
}


#[test]
fn test_deref_string_literal() {
    let lit = ConcreteLiteral::str("hello");
    let derefed = lit.deref(&None, &None).unwrap();
    
    assert_eq!(derefed.lexical_form(), "hello");
}


#[test]
fn test_deref_datatype_literal_with_prefixmap() {
    let prefixmap = PrefixMap::basic();
    let xsd_integer = IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"));
    let lit = ConcreteLiteral::lit_datatype("42", &xsd_integer);
    
    let derefed = lit.deref(&None, &Some(prefixmap)).unwrap();
    assert_eq!(derefed.lexical_form(), "42");
}


// ----------------------------------
// Other Constructor and Variant tests
// ----------------------------------


#[test]
fn test_lit_datatype_constructor() {
    let custom_iri = IriRef::iri(IriS::new_unchecked("http://example.org/customType"));
    let lit = ConcreteLiteral::lit_datatype("value", &custom_iri);
    
    assert_eq!(lit.lexical_form(), "value");
    assert_eq!(lit.datatype(), custom_iri);
}


#[test]
fn test_datatype_method_string_literal() {
    let lit = ConcreteLiteral::str("test");
    let datatype = lit.datatype();
    
    match datatype {
        IriRef::Iri(iri) => assert!(iri.as_str().contains("XMLSchema#string")),
        _ => panic!("Expected IRI datatype"),
    }
}


#[test]
fn test_datatype_method_lang_string() {
    let lit = ConcreteLiteral::lang_str("hello", Lang::new("en").unwrap());
    let datatype = lit.datatype();
    
    match datatype {
        IriRef::Iri(iri) => assert!(iri.as_str().contains("langString")),
        _ => panic!("Expected IRI datatype"),
    }
}


#[test]
fn test_datatype_method_boolean() {
    let lit = ConcreteLiteral::boolean(true);
    let datatype = lit.datatype();
    
    match datatype {
        IriRef::Iri(iri) => assert!(iri.as_str().contains("XMLSchema#boolean")),
        _ => panic!("Expected IRI datatype"),
    }
}


#[test]
fn test_datatype_method_integer() {
    let lit = ConcreteLiteral::integer(42);
    let datatype = lit.datatype();
    
    match datatype {
        IriRef::Iri(iri) => assert!(iri.as_str().contains("XMLSchema#integer")),
        _ => panic!("Expected IRI datatype"),
    }
}


// ----------------------------------
// Edge Cases and Special Values
// ----------------------------------


#[test]
fn test_default_literal() {
    let lit = ConcreteLiteral::default();
    assert_eq!(lit.lexical_form(), "");
    assert_eq!(lit.lang(), None);
}


#[test]
fn test_float_infinity() {
    let lit = ConcreteLiteral::double(f64::INFINITY);
    assert!(lit.numeric_value().is_some());
    assert_eq!(lit.lexical_form(), "inf");
}


#[test]
fn test_float_negative_infinity() {
    let lit = ConcreteLiteral::double(f64::NEG_INFINITY);
    assert!(lit.numeric_value().is_some());
    assert_eq!(lit.lexical_form(), "-inf");
}


#[test]
fn test_float_nan() {
    let lit = ConcreteLiteral::double(f64::NAN);
    assert!(lit.numeric_value().is_some());
    assert_eq!(lit.lexical_form(), "NaN");
}


#[test]
fn test_float_zero() {
    let lit = ConcreteLiteral::double(0.0);
    assert!(lit.numeric_value().is_some());
    assert_eq!(lit.lexical_form(), "0");
}


#[test]
fn test_float_negative_zero() {
    let lit = ConcreteLiteral::double(-0.0);
    assert!(lit.numeric_value().is_some());
}


#[test]
fn test_decimal_parsing_with_precision() {
    let result = ConcreteLiteral::parse_decimal("123.456789");
    assert!(result.is_ok());
    let decimal = result.unwrap();
    assert_eq!(decimal.to_string(), "123.456789");
}


#[test]
fn test_very_large_unsigned_long() {
    let lit = ConcreteLiteral::unsigned_long(u64::MAX);
    assert_eq!(lit.lexical_form(), u64::MAX.to_string());
}


#[test]
fn test_very_large_long() {
    let lit = ConcreteLiteral::long(isize::MAX);
    assert_eq!(lit.lexical_form(), isize::MAX.to_string());
}


#[test]
fn test_very_small_long() {
    let lit = ConcreteLiteral::long(isize::MIN);
    assert_eq!(lit.lexical_form(), isize::MIN.to_string());
}


#[test]
fn test_string_with_special_characters() {
    let lit = ConcreteLiteral::str("hello\nworld\ttab");
    assert_eq!(lit.lexical_form(), "hello\nworld\ttab");
}


#[test]
fn test_string_with_quotes() {
    let lit = ConcreteLiteral::str("say \"hello\"");
    assert_eq!(lit.lexical_form(), "say \"hello\"");
}


#[test]
fn test_unicode_string() {
    let lit = ConcreteLiteral::str("こんにちは 🌍");
    assert_eq!(lit.lexical_form(), "こんにちは 🌍");
}


#[test]
fn test_lang_string_different_languages() {
    let en = ConcreteLiteral::lang_str("hello", Lang::new("en").unwrap());
    let es = ConcreteLiteral::lang_str("hola", Lang::new("es").unwrap());
    let fr = ConcreteLiteral::lang_str("bonjour", Lang::new("fr").unwrap());
    
    assert_eq!(en.lang(), Some(Lang::new("en").unwrap()));
    assert_eq!(es.lang(), Some(Lang::new("es").unwrap()));
    assert_eq!(fr.lang(), Some(Lang::new("fr").unwrap()));
}


#[test]
fn test_numeric_value_none_for_string() {
    let lit = ConcreteLiteral::str("not a number");
    assert!(lit.numeric_value().is_none());
}


#[test]
fn test_numeric_value_none_for_boolean() {
    let lit = ConcreteLiteral::boolean(true);
    assert!(lit.numeric_value().is_none());
}


#[test]
fn test_equality_operator() {
    let lit1 = ConcreteLiteral::integer(42);
    let lit2 = ConcreteLiteral::integer(42);
    let lit3 = ConcreteLiteral::integer(43);
    
    assert_eq!(lit1, lit2);
    assert_ne!(lit1, lit3);
}


#[test]
fn test_clone_literal() {
    let lit1 = ConcreteLiteral::str("test");
    let lit2 = lit1.clone();
    
    assert!(lit1.match_literal(&lit2));
    assert_eq!(lit1.lexical_form(), lit2.lexical_form());
}


#[test]
fn test_hash_consistency() {
    use std::collections::HashSet;
    
    let lit1 = ConcreteLiteral::integer(42);
    let lit2 = ConcreteLiteral::integer(42);
    
    let mut set = HashSet::new();
    set.insert(lit1);
    assert!(set.contains(&lit2));
}


// ----------------------------------
// Additional Parsing Edge Cases
// ----------------------------------


#[test]
fn test_parse_bool_whitespace_invalid() {
    assert!(ConcreteLiteral::parse_bool(" true").is_err());
    assert!(ConcreteLiteral::parse_bool("true ").is_err());
}


#[test]
fn test_parse_integer_leading_zeros() {
    assert_eq!(ConcreteLiteral::parse_integer("007"), Ok(7));
}


#[test]
fn test_parse_integer_positive_sign() {
    assert_eq!(ConcreteLiteral::parse_integer("+42"), Ok(42));
}


#[test]
fn test_parse_double_scientific_notation() {
    let result = ConcreteLiteral::parse_double("1.5e10");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1.5e10);
}


#[test]
fn test_parse_double_infinity_string() {
    let result = ConcreteLiteral::parse_double("inf");
    assert!(result.is_ok());
    assert!(result.unwrap().is_infinite());
}


#[test]
fn test_parse_float_negative_scientific() {
    let result = ConcreteLiteral::parse_float("-3.14e-5");
    assert!(result.is_ok());
}


#[test]
fn test_unsigned_boundary_values() {
    assert_eq!(ConcreteLiteral::parse_unsigned_byte("0"), Ok(0));
    assert_eq!(ConcreteLiteral::parse_unsigned_byte("255"), Ok(255));
    assert_eq!(ConcreteLiteral::parse_unsigned_short("0"), Ok(0));
    assert_eq!(ConcreteLiteral::parse_unsigned_short("65535"), Ok(65535));
    assert_eq!(ConcreteLiteral::parse_unsigned_int("0"), Ok(0));
    assert_eq!(ConcreteLiteral::parse_unsigned_int("4294967295"), Ok(4294967295));
}


#[test]
fn test_signed_boundary_values() {
    assert_eq!(ConcreteLiteral::parse_byte("-128"), Ok(-128));
    assert_eq!(ConcreteLiteral::parse_byte("127"), Ok(127));
}
