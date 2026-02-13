use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use srdf::lang::Lang;
use std::fmt::Display;
use std::result;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LanguageExclusion {
    Language(Lang),
    LanguageStem(Lang),
}

impl Serialize for LanguageExclusion {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LanguageExclusion::Language(lang) => serializer.serialize_str(&lang.to_string()),
            LanguageExclusion::LanguageStem(stem) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LanguageStem")?;
                map.serialize_entry("stem", stem)?;
                map.end()
            },
        }
    }
}

impl Display for LanguageExclusion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageExclusion::Language(lang) => write!(f, "@{lang}"),
            LanguageExclusion::LanguageStem(stem) => write!(f, "{stem}~"),
        }
    }
}
