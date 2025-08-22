use crate::dctap_format::DCTapFormat as CliDCTapFormat;
use anyhow::{bail, Result};
use clap::ValueEnum;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::{CliShaclFormat, ShExFormat};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum InputConvertFormat {
    CSV,
    ShExC,
    ShExJ,
    Turtle,
    Xlsx,
}

impl InputConvertFormat {
    pub fn to_shex_format(&self) -> Result<ShExFormat> {
        match self {
            InputConvertFormat::ShExC => Ok(ShExFormat::ShExC),
            InputConvertFormat::ShExJ => Ok(ShExFormat::ShExJ),
            InputConvertFormat::Turtle => Ok(ShExFormat::Turtle),
            _ => bail!("Converting ShEx, format {self} not supported"),
        }
    }
    pub fn to_shacl_format(&self) -> Result<CliShaclFormat> {
        match self {
            InputConvertFormat::Turtle => Ok(CliShaclFormat::Turtle),
            _ => bail!("Converting to SHACL, format {self} not supported"),
        }
    }

    pub fn to_dctap_format(&self) -> Result<CliDCTapFormat> {
        match self {
            InputConvertFormat::CSV => Ok(CliDCTapFormat::CSV),
            InputConvertFormat::Xlsx => Ok(CliDCTapFormat::XLSX),
            _ => bail!("Converting to DCTAP, format {self} not supported"),
        }
    }
}

impl FromStr for InputConvertFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(InputConvertFormat::CSV),
            "xlsx" => Ok(InputConvertFormat::Xlsx),
            "shexc" => Ok(InputConvertFormat::ShExC),
            "shexj" => Ok(InputConvertFormat::ShExJ),
            "turtle" => Ok(InputConvertFormat::Turtle),
            _ => Err(format!("Unsupported input convert format {s}")),
        }
    }
}

impl Display for InputConvertFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InputConvertFormat::CSV => write!(dest, "csv"),
            InputConvertFormat::Xlsx => write!(dest, "xlsx"),
            InputConvertFormat::ShExC => write!(dest, "shexc"),
            InputConvertFormat::ShExJ => write!(dest, "shexj"),
            InputConvertFormat::Turtle => write!(dest, "turtle"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InputConvertFormat;
    use std::str::FromStr;
    #[test]
    fn test_from_str() {
        assert_eq!(
            InputConvertFormat::from_str("CSV").unwrap(),
            InputConvertFormat::CSV
        )
    }
}
