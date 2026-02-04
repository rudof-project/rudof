use super::{
    ObjectValue, iri_ref_or_wildcard::IriRefOrWildcard, string_or_wildcard::StringOrWildcard,
};
use crate::LangOrWildcard;
use crate::exclusion::Exclusion;
use crate::iri_exclusion::IriExclusion;
use crate::language_exclusion::LanguageExclusion;
use crate::literal_exclusion::LiteralExclusion;
use iri_s::error::IriSError;
use prefixmap::error::DerefError;
use prefixmap::{Deref, IriRef};
use rust_decimal::Decimal;
use serde::ser::SerializeMap;
use serde::{
    Deserialize, Serialize, Serializer,
    de::{self, MapAccess, Unexpected, Visitor},
};
use srdf::SLiteral;
use srdf::lang::Lang;
use std::{fmt, str::FromStr};
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum ValueSetValue {
    IriStem {
        stem: IriRef,
    },
    IriStemRange {
        stem: IriRefOrWildcard,
        exclusions: Option<Vec<IriExclusion>>,
    },
    LiteralStem {
        stem: String,
    },
    LiteralStemRange {
        stem: StringOrWildcard,
        exclusions: Option<Vec<LiteralExclusion>>,
    },
    Language {
        language_tag: Lang,
    },
    LanguageStem {
        stem: LangOrWildcard,
    },
    LanguageStemRange {
        stem: LangOrWildcard,
        exclusions: Option<Vec<LanguageExclusion>>,
    },

    ObjectValue(ObjectValue),
}

impl ValueSetValue {
    pub fn iri(iri: IriRef) -> ValueSetValue {
        ValueSetValue::ObjectValue(ObjectValue::iri_ref(iri))
    }

    pub fn string_literal(value: &str, lang: Option<Lang>) -> ValueSetValue {
        let ov = ObjectValue::Literal(SLiteral::StringLiteral {
            lexical_form: value.to_string(),
            lang,
        });
        ValueSetValue::ObjectValue(ov)
    }

    pub fn datatype_literal(value: &str, type_: &IriRef) -> ValueSetValue {
        let ov = ObjectValue::datatype_literal(value, type_);
        ValueSetValue::ObjectValue(ov)
    }

    pub fn object_value(value: ObjectValue) -> ValueSetValue {
        ValueSetValue::ObjectValue(value)
    }

    pub fn language(lang: Lang) -> ValueSetValue {
        ValueSetValue::Language { language_tag: lang }
    }

    pub fn language_stem(lang: Lang) -> ValueSetValue {
        ValueSetValue::LanguageStem {
            stem: LangOrWildcard::lang(lang),
        }
    }

    pub fn literal_stem(stem: String) -> ValueSetValue {
        ValueSetValue::LiteralStem { stem }
    }

    pub fn string_stem_range(str: String, exc: Vec<LiteralExclusion>) -> ValueSetValue {
        ValueSetValue::LiteralStemRange {
            stem: StringOrWildcard::String(str),
            exclusions: Some(exc),
        }
    }
}

impl Deref for ValueSetValue {
    fn deref(
        self,
        base: Option<&iri_s::IriS>,
        prefixmap: Option<&prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        match self {
            ValueSetValue::ObjectValue(ov) => {
                let ov = ov.deref(base, prefixmap)?;
                Ok(ValueSetValue::ObjectValue(ov))
            }
            ValueSetValue::Language { language_tag } => Ok(ValueSetValue::Language {
                language_tag: language_tag.clone(),
            }),
            ValueSetValue::LanguageStem { stem } => {
                Ok(ValueSetValue::LanguageStem { stem: stem.clone() })
            }
            ValueSetValue::IriStem { stem } => Ok(ValueSetValue::IriStem { stem: stem.clone() }),
            ValueSetValue::IriStemRange { stem, exclusions } => Ok(ValueSetValue::IriStemRange {
                stem: stem.clone(),
                exclusions: exclusions.clone(),
            }),
            ValueSetValue::LanguageStemRange { stem, exclusions } => {
                Ok(ValueSetValue::LanguageStemRange {
                    stem: stem.clone(),
                    exclusions: exclusions.clone(),
                })
            }
            ValueSetValue::LiteralStem { stem } => {
                Ok(ValueSetValue::LiteralStem { stem: stem.clone() })
            }
            ValueSetValue::LiteralStemRange { stem, exclusions } => {
                Ok(ValueSetValue::LiteralStemRange {
                    stem: stem.clone(),
                    exclusions: exclusions.clone(),
                })
            }
        }
    }
}

