use super::serde_string_or_struct::SerializeStringOrStruct;
use crate::ast::serde_string_or_struct::*;
use serde::{Deserialize, Serialize, Serializer};
use std::{result, str::FromStr};
use thiserror::Error;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum StringOrIriStem {
    String(String),
    IriStem { stem: String },
}

impl FromStr for StringOrIriStem {
    type Err = StringOrIriStemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StringOrIriStem::String(s.to_string()))
    }
}

#[derive(Debug, Error)]
pub enum StringOrIriStemError {
    #[error("Invalid StringOrIriStem")]
    InvalidStringOrIriStem,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub struct StringOrIriStemWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    s: StringOrIriStem,
}

impl SerializeStringOrStruct for StringOrIriStem {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            StringOrIriStem::String(r) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}
