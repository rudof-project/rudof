use clap::ValueEnum;
use std::fmt::{Display, Formatter};

use crate::mime_type::MimeType;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum ShaclFormat {
    Internal,
    #[default]
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

impl MimeType for ShaclFormat {
    fn mime_type(&self) -> String {
        match self {
            ShaclFormat::Turtle => "text/turtle".to_string(),
            ShaclFormat::NTriples => "application/n-triples".to_string(),
            ShaclFormat::RDFXML => "application/rdf+xml".to_string(),
            ShaclFormat::TriG => "application/trig".to_string(),
            ShaclFormat::N3 => "text/n3".to_string(),
            ShaclFormat::NQuads => "application/n-quads".to_string(),
            ShaclFormat::Internal => "text/turtle".to_string(),
        }
    }
}

impl Display for ShaclFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShaclFormat::Internal => write!(dest, "internal"),
            ShaclFormat::Turtle => write!(dest, "turtle"),
            ShaclFormat::NTriples => write!(dest, "NTriples"),
            ShaclFormat::RDFXML => write!(dest, "rdfxml"),
            ShaclFormat::TriG => write!(dest, "trig"),
            ShaclFormat::N3 => write!(dest, "n3"),
            ShaclFormat::NQuads => write!(dest, "nquads"),
        }
    }
}
