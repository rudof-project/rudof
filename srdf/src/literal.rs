use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::result;

use crate::RDFError;
use crate::XsdDateTime;
use crate::{lang::Lang, numeric_literal::NumericLiteral};
use iri_s::IriS;
use prefixmap::{Deref, DerefError, IriRef, PrefixMap};
use rust_decimal::{prelude::ToPrimitive, Decimal};
use serde::{Deserialize, Serialize, Serializer};

pub trait Literal: Debug + Clone + Display + PartialEq + Eq + Hash {
    fn lexical_form(&self) -> &str;

    fn lang(&self) -> Option<&str>;

    fn datatype(&self) -> &str;

    fn as_bool(&self) -> Option<bool> {
        if self.datatype() == "http://www.w3.org/2001/XMLSchema#boolean" {
            match self.lexical_form() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            }
        } else {
            None
        }
    }

    fn as_integer(&self) -> Option<isize> {
        if self.datatype() == "http://www.w3.org/2001/XMLSchema#integer" {
            self.lexical_form().parse().ok()
        } else {
            None
        }
    }

    fn as_date_time(&self) -> Option<XsdDateTime> {
        if self.datatype() == "http://www.w3.org/2001/XMLSchema#dateTime" {
            XsdDateTime::new(self.lexical_form()).ok()
        } else {
            None
        }
    }

    fn as_double(&self) -> Option<f64> {
        if self.datatype() == "http://www.w3.org/2001/XMLSchema#double" {
            self.lexical_form().parse().ok()
        } else {
            None
        }
    }

    fn as_decimal(&self) -> Option<Decimal> {
        if self.datatype() == "http://www.w3.org/2001/XMLSchema#decimal" {
            self.lexical_form().parse().ok()
        } else {
            None
        }
    }

    fn as_literal(&self) -> SLiteral {
        if let Some(bool) = self.as_bool() {
            SLiteral::boolean(bool)
        } else if let Some(int) = self.as_integer() {
            SLiteral::integer(int)
        } else if let Some(decimal) = self.as_double() {
            SLiteral::double(decimal)
        } else if let Some(decimal) = self.as_decimal() {
            SLiteral::decimal(decimal)
        } else if let Some(date_time) = self.as_date_time() {
            SLiteral::DatetimeLiteral(date_time)
        } else if let Some(lang) = self.lang() {
            SLiteral::lang_str(self.lexical_form(), Lang::new_unchecked(lang))
        } else {
            SLiteral::str(self.lexical_form())
        }
    }
}

/// Concrete representation of RDF literals
/// This representation internally uses integers or doubles to represent numeric values
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

    pub fn lit_datatype(lexical_form: &str, datatype: &IriRef) -> SLiteral {
        SLiteral::DatatypeLiteral {
            lexical_form: lexical_form.to_owned(),
            datatype: datatype.clone(),
        }
    }

    pub fn boolean(b: bool) -> SLiteral {
        SLiteral::BooleanLiteral(b)
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

    pub fn lexical_form(&self) -> String {
        match self {
            SLiteral::StringLiteral { lexical_form, .. } => lexical_form.clone(),
            SLiteral::DatatypeLiteral { lexical_form, .. } => lexical_form.clone(),
            SLiteral::NumericLiteral(nl) => nl.lexical_form(),
            SLiteral::BooleanLiteral(true) => "true".to_string(),
            SLiteral::BooleanLiteral(false) => "false".to_string(),
            SLiteral::DatetimeLiteral(dt) => dt.to_string(),
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
            SLiteral::NumericLiteral(NumericLiteral::Integer(_)) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"),
            ),
            SLiteral::NumericLiteral(NumericLiteral::Decimal(_)) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#decimal"),
            ),
            SLiteral::NumericLiteral(NumericLiteral::Double(_)) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#double"),
            ),
            SLiteral::BooleanLiteral(_) => IriRef::iri(IriS::new_unchecked(
                "http://www.w3.org/2001/XMLSchema#boolean",
            )),
            SLiteral::DatetimeLiteral(_) => IriRef::iri(IriS::new_unchecked(
                "http://www.w3.org/2001/XMLSchema#dateTime",
            )),
        }
    }

    pub fn numeric_value(&self) -> Option<NumericLiteral> {
        match self {
            SLiteral::NumericLiteral(nl) => Some(nl.clone()),
            SLiteral::StringLiteral { .. }
            | SLiteral::DatatypeLiteral { .. }
            | SLiteral::BooleanLiteral(true)
            | SLiteral::BooleanLiteral(false) => None,
            SLiteral::DatetimeLiteral(_) => None,
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
        }
    }
}

impl TryFrom<oxrdf::Literal> for SLiteral {
    type Error = RDFError;

    fn try_from(value: oxrdf::Literal) -> Result<Self, Self::Error> {
        match value.destruct() {
            (s, None, None) => Ok(SLiteral::str(&s)),
            (s, None, Some(language)) => Ok(SLiteral::lang_str(&s, Lang::new_unchecked(&language))),
            (value, Some(dtype), None) => {
                let xsd_double = oxrdf::vocab::xsd::DOUBLE.to_owned();
                let xsd_integer = oxrdf::vocab::xsd::INTEGER.to_owned();
                let xsd_decimal = oxrdf::vocab::xsd::DECIMAL.to_owned();
                let xsd_datetime = oxrdf::vocab::xsd::DATE_TIME.to_owned();
                match dtype {
                    d if d == xsd_double => {
                        let double_value: f64 =
                            value.parse().map_err(|_| RDFError::ConversionError {
                                msg: format!("Failed to parse double from value: {value}"),
                            })?;
                        Ok(SLiteral::NumericLiteral(NumericLiteral::double(
                            double_value,
                        )))
                    }
                    d if d == xsd_decimal => {
                        let num_value: Decimal =
                            value.parse().map_err(|_| RDFError::ConversionError {
                                msg: format!("Failed to parse decimal from value: {value}"),
                            })?;
                        Ok(SLiteral::NumericLiteral(NumericLiteral::decimal(num_value)))
                    }
                    d if d == xsd_integer => {
                        let num_value: isize =
                            value.parse().map_err(|_| RDFError::ConversionError {
                                msg: format!("Failed to parse integer from value: {value}"),
                            })?;
                        Ok(SLiteral::NumericLiteral(NumericLiteral::integer(num_value)))
                    }
                    d if d == xsd_datetime => {
                        let date_time =
                            XsdDateTime::new(&value).map_err(|e| RDFError::ConversionError {
                                msg: format!(
                                    "Failed to parse datetime from value: {value}, error: {e}"
                                ),
                            })?;
                        Ok(SLiteral::DatetimeLiteral(date_time))
                    }
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
            SLiteral::NumericLiteral(number) => match number {
                NumericLiteral::Integer(int) => (int as i64).into(),
                NumericLiteral::Decimal(decimal) => match decimal.to_f64() {
                    Some(decimal) => decimal.into(),
                    None => decimal.to_string().into(),
                },
                NumericLiteral::Double(double) => double.into(),
            },
            SLiteral::BooleanLiteral(bool) => bool.into(),
            SLiteral::DatetimeLiteral(date_time) => (*date_time.value()).into(),
        }
    }
}
