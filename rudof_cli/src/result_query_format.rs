use clap::ValueEnum;
use std::fmt::{Display, Formatter, write};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ResultQueryFormat {
    Internal,
    Turtle,
    NTriples,
    JsonLd,
    RdfXml,
    Csv,
    TriG,
    N3,
    NQuads,
}

impl Display for ResultQueryFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultQueryFormat::Internal => write!(dest, "internal"),
            ResultQueryFormat::Turtle => write!(dest, "turtle"),
            ResultQueryFormat::NTriples => write!(dest, "ntriples"),
            ResultQueryFormat::JsonLd => write!(dest, "json-ld"),
            ResultQueryFormat::RdfXml => write!(dest, "rdf-xml"),
            ResultQueryFormat::Csv => write!(dest, "csv"),
            ResultQueryFormat::TriG => write!(dest, "trig"),
            ResultQueryFormat::N3 => write!(dest, "n3"),
            ResultQueryFormat::NQuads => write!(dest, "nquads"),
        }
    }
}
