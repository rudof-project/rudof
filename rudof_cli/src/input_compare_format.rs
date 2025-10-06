use crate::CliShaclFormat;
use crate::dctap_format::DCTapFormat as CliDCTapFormat;
use anyhow::{Result, bail};
use clap::ValueEnum;
use iri_s::mime_type::MimeType;
use shex_ast::ShExFormat;
use srdf::RDFFormat;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum InputCompareFormat {
    #[default]
    ShExC,
    ShExJ,
    Turtle,
    RDFXML,
    NTriples,
}

impl InputCompareFormat {
    pub fn to_shex_format(&self) -> Result<ShExFormat> {
        match self {
            InputCompareFormat::ShExC => Ok(ShExFormat::ShExC),
            InputCompareFormat::ShExJ => Ok(ShExFormat::ShExJ),
            InputCompareFormat::Turtle => Ok(ShExFormat::RDFFormat(RDFFormat::Turtle)),
            InputCompareFormat::RDFXML => Ok(ShExFormat::RDFFormat(RDFFormat::RDFXML)),
            InputCompareFormat::NTriples => Ok(ShExFormat::RDFFormat(RDFFormat::NTriples)),
        }
    }
    pub fn to_shacl_format(&self) -> Result<CliShaclFormat> {
        match self {
            InputCompareFormat::Turtle => Ok(CliShaclFormat::Turtle),
            InputCompareFormat::NTriples => Ok(CliShaclFormat::NTriples),
            InputCompareFormat::RDFXML => Ok(CliShaclFormat::RDFXML),
            InputCompareFormat::ShExC => bail!("Can't convert from ShExC to SHACL yet"),
            InputCompareFormat::ShExJ => bail!("Can't convert from ShExJ to SHACL yet"),
        }
    }

    pub fn to_dctap_format(&self) -> Result<CliDCTapFormat> {
        bail!("Converting to DCTAP, format {self} not supported")
    }
}

impl FromStr for InputCompareFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shexc" => Ok(InputCompareFormat::ShExC),
            "shexj" => Ok(InputCompareFormat::ShExJ),
            "turtle" => Ok(InputCompareFormat::Turtle),
            "rdfxml" => Ok(InputCompareFormat::RDFXML),
            "ntriples" => Ok(InputCompareFormat::NTriples),
            _ => Err(format!("Unsupported input convert format {s}")),
        }
    }
}

impl Display for InputCompareFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InputCompareFormat::ShExC => write!(dest, "shexc"),
            InputCompareFormat::ShExJ => write!(dest, "shexj"),
            InputCompareFormat::Turtle => write!(dest, "turtle"),
            InputCompareFormat::RDFXML => write!(dest, "rdfxml"),
            InputCompareFormat::NTriples => write!(dest, "ntriples"),
        }
    }
}

impl MimeType for InputCompareFormat {
    fn mime_type(&self) -> &'static str {
        match &self {
            InputCompareFormat::ShExC => "text/shex",
            InputCompareFormat::ShExJ => "application/json",
            InputCompareFormat::Turtle => "text/turtle",
            InputCompareFormat::RDFXML => "application/rdf+xml",
            InputCompareFormat::NTriples => "application/n-triples",
        }
    }
}

#[cfg(test)]
mod tests {}
