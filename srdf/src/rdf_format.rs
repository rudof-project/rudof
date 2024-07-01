use std::str::FromStr;

use crate::RDFParseError;

// Posible RDF formats
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RDFFormat {
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
