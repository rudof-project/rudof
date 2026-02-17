use rdf::rdf_core::term::literal::Lang;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use std::{result, str::FromStr};
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum LangOrWildcard {
    Lang(Lang),
    Wildcard,
}

impl LangOrWildcard {
    pub fn lang(lang: Lang) -> LangOrWildcard {
        LangOrWildcard::Lang(lang)
    }

    pub fn wildcard() -> LangOrWildcard {
        LangOrWildcard::Wildcard
    }
}
#[derive(Error, Debug, PartialEq, Clone)]
pub enum LangOrWildcardParseError {
    #[error("Invalid language tag")]
    InvalidLang { tag: String, error: String },
}

impl FromStr for LangOrWildcard {
    type Err = LangOrWildcardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lang = Lang::new(s).map_err(|e| LangOrWildcardParseError::InvalidLang {
            tag: s.to_string(),
            error: e.to_string(),
        })?;
        Ok(LangOrWildcard::Lang(lang))
    }
}

impl Serialize for LangOrWildcard {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LangOrWildcard::Lang(lang) => serializer.serialize_str(&lang.to_string()),
            LangOrWildcard::Wildcard => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "Wildcard")?;
                map.end()
            },
        }
    }
}
