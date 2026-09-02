use crate::errors::{RudofError, ShExError};
use rudof_iri::MimeType;
use rudof_rdf::rdf_core::RDFFormat;
use shex_ast::ShExFormat as ShExAstShExFormat;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// ShEx schema formats supported by Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ShExFormat {
    /// Internal format - internal representation for processing
    Internal,
    /// Simple format - simplified ShEx representation
    Simple,
    /// ShExC - compact ShEx syntax (default, human-readable)
    #[default]
    ShExC,
    /// ShExJ - JSON representation of ShEx schemas
    ShExJ,
    /// JSON - generic JSON format
    Json,
    /// JSON-LD - JSON format for Linked Data
    JsonLd,
    /// Turtle - compact RDF format
    Turtle,
    /// N-Triples - line-based RDF format
    NTriples,
    /// RDF/XML - XML-based RDF serialization
    RdfXml,
    /// TriG - Turtle with named graphs support
    TriG,
    /// Notation3 - superset of Turtle
    N3,
    /// N-Quads - N-Triples with named graphs support
    NQuads,
    /// PlantUML - text-based UML diagram format for visualization
    PlantUML,
    /// SVG - Scalable Vector Graphics image format for visual output
    Svg,
    /// PNG - Portable Network Graphics image format for visual output
    Png,
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
            ShExFormat::RdfXml => write!(dest, "rdfxml"),
            ShExFormat::TriG => write!(dest, "trig"),
            ShExFormat::N3 => write!(dest, "n3"),
            ShExFormat::NQuads => write!(dest, "nquads"),
            ShExFormat::Json => write!(dest, "json"),
            ShExFormat::JsonLd => write!(dest, "jsonld"),
            ShExFormat::PlantUML => write!(dest, "plantuml"),
            ShExFormat::Svg => write!(dest, "svg"),
            ShExFormat::Png => write!(dest, "png"),
        }
    }
}

impl TryFrom<ShExFormat> for RDFFormat {
    type Error = ShExError;

    fn try_from(format: ShExFormat) -> Result<Self, Self::Error> {
        match format {
            ShExFormat::Turtle => Ok(RDFFormat::Turtle),
            ShExFormat::NTriples => Ok(RDFFormat::NTriples),
            ShExFormat::RdfXml => Ok(RDFFormat::Rdfxml),
            ShExFormat::TriG => Ok(RDFFormat::TriG),
            ShExFormat::N3 => Ok(RDFFormat::N3),
            ShExFormat::NQuads => Ok(RDFFormat::NQuads),
            other => Err(ShExError::FailedSerializingShExSchema {
                format: other.to_string(),
                error: "not an RDF serialization format".to_string(),
            }),
        }
    }
}

impl TryFrom<ShExFormat> for ShExAstShExFormat {
    type Error = RudofError;

    fn try_from(format: ShExFormat) -> Result<Self, Self::Error> {
        match format {
            ShExFormat::ShExC => Ok(ShExAstShExFormat::ShExC),
            ShExFormat::ShExJ | ShExFormat::Json | ShExFormat::JsonLd => Ok(ShExAstShExFormat::ShExJ),
            other => Err(RudofError::NotImplemented {
                msg: format!("ShEx format {other:?} validation not yet implemented"),
            }),
        }
    }
}

impl TryFrom<&ShExFormat> for ShExAstShExFormat {
    type Error = RudofError;

    fn try_from(format: &ShExFormat) -> Result<Self, Self::Error> {
        (*format).try_into()
    }
}

impl FromStr for ShExFormat {
    type Err = ShExError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "internal" => Ok(ShExFormat::Internal),
            "simple" => Ok(ShExFormat::Simple),
            "shexc" => Ok(ShExFormat::ShExC),
            "shexj" => Ok(ShExFormat::ShExJ),
            "json" => Ok(ShExFormat::Json),
            "jsonld" => Ok(ShExFormat::JsonLd),
            "turtle" => Ok(ShExFormat::Turtle),
            "ntriples" => Ok(ShExFormat::NTriples),
            "rdfxml" => Ok(ShExFormat::RdfXml),
            "trig" => Ok(ShExFormat::TriG),
            "n3" => Ok(ShExFormat::N3),
            "nquads" => Ok(ShExFormat::NQuads),
            "plantuml" => Ok(ShExFormat::PlantUML),
            "svg" => Ok(ShExFormat::Svg),
            "png" => Ok(ShExFormat::Png),
            other => Err(ShExError::UnsupportedShExFormat {
                format: other.to_string(),
            }),
        }
    }
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
            ShExFormat::RdfXml => "application/rdf+xml",
            ShExFormat::TriG => "application/trig",
            ShExFormat::N3 => "text/n3",
            ShExFormat::NQuads => "application/n-quads",
            ShExFormat::Json => "application/json",
            ShExFormat::JsonLd => "application/ld+json",
            ShExFormat::PlantUML => "text/plain",
            ShExFormat::Svg => "image/svg+xml",
            ShExFormat::Png => "image/png",
        }
    }
}
