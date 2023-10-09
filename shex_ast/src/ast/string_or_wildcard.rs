use std::{result, str::FromStr};

use serde::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use void::Void;

use super::serde_string_or_struct::SerializeStringOrStruct;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrWildcard {
    String(String),
    Wildcard {
        #[serde(rename = "type")]
        type_: String,
    },
}

impl FromStr for StringOrWildcard {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StringOrWildcard::String(s.to_string()))
    }
}

impl SerializeStringOrStruct for StringOrWildcard {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            StringOrWildcard::String(ref r) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}
