use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Display;
use std::result;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub enum LiteralExclusion {
    Literal(String),
    LiteralStem(String),
}

impl Serialize for LiteralExclusion {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LiteralExclusion::Literal(lit) => serializer.serialize_str(lit.as_str()),
            LiteralExclusion::LiteralStem(stem) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LiteralStem")?;
                map.serialize_entry("stem", stem)?;
                map.end()
            },
        }
    }
}

impl Display for LiteralExclusion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralExclusion::Literal(lit) => write!(f, "{lit}"),
            LiteralExclusion::LiteralStem(stem) => write!(f, "{stem}~"),
        }
    }
}
