use clap::ValueEnum;
use iri_s::mime_type::MimeType;
use srdf::RDFFormat;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum DataFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
    JsonLd,
}

impl From<DataFormat> for RDFFormat {
    fn from(val: DataFormat) -> Self {
        match val {
            DataFormat::Turtle => RDFFormat::Turtle,
            DataFormat::NTriples => RDFFormat::NTriples,
            DataFormat::RDFXML => RDFFormat::RDFXML,
            DataFormat::TriG => RDFFormat::TriG,
            DataFormat::N3 => RDFFormat::N3,
            DataFormat::NQuads => RDFFormat::NQuads,
            DataFormat::JsonLd => RDFFormat::JsonLd,
        }
    }
}

impl Display for DataFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DataFormat::Turtle => write!(dest, "turtle"),
            DataFormat::NTriples => write!(dest, "ntriples"),
            DataFormat::RDFXML => write!(dest, "rdfxml"),
            DataFormat::TriG => write!(dest, "trig"),
            DataFormat::N3 => write!(dest, "n3"),
            DataFormat::NQuads => write!(dest, "nquads"),
            DataFormat::JsonLd => write!(dest, "jsonld"),
        }
    }
}

impl MimeType for DataFormat {
    fn mime_type(&self) -> &'static str {
        match self {
            DataFormat::Turtle => "text/turtle",
            DataFormat::NTriples => "application/n-triples",
            DataFormat::RDFXML => "application/rdf+xml",
            DataFormat::TriG => "application/trig",
            DataFormat::N3 => "text/n3",
            DataFormat::NQuads => "application/n-quads",
            DataFormat::JsonLd => "application/ld+json",
        }
    }
}
