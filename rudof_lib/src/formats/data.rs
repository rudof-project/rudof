use crate::errors::DataError;
use iri_s::MimeType;
use rudof_generate::config::OutputFormat;
use rudof_rdf::{
    rdf_core::{RDFFormat, visualizer::uml_converter::ImageFormat},
    rdf_impl::ReaderMode,
};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// Data formats supported by Rudof for input.
///
/// Represents all data serialization formats that Rudof can read as input.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum DataFormat {
    /// Turtle - a compact, human-readable RDF format (default)
    #[default]
    Turtle,
    /// N-Triples - a line-based RDF format with one triple per line
    NTriples,
    //// RDF/XML - XML-based RDF serialization format
    RdfXml,
    /// TriG - extends Turtle with support for named graphs
    TriG,
    /// Notation3 - a superset of Turtle with additional features
    N3,
    /// N-Quads - extends N-Triples with support for named graphs
    NQuads,
    /// JSON-LD - JSON format for Linked Data
    JsonLd,
    /// Property Graph format - represents data as nodes and edges (non-RDF)
    Pg,
}

/// Data parser mode for error handling during data reading.
///
/// Determines how Rudof handles errors and malformed data when parsing input data.
/// This controls the parser's tolerance level for syntactic issues.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum DataReaderMode {
    /// Lenient parsing mode that attempts error recovery.
    Lax,
    /// Strict parsing mode that fails on any error (default).
    #[default]
    Strict,
}

/// Output formats for result data in Rudof.
///
/// Represents all serialization formats available for exporting and outputting
/// data from Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ResultDataFormat {
    /// Turtle - a compact, human-readable RDF format (default)
    #[default]
    Turtle,
    /// N-Triples - a line-based RDF format with one triple per line
    NTriples,
    /// JSON-LD - JSON format for Linked Data
    JsonLd,
    /// RDF/XML - XML-based RDF serialization format
    RdfXml,
    /// TriG - extends Turtle with support for named graphs
    TriG,
    /// Notation3 - a superset of Turtle with additional features
    N3,
    /// N-Quads - extends N-Triples with support for named graphs
    NQuads,
    /// Compact - condensed output format for results
    Compact,
    /// JSON - alias for JSON-LD when serializing RDF data
    Json,
    /// PlantUML - text-based UML diagram format for visualization
    PlantUML,
    /// SVG - Scalable Vector Graphics image format for visual output
    Svg,
    /// PNG - Portable Network Graphics image format for visual output
    Png,
}

// ============================================================================
// DataFormat
// ============================================================================

impl TryFrom<DataFormat> for RDFFormat {
    type Error = Box<DataError>;

    fn try_from(value: DataFormat) -> Result<Self, Self::Error> {
        match value {
            DataFormat::Turtle => Ok(RDFFormat::Turtle),
            DataFormat::NTriples => Ok(RDFFormat::NTriples),
            DataFormat::RdfXml => Ok(RDFFormat::Rdfxml),
            DataFormat::TriG => Ok(RDFFormat::TriG),
            DataFormat::N3 => Ok(RDFFormat::N3),
            DataFormat::NQuads => Ok(RDFFormat::NQuads),
            DataFormat::JsonLd => Ok(RDFFormat::JsonLd),
            DataFormat::Pg => Err(Box::new(DataError::NonRdfFormat {
                format: value.to_string(),
            })),
        }
    }
}

impl TryFrom<&DataFormat> for RDFFormat {
    type Error = Box<DataError>;

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

impl From<DataFormat> for OutputFormat {
    fn from(value: DataFormat) -> Self {
        match value {
            DataFormat::Turtle | DataFormat::TriG | DataFormat::N3 => OutputFormat::Turtle,
            _ => OutputFormat::NTriples,
        }
    }
}

impl FromStr for DataFormat {
    type Err = DataError;

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
            other => Err(DataError::UnsupportedDataFormat {
                format: other.to_string(),
            }),
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

// ============================================================================
// DataReaderMode
// ============================================================================

impl From<DataReaderMode> for ReaderMode {
    fn from(format: DataReaderMode) -> Self {
        match format {
            DataReaderMode::Strict => ReaderMode::Strict,
            DataReaderMode::Lax => ReaderMode::Lax,
        }
    }
}

impl From<&DataReaderMode> for ReaderMode {
    fn from(format: &DataReaderMode) -> Self {
        (*format).into()
    }
}

impl FromStr for DataReaderMode {
    type Err = DataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "strict" => Ok(DataReaderMode::Strict),
            "lax" => Ok(DataReaderMode::Lax),
            other => Err(DataError::UnsupportedReaderMode {
                mode: other.to_string(),
            }),
        }
    }
}

