use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::result;

use crate::Object;
use crate::RDFError;
use crate::XsdDateTime;
use crate::{lang::Lang, numeric_literal::NumericLiteral};
use iri_s::IriS;
use prefixmap::{Deref, IriRef, PrefixMap};
use prefixmap::error::DerefError;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize, Serializer};
use tracing::trace;

/// Concrete representation of RDF literals
/// This representation internally uses integers, doubles, booleans, etc. to represent values
/// It also supports literals with wrong datatypes to be able to parse RDF data that can have wrong datatype literals but needs to be validated
#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub enum SLiteral {
    StringLiteral {
        lexical_form: String,
        lang: Option<Lang>,
    },
    DatatypeLiteral {
        lexical_form: String,
        datatype: IriRef, // TODO: We should change this to IriS
    },
    NumericLiteral(NumericLiteral),
    DatetimeLiteral(XsdDateTime),

    #[serde(serialize_with = "serialize_boolean_literal")]
    BooleanLiteral(bool),

    /// Represents a literal with a wrong datatype
    /// For example, a value like `23` with datatype `xsd:date`
    /// These literals can be useful to parse RDF data that can have wrong datatype literals but needs to be validated
    /// Error contains the error message
    WrongDatatypeLiteral {
        lexical_form: String,
        datatype: IriRef,
        error: String,
    },
}

impl SLiteral {
    /// Returns a string representation of the literal using the given prefixmap to qualify the datatype IRI
    pub fn show_qualified(&self, prefixmap: &PrefixMap) -> String {
        trace!("Showing qualified literal: {self:?} with prefixmap: {prefixmap:?}");
        // NOTE: I am not sure if there is a simpler way to do the following
        struct Helper<'a> {
            literal: &'a SLiteral,
            prefixmap: &'a PrefixMap,
        }

