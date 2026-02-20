use iri_s::MimeType;
use rudof_rdf::rdf_core::RDFFormat;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use thiserror::Error;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DataFormat {
    Turtle,
    NTriples,
    RdfXml,
    TriG,
    N3,
    NQuads,
    JsonLd,
    Pg,
}

#[derive(Error, Clone, Debug)]
pub enum DataFormatError {
    #[error("Non-RDF format: {format}")]
    NonRdfFormat { format: String },

    #[error("Unknown data format: {format}")]
    UnknownFormat { format: String },
}

impl TryFrom<DataFormat> for RDFFormat {
    type Error = DataFormatError;

    fn try_from(value: DataFormat) -> Result<Self, Self::Error> {
        match value {
            DataFormat::Turtle => Ok(RDFFormat::Turtle),
            DataFormat::NTriples => Ok(RDFFormat::NTriples),
            DataFormat::RdfXml => Ok(RDFFormat::Rdfxml),
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

impl TryFrom<&DataFormat> for RDFFormat {
    type Error = DataFormatError;

    fn try_from(value: &DataFormat) -> Result<Self, Self::Error> {
        (*value).try_into()
    }
}

impl From<RDFFormat> for DataFormat {
    fn from(val: RDFFormat) -> Self {
        match val {
            RDFFormat::Turtle => DataFormat::Turtle,
            RDFFormat::NTriples => DataFormat::NTriples,
            RDFFormat::Rdfxml => DataFormat::RdfXml,
            RDFFormat::TriG => DataFormat::TriG,
            RDFFormat::N3 => DataFormat::N3,
            RDFFormat::NQuads => DataFormat::NQuads,
            RDFFormat::JsonLd => DataFormat::JsonLd,
        }
    }
}

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

impl FromStr for DataFormat {
    type Err = DataFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "turtle" => Ok(DataFormat::Turtle),
            "ntriples" => Ok(DataFormat::NTriples),
            "rdfxml" => Ok(DataFormat::RdfXml),
            "trig" => Ok(DataFormat::TriG),
            "n3" => Ok(DataFormat::N3),
            "nquads" => Ok(DataFormat::NQuads),
            "jsonld" => Ok(DataFormat::JsonLd),
            "pg" => Ok(DataFormat::Pg),
            other => Err(DataFormatError::UnknownFormat {
                format: other.to_string(),
            }),
        }
    }
}

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
