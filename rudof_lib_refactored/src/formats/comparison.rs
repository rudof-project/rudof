use iri_s::MimeType;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::{errors::ComparisonError, formats::{DCTapFormat, ShExFormat, ShaclFormat}};

/// Schema comparison modes supported by Rudof.
///
/// Specifies which schema language is being used when comparing two schemas.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ComparisonMode {
    /// SHACL schema comparison
    Shacl,
    /// ShEx (Shape Expressions) schema comparison
    ShEx,
    /// DC-TAP (Dublin Core Tabular Application Profiles) comparison
    Dctap,
    /// Service-based comparison (external service)
    Service,
}

/// Input formats for schema comparison supported by Rudof.
///
/// Represents the serialization formats accepted as input when comparing schemas.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ComparisonFormat {
    /// ShExC - Compact ShEx syntax
    ShExC,
    /// ShExJ - ShEx JSON format
    ShExJ,
    /// Turtle - Compact RDF format
    Turtle,
    /// RDF/XML - XML-based RDF serialization
    RdfXml,
    /// N-Triples - Line-based RDF format
    NTriples,
}

/// Output formats for schema comparison results supported by Rudof.
///
/// Represents serialization formats for the results of schema comparison operations.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ResultComparisonFormat {
    #[default]
    Internal,
    Json,
}

// ============================================================================
// ComparisonMode
// ============================================================================

impl Display for ComparisonMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ComparisonMode::Shacl => write!(dest, "shacl"),
            ComparisonMode::ShEx => write!(dest, "shex"),
            ComparisonMode::Dctap => write!(dest, "dctap"),
            ComparisonMode::Service => write!(dest, "service"),
        }
    }
}

impl FromStr for ComparisonMode {
    type Err = ComparisonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shacl" => Ok(ComparisonMode::Shacl),
            "shex" => Ok(ComparisonMode::ShEx),
            "dctap" => Ok(ComparisonMode::Dctap),
            "service" => Ok(ComparisonMode::Service),
            other => Err(ComparisonError::UnsupportedComparisonMode {
                mode: other.to_string(),
            }),
        }
    }
}

// ============================================================================
// ComparisonFormat
// ============================================================================

impl From<ComparisonFormat> for ShExFormat {
    fn from(format: ComparisonFormat) -> Self {
        match format {
            ComparisonFormat::ShExC => ShExFormat::ShExC,
            ComparisonFormat::ShExJ => ShExFormat::ShExJ,
            ComparisonFormat::Turtle => ShExFormat::Turtle,
            ComparisonFormat::RdfXml => ShExFormat::RdfXml,
            ComparisonFormat::NTriples => ShExFormat::NTriples,
        }
    }
}

impl TryFrom<ComparisonFormat> for ShaclFormat {
    type Error = ComparisonError;

    fn try_from(format: ComparisonFormat) -> Result<Self, Self::Error> {
        match format {
            ComparisonFormat::Turtle => Ok(ShaclFormat::Turtle),
            ComparisonFormat::NTriples => Ok(ShaclFormat::NTriples),
            ComparisonFormat::RdfXml => Ok(ShaclFormat::RdfXml),
            ComparisonFormat::ShExC => Err(ComparisonError::UnsupportedConversionToShacl {
                format: "ShExC".to_string(),
            }),
            ComparisonFormat::ShExJ => Err(ComparisonError::UnsupportedConversionToShacl {
                format: "ShExJ".to_string(),
            }),
        }
    }
}

impl TryFrom<ComparisonFormat> for DCTapFormat {
    type Error = ComparisonError;

    fn try_from(format: ComparisonFormat) -> Result<Self, Self::Error> {
        Err(ComparisonError::UnsupportedConversionToDCTap {
            format: format.to_string(),
        })
    }
}

impl FromStr for ComparisonFormat {
    type Err = ComparisonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shexc" => Ok(ComparisonFormat::ShExC),
            "shexj" => Ok(ComparisonFormat::ShExJ),
            "turtle" => Ok(ComparisonFormat::Turtle),
            "rdfxml" => Ok(ComparisonFormat::RdfXml),
            "ntriples" => Ok(ComparisonFormat::NTriples),
            other => Err(ComparisonError::UnsupportedComparisonFormat {
                format: other.to_string(),
            }),
        }
    }
}

impl Display for ComparisonFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ComparisonFormat::ShExC => write!(dest, "shexc"),
            ComparisonFormat::ShExJ => write!(dest, "shexj"),
            ComparisonFormat::Turtle => write!(dest, "turtle"),
            ComparisonFormat::RdfXml => write!(dest, "rdfxml"),
            ComparisonFormat::NTriples => write!(dest, "ntriples"),
        }
    }
}

impl MimeType for ComparisonFormat {
    fn mime_type(&self) -> &'static str {
        match &self {
            ComparisonFormat::ShExC => "text/shex",
            ComparisonFormat::ShExJ => "application/json",
            ComparisonFormat::Turtle => "text/turtle",
            ComparisonFormat::RdfXml => "application/rdf+xml",
            ComparisonFormat::NTriples => "application/n-triples",
        }
    }
}

// ============================================================================
// ResultComparisonFormat
// ============================================================================

impl Display for ResultComparisonFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultComparisonFormat::Internal => write!(dest, "internal"),
            ResultComparisonFormat::Json => write!(dest, "json"),
        }
    }
}

impl FromStr for ResultComparisonFormat {
    type Err = ComparisonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "internal" => Ok(ResultComparisonFormat::Internal),
            "json" => Ok(ResultComparisonFormat::Json),
            other => Err(ComparisonError::UnsupportedResultComparisonFormat {
                format: other.to_string(),
            }),
        }
    }
}