/*impl SerializeStringOrStruct for ValueSetValue {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            ValueSetValue::ObjectValue(ObjectValue::Iri(ref r)) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}*/

impl FromStr for ValueSetValue {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iri_ref = IriRef::try_from(s)?;
        Ok(ValueSetValue::ObjectValue(ObjectValue::iri_ref(iri_ref)))
    }
}

// TODO: Technical debt
// The code for serialize/deserialize value_set_value embeds the code to serialize/deserialize object value
// I would prefer to reuse the code from object_value here...

#[derive(Debug, PartialEq)]
enum ValueSetValueType {
    IriStem,
    LanguageStem,
    LiteralStem,
    IriStemRange,
    LanguageStemRange,
    LiteralStemRange,
    Language,
    Boolean,
    Integer,
    Decimal,
    Double,
    Other(IriRef),
}

const BOOLEAN_STR: &str = "http://www.w3.org/2001/XMLSchema#boolean";
const INTEGER_STR: &str = "http://www.w3.org/2001/XMLSchema#integer";
const DOUBLE_STR: &str = "http://www.w3.org/2001/XMLSchema#double";
const DECIMAL_STR: &str = "http://www.w3.org/2001/XMLSchema#decimal";

impl ValueSetValueType {
    fn parse(s: &str) -> Result<ValueSetValueType, IriSError> {
        match s {
            "IriStem" => Ok(ValueSetValueType::IriStem),
            "LanguageStem" => Ok(ValueSetValueType::LanguageStem),
            "LiteralStem" => Ok(ValueSetValueType::LiteralStem),
            "Language" => Ok(ValueSetValueType::Language),
            "IriStemRange" => Ok(ValueSetValueType::IriStemRange),
            "LanguageStemRange" => Ok(ValueSetValueType::LanguageStemRange),
            "LiteralStemRange" => Ok(ValueSetValueType::LiteralStemRange),
            BOOLEAN_STR => Ok(ValueSetValueType::Boolean),
            DECIMAL_STR => Ok(ValueSetValueType::Decimal),
            DOUBLE_STR => Ok(ValueSetValueType::Double),
            INTEGER_STR => Ok(ValueSetValueType::Integer),
            other => {
                let iri = FromStr::from_str(other)?;
                Ok(ValueSetValueType::Other(iri))
            }
        }
    }
}

//const BOOLEAN_STR: &str = "http://www.w3.org/2001/XMLSchema#boolean";
//const INTEGER_STR: &str = "http://www.w3.org/2001/XMLSchema#integer";
//const DOUBLE_STR: &str = "http://www.w3.org/2001/XMLSchema#double";
//const DECIMAL_STR: &str = "http://www.w3.org/2001/XMLSchema#decimal";

