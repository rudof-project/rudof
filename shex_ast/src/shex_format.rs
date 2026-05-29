use std::fmt::{Display, Formatter};
use std::str::FromStr;
use rudof_iri::MimeType;
use rudof_rdf::rdf_core::RDFFormat;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Contains possible ShEx formats
#[derive(Debug, PartialEq, Clone, Default)]
pub enum ShExFormat {
    #[default]
    ShExC,
    ShExJ,
    RDFFormat(RDFFormat),
}

impl ShExFormat {
    pub fn extensions(&self) -> Vec<&'static str> {
        match self {
            ShExFormat::ShExC => vec!["shex", "shexc"],
            ShExFormat::ShExJ => vec!["json", "shexj"],
            ShExFormat::RDFFormat(rdf_format) => rdf_format.extensions(),
        }
    }
}

impl MimeType for ShExFormat {
    fn mime_type(&self) -> &'static str {
        match self {
            ShExFormat::ShExC => "text/shex",
            ShExFormat::ShExJ => "application/shex+json",
            ShExFormat::RDFFormat(rdf_format) => rdf_format.mime_type(),
        }
    }
}

impl Serialize for ShExFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.to_string().to_lowercase())
    }
}

impl<'de> Deserialize<'de> for ShExFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let str = String::deserialize(deserializer)?;
        ShExFormat::from_str(&str)
            .map_err(serde::de::Error::custom)
    }
}

impl FromStr for ShExFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shexc" | "shex" => Ok(ShExFormat::ShExC),
            "shexj" | "json" => Ok(ShExFormat::ShExJ),
            _ => RDFFormat::from_str(s)
                .map(ShExFormat::RDFFormat)
                .map_err(|_| format!("Unknown ShEx format: {s}")),
        }
    }
}

impl Display for ShExFormat {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ShExFormat::ShExC => write!(f, "ShExC"),
            ShExFormat::ShExJ => write!(f, "ShExJ"),
            ShExFormat::RDFFormat(rdf_format) => write!(f, "{rdf_format}"),
        }
    }
}
