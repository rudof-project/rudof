use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use crate::errors::ShapeMapError;
use shex_ast::shapemap::ShapeMapFormat as ShexAstShapeMapFormat;

/// ShapeMap formats supported by Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum ShapeMapFormat {
    /// Compact format - human-readable ShapeMap syntax
    #[default]
    Compact,
    /// Internal format - internal representation for processing
    Internal,
    /// JSON format - machine-readable JSON serialization
    Json,
    /// Details format - verbose output with validation details
    Details,
    /// CSV format - comma-separated values for spreadsheet tools
    Csv,
}

impl Display for ShapeMapFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShapeMapFormat::Compact => write!(dest, "compact"),
            ShapeMapFormat::Internal => write!(dest, "internal"),
            ShapeMapFormat::Json => write!(dest, "json"),
            ShapeMapFormat::Details => write!(dest, "details"),
            ShapeMapFormat::Csv => write!(dest, "csv"),
        }
    }
}

impl From<ShapeMapFormat> for ShexAstShapeMapFormat {
    fn from(format: ShapeMapFormat) -> Self {
        match format {
            ShapeMapFormat::Compact => ShexAstShapeMapFormat::Compact,
            ShapeMapFormat::Internal => ShexAstShapeMapFormat::Json,
            ShapeMapFormat::Json => ShexAstShapeMapFormat::Json,
            ShapeMapFormat::Details => ShexAstShapeMapFormat::Compact,
            ShapeMapFormat::Csv => ShexAstShapeMapFormat::Csv,
        }
    }
}

impl From<&ShapeMapFormat> for ShexAstShapeMapFormat {
    fn from(format: &ShapeMapFormat) -> Self {
        (*format).into()
    }
}

impl FromStr for ShapeMapFormat {
    type Err = ShapeMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "compact" => Ok(ShapeMapFormat::Compact),
            "internal" => Ok(ShapeMapFormat::Internal),
            "json" => Ok(ShapeMapFormat::Json),
            "details" => Ok(ShapeMapFormat::Details),
            "csv" => Ok(ShapeMapFormat::Csv),
            other => Err(ShapeMapError::UnsupportedShapeMapFormat {
                format: other.to_string(),
            }),
        }
    }
}