impl Serialize for ValueSetValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ValueSetValue::ObjectValue(v) => v.serialize(serializer),
            ValueSetValue::Language { language_tag } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Language")?;
                map.serialize_entry("languageTag", &language_tag)?;
                map.end()
            }
            ValueSetValue::IriStem { stem } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "IriStem")?;
                map.serialize_entry("stem", stem)?;
                map.end()
            }
            ValueSetValue::IriStemRange { stem, exclusions } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "IriStemRange")?;
                map.serialize_entry("stem", stem)?;
                map.serialize_entry("exclusions", exclusions)?;
                map.end()
            }
            ValueSetValue::LanguageStem { stem } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LanguageStem")?;
                map.serialize_entry("stem", &stem)?;
                map.end()
            }
            ValueSetValue::LanguageStemRange { stem, exclusions } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LanguageStemRange")?;
                map.serialize_entry("stem", stem)?;
                map.serialize_entry("exclusions", exclusions)?;
                map.end()
            }
            ValueSetValue::LiteralStem { stem } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LiteralStem")?;
                map.serialize_entry("stem", stem)?;
                map.end()
            }
            ValueSetValue::LiteralStemRange { stem, exclusions } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LiteralStemRange")?;
                map.serialize_entry("stem", stem)?;
                map.serialize_entry("exclusions", exclusions)?;
                map.end()
            }
        }
    }
}

#[derive(Serialize, Debug, Clone)]
enum Stem {
    Str(String),
    Wildcard,
}

#[derive(Debug, Error)]
enum StemError {
    //#[error("Stem is no string or wildcard: {stem:?}")]
    //NoStringOrWildCard { stem: Stem },
    #[error("Stem is no IriRef or wildcard. Stem: {stem:?}, IriError: {err}")]
    NoIriRefOrWildCard { stem: Stem, err: IriSError },

    //#[error("Stem is no lang or wildcard: {stem:?}")]
    //NoLangOrWildcard { stem: Stem },
    #[error("Stem should be IriRef but is wildcard")]
    StemAsIriRefIsWildcard,

    #[error("Error obtaining IRI for stem: {stem:?} Error: {err}")]
    IriError { stem: Stem, err: IriSError },

    #[error("Stem is no language: {stem:?}")]
    NoLanguage { stem: Stem },

    #[error("Stem can not act as string: {stem:?}")]
    NoString { stem: Stem },

    #[error("Stem can not act as lang: {str}")]
    NoLang { str: String },
}

impl Stem {
    fn as_iri(&self) -> Result<IriRef, StemError> {
        match self {
            Stem::Str(s) => {
                let iri_ref = IriRef::from_str(s.as_str()).map_err(|e| StemError::IriError {
                    stem: self.clone(),
                    err: e,
                })?;
                Ok(iri_ref)
            }
            _ => Err(StemError::StemAsIriRefIsWildcard),
        }
    }

    fn as_language(&self) -> Result<String, StemError> {
        match self {
            Stem::Str(s) => Ok(s.clone()),
            _ => Err(StemError::NoLanguage { stem: self.clone() }),
        }
    }

    fn as_string(&self) -> Result<String, StemError> {
        match self {
            Stem::Str(s) => Ok(s.clone()),
            _ => Err(StemError::NoString { stem: self.clone() }),
        }
    }

    fn as_string_or_wildcard(&self) -> Result<StringOrWildcard, StemError> {
        match self {
            Stem::Str(s) => Ok(StringOrWildcard::String(s.clone())),
            Stem::Wildcard => Ok(StringOrWildcard::Wildcard),
        }
    }

    fn as_iri_or_wildcard(&self) -> Result<IriRefOrWildcard, StemError> {
        match self {
            Stem::Str(s) => {
                let iri = FromStr::from_str(s).map_err(|err| StemError::NoIriRefOrWildCard {
                    stem: self.clone(),
                    err,
                })?;
                Ok(IriRefOrWildcard::IriRef(IriRef::iri(iri)))
            }
            Stem::Wildcard => Ok(IriRefOrWildcard::Wildcard),
        }
    }

    fn as_lang_or_wildcard(&self) -> Result<LangOrWildcard, StemError> {
        match self {
            Stem::Str(s) => {
                if s.is_empty() {
                    return Ok(LangOrWildcard::Wildcard);
                }
                let lang =
                    Lang::new(s.as_str()).map_err(|_e| StemError::NoLang { str: s.clone() })?;
                Ok(LangOrWildcard::Lang(lang))
            }
            Stem::Wildcard => Ok(LangOrWildcard::Wildcard),
        }
    }
}

