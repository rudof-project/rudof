use crate::NumericLiteral;
use crate::{ast::serde_string_or_struct::*, Deref, DerefError, LangOrWildcard};
use iri_s::IriSError;
use serde::ser::SerializeMap;
use serde::{
    de::{self, MapAccess, Unexpected, Visitor},
    Deserialize, Serialize, Serializer,
};
use serde_derive::Serialize;
use srdf::lang::Lang;
use std::{fmt, result, str::FromStr};

use super::{
    iri_ref::IriRef, iri_ref_or_wildcard::IriRefOrWildcard,
    string_or_iri_stem::StringOrIriStemWrapper, string_or_literal_stem::StringOrLiteralStemWrapper,
    string_or_wildcard::StringOrWildcard, ObjectValue,
};

#[derive(Debug, PartialEq, Clone)]
pub enum ValueSetValue {
    IriStem {
        stem: IriRef,
    },
    IriStemRange {
        stem: IriRefOrWildcard,
        exclusions: Option<Vec<StringOrIriStemWrapper>>,
    },
    LiteralStem {
        stem: String,
    },
    LiteralStemRange {
        stem: StringOrWildcard,

        exclusions: Option<Vec<StringOrLiteralStemWrapper>>,
    },
    Language {
        language_tag: Lang,
    },
    LanguageStem {
        stem: Lang,
    },
    LanguageStemRange {
        stem: LangOrWildcard,
        exclusions: Option<Vec<StringOrLiteralStemWrapper>>,
    },

    ObjectValue(ObjectValue),
}

impl ValueSetValue {
    pub fn iri(iri: IriRef) -> ValueSetValue {
        ValueSetValue::ObjectValue(ObjectValue::IriRef(iri))
    }

    pub fn literal(value: &str, language: Option<Lang>, type_: Option<IriRef>) -> ValueSetValue {
        let ov = ObjectValue::ObjectLiteral {
            value: value.to_string(),
            language,
            type_,
        };
        ValueSetValue::ObjectValue(ov)
    }

    pub fn object_value(value: ObjectValue) -> ValueSetValue {
        ValueSetValue::ObjectValue(value)
    }

    pub fn language(lang: Lang) -> ValueSetValue {
        ValueSetValue::Language { language_tag: lang }
    }

    pub fn language_stem(lang: Lang) -> ValueSetValue {
        ValueSetValue::LanguageStem { stem: lang }
    }
}

impl Deref for ValueSetValue {
    fn deref(
        &self,
        base: &Option<iri_s::IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
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
            _ => {
                todo!()
            }
        }
    }
}

/*#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub struct ValueSetValueWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    pub vs: ValueSetValue,
}

impl ValueSetValueWrapper {
    pub fn new(vs: ValueSetValue) -> ValueSetValueWrapper {
        ValueSetValueWrapper { vs: vs }
    }

    pub fn value(&self) -> ValueSetValue {
        self.vs.clone()
    }
}

impl Deref for ValueSetValueWrapper {
    fn deref(
        &self,
        base: &Option<iri_s::IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        let vs = self.vs.deref(base, prefixmap)?;
        Ok(ValueSetValueWrapper { vs })
    }
}*/

impl SerializeStringOrStruct for ValueSetValue {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            ValueSetValue::ObjectValue(ObjectValue::IriRef(ref r)) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}

impl FromStr for ValueSetValue {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iri_ref = IriRef::try_from(s)?;
        Ok(ValueSetValue::ObjectValue(ObjectValue::IriRef(iri_ref)))
    }
}

fn serialize_lang<S>(lang: &Lang, serializer: S) -> result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(lang.value().as_str())
}

fn deserialize_lang<'de, D>(deserializer: D) -> Result<Lang, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct LangVisitor;

    impl<'de> Visitor<'de> for LangVisitor {
        type Value = Lang;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("Lang")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Lang::new(v))
        }
    }
    deserializer.deserialize_str(LangVisitor)
}

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
}

