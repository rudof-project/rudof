use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use clap::ValueEnum;
use shex_ast::shapemap::ShapeMapFormat as ShexAstShapeMapFormat;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShapeMapFormat {
    Compact,
    Internal,
    Json,
    Details,
}

impl Display for ShapeMapFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShapeMapFormat::Compact => write!(dest, "compact"),
            ShapeMapFormat::Internal => write!(dest, "internal"),
            ShapeMapFormat::Json => write!(dest, "json"),
            ShapeMapFormat::Details => write!(dest, "details"),
        }
    }
}

impl From<&ShapeMapFormat> for ShexAstShapeMapFormat {
    fn from(format: &ShapeMapFormat) -> Self {
        match format {
            ShapeMapFormat::Compact => ShexAstShapeMapFormat::Compact,
            ShapeMapFormat::Internal => ShexAstShapeMapFormat::JSON,
            ShapeMapFormat::Json => ShexAstShapeMapFormat::JSON,
            ShapeMapFormat::Details => ShexAstShapeMapFormat::Compact,
        }
    }
}

impl FromStr for ShapeMapFormat {
    type Err = crate::RudofError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "compact" => Ok(ShapeMapFormat::Compact),
            "internal" => Ok(ShapeMapFormat::Internal),
            "json" => Ok(ShapeMapFormat::Json),
            "details" => Ok(ShapeMapFormat::Details),
            other => Err(crate::RudofError::UnsupportedShapeMapFormat { format: other.to_string() }),
        }
    }
}
