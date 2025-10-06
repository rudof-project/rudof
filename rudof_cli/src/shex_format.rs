use std::fmt::{Display, Formatter};

use clap::ValueEnum;

use iri_s::mime_type::MimeType;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum ShExFormat {
    Internal,
    Simple,
    #[default]
    ShExC,
    ShExJ,
    JSON,
    JSONLD,
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

impl MimeType for ShExFormat {
    fn mime_type(&self) -> &'static str {
        match self {
            ShExFormat::Internal => "text/turtle",
            ShExFormat::Simple => "text/turtle",
            ShExFormat::ShExC => "text/shex",
            ShExFormat::ShExJ => "application/json",
            ShExFormat::Turtle => "text/turtle",
            ShExFormat::NTriples => "application/n-triples",
            ShExFormat::RDFXML => "application/rdf+xml",
            ShExFormat::TriG => "application/trig",
            ShExFormat::N3 => "text/n3",
            ShExFormat::NQuads => "application/n-quads",
            ShExFormat::JSON => "application/json",
            ShExFormat::JSONLD => "application/ld+json",
        }
    }
}

impl Display for ShExFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShExFormat::Internal => write!(dest, "internal"),
            ShExFormat::Simple => write!(dest, "simple"),
            ShExFormat::ShExC => write!(dest, "shexc"),
            ShExFormat::ShExJ => write!(dest, "shexj"),
            ShExFormat::Turtle => write!(dest, "turtle"),
            ShExFormat::NTriples => write!(dest, "ntriples"),
            ShExFormat::RDFXML => write!(dest, "rdfxml"),
            ShExFormat::TriG => write!(dest, "trig"),
            ShExFormat::N3 => write!(dest, "n3"),
            ShExFormat::NQuads => write!(dest, "nquads"),
            ShExFormat::JSON => write!(dest, "json"),
            ShExFormat::JSONLD => write!(dest, "jsonld"),
        }
    }
}
