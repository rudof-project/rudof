use iri_s::IriS;
use prefixmap::IriRef;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Display;
use std::result;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub enum IriExclusion {
    Iri(IriRef),
    IriStem(IriRef),
}

impl Serialize for IriExclusion {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            IriExclusion::Iri(iri) => serializer.serialize_str(iri.to_string().as_str()),
            IriExclusion::IriStem(stem) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "IriStem")?;
                map.serialize_entry("stem", stem)?;
                map.end()
            }
        }
    }
}

impl Display for IriExclusion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IriExclusion::Iri(iri) => write!(f, "{iri}"),
            IriExclusion::IriStem(stem) => write!(f, "{stem}~"),
        }
    }
}
