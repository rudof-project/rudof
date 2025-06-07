use std::{fmt::Display, result};

use iri_s::IriS;
use rust_decimal::{prelude::ToPrimitive, Decimal};
use serde::{Deserialize, Serialize, Serializer};

use crate::{lang::Lang, numeric_literal::NumericLiteral};
use prefixmap::{Deref, DerefError, IriRef, PrefixMap};

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub enum Literal {
    StringLiteral {
        lexical_form: String,
        lang: Option<Lang>,
    },
    DatatypeLiteral {
        lexical_form: String,
        datatype: IriRef,
    },
    NumericLiteral(NumericLiteral),

    #[serde(serialize_with = "serialize_boolean_literal")]
    BooleanLiteral(bool),
}

impl Literal {
    pub fn integer(n: isize) -> Literal {
        Literal::NumericLiteral(NumericLiteral::integer(n))
    }

    pub fn double(d: f64) -> Literal {
        Literal::NumericLiteral(NumericLiteral::double(d))
    }

    pub fn decimal(d: Decimal) -> Literal {
        Literal::NumericLiteral(NumericLiteral::decimal(d))
    }

    pub fn lit_datatype(lexical_form: &str, datatype: &IriRef) -> Literal {
        Literal::DatatypeLiteral {
            lexical_form: lexical_form.to_owned(),
            datatype: datatype.clone(),
        }
    }

    pub fn boolean(b: bool) -> Literal {
        Literal::BooleanLiteral(b)
    }

    pub fn str(lexical_form: &str) -> Literal {
        Literal::StringLiteral {
            lexical_form: lexical_form.to_owned(),
            lang: None,
        }
    }

    pub fn lang_str(lexical_form: &str, lang: Lang) -> Literal {
        Literal::StringLiteral {
            lexical_form: lexical_form.to_owned(),
            lang: Some(lang),
        }
    }

    pub fn lexical_form(&self) -> String {
        match self {
            Literal::StringLiteral { lexical_form, .. } => lexical_form.clone(),
            Literal::DatatypeLiteral { lexical_form, .. } => lexical_form.clone(),
            Literal::NumericLiteral(nl) => nl.lexical_form(),
            Literal::BooleanLiteral(true) => "true".to_string(),
            Literal::BooleanLiteral(false) => "false".to_string(),
        }
    }

    pub fn display_qualified(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        prefixmap: &PrefixMap,
    ) -> std::fmt::Result {
        match self {
            Literal::StringLiteral {
                lexical_form,
                lang: None,
            } => write!(f, "\"{lexical_form}\""),
            Literal::StringLiteral {
                lexical_form,
                lang: Some(lang),
            } => write!(f, "\"{lexical_form}\"{lang}"),
            Literal::DatatypeLiteral {
                lexical_form,
                datatype,
            } => match datatype {
                IriRef::Iri(iri) => write!(f, "\"{lexical_form}\"^^{}", prefixmap.qualify(iri)),
                IriRef::Prefixed { prefix, local } => {
                    write!(f, "\"{lexical_form}\"^^{}:{}", prefix, local)
                }
            },
            Literal::NumericLiteral(n) => write!(f, "{}", n),
            Literal::BooleanLiteral(true) => write!(f, "true"),
            Literal::BooleanLiteral(false) => write!(f, "false"),
        }
    }

    pub fn datatype(&self) -> IriRef {
        match self {
            Literal::DatatypeLiteral { datatype, .. } => datatype.clone(),
            Literal::StringLiteral {
                lexical_form: _,
                lang: None,
            } => IriRef::iri(IriS::new_unchecked(
                "http://www.w3.org/2001/XMLSchema#string",
            )),
            Literal::StringLiteral {
                lexical_form: _,
                lang: Some(_),
            } => IriRef::iri(IriS::new_unchecked(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString",
            )),
            Literal::NumericLiteral(NumericLiteral::Integer(_)) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"),
            ),
            Literal::NumericLiteral(NumericLiteral::Decimal(_)) => IriRef::iri(
                IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#decimal"),
            ),
            Literal::NumericLiteral(NumericLiteral::Double(_)) => IriRef::iri(IriS::new_unchecked(
                "http://www.w3.org/2001/XMLSchema#double",
            )),
            Literal::BooleanLiteral(_) => IriRef::iri(IriS::new_unchecked(
                "http://www.w3.org/2001/XMLSchema#boolean",
            )),
        }
    }

    pub fn numeric_value(&self) -> Option<NumericLiteral> {
        match self {
            Literal::NumericLiteral(nl) => Some(nl.clone()),
            Literal::StringLiteral { .. }
            | Literal::DatatypeLiteral { .. }
            | Literal::BooleanLiteral(true)
            | Literal::BooleanLiteral(false) => None,
        }
    }
}

impl Default for Literal {
    fn default() -> Self {
        Literal::StringLiteral {
            lexical_form: String::default(),
            lang: None,
        }
    }
}

impl Display for Literal {
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

impl Deref for Literal {
    fn deref(
        &self,
        base: &Option<iri_s::IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError> {
        match self {
            Literal::NumericLiteral(n) => Ok(Literal::NumericLiteral(n.clone())),
            Literal::BooleanLiteral(b) => Ok(Literal::BooleanLiteral(*b)),
            Literal::StringLiteral { lexical_form, lang } => Ok(Literal::StringLiteral {
                lexical_form: lexical_form.clone(),
                lang: lang.clone(),
            }),
            Literal::DatatypeLiteral {
                lexical_form,
                datatype,
            } => {
                let dt = datatype.deref(base, prefixmap)?;
                Ok(Literal::DatatypeLiteral {
                    lexical_form: lexical_form.clone(),
                    datatype: dt,
                })
            }
        }
    }
}

impl From<oxrdf::Literal> for Literal {
    fn from(value: oxrdf::Literal) -> Self {
        match value.destruct() {
            (s, None, None) => Literal::str(&s),
            (s, None, Some(language)) => Literal::lang_str(&s, Lang::new_unchecked(&language)),
            (value, Some(dtype), None) => {
                let datatype = IriRef::iri(IriS::new_unchecked(dtype.as_str()));
                Literal::lit_datatype(&value, &datatype)
            }
            _ => todo!(),
        }
    }
}

impl From<Literal> for oxrdf::Literal {
    fn from(value: Literal) -> Self {
        match value {
            Literal::StringLiteral { lexical_form, lang } => match lang {
                Some(lang) => oxrdf::Literal::new_language_tagged_literal_unchecked(
                    lexical_form,
                    lang.to_string(),
                ),
                None => lexical_form.into(),
            },
            Literal::DatatypeLiteral {
                lexical_form,
                datatype,
            } => match datatype.get_iri() {
                Ok(datatype) => oxrdf::Literal::new_typed_literal(
                    lexical_form,
                    datatype.as_named_node().to_owned(),
                ),
                Err(_) => lexical_form.into(),
            },
            Literal::NumericLiteral(number) => match number {
                NumericLiteral::Integer(int) => (int as i64).into(),
                NumericLiteral::Decimal(decimal) => match decimal.to_f64() {
                    Some(decimal) => decimal.into(),
                    None => decimal.to_string().into(),
                },
                NumericLiteral::Double(double) => double.into(),
            },
            Literal::BooleanLiteral(bool) => bool.into(),
        }
    }
}
