use clap::ValueEnum;
use iri_s::MimeType;
use rdf::rdf_core::RDFFormat;
use std::fmt::{Display, Formatter};
use thiserror::Error;

// Represents the various RDF data serialization formats
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum DataFormat {
    // RDF formats
    Turtle,
    NTriples,
    RdfXml,
    TriG,
    N3,
    NQuads,
    JsonLd,
    // For property graphs
    Pg,
}

// Converts a `DataFormat` into the corresponding `RDFFormat` from the `srdf` crate.
impl TryFrom<DataFormat> for RDFFormat {
    type Error = DataFormatError;

    fn try_from(value: DataFormat) -> Result<Self, Self::Error> {
        match value {
            DataFormat::Turtle => Ok(RDFFormat::Turtle),
            DataFormat::NTriples => Ok(RDFFormat::NTriples),
            DataFormat::RdfXml => Ok(RDFFormat::RdfXml),
            DataFormat::TriG => Ok(RDFFormat::TriG),
            DataFormat::N3 => Ok(RDFFormat::N3),
            DataFormat::NQuads => Ok(RDFFormat::NQuads),
            DataFormat::JsonLd => Ok(RDFFormat::JsonLd),
            DataFormat::Pg => Err(DataFormatError::NonRdfFormat {
                format: value.to_string(),
            }),
        }
    }
}

#[derive(Error, Clone, Debug)]
pub enum DataFormatError {
    #[error("Non RDF format: {format}")]
    NonRdfFormat { format: String },
}

// Converts an `RDFFormat` from the `srdf` crate into the corresponding `DataFormat`.
impl From<RDFFormat> for DataFormat {
    fn from(val: RDFFormat) -> Self {
        match val {
            RDFFormat::Turtle => DataFormat::Turtle,
            RDFFormat::NTriples => DataFormat::NTriples,
            RDFFormat::RdfXml => DataFormat::RdfXml,
            RDFFormat::TriG => DataFormat::TriG,
            RDFFormat::N3 => DataFormat::N3,
            RDFFormat::NQuads => DataFormat::NQuads,
            RDFFormat::JsonLd => DataFormat::JsonLd,
        }
    }
}

// Provides a string representation of the data format for display purposes.
impl Display for DataFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DataFormat::Turtle => write!(dest, "turtle"),
            DataFormat::NTriples => write!(dest, "ntriples"),
            DataFormat::RdfXml => write!(dest, "rdfxml"),
            DataFormat::TriG => write!(dest, "trig"),
            DataFormat::N3 => write!(dest, "n3"),
            DataFormat::NQuads => write!(dest, "nquads"),
            DataFormat::JsonLd => write!(dest, "jsonld"),
            DataFormat::Pg => write!(dest, "pg"),
        }
    }
}

// Provides the MIME type for each `DataFormat`.
impl MimeType for DataFormat {
    fn mime_type(&self) -> &'static str {
        match self {
            DataFormat::Turtle => "text/turtle",
            DataFormat::NTriples => "application/n-triples",
            DataFormat::RdfXml => "application/rdf+xml",
            DataFormat::TriG => "application/trig",
            DataFormat::N3 => "text/n3",
            DataFormat::NQuads => "application/n-quads",
            DataFormat::JsonLd => "application/ld+json",
            DataFormat::Pg => "application/pg",
        }
    }
}
