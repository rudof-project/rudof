use clap::ValueEnum;
use std::fmt::{Display, Formatter};

use crate::mime_type::MimeType;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum CliShaclFormat {
    Internal,
    #[default]
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

impl MimeType for CliShaclFormat {
    fn mime_type(&self) -> String {
        match self {
            CliShaclFormat::Turtle => "text/turtle".to_string(),
            CliShaclFormat::NTriples => "application/n-triples".to_string(),
            CliShaclFormat::RDFXML => "application/rdf+xml".to_string(),
            CliShaclFormat::TriG => "application/trig".to_string(),
            CliShaclFormat::N3 => "text/n3".to_string(),
            CliShaclFormat::NQuads => "application/n-quads".to_string(),
            CliShaclFormat::Internal => "text/turtle".to_string(),
        }
    }
}

impl Display for CliShaclFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CliShaclFormat::Internal => write!(dest, "internal"),
            CliShaclFormat::Turtle => write!(dest, "turtle"),
            CliShaclFormat::NTriples => write!(dest, "NTriples"),
            CliShaclFormat::RDFXML => write!(dest, "rdfxml"),
            CliShaclFormat::TriG => write!(dest, "trig"),
            CliShaclFormat::N3 => write!(dest, "n3"),
            CliShaclFormat::NQuads => write!(dest, "nquads"),
        }
    }
}
