use std::{result, str::FromStr};

use serde::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use srdf::lang::Lang;
use void::Void;

use super::serde_string_or_struct::SerializeStringOrStruct;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum LangOrWildcard {
    Lang(Lang),
    Wildcard,
}

impl FromStr for LangOrWildcard {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LangOrWildcard::Lang(Lang::new(s)))
    }
}

impl SerializeStringOrStruct for LangOrWildcard {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            LangOrWildcard::Lang(ref lang) => lang.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}
