use anyhow::{bail, Result};
use clap::ValueEnum;

use std::fmt::{Display, Formatter};

use crate::{CliShaclFormat, ShExFormat};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum OutputConvertFormat {
    Default,
    Internal,
    JSON,
    ShExC,
    ShExJ,
    Turtle,
    PlantUML,
    HTML,
    SVG,
    PNG,
}

impl OutputConvertFormat {
    pub fn to_shex_format(&self) -> Result<ShExFormat> {
        match self {
            OutputConvertFormat::ShExC => Ok(ShExFormat::ShExC),
            OutputConvertFormat::ShExJ => Ok(ShExFormat::ShExJ),
            OutputConvertFormat::Turtle => Ok(ShExFormat::Turtle),
            _ => bail!("Converting ShEx, format {self} not supported"),
        }
    }

    pub fn to_shacl_format(&self) -> Result<CliShaclFormat> {
        match self {
            OutputConvertFormat::Default => Ok(CliShaclFormat::Internal),
            OutputConvertFormat::Turtle => Ok(CliShaclFormat::Turtle),
            _ => bail!("Converting to SHACL, format {self} not supported"),
        }
    }
}

impl Display for OutputConvertFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OutputConvertFormat::Internal => write!(dest, "internal"),
            OutputConvertFormat::JSON => write!(dest, "json"),
            OutputConvertFormat::Default => write!(dest, "default"),
            OutputConvertFormat::ShExC => write!(dest, "shexc"),
            OutputConvertFormat::ShExJ => write!(dest, "shexj"),
            OutputConvertFormat::Turtle => write!(dest, "turtle"),
            OutputConvertFormat::PlantUML => write!(dest, "uml"),
            OutputConvertFormat::HTML => write!(dest, "html"),
            OutputConvertFormat::PNG => write!(dest, "png"),
            OutputConvertFormat::SVG => write!(dest, "svg"),
        }
    }
}
