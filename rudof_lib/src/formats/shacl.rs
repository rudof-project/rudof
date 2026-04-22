use crate::errors::ShaclError;
use iri_s::MimeType;
use rudof_rdf::rdf_core::RDFFormat;
use shacl::types::ShaclFormat as InnerShaclFormat;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// SHACL schema formats supported by Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ShaclFormat {
    /// Internal format - internal representation for processing
    Internal,
    /// Turtle - compact, human-readable RDF format (default)
    #[default]
    Turtle,
    /// N-Triples - line-based RDF format with one triple per line
    NTriples,
    /// RDF/XML - XML-based RDF serialization format
    RdfXml,
    /// TriG - extends Turtle with support for named graphs
    TriG,
    /// Notation3 - superset of Turtle with additional features
    N3,
    /// N-Quads - extends N-Triples with support for named graphs
    NQuads,
    /// JSON-LD - JSON format for Linked Data
    JsonLd,
}

impl From<ShaclFormat> for InnerShaclFormat {
    fn from(format: ShaclFormat) -> Self {
        match format {
            ShaclFormat::Turtle => InnerShaclFormat::Turtle,
            ShaclFormat::RdfXml => InnerShaclFormat::RdfXml,
            ShaclFormat::NTriples => InnerShaclFormat::NTriples,
            ShaclFormat::TriG => InnerShaclFormat::TriG,
            ShaclFormat::N3 => InnerShaclFormat::N3,
            ShaclFormat::NQuads => InnerShaclFormat::NQuads,
            ShaclFormat::Internal => InnerShaclFormat::Internal,
            ShaclFormat::JsonLd => InnerShaclFormat::JsonLd,
        }
    }
}

impl From<&ShaclFormat> for InnerShaclFormat {
    fn from(format: &ShaclFormat) -> Self {
        (*format).into()
    }
}

impl TryFrom<ShaclFormat> for RDFFormat {
    type Error = ShaclError;

    fn try_from(format: ShaclFormat) -> Result<Self, Self::Error> {
        match format {
            ShaclFormat::Turtle => Ok(RDFFormat::Turtle),
            ShaclFormat::RdfXml => Ok(RDFFormat::Rdfxml),
            ShaclFormat::NTriples => Ok(RDFFormat::NTriples),
            ShaclFormat::TriG => Ok(RDFFormat::TriG),
            ShaclFormat::N3 => Ok(RDFFormat::N3),
            ShaclFormat::NQuads => Ok(RDFFormat::NQuads),
            ShaclFormat::JsonLd => Ok(RDFFormat::JsonLd),
            ShaclFormat::Internal => Err(ShaclError::InternalSHACLFormatNonReadable),
        }
    }
}

impl Display for ShaclFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShaclFormat::Internal => write!(dest, "internal"),
            ShaclFormat::Turtle => write!(dest, "turtle"),
            ShaclFormat::NTriples => write!(dest, "ntriples"),
            ShaclFormat::RdfXml => write!(dest, "rdfxml"),
            ShaclFormat::TriG => write!(dest, "trig"),
            ShaclFormat::N3 => write!(dest, "n3"),
            ShaclFormat::NQuads => write!(dest, "nquads"),
            ShaclFormat::JsonLd => write!(dest, "jsonld"),
        }
    }
}

impl FromStr for ShaclFormat {
    type Err = ShaclError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "internal" => Ok(ShaclFormat::Internal),
            "turtle" => Ok(ShaclFormat::Turtle),
            "ntriples" => Ok(ShaclFormat::NTriples),
            "rdfxml" => Ok(ShaclFormat::RdfXml),
            "trig" => Ok(ShaclFormat::TriG),
            "n3" => Ok(ShaclFormat::N3),
            "nquads" => Ok(ShaclFormat::NQuads),
            "jsonld" => Ok(ShaclFormat::JsonLd),
            other => Err(ShaclError::UnsupportedShaclSchemaFormat {
                format: other.to_string(),
            }),
        }
    }
}

impl MimeType for ShaclFormat {
    fn mime_type(&self) -> &'static str {
        match self {
            ShaclFormat::Turtle => "text/turtle",
            ShaclFormat::NTriples => "application/n-triples",
            ShaclFormat::RdfXml => "application/rdf+xml",
            ShaclFormat::TriG => "application/trig",
            ShaclFormat::N3 => "text/n3",
            ShaclFormat::NQuads => "application/n-quads",
            ShaclFormat::Internal => "text/turtle",
            ShaclFormat::JsonLd => "application/ld+json",
        }
    }
}