        impl<'a> Display for Helper<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.literal.display_qualified(f, self.prefixmap)
            }
        }
        format!(
            "{}",
            Helper {
                literal: self,
                prefixmap,
            }
        )
    }

    /// Returns a literal after checking that the lexical form matches the declared datatype
    /// This can be useful to validate datatypes that are wrong like `"hello"^^xsd:integer`
    pub fn as_checked_literal(&self) -> Result<SLiteral, RDFError> {
        match self {
            SLiteral::DatatypeLiteral {
                lexical_form,
                datatype,
            } => check_literal_datatype(lexical_form, datatype),
            _ => Ok(self.clone()),
        }
    }

    pub fn match_literal(&self, literal_expected: &SLiteral) -> bool {
        let result = match self {
            SLiteral::StringLiteral { lexical_form, lang } => match literal_expected {
                SLiteral::StringLiteral {
                    lexical_form: expected_lexical_form,
                    lang: expected_lang,
                } => {
                    trace!(
                        "Comparing string literals: {lexical_form} ({lang:?}) with expected {expected_lexical_form} ({expected_lang:?})"
                    );
                    lexical_form == expected_lexical_form && lang == expected_lang
                }
                _ => false,
            },
            SLiteral::DatatypeLiteral {
                lexical_form,
                datatype,
            } => match literal_expected {
                SLiteral::DatatypeLiteral {
                    lexical_form: expected_lexical_form,
                    datatype: expected_datatype,
                } => lexical_form == expected_lexical_form && datatype == expected_datatype,
                _ => false,
            },
            SLiteral::NumericLiteral(numeric_literal) => match literal_expected {
                SLiteral::NumericLiteral(expected_numeric_literal) => {
                    numeric_literal == expected_numeric_literal
                }
                _ => false,
            },
            SLiteral::DatetimeLiteral(xsd_date_time) => match literal_expected {
                SLiteral::DatetimeLiteral(expected_xsd_date_time) => {
                    xsd_date_time == expected_xsd_date_time
                }
                _ => false,
            },
            SLiteral::BooleanLiteral(b) => match literal_expected {
                SLiteral::BooleanLiteral(expected_bool) => b == expected_bool,
                _ => false,
            },
            SLiteral::WrongDatatypeLiteral {
                lexical_form,
                datatype,
                error: _,
            } => match literal_expected {
                SLiteral::WrongDatatypeLiteral {
                    lexical_form: expected_lexical_form,
                    datatype: expected_datatype,
                    error: _,
                } => lexical_form == expected_lexical_form && datatype == expected_datatype,
                _ => false,
            },
        };
        trace!("match_literal: self: {self}, expected: {literal_expected}: {result}");
        result
    }

    pub fn integer(n: isize) -> SLiteral {
        SLiteral::NumericLiteral(NumericLiteral::integer(n))
    }

    pub fn non_negative_integer(n: usize) -> SLiteral {
        SLiteral::NumericLiteral(NumericLiteral::non_negative_integer(n))
    }

    pub fn non_positive_integer(n: isize) -> SLiteral {
        SLiteral::NumericLiteral(NumericLiteral::non_positive_integer(n))
    }

    pub fn positive_integer(n: usize) -> SLiteral {
        SLiteral::NumericLiteral(NumericLiteral::positive_integer(n))
    }

    pub fn negative_integer(n: isize) -> SLiteral {
        SLiteral::NumericLiteral(NumericLiteral::negative_integer(n))
    }

    pub fn double(d: f64) -> SLiteral {
        SLiteral::NumericLiteral(NumericLiteral::double(d))
    }

    pub fn decimal(d: Decimal) -> SLiteral {
        SLiteral::NumericLiteral(NumericLiteral::decimal(d))
    }

    pub fn long(n: isize) -> SLiteral {
        SLiteral::NumericLiteral(NumericLiteral::long(n))
    }

    pub fn unsigned_byte(n: u8) -> SLiteral {
        SLiteral::NumericLiteral(NumericLiteral::unsigned_byte(n))
    }

    pub fn unsigned_short(n: u16) -> SLiteral {
        SLiteral::NumericLiteral(NumericLiteral::unsigned_short(n))
    }

    pub fn unsigned_int(n: u32) -> SLiteral {
        SLiteral::NumericLiteral(NumericLiteral::unsigned_int(n))
    }

    pub fn unsigned_long(n: u64) -> SLiteral {
        SLiteral::NumericLiteral(NumericLiteral::unsigned_long(n))
    }

    pub fn byte(n: i8) -> SLiteral {
        SLiteral::NumericLiteral(NumericLiteral::byte(n))
    }

    pub fn float(n: f64) -> SLiteral {
        SLiteral::NumericLiteral(NumericLiteral::float(n))
    }

    pub fn lit_datatype(lexical_form: &str, datatype: &IriRef) -> SLiteral {
        SLiteral::DatatypeLiteral {
            lexical_form: lexical_form.to_owned(),
            datatype: datatype.clone(),
        }
    }

    pub fn boolean(b: bool) -> SLiteral {
        SLiteral::BooleanLiteral(b)
    }

    /// Parses a string that should represent a lexical form of a boolean
    /// This parsing follows the rules of XSD boolean datatype
    /// Valid values are "true", "false", "0", "1"
    /// Returns an error if the string cannot be parsed as a boolean
    /// The parsing is case sensitive
    pub fn parse_bool(str: &str) -> Result<bool, String> {
        match str {
            "true" => Ok(true),
            "false" => Ok(false),
            "0" => Ok(true),
            "1" => Ok(false),
            _ => Err(format!("Cannot convert {str} to boolean")),
        }
    }

    /// Parses a string that should represent a lexical form of an integer
    /// This parsing follows the rules of XSD integer datatype
    /// Valid values are any valid integer string
    /// Returns an error if the string cannot be parsed as an integer
    pub fn parse_integer(str: &str) -> Result<isize, String> {
        match str::parse::<isize>(str) {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Cannot convert {str} to integer")),
        }
    }

    /// Parses a string that should represent a lexical form of a negative integer
    /// This parsing follows the rules of XSD negative integer datatype
    /// Valid values are any valid integer string
    /// Returns an error if the string cannot be parsed as a negative integer
    pub fn parse_negative_integer(str: &str) -> Result<isize, String> {
        match str::parse::<isize>(str) {
            Ok(value) if value < 0 => Ok(value),
            Ok(_) => Err(format!("Cannot convert {str} to negative integer")),
            Err(_) => Err(format!("Cannot convert {str} to negative integer")),
        }
    }

    /// Parses a string that should represent a lexical form of a non-positive integer
    /// This parsing follows the rules of XSD non-positive integer datatype
    /// Valid values are any valid integer string
    /// Returns an error if the string cannot be parsed as a non-positive integer
    pub fn parse_non_positive_integer(str: &str) -> Result<isize, String> {
        match str::parse::<isize>(str) {
            Ok(value) if value <= 0 => Ok(value),
            Ok(_) => Err(format!("Cannot convert {str} to non-positive integer")),
            Err(_) => Err(format!("Cannot convert {str} to non-positive integer")),
        }
    }

    /// Parses a string that should represent a lexical form of a positive integer
    /// This parsing follows the rules of XSD positive integer datatype
    /// Valid values are any valid integer string
    /// Returns an error if the string cannot be parsed as a positive integer
    pub fn parse_positive_integer(str: &str) -> Result<usize, String> {
        match str::parse::<usize>(str) {
            Ok(value) if value > 0 => Ok(value),
            Ok(_) => Err(format!("Cannot convert {str} to positive integer")),
            Err(_) => Err(format!("Cannot convert {str} to positive integer")),
        }
    }

    /// Parses a string that should represent a lexical form of a non-negative integer
    /// This parsing follows the rules of XSD non-negative integer datatype
    /// Valid values are any valid integer string
    /// Returns an error if the string cannot be parsed as a non-negative integer
    pub fn parse_non_negative_integer(str: &str) -> Result<usize, String> {
        str::parse::<usize>(str)
            .map_err(|e| format!("Cannot convert {str} to non-negative integer: {e}"))
    }

    /// Parses a string that should represent a lexical form of a unsigned byte
    /// This parsing follows the rules of XSD unsignedByte datatype
    /// Valid values are any valid integer string
    /// Returns an error if the string cannot be parsed as a unsigned byte
    pub fn parse_unsigned_byte(str: &str) -> Result<u8, String> {
        match str::parse::<u8>(str) {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Cannot convert {str} to unsigned byte")),
        }
    }

    /// Parses a string that should represent a lexical form of a unsigned short
    /// This parsing follows the rules of XSD unsignedShort datatype
    /// Valid values are any valid integer string
    /// Returns an error if the string cannot be parsed as a unsigned short
    pub fn parse_unsigned_short(str: &str) -> Result<u16, String> {
        match str::parse::<u16>(str) {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Cannot convert {str} to unsigned short")),
        }
    }
    /// Parses a string that should represent a lexical form of a unsigned integer
    /// This parsing follows the rules of XSD unsignedInt datatype
    /// Valid values are any valid integer string
    /// Returns an error if the string cannot be parsed as a unsigned integer
    pub fn parse_unsigned_int(str: &str) -> Result<u32, String> {
        match str::parse::<u32>(str) {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Cannot convert {str} to unsigned int")),
        }
    }

    /// Parses a string that should represent a lexical form of a unsigned long
    /// This parsing follows the rules of XSD unsignedLong datatype
    /// Valid values are any valid integer string
    /// Returns an error if the string cannot be parsed as a unsigned long
    pub fn parse_unsigned_long(str: &str) -> Result<u64, String> {
        match str::parse::<u64>(str) {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Cannot convert {str} to unsigned long")),
        }
    }

    /// Parses a string that should represent a lexical form of an double
    /// This parsing follows the rules of XSD double datatype
    /// Valid values are any valid double string
    /// Returns an error if the string cannot be parsed as a double
    pub fn parse_double(str: &str) -> Result<f64, String> {
        match str::parse::<f64>(str) {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Cannot convert {str} to double")),
        }
    }

    /// Parses a string that should represent a lexical form of an long
    /// This parsing follows the rules of XSD long  datatype
    /// Valid values are any valid long string
    /// Returns an error if the string cannot be parsed as a long
    pub fn parse_long(str: &str) -> Result<isize, String> {
        match str::parse::<isize>(str) {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Cannot convert {str} to long")),
        }
    }

    /// Parses a string that should represent a lexical form of an decimal
    /// This parsing follows the rules of XSD decimal datatype
    /// Valid values are any valid decimal string
    /// Returns an error if the string cannot be parsed as a decimal
    pub fn parse_decimal(str: &str) -> Result<Decimal, String> {
        match str::parse::<Decimal>(str) {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Cannot convert {str} to decimal")),
        }
    }

    /// Parses a string that should represent a lexical form of an float
    /// This parsing follows the rules of XSD float datatype
    /// Valid values are any valid float string
    /// Returns an error if the string cannot be parsed as a float
    pub fn parse_float(str: &str) -> Result<f64, String> {
        match str::parse::<f64>(str) {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Cannot convert {str} to float")),
        }
    }

    /// Parses a string that should represent a lexical form of an byte
    /// This parsing follows the rules of XSD byte datatype
    /// Valid values are any valid byte string
    /// Returns an error if the string cannot be parsed as a byte
    pub fn parse_byte(str: &str) -> Result<i8, String> {
        match str::parse::<i8>(str) {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Cannot convert {str} to byte")),
        }
    }

    pub fn str(lexical_form: &str) -> SLiteral {
        SLiteral::StringLiteral {
            lexical_form: lexical_form.to_owned(),
            lang: None,
        }
    }

    pub fn lang_str(lexical_form: &str, lang: Lang) -> SLiteral {
        SLiteral::StringLiteral {
            lexical_form: lexical_form.to_owned(),
            lang: Some(lang),
        }
    }

    pub fn lang(&self) -> Option<Lang> {
        match self {
            SLiteral::StringLiteral { lang, .. } => lang.clone(),
            _ => None,
        }
    }

    pub fn lexical_form(&self) -> String {
        match self {
            SLiteral::StringLiteral { lexical_form, .. } => lexical_form.clone(),
            SLiteral::DatatypeLiteral { lexical_form, .. } => lexical_form.clone(),
            SLiteral::NumericLiteral(nl) => nl.lexical_form(),
            SLiteral::BooleanLiteral(true) => "true".to_string(),
            SLiteral::BooleanLiteral(false) => "false".to_string(),
            SLiteral::DatetimeLiteral(dt) => dt.to_string(),
            SLiteral::WrongDatatypeLiteral { lexical_form, .. } => lexical_form.clone(),
        }
    }

    pub fn display_qualified(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        prefixmap: &PrefixMap,
    ) -> std::fmt::Result {
        match self {
            SLiteral::StringLiteral {
                lexical_form,
                lang: None,
            } => write!(f, "\"{lexical_form}\""),
            SLiteral::StringLiteral {
                lexical_form,
                lang: Some(lang),
            } => write!(f, "\"{lexical_form}\"{lang}"),
            SLiteral::DatatypeLiteral {
                lexical_form,
                datatype,
            } => match datatype {
                IriRef::Iri(iri) => write!(f, "\"{lexical_form}\"^^{}", prefixmap.qualify(iri)),
                IriRef::Prefixed { prefix, local } => {
                    write!(f, "\"{lexical_form}\"^^{prefix}:{local}")
                }
            },
            SLiteral::NumericLiteral(n) => write!(f, "{n}"),
            SLiteral::BooleanLiteral(true) => write!(f, "true"),
            SLiteral::BooleanLiteral(false) => write!(f, "false"),
            SLiteral::DatetimeLiteral(date_time) => write!(f, "{}", date_time.value()),
            SLiteral::WrongDatatypeLiteral {
                lexical_form,
                datatype,
                ..
            } => match datatype {
                IriRef::Iri(iri) => write!(f, "\"{lexical_form}\"^^{}", prefixmap.qualify(iri)),
                IriRef::Prefixed { prefix, local } => {
                    write!(f, "\"{lexical_form}\"^^{prefix}:{local}")
                }
            },
        }
    }

    pub fn datatype(&self) -> IriRef {
        match self {
            SLiteral::DatatypeLiteral { datatype, .. } => datatype.clone(),
            SLiteral::StringLiteral {
                lexical_form: _,
                lang: None,
            } => IriRef::iri(IriS::new_unchecked(
                "http://www.w3.org/2001/XMLSchema#string",
            )),
            SLiteral::StringLiteral {
                lexical_form: _,
                lang: Some(_),
            } => IriRef::iri(IriS::new_unchecked(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString",
            )),
            SLiteral::NumericLiteral(nl) => IriRef::iri(IriS::new_unchecked(nl.datatype())),
            SLiteral::BooleanLiteral(_) => IriRef::iri(IriS::new_unchecked(
                "http://www.w3.org/2001/XMLSchema#boolean",
            )),
            SLiteral::DatetimeLiteral(_) => IriRef::iri(IriS::new_unchecked(
                "http://www.w3.org/2001/XMLSchema#dateTime",
            )),
            SLiteral::WrongDatatypeLiteral { datatype, .. } => datatype.clone(),
        }
    }

    pub fn numeric_value(&self) -> Option<NumericLiteral> {
        match self {
            SLiteral::NumericLiteral(nl) => Some(nl.clone()),
            _ => None,
        }
    }
}

