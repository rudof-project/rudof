use std::{result, str::FromStr};

use crate::schema_json::serde_string_or_struct::*;
use serde::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use void::Void;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(transparent)]
pub struct StringOrLiteralStemWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    s: StringOrLiteralStem,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrLiteralStem {
    String(String),
    LiteralStem { stem: String },
}

impl FromStr for StringOrLiteralStemWrapper {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StringOrLiteralStemWrapper {
            s: StringOrLiteralStem::String(s.to_string()),
        })
    }
}

impl FromStr for StringOrLiteralStem {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StringOrLiteralStem::String(s.to_string()))
    }
}

impl SerializeStringOrStruct for StringOrLiteralStem {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            StringOrLiteralStem::String(ref r) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}
