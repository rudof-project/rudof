use super::object_value::ObjectValue;
use iri_s::IriS;
use srdf::{lang::Lang, Object};
use std::fmt::Display;

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
        language_tag: Lang,
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

impl ValueSetValue {
    pub fn match_value(&self, object: &Object) -> bool {
        match self {
            ValueSetValue::IriStem { stem } => match object {
                Object::Iri(iri_s) => iri_s.as_str().starts_with(stem),
                Object::BlankNode(_) => false,
                Object::Literal(_) => false,
            },
            ValueSetValue::IriStemRange { .. } => todo!(),
            ValueSetValue::LiteralStem { .. } => todo!(),
            ValueSetValue::LiteralStemRange { .. } => todo!(),
            ValueSetValue::Language { language_tag } => match object {
                Object::Iri(_iri_s) => false,
                Object::BlankNode(_) => false,
                Object::Literal(sliteral) => match sliteral {
                    srdf::SLiteral::StringLiteral { lang, .. } => match lang {
                        Some(lang) => language_tag == lang,
                        None => false,
                    },
                    srdf::SLiteral::DatatypeLiteral { .. } => false,
                    srdf::SLiteral::NumericLiteral(_) => false,
                    srdf::SLiteral::DatetimeLiteral(_) => false,
                    srdf::SLiteral::BooleanLiteral(_) => false,
                },
            },
            ValueSetValue::LanguageStem => todo!(),
            ValueSetValue::LanguageStemRange => todo!(),
            ValueSetValue::ObjectValue(v) => v.match_value(object),
        }
    }
}

impl Display for ValueSetValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueSetValue::IriStem { .. } => todo!(),
            ValueSetValue::IriStemRange { .. } => todo!(),
            ValueSetValue::LiteralStem { .. } => todo!(),
            ValueSetValue::LiteralStemRange { .. } => todo!(),
            ValueSetValue::Language { language_tag } => write!(f, "@{language_tag}"),
            ValueSetValue::LanguageStem => todo!(),
            ValueSetValue::LanguageStemRange => todo!(),
            ValueSetValue::ObjectValue(ov) => write!(f, "{ov}"),
        }
    }
}