impl Default for SLiteral {
    fn default() -> Self {
        SLiteral::StringLiteral {
            lexical_form: String::default(),
            lang: None,
        }
    }
}

// The comparison between literals is based on SPARQL comparison rules.
// String literals are compared lexicographically, datatype literals are compared based on their datatype and lexical form,
// numeric literals are compared based on their numeric value, and boolean literals are compared as true >
// See: https://www.w3.org/TR/sparql11-query/#OperatorMapping
// Numeric arguments are promoted as necessary to fit the expected types for that function or operator.
#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for SLiteral {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            SLiteral::DatetimeLiteral(date_time1) => match other {
                SLiteral::DatetimeLiteral(date_time2) => date_time1.partial_cmp(date_time2),
                _ => None,
            },
            SLiteral::StringLiteral { lexical_form, .. } => match other {
                SLiteral::StringLiteral {
                    lexical_form: other_lexical_form,
                    ..
                } => Some(lexical_form.cmp(other_lexical_form)),
                _ => None,
            },
            SLiteral::DatatypeLiteral {
                lexical_form,
                datatype,
            } => match other {
                SLiteral::DatatypeLiteral {
                    lexical_form: other_lexical_form,
                    datatype: other_datatype,
                } => {
                    if datatype == other_datatype {
                        Some(lexical_form.cmp(other_lexical_form))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            SLiteral::NumericLiteral(nl) => match other {
                SLiteral::NumericLiteral(other_nl) => nl.partial_cmp(other_nl),
                _ => None,
            },
            SLiteral::BooleanLiteral(b) => match other {
                SLiteral::BooleanLiteral(other_b) => Some(b.cmp(other_b)),
                _ => None,
            },
            SLiteral::WrongDatatypeLiteral {
                lexical_form,
                datatype,
                ..
            } => match other {
                SLiteral::DatatypeLiteral {
                    lexical_form: other_lexical_form,
                    datatype: other_datatype,
                } => {
                    if datatype == other_datatype {
                        Some(lexical_form.cmp(other_lexical_form))
                    } else {
                        None
                    }
                }
                _ => None,
            },
        }
    }
}

