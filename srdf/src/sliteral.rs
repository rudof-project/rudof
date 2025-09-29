use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::result;

use crate::RDFError;
use crate::XsdDateTime;
use crate::{lang::Lang, numeric_literal::NumericLiteral};
use iri_s::IriS;
use prefixmap::{Deref, DerefError, IriRef, PrefixMap};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize, Serializer};

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
        datatype: IriRef,
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
    pub fn integer(n: isize) -> SLiteral {
        SLiteral::NumericLiteral(NumericLiteral::integer(n))
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
        &self,
        base: &Option<iri_s::IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError> {
        match self {
            SLiteral::NumericLiteral(n) => Ok(SLiteral::NumericLiteral(n.clone())),
            SLiteral::BooleanLiteral(b) => Ok(SLiteral::BooleanLiteral(*b)),
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
        match value.destruct() {
            (s, None, None, None) => Ok(SLiteral::str(&s)),
            (s, None, Some(language), None) => {
                Ok(SLiteral::lang_str(&s, Lang::new_unchecked(&language)))
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
                None => lexical_form.into(),
            },
            SLiteral::DatatypeLiteral {
                lexical_form,
                datatype,
            } => match datatype.get_iri() {
                Ok(datatype) => oxrdf::Literal::new_typed_literal(
                    lexical_form,
                    datatype.as_named_node().to_owned(),
                ),
                Err(_) => lexical_form.into(),
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
