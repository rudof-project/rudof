use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use std::{result, str::FromStr};
use thiserror::Error;
#[derive(Debug, PartialEq, Clone)]
pub enum StringOrWildcard {
    String(String),
    Wildcard,
}

impl FromStr for StringOrWildcard {
    type Err = StringOrWildcardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StringOrWildcard::String(s.to_string()))
    }
}

#[derive(Debug, Error)]
pub enum StringOrWildcardError {
    #[error("Invalid StringOrWildcard")]
    InvalidStringOrWildcard,
}

impl Serialize for StringOrWildcard {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            StringOrWildcard::String(s) => serializer.serialize_str(s),
            StringOrWildcard::Wildcard => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "Wildcard")?;
                map.end()
            },
        }
    }
}