// TODO: We implement Ord only for literals that can be compared
// This is a temporary solution to be able to compare literal when we sort the results of validation
// The problematic cases are f64 NaN and literals with different datatypes
// A better solution would be to define a total order for all literals
// See: https://www.w3.org/TR/sparql11-query/#OperatorMapping
impl Ord for SLiteral {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other) {
            Some(ordering) => ordering,
            None => panic!("Cannot compare literals {self} and {other}"),
        }
    }
}

impl Display for SLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display_qualified(f, &PrefixMap::basic())
    }
}

fn serialize_boolean_literal<S>(value: &bool, serializer: S) -> result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        false => serializer.serialize_str("false"),
        true => serializer.serialize_str("true"),
    }
}

impl Deref for SLiteral {
    fn deref(
        self,
        base: Option<&IriS>,
        prefixmap: Option<&PrefixMap>,
    ) -> Result<Self, DerefError> {
        match self {
            SLiteral::NumericLiteral(n) => Ok(SLiteral::NumericLiteral(n.clone())),
            SLiteral::BooleanLiteral(b) => Ok(SLiteral::BooleanLiteral(b)),
            SLiteral::StringLiteral { lexical_form, lang } => Ok(SLiteral::StringLiteral {
                lexical_form: lexical_form.clone(),
                lang: lang.clone(),
            }),
            SLiteral::DatatypeLiteral {
                lexical_form,
                datatype,
            } => {
                let dt = datatype.deref(base, prefixmap)?;
                Ok(SLiteral::DatatypeLiteral {
                    lexical_form: lexical_form.clone(),
                    datatype: dt,
                })
            }
            SLiteral::DatetimeLiteral(date_time) => {
                Ok(SLiteral::DatetimeLiteral(date_time.clone()))
            }
            SLiteral::WrongDatatypeLiteral {
                lexical_form,
                datatype,
                ..
            } => {
                let dt = datatype.deref(base, prefixmap)?;
                Ok(SLiteral::DatatypeLiteral {
                    lexical_form: lexical_form.clone(),
                    datatype: dt,
                })
            }
        }
    }
}

