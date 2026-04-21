use crate::errors::RdfConfigError;
use rdf_config::RdfConfigFormat as RdfConfigFormatEnum;
use std::fmt::Display;
use std::str::FromStr;

/// RDF-config file formats supported by Rudof.
///
/// RDF-config is a tool to generate SPARQL queries, schema diagrams, and files
/// required for Grasp, TogoStanza, and ShEx validators from simple YAML-based
/// configuration files.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum RdfConfigFormat {
    /// YAML - YAML-based RDF-config specification format (default)
    #[default]
    Yaml,
}

/// Output formats for RDF-config processing results.
///
/// Represents serialization formats for outputs generated from RDF-config specifications.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ResultRdfConfigFormat {
    /// Internal format - internal representation for processing (default)
    #[default]
    Internal,
    /// YAML - YAML-based output format
    Yaml,
}

// ============================================================================
// RdfConfigFormat
// ============================================================================

impl FromStr for RdfConfigFormat {
    type Err = RdfConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "yaml" => Ok(RdfConfigFormat::Yaml),
            other => Err(RdfConfigError::UnsupportedRdfConfigFormat {
                format: other.to_string(),
            }),
        }
    }
}

impl Display for RdfConfigFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RdfConfigFormat::Yaml => write!(f, "yaml"),
        }
    }
}

// ============================================================================
// ResultRdfConfigFormat
// ============================================================================

impl FromStr for ResultRdfConfigFormat {
    type Err = RdfConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "internal" => Ok(ResultRdfConfigFormat::Internal),
            "yaml" => Ok(ResultRdfConfigFormat::Yaml),
            other => Err(RdfConfigError::UnsupportedResultRdfConfigFormat {
                format: other.to_string(),
            }),
        }
    }
}

impl Display for ResultRdfConfigFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultRdfConfigFormat::Internal => write!(f, "internal"),
            ResultRdfConfigFormat::Yaml => write!(f, "yaml"),
        }
    }
}

impl From<ResultRdfConfigFormat> for RdfConfigFormatEnum {
    fn from(format: ResultRdfConfigFormat) -> Self {
        match format {
            ResultRdfConfigFormat::Internal => RdfConfigFormatEnum::Internal,
            ResultRdfConfigFormat::Yaml => RdfConfigFormatEnum::Yaml,
        }
    }
}
