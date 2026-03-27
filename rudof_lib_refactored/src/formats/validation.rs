use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use shex_ast::shapemap::result_shape_map::SortMode;
use shacl_validation::validation_report::report::SortModeReport;
use rudof_rdf::rdf_core::RDFFormat;
use shacl_validation::shacl_processor::ShaclValidationMode as ShaclValidationModeReport;
use crate::{formats::ShapeMapFormat, errors::ValidationError};

/// Validation modes supported by Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ValidationMode {
    /// ShEx (Shape Expressions) - validation for RDF data using shape-based constraints
    ShEx,
    /// SHACL (Shapes Constraint Language) - W3C standard for RDF validation
    Shacl,
    /// Property Graph schema validation
    PGSchema,
}

/// Backends used for SHACL validation supported by Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ShaclValidationMode {
    /// Rust native engine using functions implemented with Rust native code (default)
    #[default]
    Native,
    /// SPARQL-based engine using SPARQL queries to validate the data
    Sparql,
}

/// Sorting modes for validation results supported by Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ValidationSortByMode {
    /// Sort by node - groups results by RDF node/entity (default)
    #[default]
    Node,
    /// Sort by details - groups results by validation detail level
    Details,
}

/// Sorting modes for ShEx validation results supported by Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ShExValidationSortByMode {
    /// Sort by node - groups results by RDF node/entity (default)
    #[default]
    Node,
    /// Sort by shape - groups results by ShEx shape
    Shape,
    /// Sort by status - groups results by validation status (pass/fail)
    Status,
    /// Sort by details - groups results by validation detail level
    Details,
}

/// Sorting modes for SHACL validation results supported by Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ShaclValidationSortByMode {
    /// Sort by severity - groups results by violation severity level (default)
    #[default]
    Severity,
    /// Sort by node - groups results by the focus node being validated
    Node,
    /// Sort by component - groups results by SHACL constraint component
    Component,
    /// Sort by value - groups results by the actual value that caused the violation
    Value,
    /// Sort by path - groups results by the property path where violations occur
    Path,
    /// Sort by source shape - groups results by the SHACL shape that was violated
    SourceShape,
    /// Sort by details - groups results by validation detail level
    Details,
}

/// Output formats for validation results supported by Rudof.
///
/// Represents serialization formats for validation outputs, supporting both
/// RDF-based formats (for ShEx/SHACL) and property graph formats (for PGSchema).
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ResultValidationFormat {
    /// Turtle - compact, human-readable RDF format
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
    /// Compact format - concise validation output
    Compact,
    /// Details format - verbose output with validation details (default)
    #[default]
    Details,
    /// JSON format - machine-readable JSON serialization
    Json,
    /// CSV format - comma-separated values for spreadsheet tools
    Csv,
}

/// Output formats for ShEx validation results supported by Rudof.
/// 
/// Represents serialization formats specifically for ShEx validation outputs.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ResultShExValidationFormat {
    /// Details format - verbose output with validation details (default)
    #[default]
    Details,
    /// Turtle - compact, human-readable RDF format
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
    /// Compact format - concise ShapeMap representation
    Compact,
    /// JSON format - machine-readable JSON serialization
    Json,
    /// CSV format - comma-separated values for spreadsheet tools
    Csv,
}

/// Output formats for SHACL validation results.
///
/// Represents serialization formats specifically for SHACL validation outputs.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ResultShaclValidationFormat {
    /// Details format - verbose output with validation details (default)
    #[default]
    Details,
    /// Turtle - compact, human-readable RDF format
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
    /// Minimal format - minimal validation report output
    Minimal,
    /// Compact format - concise validation output
    Compact,
    /// JSON format - machine-readable JSON serialization
    Json,
    /// CSV format - comma-separated values for spreadsheet tools
    Csv,
}

/// Output formats for Property Graph schema validation results.
///
/// Represents serialization formats specifically for Property Graph schema validation outputs.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ResultPgSchemaValidationFormat {
    /// Compact format - concise validation output (default)
    #[default]
    Compact,
    /// Details format - verbose output with validation details
    Details,
    /// JSON format - machine-readable JSON serialization
    Json,
    /// CSV format - comma-separated values for spreadsheet tools
    Csv,
}

// ============================================================================
// ValidationMode
// ============================================================================