impl TryFrom<oxrdf::Literal> for SLiteral {
    type Error = RDFError;

    fn try_from(value: oxrdf::Literal) -> Result<Self, Self::Error> {
        let value_str = value.to_string();
        match value.destruct() {
            (s, None, None, None) => Ok(SLiteral::str(&s)),
            (s, None, Some(language), None) => {
                let lang_str = language.to_string();
                match Lang::new(language) {
                    Err(e) => Err(RDFError::LanguageTagError {
                        literal: value_str,
                        language: lang_str,
                        error: e.to_string(),
                    }),
                    Ok(lang) => Ok(SLiteral::lang_str(&s, lang)),
                }
            }
            (value, Some(dtype), None, None) => {
                let xsd_double = oxrdf::vocab::xsd::DOUBLE.to_owned();
                let xsd_integer = oxrdf::vocab::xsd::INTEGER.to_owned();
                let xsd_long = oxrdf::vocab::xsd::LONG.to_owned();
                let xsd_decimal = oxrdf::vocab::xsd::DECIMAL.to_owned();
                let xsd_float = oxrdf::vocab::xsd::FLOAT.to_owned();
                let xsd_datetime = oxrdf::vocab::xsd::DATE_TIME.to_owned();
                let xsd_byte = oxrdf::vocab::xsd::BYTE.to_owned();
                let xsd_boolean = oxrdf::vocab::xsd::BOOLEAN.to_owned();
                match &dtype {
                    d if *d == xsd_boolean => match SLiteral::parse_bool(&value) {
                        Ok(b) => Ok(SLiteral::BooleanLiteral(b)),
                        Err(e) => {
                            let datatype = IriRef::iri(IriS::new_unchecked(dtype.as_str()));
                            Ok(SLiteral::WrongDatatypeLiteral {
                                lexical_form: value,
                                datatype,
                                error: e.to_string(),
                            })
                        }
                    },
                    d if *d == xsd_double => match SLiteral::parse_double(&value) {
                        Ok(double_value) => Ok(SLiteral::double(double_value)),
                        Err(e) => {
                            let datatype = IriRef::iri(IriS::new_unchecked(dtype.as_str()));
                            Ok(SLiteral::WrongDatatypeLiteral {
                                lexical_form: value,
                                datatype,
                                error: e.to_string(),
                            })
                        }
                    },
                    d if *d == xsd_decimal => match SLiteral::parse_decimal(&value) {
                        Ok(num_value) => Ok(SLiteral::decimal(num_value)),
                        Err(e) => {
                            let datatype = IriRef::iri(IriS::new_unchecked(dtype.as_str()));
                            Ok(SLiteral::WrongDatatypeLiteral {
                                lexical_form: value,
                                datatype,
                                error: e.to_string(),
                            })
                        }
                    },
                    d if *d == xsd_float => match SLiteral::parse_float(&value) {
                        Ok(num_value) => Ok(SLiteral::float(num_value)),
                        Err(e) => {
                            let datatype = IriRef::iri(IriS::new_unchecked(dtype.as_str()));
                            Ok(SLiteral::WrongDatatypeLiteral {
                                lexical_form: value,
                                datatype,
                                error: e.to_string(),
                            })
                        }
                    },
                    d if *d == xsd_long => match SLiteral::parse_long(&value) {
                        Ok(num_value) => {
                            Ok(SLiteral::NumericLiteral(NumericLiteral::long(num_value)))
                        }
                        Err(e) => {
                            let datatype = IriRef::iri(IriS::new_unchecked(dtype.as_str()));
                            Ok(SLiteral::WrongDatatypeLiteral {
                                lexical_form: value,
                                datatype,
                                error: e.to_string(),
                            })
                        }
                    },
                    d if *d == xsd_integer => match SLiteral::parse_integer(&value) {
                        Ok(num_value) => Ok(SLiteral::integer(num_value)),
                        Err(e) => {
                            let datatype = IriRef::iri(IriS::new_unchecked(dtype.as_str()));
                            Ok(SLiteral::WrongDatatypeLiteral {
                                lexical_form: value,
                                datatype,
                                error: e.to_string(),
                            })
                        }
                    },
                    d if *d == xsd_byte => match SLiteral::parse_byte(&value) {
                        Ok(num_value) => Ok(SLiteral::byte(num_value)),
                        Err(e) => {
                            let datatype = IriRef::iri(IriS::new_unchecked(dtype.as_str()));
                            Ok(SLiteral::WrongDatatypeLiteral {
                                lexical_form: value,
                                datatype,
                                error: e.to_string(),
                            })
                        }
                    },
                    d if *d == xsd_datetime => match XsdDateTime::new(&value) {
                        Ok(date_time) => Ok(SLiteral::DatetimeLiteral(date_time)),
                        Err(e) => {
                            let datatype = IriRef::iri(IriS::new_unchecked(dtype.as_str()));
                            Ok(SLiteral::WrongDatatypeLiteral {
                                lexical_form: value,
                                datatype,
                                error: e.to_string(),
                            })
                        }
                    },
                    _ => {
                        let datatype = IriRef::iri(IriS::new_unchecked(dtype.as_str()));
                        Ok(SLiteral::lit_datatype(&value, &datatype))
                    }
                }
            }
            _ => Err(RDFError::ConversionError {
                msg: "Unknwon literal value: {value}".to_string(),
            }),
        }
    }
}

