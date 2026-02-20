use crate::{RDFFormat, ShExFormat, dctap_format::DCTapFormat, shacl_format::ShaclFormat};
use iri_s::MimeType;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum InputCompareMode {
    Shacl,

    #[default]
    ShEx,
    Dctap,
    Service,
}

impl Display for InputCompareMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InputCompareMode::Shacl => write!(dest, "shacl"),
            InputCompareMode::ShEx => write!(dest, "shex"),
            InputCompareMode::Dctap => write!(dest, "dctap"),
            InputCompareMode::Service => write!(dest, "service"),
        }
    }
}

impl FromStr for InputCompareMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shacl" => Ok(InputCompareMode::Shacl),
            "shex" => Ok(InputCompareMode::ShEx),
            "dctap" => Ok(InputCompareMode::Dctap),
            "service" => Ok(InputCompareMode::Service),
            _ => Err(format!(
                "Unknown input compare mode: '{}'. Supported modes: shacl, shex, dctap, service",
                s
            )),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum InputCompareFormat {
    #[default]
    ShExC,
    ShExJ,
    Turtle,
    RdfXml,
    NTriples,
}

impl InputCompareFormat {
    pub fn to_shex_format(self) -> Result<ShExFormat, String> {
        match self {
            InputCompareFormat::ShExC => Ok(ShExFormat::ShExC),
            InputCompareFormat::ShExJ => Ok(ShExFormat::ShExJ),
            InputCompareFormat::Turtle => Ok(ShExFormat::RDFFormat(RDFFormat::Turtle)),
            InputCompareFormat::RdfXml => Ok(ShExFormat::RDFFormat(RDFFormat::Rdfxml)),
            InputCompareFormat::NTriples => Ok(ShExFormat::RDFFormat(RDFFormat::NTriples)),
        }
    }
    pub fn to_shacl_format(self) -> Result<ShaclFormat, String> {
        match self {
            InputCompareFormat::Turtle => Ok(ShaclFormat::Turtle),
            InputCompareFormat::NTriples => Ok(ShaclFormat::NTriples),
            InputCompareFormat::RdfXml => Ok(ShaclFormat::RdfXml),
            InputCompareFormat::ShExC => Err("Can't convert from ShExC to SHACL yet".into()),
            InputCompareFormat::ShExJ => Err("Can't convert from ShExJ to SHACL yet".into()),
        }
    }

    pub fn to_dctap_format(self) -> Result<DCTapFormat, String> {
        Err(format!("Converting to DCTAP, format {self} not supported"))
    }
}

impl FromStr for InputCompareFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shexc" => Ok(InputCompareFormat::ShExC),
            "shexj" => Ok(InputCompareFormat::ShExJ),
            "turtle" => Ok(InputCompareFormat::Turtle),
            "rdfxml" => Ok(InputCompareFormat::RdfXml),
            "ntriples" => Ok(InputCompareFormat::NTriples),
            _ => Err(format!("Unsupported input convert format {s}")),
        }
    }
}

impl Display for InputCompareFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InputCompareFormat::ShExC => write!(dest, "shexc"),
            InputCompareFormat::ShExJ => write!(dest, "shexj"),
            InputCompareFormat::Turtle => write!(dest, "turtle"),
            InputCompareFormat::RdfXml => write!(dest, "rdfxml"),
            InputCompareFormat::NTriples => write!(dest, "ntriples"),
        }
    }
}

impl MimeType for InputCompareFormat {
    fn mime_type(&self) -> &'static str {
        match &self {
            InputCompareFormat::ShExC => "text/shex",
            InputCompareFormat::ShExJ => "application/json",
            InputCompareFormat::Turtle => "text/turtle",
            InputCompareFormat::RdfXml => "application/rdf+xml",
            InputCompareFormat::NTriples => "application/n-triples",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ResultCompareFormat {
    #[default]
    Internal,
    Json,
}

impl Display for ResultCompareFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultCompareFormat::Internal => write!(dest, "internal"),
            ResultCompareFormat::Json => write!(dest, "json"),
        }
    }
}

impl FromStr for ResultCompareFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "internal" => Ok(ResultCompareFormat::Internal),
            "json" => Ok(ResultCompareFormat::Json),
            _ => Err(format!(
                "Unknown result compare format: '{}'. Supported formats: internal, json",
                s
            )),
        }
    }
}
