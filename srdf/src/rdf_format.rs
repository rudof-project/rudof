use iri_s::MimeType;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

use crate::RDFParseError;

/// Posible RDF formats
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub enum RDFFormat {
    #[default]
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
    JsonLd,
}

impl RDFFormat {}

impl MimeType for RDFFormat {
    fn mime_type(&self) -> &'static str {
        match self {
            RDFFormat::Turtle => "text/turtle",
            RDFFormat::NTriples => "application/n-triples",
            RDFFormat::RDFXML => "application/rdf+xml",
            RDFFormat::TriG => "application/trig",
            RDFFormat::N3 => "text/n3",
            RDFFormat::NQuads => "application/n-quads",
            RDFFormat::JsonLd => "application/ld+json",
        }
    }
}

impl FromStr for RDFFormat {
    type Err = RDFParseError;

    fn from_str(s: &str) -> Result<RDFFormat, RDFParseError> {
        match s {
            "ttl" => Ok(RDFFormat::Turtle),
            "nt" => Ok(RDFFormat::NTriples),
            "rdf" => Ok(RDFFormat::RDFXML),
            "trig" => Ok(RDFFormat::TriG),
            "n3" => Ok(RDFFormat::N3),
            "nq" => Ok(RDFFormat::NQuads),
            _ => Err(RDFParseError::SRDFError {
                err: format!("Format {s} not supported").to_string(),
            }),
        }
    }
}

impl Display for RDFFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RDFFormat::Turtle => write!(f, "Turtle"),
            RDFFormat::NTriples => write!(f, "N-Triples"),
            RDFFormat::RDFXML => write!(f, "RDF/XML"),
            RDFFormat::TriG => write!(f, "TriG"),
            RDFFormat::N3 => write!(f, "N3"),
            RDFFormat::NQuads => write!(f, "NQuads"),
            RDFFormat::JsonLd => write!(f, "JSONLD"),
        }
    }
}