impl From<SLiteral> for oxrdf::Literal {
    fn from(value: SLiteral) -> Self {
        match value {
            SLiteral::StringLiteral { lexical_form, lang } => match lang {
                Some(lang) => oxrdf::Literal::new_language_tagged_literal_unchecked(
                    lexical_form,
                    lang.to_string(),
                ),
                None => lexical_form.clone().into(),
            },
            SLiteral::DatatypeLiteral {
                lexical_form,
                datatype,
            } => match datatype.get_iri() {
                Ok(datatype) => oxrdf::Literal::new_typed_literal(
                    lexical_form,
                    datatype.as_named_node().to_owned(),
                ),
                Err(_) => lexical_form.clone().into(),
            },
            SLiteral::NumericLiteral(number) => From::<NumericLiteral>::from(number),
            SLiteral::BooleanLiteral(bool) => bool.into(),
            SLiteral::DatetimeLiteral(date_time) => (*date_time.value()).into(),
            SLiteral::WrongDatatypeLiteral {
                lexical_form,
                datatype,
                ..
            } => match datatype.get_iri() {
                Ok(datatype) => oxrdf::Literal::new_typed_literal(
                    lexical_form,
                    datatype.as_named_node().to_owned(),
                ),
                Err(_) => lexical_form.into(),
            },
        }
    }
}

