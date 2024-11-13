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
                err: format!("Format {} not supported", s).to_string(),
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
        }
    }
}
