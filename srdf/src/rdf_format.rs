use std::fmt::Display;
use std::str::FromStr;

use crate::RDFParseError;

/// Posible RDF formats
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
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

impl RDFFormat {
    /// Get the file extension for the RDF format
    pub fn file_extension(&self) -> &str {
        match self {
            RDFFormat::Turtle => "ttl",
            RDFFormat::NTriples => "nt",
            RDFFormat::RDFXML => "rdf",
            RDFFormat::TriG => "trig",
            RDFFormat::N3 => "n3",
            RDFFormat::NQuads => "nq",
            RDFFormat::JsonLd => "jsonld",
        }
    }

    pub fn from_file_extension(ext: &str) -> Result<RDFFormat, RDFParseError> {
        match ext {
            "ttl" => Ok(RDFFormat::Turtle),
            "nt" => Ok(RDFFormat::NTriples),
            "rdf" => Ok(RDFFormat::RDFXML),
            "trig" => Ok(RDFFormat::TriG),
            "n3" => Ok(RDFFormat::N3),
            "nq" => Ok(RDFFormat::NQuads),
            "jsonld" => Ok(RDFFormat::JsonLd),
            _ => Err(RDFParseError::SRDFError {
                err: format!("File extension {ext} not supported").to_string(),
            }),
        }
    }
}

impl FromStr for RDFFormat {
    type Err = RDFParseError;

    fn from_str(s: &str) -> Result<RDFFormat, RDFParseError> {
        match s.to_lowercase().as_str() {
            "ttl" => Ok(RDFFormat::Turtle),
            "turtle" => Ok(RDFFormat::Turtle),
            "ntriples" => Ok(RDFFormat::NTriples),
            "nt" => Ok(RDFFormat::NTriples),
            "rdf/xml" => Ok(RDFFormat::RDFXML),
            "rdf" => Ok(RDFFormat::RDFXML),
            "trig" => Ok(RDFFormat::TriG),
            "n3" => Ok(RDFFormat::N3),
            "nquads" => Ok(RDFFormat::NQuads),
            "nq" => Ok(RDFFormat::NQuads),
            "jsonld" => Ok(RDFFormat::JsonLd),
            "json" => Ok(RDFFormat::JsonLd),
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