impl Display for ValidationMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ValidationMode::ShEx => write!(dest, "shex"),
            ValidationMode::Shacl => write!(dest, "shacl"),
            ValidationMode::PGSchema => write!(dest, "pgschema"),
        }
    }
}

impl FromStr for ValidationMode {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shex" => Ok(ValidationMode::ShEx),
            "shacl" => Ok(ValidationMode::Shacl),
            "pgschema" => Ok(ValidationMode::PGSchema),
            other => Err(ValidationError::UnsupportedValidationMode {
                mode: other.to_string(),
            }),
        }
    }
}

// ============================================================================
// ShaclValidationMode
// ============================================================================

impl Display for ShaclValidationMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShaclValidationMode::Native => write!(dest, "native"),
            ShaclValidationMode::Sparql => write!(dest, "sparql"),
        }
    }
}

impl FromStr for ShaclValidationMode {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "native" => Ok(ShaclValidationMode::Native),
            "sparql" => Ok(ShaclValidationMode::Sparql),
            other => Err(ValidationError::UnsupportedSHACLValidationMode {
                mode: other.to_string(),
            }),
        }
    }
}

impl From<ShaclValidationMode> for ShaclValidationModeReport {
    fn from(mode: ShaclValidationMode) -> Self {
        match mode {
            ShaclValidationMode::Native => ShaclValidationModeReport::Native,
            ShaclValidationMode::Sparql => ShaclValidationModeReport::Sparql,
        }
    }
}

// ============================================================================
// ValidationSortByMode
// ============================================================================

impl Display for ValidationSortByMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ValidationSortByMode::Node => write!(dest, "node"),
            ValidationSortByMode::Details => write!(dest, "details"),
        }
    }
}

impl FromStr for ValidationSortByMode {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "node" => Ok(ValidationSortByMode::Node),
            "details" => Ok(ValidationSortByMode::Details),
            other => Err(ValidationError::UnsupportedValidationSortByMode {
                mode: other.to_string(),
            }),
        }
    }
}


// ============================================================================
// ShExValidationSortByMode
// ============================================================================

impl Display for ShExValidationSortByMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShExValidationSortByMode::Node => write!(dest, "node"),
            ShExValidationSortByMode::Shape => write!(dest, "shape"),
            ShExValidationSortByMode::Status => write!(dest, "status"),
            ShExValidationSortByMode::Details => write!(dest, "details"),
        }
    }
}

impl From<ShExValidationSortByMode> for SortMode {
    fn from(format: ShExValidationSortByMode) -> Self {
        match format {
            ShExValidationSortByMode::Node => SortMode::Node,
            ShExValidationSortByMode::Shape => SortMode::Shape,
            ShExValidationSortByMode::Status => SortMode::Status,
            ShExValidationSortByMode::Details => SortMode::Details,
        }
    }
}

impl From<&ShExValidationSortByMode> for SortMode {
    fn from(format: &ShExValidationSortByMode) -> Self {
        (*format).into()
    }
}

impl FromStr for ShExValidationSortByMode {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "node" => Ok(ShExValidationSortByMode::Node),
            "shape" => Ok(ShExValidationSortByMode::Shape),
            "status" => Ok(ShExValidationSortByMode::Status),
            "details" => Ok(ShExValidationSortByMode::Details),
            other => Err(ValidationError::UnsupportedShExValidationSortByMode {
                mode: other.to_string(),
            }),
        }
    }
}

// ============================================================================
// ShaclValidationSortByMode
// ============================================================================

impl Display for ShaclValidationSortByMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShaclValidationSortByMode::Severity => write!(dest, "severity"),
            ShaclValidationSortByMode::Node => write!(dest, "node"),
            ShaclValidationSortByMode::Component => write!(dest, "component"),
            ShaclValidationSortByMode::Value => write!(dest, "value"),
            ShaclValidationSortByMode::Path => write!(dest, "path"),
            ShaclValidationSortByMode::SourceShape => write!(dest, "sourceShape"),
            ShaclValidationSortByMode::Details => write!(dest, "details"),
        }
    }
}