impl From<&SLiteral> for oxrdf::Literal {
    fn from(value: &SLiteral) -> Self {
        oxrdf::Literal::from(value.clone())
    }
}

impl From<&SLiteral> for Object {
    fn from(value: &SLiteral) -> Self {
        Object::Literal(value.clone())
    }
}

fn check_literal_datatype(lexical_form: &str, datatype: &IriRef) -> Result<SLiteral, RDFError> {
    trace!("check_literal_datatype: {lexical_form}^^{datatype}");
    let iri = datatype.get_iri().map_err(|_e| RDFError::IriRefError {
        iri_ref: datatype.to_string(),
    })?;
    match iri.as_str() {
        "http://www.w3.org/2001/XMLSchema#integer" => match SLiteral::parse_integer(lexical_form) {
            Ok(n) => Ok(SLiteral::integer(n)),
            Err(err) => Ok(SLiteral::WrongDatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
                error: err.to_string(),
            }),
        },
        "http://www.w3.org/2001/XMLSchema#long" => match SLiteral::parse_long(lexical_form) {
            Ok(n) => Ok(SLiteral::long(n)),
            Err(err) => Ok(SLiteral::WrongDatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
                error: err.to_string(),
            }),
        },
        "http://www.w3.org/2001/XMLSchema#double" => match SLiteral::parse_double(lexical_form) {
            Ok(d) => Ok(SLiteral::double(d)),
            Err(err) => Ok(SLiteral::WrongDatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
                error: err.to_string(),
            }),
        },
        "http://www.w3.org/2001/XMLSchema#boolean" => match SLiteral::parse_bool(lexical_form) {
            Ok(b) => Ok(SLiteral::boolean(b)),
            Err(err) => Ok(SLiteral::WrongDatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
                error: err.to_string(),
            }),
        },
        "http://www.w3.org/2001/XMLSchema#float" => match SLiteral::parse_float(lexical_form) {
            Ok(d) => Ok(SLiteral::float(d)),
            Err(err) => Ok(SLiteral::WrongDatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
                error: err.to_string(),
            }),
        },
        "http://www.w3.org/2001/XMLSchema#decimal" => match SLiteral::parse_decimal(lexical_form) {
            Ok(d) => Ok(SLiteral::decimal(d)),
            Err(err) => Ok(SLiteral::WrongDatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
                error: err.to_string(),
            }),
        },
        "http://www.w3.org/2001/XMLSchema#negativeInteger" => {
            match SLiteral::parse_negative_integer(lexical_form) {
                Ok(d) => Ok(SLiteral::negative_integer(d)),
                Err(err) => Ok(SLiteral::WrongDatatypeLiteral {
                    lexical_form: lexical_form.to_string(),
                    datatype: datatype.clone(),
                    error: err.to_string(),
                }),
            }
        }
        "http://www.w3.org/2001/XMLSchema#positiveInteger" => {
            match SLiteral::parse_positive_integer(lexical_form) {
                Ok(d) => Ok(SLiteral::positive_integer(d)),
                Err(err) => Ok(SLiteral::WrongDatatypeLiteral {
                    lexical_form: lexical_form.to_string(),
                    datatype: datatype.clone(),
                    error: err.to_string(),
                }),
            }
        }
        "http://www.w3.org/2001/XMLSchema#nonNegativeInteger" => {
            match SLiteral::parse_non_negative_integer(lexical_form) {
                Ok(d) => Ok(SLiteral::non_negative_integer(d)),
                Err(err) => Ok(SLiteral::WrongDatatypeLiteral {
                    lexical_form: lexical_form.to_string(),
                    datatype: datatype.clone(),
                    error: err.to_string(),
                }),
            }
        }
        "http://www.w3.org/2001/XMLSchema#nonPositiveInteger" => {
            match SLiteral::parse_non_positive_integer(lexical_form) {
                Ok(d) => Ok(SLiteral::non_positive_integer(d)),
                Err(err) => Ok(SLiteral::WrongDatatypeLiteral {
                    lexical_form: lexical_form.to_string(),
                    datatype: datatype.clone(),
                    error: err.to_string(),
                }),
            }
        }
        "http://www.w3.org/2001/XMLSchema#unsignedInt" => {
            match SLiteral::parse_unsigned_int(lexical_form) {
                Ok(d) => Ok(SLiteral::unsigned_int(d)),
                Err(err) => Ok(SLiteral::WrongDatatypeLiteral {
                    lexical_form: lexical_form.to_string(),
                    datatype: datatype.clone(),
                    error: err.to_string(),
                }),
            }
        }
        "http://www.w3.org/2001/XMLSchema#unsignedLong" => {
            match SLiteral::parse_unsigned_long(lexical_form) {
                Ok(d) => Ok(SLiteral::unsigned_long(d)),
                Err(err) => Ok(SLiteral::WrongDatatypeLiteral {
                    lexical_form: lexical_form.to_string(),
                    datatype: datatype.clone(),
                    error: err.to_string(),
                }),
            }
        }
        "http://www.w3.org/2001/XMLSchema#unsignedByte" => {
            match SLiteral::parse_unsigned_byte(lexical_form) {
                Ok(d) => Ok(SLiteral::unsigned_byte(d)),
                Err(err) => Ok(SLiteral::WrongDatatypeLiteral {
                    lexical_form: lexical_form.to_string(),
                    datatype: datatype.clone(),
                    error: err.to_string(),
                }),
            }
        }
        "http://www.w3.org/2001/XMLSchema#unsignedShort" => {
            match SLiteral::parse_unsigned_short(lexical_form) {
                Ok(d) => Ok(SLiteral::unsigned_short(d)),
                Err(err) => Ok(SLiteral::WrongDatatypeLiteral {
                    lexical_form: lexical_form.to_string(),
                    datatype: datatype.clone(),
                    error: err.to_string(),
                }),
            }
        }

        _ => {
            // For other datatypes, we do not check the lexical form
            // We assume it is correct
            // This includes rdf:langString
            trace!("Not checking datatype {iri}");
            Ok(SLiteral::DatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned_long() {
        let str = "-1";
        let result = SLiteral::parse_unsigned_long(str);
        assert!(result.is_err());
    }
}
