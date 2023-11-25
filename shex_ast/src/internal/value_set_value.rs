use crate::{ast::ObjectValueWrapper, internal::ValueSet};
use iri_s::IriS;
use rbe::Value;
use serde::Serializer;
use srdf::{lang::Lang, literal::Literal, Object};
use std::{fmt::Display, result, str::FromStr};
use void::Void;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ValueSetValue {
    IriStem {
        stem: IriS,
    },
    IriStemRange {
        stem: IriRefOrWildcard,
        exclusions: Option<Vec<StringOrIriStem>>,
    },
    LiteralStem {
        stem: String,
    },
    LiteralStemRange {
        stem: StringOrWildcard,
        exclusions: Option<Vec<StringOrLiteralStem>>,
    },
    Language {
        language_tag: String,
    },
    LanguageStem,
    LanguageStemRange,
    ObjectValue(ObjectValue),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum StringOrLiteralStem {
    String(String),
    LiteralStem { stem: String },
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum IriRefOrWildcard {
    IriRef(IriS),
    Wildcard { type_: String },
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum StringOrWildcard {
    String(String),
    Wildcard { type_: String },
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum StringOrIriStem {
    String(String),
    IriStem { stem: String },
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ObjectValue {
    IriRef(IriS),
    ObjectLiteral {
        value: String,
        language: Option<Lang>,
        // type_: Option<String>,
    },
}

impl ObjectValue {
    fn match_value(&self, object: &Object) -> bool {
        match self {
            ObjectValue::IriRef(iri_expected) => match object {
                Object::Iri { iri } => iri == iri_expected,
                _ => false,
            },
            ObjectValue::ObjectLiteral { value, language } => match object {
                Object::Literal(lit) => match lit {
                    Literal::StringLiteral { lexical_form, lang } => {
                        value == lexical_form && language == lang
                    }
                    Literal::DatatypeLiteral {
                        lexical_form,
                        datatype,
                    } => todo!(),
                    Literal::BooleanLiteral(_) => todo!(),
                    Literal::NumericLiteral(_) => todo!(),
                },
                _ => false,
            },
        }
    }
}

impl Display for ObjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectValue::IriRef(iri) => {
                write!(f, "{iri}")?;
                Ok(())
            }
            ObjectValue::ObjectLiteral { value, language } => {
                write!(f, "\"{value}\"")?;
                match language {
                    None => Ok(()),
                    Some(lang) => {
                        write!(f, "@{lang}")?;
                        Ok(())
                    }
                }
            }
        }
    }
}

impl ValueSetValue {
    pub fn match_value(&self, object: &Object) -> bool {
        match self {
            ValueSetValue::IriStem { stem } => todo!(),
            ValueSetValue::IriStemRange { stem, exclusions } => todo!(),
            ValueSetValue::LiteralStem { stem } => todo!(),
            ValueSetValue::LiteralStemRange { stem, exclusions } => todo!(),
            ValueSetValue::Language { language_tag } => todo!(),
            ValueSetValue::LanguageStem => todo!(),
            ValueSetValue::LanguageStemRange => todo!(),
            ValueSetValue::ObjectValue(v) => v.match_value(object),
        }
    }
}

impl Display for ValueSetValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueSetValue::IriStem { stem } => todo!(),
            ValueSetValue::IriStemRange { stem, exclusions } => todo!(),
            ValueSetValue::LiteralStem { stem } => todo!(),
            ValueSetValue::LiteralStemRange { stem, exclusions } => todo!(),
            ValueSetValue::Language { language_tag } => todo!(),
            ValueSetValue::LanguageStem => todo!(),
            ValueSetValue::LanguageStemRange => todo!(),
            ValueSetValue::ObjectValue(ov) => write!(f, "{ov}"),
        }
    }
}
