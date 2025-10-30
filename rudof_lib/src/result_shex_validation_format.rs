use anyhow::{Result, bail};
use clap::ValueEnum;
use std::{fmt::{Display, Formatter}, str::FromStr};

use crate::{shapemap_format::ShapeMapFormat, RudofError};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum ResultShExValidationFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
    Compact,
    #[default]
    Details,
    Json,
}

impl ResultShExValidationFormat {
    pub fn to_shapemap_format(&self) -> Result<ShapeMapFormat> {
        match self {
            ResultShExValidationFormat::Compact => Ok(ShapeMapFormat::Compact),
            ResultShExValidationFormat::Details => Ok(ShapeMapFormat::Details),
            ResultShExValidationFormat::Json => Ok(ShapeMapFormat::Json),
            _ => bail!(
                "Conversion to ShapeMapFormat not supported for {self}. \
                 Use a different format or implement conversion."
            ),
        }
    }
}

impl Display for ResultShExValidationFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultShExValidationFormat::Turtle => write!(dest, "turtle"),
            ResultShExValidationFormat::NTriples => write!(dest, "ntriples"),
            ResultShExValidationFormat::RDFXML => write!(dest, "rdfxml"),
            ResultShExValidationFormat::TriG => write!(dest, "trig"),
            ResultShExValidationFormat::N3 => write!(dest, "n3"),
            ResultShExValidationFormat::NQuads => write!(dest, "nquads"),
            ResultShExValidationFormat::Compact => write!(dest, "compact"),
            ResultShExValidationFormat::Json => write!(dest, "json"),
            ResultShExValidationFormat::Details => write!(dest, "details"),
        }
    }
}

impl TryFrom<&ResultShExValidationFormat> for ShapeMapFormat {
    type Error = RudofError;

    fn try_from(format: &ResultShExValidationFormat) -> Result<Self, Self::Error> {
        match format {
            ResultShExValidationFormat::Compact => Ok(ShapeMapFormat::Compact),
            ResultShExValidationFormat::Details => Ok(ShapeMapFormat::Details),
            ResultShExValidationFormat::Json => Ok(ShapeMapFormat::Json),
            other => return Err(RudofError::NotImplemented { msg: format!("Result ShEx validation format {other:?} not yet implemented")})
        }
    }
}

impl FromStr for ResultShExValidationFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "turtle" => Ok(ResultShExValidationFormat::Turtle),
            "ntriples" => Ok(ResultShExValidationFormat::NTriples),
            "rdfxml" => Ok(ResultShExValidationFormat::RDFXML),
            "trig" => Ok(ResultShExValidationFormat::TriG),
            "n3" => Ok(ResultShExValidationFormat::N3),
            "nquads" => Ok(ResultShExValidationFormat::NQuads),
            "compact" => Ok(ResultShExValidationFormat::Compact),
            "details" => Ok(ResultShExValidationFormat::Details),
            "jspn" => Ok(ResultShExValidationFormat::Json),
            _ => Err(format!("Unknown result ShEx validation format: {s}")),
        }
    }
}