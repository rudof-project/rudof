use std::str::FromStr;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("Format {} not supported by RDF", ._0)]
pub struct FormatError(String);

/// Posible RDF formats
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum RdfFormat {
    #[default]
    Turtle,
    N3,
    RdfXml,
    NQuads,
    NTriples,
    TriG,
}

impl FromStr for RdfFormat {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<RdfFormat, FormatError> {
        match s {
            "ttl" => Ok(RdfFormat::Turtle),
            "nt" => Ok(RdfFormat::NTriples),
            "rdf" => Ok(RdfFormat::RdfXml),
            "trig" => Ok(RdfFormat::TriG),
            "n3" => Ok(RdfFormat::N3),
            "nq" => Ok(RdfFormat::NQuads),
            _ => Err(FormatError(s.to_string())),
        }
    }
}