impl<'de> Deserialize<'de> for Stem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Type,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("stem value or wildcard with `type`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "type" => Ok(Field::Type),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct StemVisitor;

        const FIELDS: &[&str] = &["type"];

        impl<'de> Visitor<'de> for StemVisitor {
            type Value = Stem;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("stem value")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Stem::Str(s.to_string()))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Stem, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut type_: Option<StemType> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if type_.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            let value: String = map.next_value()?;

                            let parsed_type_ = StemType::parse(value.as_str()).map_err(|e| {
                                de::Error::custom(format!(
                                    "Error parsing stem type, found: {value}. Error: {e:?}"
                                ))
                            })?;
                            type_ = Some(parsed_type_);
                        }
                    }
                }
                match type_ {
                    Some(StemType::Wildcard) => Ok(Stem::Wildcard),
                    _ => Err(de::Error::custom("Unknown stem type")),
                }
            }
        }
        deserializer.deserialize_any(StemVisitor)
    }
}

enum StemType {
    // Str,
    Wildcard,
}

#[derive(Debug)]
#[allow(dead_code)]
struct StemTypeError {
    stem_type: String,
}

impl StemType {
    fn parse(s: &str) -> Result<StemType, StemTypeError> {
        match s {
            "Wildcard" => Ok(StemType::Wildcard),
            _ => Err(StemTypeError {
                stem_type: s.to_string(),
            }),
        }
    }
}

