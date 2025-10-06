use clap::ValueEnum;
use std::fmt::{Display, Formatter};

use iri_s::mime_type::MimeType;

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
    JsonLd,
}

impl MimeType for CliShaclFormat {
    fn mime_type(&self) -> &'static str {
        match self {
            CliShaclFormat::Turtle => "text/turtle",
            CliShaclFormat::NTriples => "application/n-triples",
            CliShaclFormat::RDFXML => "application/rdf+xml",
            CliShaclFormat::TriG => "application/trig",
            CliShaclFormat::N3 => "text/n3",
            CliShaclFormat::NQuads => "application/n-quads",
            CliShaclFormat::Internal => "text/turtle",
            CliShaclFormat::JsonLd => "application/ld+json",
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
            CliShaclFormat::JsonLd => write!(dest, "jsonld"),
        }
    }
}
