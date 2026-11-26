use crate::errors::GenerationError;
use serde::Deserialize;
use std::fmt;
use std::str::FromStr;

/// Schema formats for generating synthetic RDF data supported by Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Deserialize)]
pub enum GenerationSchemaFormat {
    /// Automatically detect schema format from file extension (default)
    #[default]
    Auto,
    /// ShEx (Shape Expressions) schema - generate data conforming to ShEx shapes
    ShEx,
    /// SHACL (Shapes Constraint Language) schema - generate data conforming to SHACL shapes
    Shacl,
}

impl fmt::Display for GenerationSchemaFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerationSchemaFormat::Auto => write!(f, "auto"),
            GenerationSchemaFormat::ShEx => write!(f, "shex"),
            GenerationSchemaFormat::Shacl => write!(f, "shacl"),
        }
    }
}

impl FromStr for GenerationSchemaFormat {
    type Err = GenerationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(GenerationSchemaFormat::Auto),
            "shex" => Ok(GenerationSchemaFormat::ShEx),
            "shacl" => Ok(GenerationSchemaFormat::Shacl),
            other => Err(GenerationError::UnsupportedGenerationSchemaFormat {
                format: other.to_string(),
            }),
        }
    }
}