impl ValueSetValueType {
    fn parse(s: &str) -> Option<ValueSetValueType> {
        match s {
            "IriStem" => Some(ValueSetValueType::IriStem),
            "LanguageStem" => Some(ValueSetValueType::LanguageStem),
            "LiteralStem" => Some(ValueSetValueType::LiteralStem),
            "Language" => Some(ValueSetValueType::Language),
            "IriStemRange" => Some(ValueSetValueType::IriStemRange),
            "LanguageStemRange" => Some(ValueSetValueType::LanguageStemRange),
            "LiteralStemRange" => Some(ValueSetValueType::LiteralStemRange),
            BOOLEAN_STR => Some(ValueSetValueType::Boolean),
            DECIMAL_STR => Some(ValueSetValueType::Decimal),
            DOUBLE_STR => Some(ValueSetValueType::Double),
            INTEGER_STR => Some(ValueSetValueType::Integer),
            _ => None,
        }
    }
}

const BOOLEAN_STR: &str = "http://www.w3.org/2001/XMLSchema#boolean";
const INTEGER_STR: &str = "http://www.w3.org/2001/XMLSchema#integer";
const DOUBLE_STR: &str = "http://www.w3.org/2001/XMLSchema#double";
const DECIMAL_STR: &str = "http://www.w3.org/2001/XMLSchema#decimal";

impl Serialize for ValueSetValue {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ValueSetValue::ObjectValue(v) => match v {
                ObjectValue::BooleanLiteral { value } => {
                    let mut map = serializer.serialize_map(Some(2))?;
                    map.serialize_entry("type", BOOLEAN_STR)?;
                    let value_str = if *value { "true" } else { "false" };
                    map.serialize_entry("value", value_str)?;
                    map.end()
                }
                ObjectValue::NumericLiteral(num) => {
                    let mut map = serializer.serialize_map(Some(2))?;
                    map.serialize_entry("type", get_type_str(num))?;
                    map.serialize_entry("value", &num.to_string());
                    map.end()
                }
                _ => {
                    todo!()
                }
            },
            _ => todo!(),
        }
    }
}

fn get_type_str(n: &NumericLiteral) -> &str {
    match n {
        NumericLiteral::Integer(_) => INTEGER_STR,
        NumericLiteral::Double(_) => DOUBLE_STR,
        NumericLiteral::Decimal(_) => DECIMAL_STR,
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

                impl<'de> Visitor<'de> for FieldVisitor {
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

        const FIELDS: &'static [&'static str] = &[
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
                    .map_err(|e| de::Error::invalid_value(Unexpected::Str(s), &self))
            }

            fn visit_map<V>(self, mut map: V) -> Result<ValueSetValue, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut type_: Option<ValueSetValueType> = None;
                let mut stem: Option<String> = None;
                let mut value: Option<String> = None;
                let mut language_tag: Option<String> = None;
                let mut language: Option<String> = None;
                let mut exclusions: Option<String> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if type_.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            let value: String = map.next_value()?;

                            if let Some(parsed_type_) = ValueSetValueType::parse(&value.as_str()) {
                                type_ = Some(parsed_type_)
                            } else {
                                return Err(de::Error::custom(format!(
                                    "Expected ValueSetValue type, found: {value}"
                                )));
                            }
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
                    Some(ValueSetValueType::Language) => match language_tag {
                        Some(language_tag) => Ok(ValueSetValue::Language {
                            language_tag: Lang::new(language_tag.as_str()),
                        }),
                        None => Err(de::Error::missing_field("languageTag")),
                    },
                    Some(ValueSetValueType::IriStem) => match stem {
                        Some(stem) => {
                            let iri_ref = TryFrom::try_from(stem.as_str()).map_err(|e| {
                                de::Error::custom(format!(
                                    "Can't parse stem as IRIREF for IriStem: {e}"
                                ))
                            })?;
                            Ok(ValueSetValue::IriStem { stem: iri_ref })
                        }
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ValueSetValueType::Boolean) => match value {
                        Some(s) => match s.as_str() {
                            "false" => {
                                Ok(ValueSetValue::ObjectValue(ObjectValue::BooleanLiteral {
                                    value: false,
                                }))
                            }
                            "true" => Ok(ValueSetValue::ObjectValue(ObjectValue::BooleanLiteral {
                                value: true,
                            })),
                            _ => Err(de::Error::invalid_value(Unexpected::Str(&s), &self)),
                        },
                        None => Err(de::Error::missing_field("value")),
                    },
                    
                    Some(v) => Err(de::Error::custom(format!(
                        "Unknown ValueSetValueType {v:?}"
                    ))),
                    None => todo!(),
                }
            }
        }

        deserializer.deserialize_any(ValueSetValueVisitor)
    }
}