impl From<ShaclValidationSortByMode> for SortModeReport {
    fn from(sort_by: ShaclValidationSortByMode) -> Self {
        match sort_by {
            ShaclValidationSortByMode::Severity => SortModeReport::Severity,
            ShaclValidationSortByMode::Node => SortModeReport::Node,
            ShaclValidationSortByMode::Component => SortModeReport::Component,
            ShaclValidationSortByMode::Value => SortModeReport::Value,
            ShaclValidationSortByMode::Path => SortModeReport::Path,
            ShaclValidationSortByMode::SourceShape => SortModeReport::Source,
            ShaclValidationSortByMode::Details => SortModeReport::Details,
        }
    }
}

impl FromStr for ShaclValidationSortByMode {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "severity" => Ok(ShaclValidationSortByMode::Severity),
            "node" => Ok(ShaclValidationSortByMode::Node),
            "component" => Ok(ShaclValidationSortByMode::Component),
            "value" => Ok(ShaclValidationSortByMode::Value),
            "path" => Ok(ShaclValidationSortByMode::Path),
            "sourceshape" => Ok(ShaclValidationSortByMode::SourceShape),
            "details" => Ok(ShaclValidationSortByMode::Details),
            other => Err(ValidationError::UnsupportedShaclValidationSortByMode {
                mode: other.to_string(),
            }),
        }
    }
}

// ============================================================================
// ResultValidationFormat
// ============================================================================

impl ResultValidationFormat {
    /// Checks if this is an RDF-based format.
    ///
    /// # Returns
    /// `true` for Turtle, NTriples, RdfXml, TriG, N3, and NQuads.
    pub fn is_rdf_format(&self) -> bool {
        matches!(
            self,
            ResultValidationFormat::Turtle
                | ResultValidationFormat::NTriples
                | ResultValidationFormat::RdfXml
                | ResultValidationFormat::TriG
                | ResultValidationFormat::N3
                | ResultValidationFormat::NQuads
        )
    }

    /// Checks if this is a Property Graph-compatible format.
    ///
    /// # Returns
    /// `true` for Compact, Details, Json, and Csv.
    pub fn is_pg_format(&self) -> bool {
        matches!(
            self,
            ResultValidationFormat::Compact
                | ResultValidationFormat::Details
                | ResultValidationFormat::Json
                | ResultValidationFormat::Csv
        )
    }
}

impl Display for ResultValidationFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultValidationFormat::Turtle => write!(dest, "turtle"),
            ResultValidationFormat::NTriples => write!(dest, "ntriples"),
            ResultValidationFormat::RdfXml => write!(dest, "rdfxml"),
            ResultValidationFormat::TriG => write!(dest, "trig"),
            ResultValidationFormat::N3 => write!(dest, "n3"),
            ResultValidationFormat::NQuads => write!(dest, "nquads"),
            ResultValidationFormat::Compact => write!(dest, "compact"),
            ResultValidationFormat::Json => write!(dest, "json"),
            ResultValidationFormat::Details => write!(dest, "details"),
            ResultValidationFormat::Csv => write!(dest, "csv"),
        }
    }
}

impl FromStr for ResultValidationFormat {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "turtle" => Ok(ResultValidationFormat::Turtle),
            "ntriples" => Ok(ResultValidationFormat::NTriples),
            "rdfxml" => Ok(ResultValidationFormat::RdfXml),
            "trig" => Ok(ResultValidationFormat::TriG),
            "n3" => Ok(ResultValidationFormat::N3),
            "nquads" => Ok(ResultValidationFormat::NQuads),
            "compact" => Ok(ResultValidationFormat::Compact),
            "details" => Ok(ResultValidationFormat::Details),
            "json" => Ok(ResultValidationFormat::Json),
            "csv" => Ok(ResultValidationFormat::Csv),
            other => Err(ValidationError::UnsupportedValidationResultFormat {
                format: other.to_string(),
            }),
        }
    }
}

impl From<ResultValidationFormat> for ResultShExValidationFormat {
    fn from(format: ResultValidationFormat) -> Self {
        match format {
            ResultValidationFormat::Turtle => ResultShExValidationFormat::Turtle,
            ResultValidationFormat::NTriples => ResultShExValidationFormat::NTriples,
            ResultValidationFormat::RdfXml => ResultShExValidationFormat::RdfXml,
            ResultValidationFormat::TriG => ResultShExValidationFormat::TriG,
            ResultValidationFormat::N3 => ResultShExValidationFormat::N3,
            ResultValidationFormat::NQuads => ResultShExValidationFormat::NQuads,
            ResultValidationFormat::Compact => ResultShExValidationFormat::Compact,
            ResultValidationFormat::Details => ResultShExValidationFormat::Details,
            ResultValidationFormat::Json => ResultShExValidationFormat::Json,
            ResultValidationFormat::Csv => ResultShExValidationFormat::Csv,
        }
    }
}