impl Display for DataReaderMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self {
            DataReaderMode::Strict => write!(dest, "strict"),
            DataReaderMode::Lax => write!(dest, "lax"),
        }
    }
}

// ============================================================================
// ResultDataFormat
// ============================================================================

impl ResultDataFormat {
    pub fn is_image_visualization_format(&self) -> bool {
        matches!(self, ResultDataFormat::Svg | ResultDataFormat::Png)
    }

    pub fn is_rdf_format(&self) -> bool {
        matches!(
            self,
            ResultDataFormat::Turtle
                | ResultDataFormat::NTriples
                | ResultDataFormat::RdfXml
                | ResultDataFormat::TriG
                | ResultDataFormat::N3
                | ResultDataFormat::NQuads
                | ResultDataFormat::JsonLd
                | ResultDataFormat::Json
        )
    }
}

impl FromStr for ResultDataFormat {
    type Err = DataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "turtle" => Ok(ResultDataFormat::Turtle),
            "ntriples" => Ok(ResultDataFormat::NTriples),
            "jsonld" | "json-ld" => Ok(ResultDataFormat::JsonLd),
            "rdfxml" => Ok(ResultDataFormat::RdfXml),
            "trig" => Ok(ResultDataFormat::TriG),
            "n3" => Ok(ResultDataFormat::N3),
            "nquads" => Ok(ResultDataFormat::NQuads),
            "compact" => Ok(ResultDataFormat::Compact),
            "json" => Ok(ResultDataFormat::Json),
            "plantuml" => Ok(ResultDataFormat::PlantUML),
            "svg" => Ok(ResultDataFormat::Svg),
            "png" => Ok(ResultDataFormat::Png),
            _ => Err(DataError::UnsupportedResultDataFormat { format: s.to_string() }),
        }
    }
}

impl Display for ResultDataFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultDataFormat::Turtle => write!(dest, "turtle"),
            ResultDataFormat::NTriples => write!(dest, "ntriples"),
            ResultDataFormat::JsonLd => write!(dest, "jsonld"),
            ResultDataFormat::RdfXml => write!(dest, "rdfxml"),
            ResultDataFormat::TriG => write!(dest, "trig"),
            ResultDataFormat::N3 => write!(dest, "n3"),
            ResultDataFormat::NQuads => write!(dest, "nquads"),
            ResultDataFormat::Compact => write!(dest, "compact"),
            ResultDataFormat::Json => write!(dest, "json"),
            ResultDataFormat::PlantUML => write!(dest, "plantuml"),
            ResultDataFormat::Svg => write!(dest, "svg"),
            ResultDataFormat::Png => write!(dest, "png"),
        }
    }
}

impl TryFrom<ResultDataFormat> for RDFFormat {
    type Error = Box<DataError>;

    fn try_from(value: ResultDataFormat) -> Result<Self, Self::Error> {
        match value {
            ResultDataFormat::Turtle => Ok(RDFFormat::Turtle),
            ResultDataFormat::NTriples => Ok(RDFFormat::NTriples),
            ResultDataFormat::RdfXml => Ok(RDFFormat::Rdfxml),
            ResultDataFormat::TriG => Ok(RDFFormat::TriG),
            ResultDataFormat::N3 => Ok(RDFFormat::N3),
            ResultDataFormat::NQuads => Ok(RDFFormat::NQuads),
            ResultDataFormat::JsonLd => Ok(RDFFormat::JsonLd),
            ResultDataFormat::Json => Ok(RDFFormat::JsonLd),
            _ => Err(Box::new(DataError::NonRdfFormat {
                format: value.to_string(),
            })),
        }
    }
}

impl TryFrom<ResultDataFormat> for ImageFormat {
    type Error = Box<DataError>;

    fn try_from(value: ResultDataFormat) -> Result<Self, Self::Error> {
        match value {
            ResultDataFormat::Svg => Ok(ImageFormat::SVG),
            ResultDataFormat::Png => Ok(ImageFormat::PNG),
            _ => Err(Box::new(DataError::NonImageVisualizationFormat {
                format: value.to_string(),
            })),
        }
    }
}
