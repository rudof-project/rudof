use std::{result, str::FromStr, fmt};

use crate::{ast::serde_string_or_struct::*, Deref, DerefError, LangOrWildcard};
use iri_s::{IriS, IriSError};
use serde::{Serialize, Serializer, de::Visitor};
use serde_derive::{Deserialize, Serialize};
use srdf::{lang::Lang, literal::Literal};

use super::{
    iri_ref::IriRef, iri_ref_or_wildcard::IriRefOrWildcard,
    string_or_iri_stem::StringOrIriStemWrapper, string_or_literal_stem::StringOrLiteralStemWrapper,
    string_or_wildcard::StringOrWildcard, ObjectValue, ObjectValueWrapper,
};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum ValueSetValue {
    IriStem {
        #[serde(rename = "type")]
        type_: String,

        stem: IriRef,
    },
    IriStemRange {
        #[serde(rename = "type")]
        type_: String,

        #[serde(
            serialize_with = "serialize_string_or_struct",
            deserialize_with = "deserialize_string_or_struct"
        )]
        stem: IriRefOrWildcard,

        #[serde(skip_serializing_if = "Option::is_none")]
        exclusions: Option<Vec<StringOrIriStemWrapper>>,
    },
    LiteralStem {
        #[serde(rename = "type")]
        type_: String,

        stem: String,
    },
    LiteralStemRange {
        #[serde(rename = "type")]
        type_: String,

        #[serde(
            serialize_with = "serialize_string_or_struct",
            deserialize_with = "deserialize_string_or_struct"
        )]
        stem: StringOrWildcard,

        #[serde(skip_serializing_if = "Option::is_none")]
        exclusions: Option<Vec<StringOrLiteralStemWrapper>>,
    },
    Language {
        #[serde(rename = "type")]
        type_: String,

        #[serde(rename = "languageTag", 
           serialize_with = "serialize_lang", 
           deserialize_with = "deserialize_lang")]
        language_tag: Lang,
    },
    LanguageStem {
        #[serde(rename = "type")]
        type_: String,

        stem: Lang,
    },
    LanguageStemRange {
        #[serde(rename = "type")]
        type_: String,

        #[serde(
            serialize_with = "serialize_string_or_struct",
            deserialize_with = "deserialize_string_or_struct"
        )]
        stem: LangOrWildcard,

        #[serde(skip_serializing_if = "Option::is_none")]
        exclusions: Option<Vec<StringOrLiteralStemWrapper>>,
    },
    ObjectValue(ObjectValueWrapper),
}

impl ValueSetValue {
    pub fn iri(iri: IriRef) -> ValueSetValue {
        ValueSetValue::ObjectValue(ObjectValueWrapper {
            ov: ObjectValue::IriRef(iri),
        })
    }

    pub fn literal(value: &str, language: Option<Lang>, type_: Option<IriRef>) -> ValueSetValue {
        let ov = ObjectValue::ObjectLiteral {
            value: value.to_string(),
            language,
            type_,
        };
        ValueSetValue::ObjectValue(ObjectValueWrapper { ov })
    }

    pub fn object_value(value: ObjectValue) -> ValueSetValue {
        ValueSetValue::ObjectValue(ObjectValueWrapper { ov: value })
    }

    pub fn language(lang: Lang) -> ValueSetValue {
        ValueSetValue::Language { 
           type_: "Language".to_string(),
           language_tag: lang
        }
    }

    pub fn language_stem(lang: Lang) -> ValueSetValue {
        ValueSetValue::LanguageStem { 
           type_: "LanguageStem".to_string(),
           stem: lang
        }
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
            },
            ValueSetValue::Language { type_, language_tag } => {
                Ok(ValueSetValue::Language { type_: type_.clone(), language_tag: language_tag.clone() })
            }
            ValueSetValue::LanguageStem { type_, stem } => {
                Ok(ValueSetValue::LanguageStem { type_: type_.clone(), stem: stem.clone() })
            }
            _ => {
                todo!()
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
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
}

impl SerializeStringOrStruct for ValueSetValue {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            ValueSetValue::ObjectValue(ObjectValueWrapper {
                ov: ObjectValue::IriRef(ref r),
            }) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}

impl FromStr for ValueSetValue {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iri_ref = IriRef::try_from(s)?;
        Ok(ValueSetValue::ObjectValue(ObjectValueWrapper {
            ov: ObjectValue::IriRef(iri_ref),
        }))
    }
}

fn serialize_lang<S>(p: &Lang, serializer: S) -> result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
   serializer.serialize_str(p.value().as_str())
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