impl From<ResultValidationFormat> for ResultShaclValidationFormat {
    fn from(format: ResultValidationFormat) -> Self {
        match format {
            ResultValidationFormat::Turtle => ResultShaclValidationFormat::Turtle,
            ResultValidationFormat::NTriples => ResultShaclValidationFormat::NTriples,
            ResultValidationFormat::RdfXml => ResultShaclValidationFormat::RdfXml,
            ResultValidationFormat::TriG => ResultShaclValidationFormat::TriG,
            ResultValidationFormat::N3 => ResultShaclValidationFormat::N3,
            ResultValidationFormat::NQuads => ResultShaclValidationFormat::NQuads,
            ResultValidationFormat::Compact => ResultShaclValidationFormat::Compact,
            ResultValidationFormat::Details => ResultShaclValidationFormat::Details,
            ResultValidationFormat::Json => ResultShaclValidationFormat::Json,
            ResultValidationFormat::Csv => ResultShaclValidationFormat::Csv,
        }
    }
}

impl TryFrom<ResultValidationFormat> for ResultPgSchemaValidationFormat {
    type Error = ValidationError;

    fn try_from(format: ResultValidationFormat) -> Result<Self, Self::Error> {
        match format {
            ResultValidationFormat::Compact => Ok(ResultPgSchemaValidationFormat::Compact),
            ResultValidationFormat::Details => Ok(ResultPgSchemaValidationFormat::Details),
            ResultValidationFormat::Json => Ok(ResultPgSchemaValidationFormat::Json),
            ResultValidationFormat::Csv => Ok(ResultPgSchemaValidationFormat::Csv),
            other => Err(ValidationError::UnsupportedConversionToPgSchemaValidationResultFormat {
                format: other.to_string(),
            }),
        }
    }
}

// ============================================================================
// ResultShExValidationFormat
// ============================================================================

impl Display for ResultShExValidationFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultShExValidationFormat::Turtle => write!(dest, "turtle"),
            ResultShExValidationFormat::NTriples => write!(dest, "ntriples"),
            ResultShExValidationFormat::RdfXml => write!(dest, "rdfxml"),
            ResultShExValidationFormat::TriG => write!(dest, "trig"),
            ResultShExValidationFormat::N3 => write!(dest, "n3"),
            ResultShExValidationFormat::NQuads => write!(dest, "nquads"),
            ResultShExValidationFormat::Compact => write!(dest, "compact"),
            ResultShExValidationFormat::Json => write!(dest, "json"),
            ResultShExValidationFormat::Details => write!(dest, "details"),
            ResultShExValidationFormat::Csv => write!(dest, "csv"),
        }
    }
}

impl TryFrom<ResultShExValidationFormat> for ShapeMapFormat {
    type Error = ValidationError;

    fn try_from(format: ResultShExValidationFormat) -> Result<Self, Self::Error> {
        match format {
            ResultShExValidationFormat::Compact => Ok(ShapeMapFormat::Compact),
            ResultShExValidationFormat::Details => Ok(ShapeMapFormat::Details),
            ResultShExValidationFormat::Json => Ok(ShapeMapFormat::Json),
            ResultShExValidationFormat::Csv => Ok(ShapeMapFormat::Csv),
            other => Err(ValidationError::UnsupportedConversionToShapeMap {
                format: format!("{other:?}"),
            }),
        }
    }
}

impl TryFrom<&ResultShExValidationFormat> for ShapeMapFormat {
    type Error = ValidationError;

    /// Attempts to convert a reference to ShapeMap format.
    fn try_from(format: &ResultShExValidationFormat) -> Result<Self, Self::Error> {
        (*format).try_into()
    }
}

