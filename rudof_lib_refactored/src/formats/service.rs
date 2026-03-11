use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::errors::ServiceError;

/// Output formats for SPARQL service description results supported by Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ResultServiceFormat {
    /// Internal format - internal representation for processing
    #[default]
    Internal,
    /// MIE format - Metadata Information Exchange format
    Mie,
    /// JSON format - machine-readable JSON serialization
    Json,
}

impl FromStr for ResultServiceFormat {
    type Err = ServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "internal" => Ok(ResultServiceFormat::Internal),
            "mie" => Ok(ResultServiceFormat::Mie),
            "json" => Ok(ResultServiceFormat::Json),
            other => Err(ServiceError::UnsupportedResultServiceFormat {
                format: other.to_string(),
            }),
        }
    }
}

impl Display for ResultServiceFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultServiceFormat::Internal => write!(dest, "internal"),
            ResultServiceFormat::Mie => write!(dest, "mie"),
            ResultServiceFormat::Json => write!(dest, "json"),
        }
    }
}