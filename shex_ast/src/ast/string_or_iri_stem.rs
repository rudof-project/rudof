use std::{result, str::FromStr};

use crate::ast::serde_string_or_struct::*;

use serde::{Deserialize, Serialize, Serializer};
use void::Void;

use super::serde_string_or_struct::SerializeStringOrStruct;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum StringOrIriStem {
    String(String),
    IriStem { stem: String },
}

impl FromStr for StringOrIriStem {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StringOrIriStem::String(s.to_string()))
    }
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
            StringOrIriStem::String(ref r) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}
