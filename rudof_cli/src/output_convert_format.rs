use anyhow::{Result, bail};
use clap::ValueEnum;

use std::fmt::{Display, Formatter};

use rudof_lib::shacl_format::CliShaclFormat;
use rudof_lib::shex_format::ShExFormat;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum OutputConvertFormat {
    Default,
    Internal,
    Json,
    ShExC,
    ShExJ,
    Turtle,
    PlantUML,
    Html,
    Svg,
    Png,
}

impl OutputConvertFormat {
    pub fn to_shex_format(self) -> Result<ShExFormat> {
        match self {
            OutputConvertFormat::ShExC => Ok(ShExFormat::ShExC),
            OutputConvertFormat::ShExJ => Ok(ShExFormat::ShExJ),
            OutputConvertFormat::Turtle => Ok(ShExFormat::Turtle),
            _ => bail!("Converting ShEx, format {self} not supported"),
        }
    }

    pub fn to_shacl_format(self) -> Result<CliShaclFormat> {
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
            OutputConvertFormat::Json => write!(dest, "json"),
            OutputConvertFormat::Default => write!(dest, "default"),
            OutputConvertFormat::ShExC => write!(dest, "shexc"),
            OutputConvertFormat::ShExJ => write!(dest, "shexj"),
            OutputConvertFormat::Turtle => write!(dest, "turtle"),
            OutputConvertFormat::PlantUML => write!(dest, "uml"),
            OutputConvertFormat::Html => write!(dest, "html"),
            OutputConvertFormat::Png => write!(dest, "png"),
            OutputConvertFormat::Svg => write!(dest, "svg"),
        }
    }
}
