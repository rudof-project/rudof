use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::{
    errors::ConversionError,
    formats::{DCTapFormat, ShExFormat, ShaclFormat},
};

/// Conversion input modes supported by Rudof.
///
/// Specifies which language is being converted from.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ConversionMode {
    /// Convert from SHACL schema
    Shacl,
    /// Convert from ShEx (Shape Expressions) schema
    ShEx,
    /// Convert from DC-TAP (Dublin Core Tabular Application Profiles)
    Dctap,
}

/// Input formats for conversion supported by Rudof.
///
/// Represents the serialization formats accepted as input when converting.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ConversionFormat {
    /// CSV - Comma-Separated Values format (for DC-TAP)
    Csv,
    /// ShExC - Compact ShEx syntax
    ShExC,
    /// ShExJ - ShEx JSON format
    ShExJ,
    /// Turtle - Compact RDF format
    Turtle,
    /// XLSX - Excel 2007+ format (for DC-TAP)
    Xlsx,
}

/// Conversion output modes supported by Rudof.
///
/// Specifies which language is being converted to.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ResultConversionMode {
    /// Convert to SPARQL queries
    Sparql,
    /// Convert to ShEx schema
    ShEx,
    /// Convert to UML diagram
    Uml,
    /// Convert to HTML documentation
    Html,
    /// Convert to SHACL schema
    Shacl,
}

/// Output formats for conversion supported by Rudof.
///
/// Represents the serialization formats accepted as output when converting.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ResultConversionFormat {
    /// Default format for the target conversion mode
    Default,
    /// Internal format - internal representation
    Internal,
    /// JSON format - machine-readable JSON serialization
    Json,
    /// ShExC - Compact ShEx syntax
    ShExC,
    /// ShExJ - ShEx JSON format
    ShExJ,
    /// Turtle - Compact RDF format
    Turtle,
    /// PlantUML - text-based UML diagram format
    PlantUML,
    /// HTML - HTML documentation format
    Html,
    /// SVG - Scalable Vector Graphics image format
    Svg,
    /// PNG - Portable Network Graphics image format
    Png,
}

// ============================================================================
// ConvertMode
// ============================================================================

impl Display for ConversionMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ConversionMode::Shacl => write!(dest, "shacl"),
            ConversionMode::ShEx => write!(dest, "shex"),
            ConversionMode::Dctap => write!(dest, "dctap"),
        }
    }
}

impl FromStr for ConversionMode {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shacl" => Ok(ConversionMode::Shacl),
            "shex" => Ok(ConversionMode::ShEx),
            "dctap" => Ok(ConversionMode::Dctap),
            other => Err(ConversionError::UnsupportedConversionMode {
                mode: other.to_string(),
            }),
        }
    }
}

// ============================================================================
// ConversionFormat
// ============================================================================

impl TryFrom<ConversionFormat> for ShExFormat {
    type Error = ConversionError;

    fn try_from(format: ConversionFormat) -> Result<Self, Self::Error> {
        match format {
            ConversionFormat::ShExC => Ok(ShExFormat::ShExC),
            ConversionFormat::ShExJ => Ok(ShExFormat::ShExJ),
            ConversionFormat::Turtle => Ok(ShExFormat::Turtle),
            other => Err(ConversionError::UnsupportedConversionToShEx {
                format: other.to_string(),
            }),
        }
    }
}

impl TryFrom<ConversionFormat> for ShaclFormat {
    type Error = ConversionError;

    fn try_from(format: ConversionFormat) -> Result<Self, Self::Error> {
        match format {
            ConversionFormat::Turtle => Ok(ShaclFormat::Turtle),
            other => Err(ConversionError::UnsupportedConversionToShacl {
                format: other.to_string(),
            }),
        }
    }
}

impl TryFrom<ConversionFormat> for DCTapFormat {
    type Error = ConversionError;

    fn try_from(format: ConversionFormat) -> Result<Self, Self::Error> {
        match format {
            ConversionFormat::Csv => Ok(DCTapFormat::Csv),
            ConversionFormat::Xlsx => Ok(DCTapFormat::Xlsx),
            other => Err(ConversionError::UnsupportedConversionToDCTap {
                format: other.to_string(),
            }),
        }
    }
}

