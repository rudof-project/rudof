use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use srdf::lang::Lang;
use std::{result, str::FromStr};
use void::Void;

#[derive(Debug, PartialEq, Clone)]
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

impl Serialize for LangOrWildcard {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LangOrWildcard::Lang(lang) => serializer.serialize_str(&lang.value()),
            LangOrWildcard::Wildcard => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "Wildcard")?;
                map.end()
            }
        }
    }
}
