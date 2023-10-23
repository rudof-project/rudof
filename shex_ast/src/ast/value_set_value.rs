use std::{result, str::FromStr};

use crate::ast::serde_string_or_struct::*;
use iri_s::IriSError;
use serde::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};

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

        #[serde(rename = "languageTag")]
        language_tag: String,
    },
    LanguageStem,
    LanguageStemRange,
    ObjectValue(ObjectValueWrapper),
}

impl ValueSetValue {
    pub fn iri(iri: IriRef) -> ValueSetValue {
        ValueSetValue::ObjectValue(ObjectValueWrapper {
            ov: ObjectValue::IriRef(iri),
        })
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub struct ValueSetValueWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    vs: ValueSetValue,
}

impl ValueSetValueWrapper {
    pub fn new(vs: ValueSetValue) -> ValueSetValueWrapper {
        ValueSetValueWrapper { vs: vs }
    }

    pub fn value(&self) -> ValueSetValue {
        self.vs.clone()
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