impl FromStr for ConversionFormat {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(ConversionFormat::Csv),
            "xlsx" => Ok(ConversionFormat::Xlsx),
            "shexc" => Ok(ConversionFormat::ShExC),
            "shexj" => Ok(ConversionFormat::ShExJ),
            "turtle" => Ok(ConversionFormat::Turtle),
            other => Err(ConversionError::UnsupportedConversionFormat {
                format: other.to_string(),
            }),
        }
    }
}

impl Display for ConversionFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ConversionFormat::Csv => write!(dest, "csv"),
            ConversionFormat::Xlsx => write!(dest, "xlsx"),
            ConversionFormat::ShExC => write!(dest, "shexc"),
            ConversionFormat::ShExJ => write!(dest, "shexj"),
            ConversionFormat::Turtle => write!(dest, "turtle"),
        }
    }
}

// ============================================================================
// ResultConversionMode
// ============================================================================

impl Display for ResultConversionMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultConversionMode::Sparql => write!(dest, "sparql"),
            ResultConversionMode::ShEx => write!(dest, "shex"),
            ResultConversionMode::Uml => write!(dest, "uml"),
            ResultConversionMode::Html => write!(dest, "html"),
            ResultConversionMode::Shacl => write!(dest, "shacl"),
        }
    }
}

impl FromStr for ResultConversionMode {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sparql" => Ok(ResultConversionMode::Sparql),
            "shex" => Ok(ResultConversionMode::ShEx),
            "uml" => Ok(ResultConversionMode::Uml),
            "html" => Ok(ResultConversionMode::Html),
            "shacl" => Ok(ResultConversionMode::Shacl),
            other => Err(ConversionError::UnsupportedResultConversionMode {
                mode: other.to_string(),
            }),
        }
    }
}

// ============================================================================
// ResultConversionFormat
// ============================================================================

impl TryFrom<ResultConversionFormat> for ShExFormat {
    type Error = ConversionError;
    fn try_from(format: ResultConversionFormat) -> Result<Self, Self::Error> {
        match format {
            ResultConversionFormat::ShExC => Ok(ShExFormat::ShExC),
            ResultConversionFormat::ShExJ => Ok(ShExFormat::ShExJ),
            ResultConversionFormat::Turtle => Ok(ShExFormat::Turtle),
            other => Err(ConversionError::UnsupportedResultConversionFormatToShEx {
                format: other.to_string(),
            }),
        }
    }
}

impl TryFrom<ResultConversionFormat> for ShaclFormat {
    type Error = ConversionError;

    fn try_from(format: ResultConversionFormat) -> Result<Self, Self::Error> {
        match format {
            ResultConversionFormat::Default => Ok(ShaclFormat::Internal),
            ResultConversionFormat::Turtle => Ok(ShaclFormat::Turtle),
            other => Err(ConversionError::UnsupportedResultConversionFormatToShacl {
                format: other.to_string(),
            }),
        }
    }
}

impl Display for ResultConversionFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultConversionFormat::Internal => write!(dest, "internal"),
            ResultConversionFormat::Json => write!(dest, "json"),
            ResultConversionFormat::Default => write!(dest, "default"),
            ResultConversionFormat::ShExC => write!(dest, "shexc"),
            ResultConversionFormat::ShExJ => write!(dest, "shexj"),
            ResultConversionFormat::Turtle => write!(dest, "turtle"),
            ResultConversionFormat::PlantUML => write!(dest, "plantuml"),
            ResultConversionFormat::Html => write!(dest, "html"),
            ResultConversionFormat::Png => write!(dest, "png"),
            ResultConversionFormat::Svg => write!(dest, "svg"),
        }
    }
}

impl FromStr for ResultConversionFormat {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "default" => Ok(ResultConversionFormat::Default),
            "internal" => Ok(ResultConversionFormat::Internal),
            "json" => Ok(ResultConversionFormat::Json),
            "shexc" => Ok(ResultConversionFormat::ShExC),
            "shexj" => Ok(ResultConversionFormat::ShExJ),
            "turtle" => Ok(ResultConversionFormat::Turtle),
            "uml" => Ok(ResultConversionFormat::PlantUML),
            "html" => Ok(ResultConversionFormat::Html),
            "svg" => Ok(ResultConversionFormat::Svg),
            "png" => Ok(ResultConversionFormat::Png),
            _ => Err(ConversionError::UnsupportedResultConversionFormat { format: s.to_string() }),
        }
    }
}
