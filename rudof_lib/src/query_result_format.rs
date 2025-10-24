use crate::RudofError;
use clap::ValueEnum;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

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

impl FromStr for ResultQueryFormat {
    type Err = RudofError;

    fn from_str(s: &str) -> Result<Self, RudofError> {
        match s.to_lowercase().as_str() {
            "internal" => Ok(ResultQueryFormat::Internal),
            "turtle" => Ok(ResultQueryFormat::Turtle),
            "ntriples" => Ok(ResultQueryFormat::NTriples),
            "json-ld" => Ok(ResultQueryFormat::JsonLd),
            "rdf-xml" => Ok(ResultQueryFormat::RdfXml),
            "csv" => Ok(ResultQueryFormat::Csv),
            "trig" => Ok(ResultQueryFormat::TriG),
            "n3" => Ok(ResultQueryFormat::N3),
            "nquads" => Ok(ResultQueryFormat::NQuads),
            _ => Err(RudofError::QueryResultFormatParseError {
                format: s.to_string(),
                error: format!("Format {s} not supported").to_string(),
            }),
        }
    }
}
