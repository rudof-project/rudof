use anyhow::{Result, bail};
use clap::ValueEnum;
use std::fmt::{Display, Formatter};

use crate::ShapeMapFormat;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ResultShExValidationFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
    Compact,
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
