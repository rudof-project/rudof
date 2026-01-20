use super::*;

// ----------------------------------
// lang.rs tests
// ----------------------------------

#[test]
    fn test_lang_equality() {
        let en = Lang::new("en").unwrap();
        let en_us = Lang::new("en-US").unwrap();
        let fr = Lang::new("fr").unwrap();
        let en1 = Lang::new("en").unwrap();
        let en_fr = Lang::new("en-fr").unwrap();

        assert_ne!(en, en_us);
        assert_ne!(en, fr);
        assert_eq!(en, en1);
        assert_ne!(en, en_fr);
    }

// ----------------------------------
// sliteral.rs tests
// ----------------------------------

#[test]
fn test_unsigned_long_negative() {
    let result = SLiteral::parse_unsigned_long("-1");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("unsigned long"));
}

#[test]
fn test_unsigned_long_valid() {
    let result = SLiteral::parse_unsigned_long("42");
    assert_eq!(result, Ok(42));
}

#[test]
fn test_boolean_parsing() {
    assert_eq!(SLiteral::parse_bool("true"), Ok(true));
    assert_eq!(SLiteral::parse_bool("false"), Ok(false));
    assert_eq!(SLiteral::parse_bool("1"), Ok(true));
    assert_eq!(SLiteral::parse_bool("0"), Ok(false));
    assert!(SLiteral::parse_bool("invalid").is_err());
}

#[test]
fn test_positive_integer() {
    assert!(SLiteral::parse_positive_integer("0").is_err());
    assert_eq!(SLiteral::parse_positive_integer("1"), Ok(1));
    assert!(SLiteral::parse_positive_integer("-1").is_err());
}

#[test]
fn test_negative_integer() {
    assert!(SLiteral::parse_negative_integer("0").is_err());
    assert!(SLiteral::parse_negative_integer("1").is_err());
    assert_eq!(SLiteral::parse_negative_integer("-1"), Ok(-1));
}

#[test]
fn test_literal_equality() {
    let lit1 = SLiteral::integer(42);
    let lit2 = SLiteral::integer(42);
    let lit3 = SLiteral::integer(43);

    assert!(lit1.match_literal(&lit2));
    assert!(!lit1.match_literal(&lit3));
}

#[test]
fn test_string_literal_with_lang() {
    let lit = SLiteral::lang_str("Hello", Lang::new("en").unwrap());
    assert_eq!(lit.lang(), Some(Lang::new("en").unwrap()));
    assert_eq!(lit.lexical_form(), "Hello");
}