impl<'de> Deserialize<'de> for ValueSetValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Type,
            Value,
            Language,
            Stem,
            Exclusions,
            LanguageTag,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(
                                "field of value set value: `type` or `value` or `language` or `stem` or `exclusions`",
                            )
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "type" => Ok(Field::Type),
                            "value" => Ok(Field::Value),
                            "stem" => Ok(Field::Stem),
                            "language" => Ok(Field::Language),
                            "languageTag" => Ok(Field::LanguageTag),
                            "exclusions" => Ok(Field::Exclusions),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ValueSetValueVisitor;

        const FIELDS: &[&str] = &[
            "type",
            "value",
            "stem",
            "language",
            "languageTag",
            "exclusions",
        ];

        impl<'de> Visitor<'de> for ValueSetValueVisitor {
            type Value = ValueSetValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ValueSet value")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                FromStr::from_str(s)
                    .map_err(|e| de::Error::custom(format!("Error parsing string `{s}`: {e}")))
            }

            fn visit_map<V>(self, mut map: V) -> Result<ValueSetValue, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut type_: Option<ValueSetValueType> = None;
                let mut stem: Option<Stem> = None;
                let mut value: Option<String> = None;
                let mut language_tag: Option<String> = None;
                let mut language: Option<String> = None;
                let mut exclusions: Option<Vec<Exclusion>> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if type_.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            let value: String = map.next_value()?;
                            let parsed_type_ =
                                ValueSetValueType::parse(value.as_str()).map_err(|e| {
                                    de::Error::custom(format!(
                                    "Error parsing ValueSetValue type, found: {value}. Error: {e}"
                                ))
                                })?;
                            type_ = Some(parsed_type_);
                        }
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                        Field::Language => {
                            if language.is_some() {
                                return Err(de::Error::duplicate_field("language"));
                            }
                            language = Some(map.next_value()?);
                        }
                        Field::Stem => {
                            if stem.is_some() {
                                return Err(de::Error::duplicate_field("stem"));
                            }
                            stem = Some(map.next_value()?);
                        }
                        Field::Exclusions => {
                            if exclusions.is_some() {
                                return Err(de::Error::duplicate_field("exclusions"));
                            }
                            exclusions = Some(map.next_value()?);
                        }
                        Field::LanguageTag => {
                            if language_tag.is_some() {
                                return Err(de::Error::duplicate_field("languageTag"));
                            }
                            language_tag = Some(map.next_value()?);
                        }
                    }
                }
                match type_ {
                    Some(ValueSetValueType::LiteralStemRange) => match stem {
                        Some(stem) => match exclusions {
                            Some(excs) => {
                                let lit_excs = Exclusion::parse_literal_exclusions(excs).map_err(|e| {
                                    de::Error::custom(format!("LiteralStemRange: some exclusions are not literal exclusions: {e:?}"))
                                })?;
                                let stem = stem.as_string_or_wildcard().map_err(|e| {
                                    de::Error::custom(format!("LiteralStemRange: stem is not string or wildcard. stem `{stem:?}`: {e:?}"))
                                })?;
                                Ok(ValueSetValue::LiteralStemRange {
                                    stem,
                                    exclusions: Some(lit_excs),
                                })
                            }
                            None => {
                                todo!()
                            }
                        },
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ValueSetValueType::LanguageStemRange) => match stem {
                        Some(stem) => match exclusions {
                            Some(excs) => {
                                let lang_excs = Exclusion::parse_language_exclusions(excs).map_err(|e| {
                                    de::Error::custom(format!("LanguageStemRange: some exclusions are not Lang exclusions: {e:?}"))
                                })?;
                                let stem = stem.as_lang_or_wildcard().map_err(|e| {
                                    de::Error::custom(format!("LanguageStemRange: stem is not lang or wildcard. stem `{stem:?}`: {e:?}"))
                                })?;
                                Ok(ValueSetValue::LanguageStemRange {
                                    stem,
                                    exclusions: Some(lang_excs),
                                })
                            }
                            None => Err(de::Error::missing_field("exclusions")),
                        },
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ValueSetValueType::IriStemRange) => match stem {
                        Some(stem) => match exclusions {
                            Some(excs) => {
                                let iri_excs = Exclusion::parse_iri_exclusions(excs).map_err(|e| {
                                    de::Error::custom(format!("IriStemRange: some exclusions are not IRI exclusions: {e:?}"))
                                })?;
                                let stem = stem.as_iri_or_wildcard().map_err(|e| {
                                    de::Error::custom(format!("IriStemRange: stem is not string or wildcard. stem `{stem:?}`: {e:?}"))
                                })?;
                                Ok(ValueSetValue::IriStemRange {
                                    stem,
                                    exclusions: Some(iri_excs),
                                })
                            }
                            None => Err(de::Error::missing_field("exclusions")),
                        },
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ValueSetValueType::LiteralStem) => match stem {
                        Some(stem) => {
                            let stem = stem.as_string().map_err(|_e| {
                                de::Error::custom(
                                    "LiteralStem: value of stem must be a string".to_string(),
                                )
                            })?;
                            Ok(ValueSetValue::LiteralStem { stem })
                        }
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ValueSetValueType::LanguageStem) => match stem {
                        Some(stem) => {
                            let stem = stem.as_language().map_err(|e| {
                                de::Error::custom(format!(
                                    "LanguageStem: stem is not a language: {e:?}"
                                ))
                            })?;
                            if stem.is_empty() {
                                return Ok(ValueSetValue::LanguageStem {
                                    stem: LangOrWildcard::wildcard(),
                                });
                            }
                            let lang = Lang::new(&stem).map_err(|e| {
                                de::Error::custom(format!(
                                    "LanguageStem: stem is not a valid language tag: {e:?}"
                                ))
                            })?;
                            Ok(ValueSetValue::LanguageStem {
                                stem: LangOrWildcard::Lang(lang),
                            })
                        }
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ValueSetValueType::Language) => match language_tag {
                        Some(language_tag) => {
                            let lang = Lang::new(&language_tag).map_err(|e| {
                                de::Error::custom(format!(
                                    "LanguageStem: stem is not a valid language tag: {e:?}"
                                ))
                            })?;
                            Ok(ValueSetValue::Language { language_tag: lang })
                        }
                        None => Err(de::Error::missing_field("languageTag")),
                    },
                    Some(ValueSetValueType::IriStem) => match stem {
                        Some(stem) => {
                            let iri_ref = stem.as_iri().map_err(|e| {
                                de::Error::custom(format!(
                                    "Can't parse stem `{stem:?}` as IRIREF for IriStem. Error: {e:?}"
                                ))
                            })?;
                            Ok(ValueSetValue::IriStem { stem: iri_ref })
                        }
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ValueSetValueType::Boolean) => match value {
                        Some(s) => match s.as_str() {
                            "false" => Ok(ValueSetValue::ObjectValue(ObjectValue::bool(false))),
                            "true" => Ok(ValueSetValue::ObjectValue(ObjectValue::bool(true))),
                            _ => Err(de::Error::invalid_value(Unexpected::Str(&s), &self)),
                        },
                        None => Err(de::Error::missing_field("value")),
                    },
                    Some(ValueSetValueType::Double) => match value {
                        Some(s) => {
                            let n = f64::from_str(&s).map_err(|e| {
                                de::Error::custom(format!(
                                    "Can't parse value {s} as double: Error {e}"
                                ))
                            })?;
                            Ok(ValueSetValue::ObjectValue(ObjectValue::double(n)))
                        }
                        None => Err(de::Error::missing_field("value")),
                    },
                    Some(ValueSetValueType::Decimal) => match value {
                        Some(s) => {
                            let n = Decimal::from_str(&s).map_err(|e| {
                                de::Error::custom(format!(
                                    "Can't parse value {s} as decimal: Error {e}"
                                ))
                            })?;
                            let v = ValueSetValue::ObjectValue(ObjectValue::decimal(n));
                            Ok(v)
                        }
                        None => Err(de::Error::missing_field("value")),
                    },
                    Some(ValueSetValueType::Integer) => match value {
                        Some(s) => {
                            let n = isize::from_str(&s).map_err(|e| {
                                de::Error::custom(format!(
                                    "Can't parse value {s} as integer: Error {e}"
                                ))
                            })?;
                            let v = ValueSetValue::ObjectValue(ObjectValue::integer(n));
                            Ok(v)
                        }
                        None => Err(de::Error::missing_field("value")),
                    },
                    Some(ValueSetValueType::Other(iri)) => match value {
                        Some(v) => match language_tag {
                            Some(lang) => {
                                let lang = Lang::new(&lang).map_err(|e| {
                                    de::Error::custom(format!(
                                        "Can't parse language tag {lang} for literal: Error {e}"
                                    ))
                                })?;
                                Ok(ValueSetValue::ObjectValue(ObjectValue::Literal(
                                    SLiteral::StringLiteral {
                                        lexical_form: v,
                                        lang: Some(lang),
                                    },
                                )))
                            }
                            None => Ok(ValueSetValue::datatype_literal(&v, &iri)),
                        },
                        None => Err(de::Error::missing_field("value")),
                    },
                    None => match value {
                        Some(lexical_form) => match language {
                            Some(language) => {
                                let lang = Lang::new(&language).map_err(|e| {
                                    de::Error::custom(format!(
                                        "Can't parse language tag {language} for literal: Error {e}"
                                    ))
                                })?;
                                Ok(ValueSetValue::ObjectValue(ObjectValue::Literal(
                                    SLiteral::StringLiteral {
                                        lexical_form,
                                        lang: Some(lang),
                                    },
                                )))
                            }
                            None => Ok(ValueSetValue::ObjectValue(ObjectValue::Literal(
                                SLiteral::StringLiteral {
                                    lexical_form,
                                    lang: None,
                                },
                            ))),
                        },
                        None => Err(de::Error::missing_field("value")),
                    },
                }
            }
        }
        deserializer.deserialize_any(ValueSetValueVisitor)
    }
}
