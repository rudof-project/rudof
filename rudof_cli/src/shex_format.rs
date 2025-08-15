use std::fmt::{Display, Formatter};

use clap::ValueEnum;

use crate::mime_type::MimeType;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum ShExFormat {
    Internal,
    Simple,
    #[default]
    ShExC,
    ShExJ,
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

impl MimeType for ShExFormat {
    fn mime_type(&self) -> String {
        match self {
            ShExFormat::Internal => "text/turtle".to_string(),
            ShExFormat::Simple => "text/turtle".to_string(),
            ShExFormat::ShExC => "text/shex".to_string(),
            ShExFormat::ShExJ => "application/json".to_string(),
            ShExFormat::Turtle => "text/turtle".to_string(),
            ShExFormat::NTriples => "application/n-triples".to_string(),
            ShExFormat::RDFXML => "application/rdf+xml".to_string(),
            ShExFormat::TriG => "application/trig".to_string(),
            ShExFormat::N3 => "text/n3".to_string(),
            ShExFormat::NQuads => "application/n-quads".to_string(),
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
        }
    }
}