impl FromStr for ResultShExValidationFormat {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "turtle" => Ok(ResultShExValidationFormat::Turtle),
            "ntriples" => Ok(ResultShExValidationFormat::NTriples),
            "rdfxml" => Ok(ResultShExValidationFormat::RdfXml),
            "trig" => Ok(ResultShExValidationFormat::TriG),
            "n3" => Ok(ResultShExValidationFormat::N3),
            "nquads" => Ok(ResultShExValidationFormat::NQuads),
            "compact" => Ok(ResultShExValidationFormat::Compact),
            "details" => Ok(ResultShExValidationFormat::Details),
            "json" => Ok(ResultShExValidationFormat::Json),
            "csv" => Ok(ResultShExValidationFormat::Csv),
            other => Err(ValidationError::UnsupportedShExValidationResultFormat {
                format: other.to_string(),
            }),
        }
    }
}

// ============================================================================
// ResultShaclValidationFormat
// ============================================================================

impl Display for ResultShaclValidationFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultShaclValidationFormat::Turtle => write!(dest, "turtle"),
            ResultShaclValidationFormat::NTriples => write!(dest, "ntriples"),
            ResultShaclValidationFormat::RdfXml => write!(dest, "rdfxml"),
            ResultShaclValidationFormat::TriG => write!(dest, "trig"),
            ResultShaclValidationFormat::N3 => write!(dest, "n3"),
            ResultShaclValidationFormat::NQuads => write!(dest, "nquads"),
            ResultShaclValidationFormat::Compact => write!(dest, "compact"),
            ResultShaclValidationFormat::Minimal => write!(dest, "minimal"),
            ResultShaclValidationFormat::Details => write!(dest, "details"),
            ResultShaclValidationFormat::Json => write!(dest, "json"),
            ResultShaclValidationFormat::Csv => write!(dest, "csv"),
        }
    }
}

impl TryFrom<ResultShaclValidationFormat> for RDFFormat {
    type Error = ValidationError;

    fn try_from(format: ResultShaclValidationFormat) -> Result<Self, Self::Error> {
        match format {
            ResultShaclValidationFormat::Turtle => Ok(RDFFormat::Turtle),
            ResultShaclValidationFormat::NTriples => Ok(RDFFormat::NTriples),
            ResultShaclValidationFormat::RdfXml => Ok(RDFFormat::Rdfxml),
            ResultShaclValidationFormat::TriG => Ok(RDFFormat::TriG),
            ResultShaclValidationFormat::N3 => Ok(RDFFormat::N3),
            ResultShaclValidationFormat::NQuads => Ok(RDFFormat::NQuads),
            other => Err(ValidationError::UnsupportedConversionToRDFFormat {
                format: other.to_string(),
            }),
        }
    }
}


impl FromStr for ResultShaclValidationFormat {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "turtle" => Ok(ResultShaclValidationFormat::Turtle),
            "ntriples" => Ok(ResultShaclValidationFormat::NTriples),
            "rdfxml" => Ok(ResultShaclValidationFormat::RdfXml),
            "trig" => Ok(ResultShaclValidationFormat::TriG),
            "n3" => Ok(ResultShaclValidationFormat::N3),
            "nquads" => Ok(ResultShaclValidationFormat::NQuads),
            "minimal" => Ok(ResultShaclValidationFormat::Minimal),
            "compact" => Ok(ResultShaclValidationFormat::Compact),
            "details" => Ok(ResultShaclValidationFormat::Details),
            "json" => Ok(ResultShaclValidationFormat::Json),
            "csv" => Ok(ResultShaclValidationFormat::Csv),
            other => Err(ValidationError::UnsupportedShaclValidationResultFormat {
                format: other.to_string(),
            }),
        }
    }
}

// ============================================================================
// ResultPgSchemaValidationFormat
// ============================================================================

impl Display for ResultPgSchemaValidationFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ResultPgSchemaValidationFormat::Compact => "compact",
            ResultPgSchemaValidationFormat::Details => "details",
            ResultPgSchemaValidationFormat::Json => "json",
            ResultPgSchemaValidationFormat::Csv => "csv",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for ResultPgSchemaValidationFormat {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "compact" => Ok(ResultPgSchemaValidationFormat::Compact),
            "details" => Ok(ResultPgSchemaValidationFormat::Details),
            "json" => Ok(ResultPgSchemaValidationFormat::Json),
            "csv" => Ok(ResultPgSchemaValidationFormat::Csv),
            other => Err(ValidationError::UnsupportedPgSchemaValidationResultFormat {
                format: other.to_string(),
            }),
        }
    }
}
