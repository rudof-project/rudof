use crate::RudofError;
use clap::ValueEnum;
use iri_s::MimeType;
use shex_ast::ShExFormat as ShExAstShExFormat;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

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

impl TryFrom<ShExFormat> for ShExAstShExFormat {
    type Error = RudofError;

    fn try_from(format: ShExFormat) -> Result<Self, Self::Error> {
        match format {
            ShExFormat::ShExC => Ok(ShExAstShExFormat::ShExC),
            ShExFormat::ShExJ | ShExFormat::JSON | ShExFormat::JSONLD => Ok(ShExAstShExFormat::ShExJ),
            other => Err(RudofError::NotImplemented {
                msg: format!("ShEx format {other:?} validation not yet implemented"),
            }),
        }
    }
}

impl FromStr for ShExFormat {
    type Err = RudofError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "internal" => Ok(ShExFormat::Internal),
            "simple" => Ok(ShExFormat::Simple),
            "shexc" => Ok(ShExFormat::ShExC),
            "shexj" => Ok(ShExFormat::ShExJ),
            "json" => Ok(ShExFormat::JSON),
            "jsonld" => Ok(ShExFormat::JSONLD),
            "turtle" => Ok(ShExFormat::Turtle),
            "ntriples" => Ok(ShExFormat::NTriples),
            "rdfxml" => Ok(ShExFormat::RDFXML),
            "trig" => Ok(ShExFormat::TriG),
            "n3" => Ok(ShExFormat::N3),
            "nquads" => Ok(ShExFormat::NQuads),
            other => Err(RudofError::UnsupportedShExFormat {
                format: other.to_string(),
            }),
        }
    }
}
