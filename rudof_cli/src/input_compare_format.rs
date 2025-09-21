use crate::CliShaclFormat;
use crate::{dctap_format::DCTapFormat as CliDCTapFormat, mime_type::MimeType};
use anyhow::{Result, bail};
use clap::ValueEnum;
use rudof_lib::ShExFormat;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum InputCompareFormat {
    #[default]
    ShExC,
    ShExJ,
    Turtle,
}

impl InputCompareFormat {
    pub fn to_shex_format(&self) -> Result<ShExFormat> {
        match self {
            InputCompareFormat::ShExC => Ok(ShExFormat::ShExC),
            InputCompareFormat::ShExJ => Ok(ShExFormat::ShExJ),
            InputCompareFormat::Turtle => Ok(ShExFormat::Turtle),
        }
    }
    pub fn to_shacl_format(&self) -> Result<CliShaclFormat> {
        match self {
            InputCompareFormat::Turtle => Ok(CliShaclFormat::Turtle),
            _ => bail!("Converting to SHACL, format {self} not supported"),
        }
    }

    pub fn to_dctap_format(&self) -> Result<CliDCTapFormat> {
        bail!("Converting to DCTAP, format {self} not supported")
    }
}

impl FromStr for InputCompareFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shexc" => Ok(InputCompareFormat::ShExC),
            "shexj" => Ok(InputCompareFormat::ShExJ),
            "turtle" => Ok(InputCompareFormat::Turtle),
            _ => Err(format!("Unsupported input convert format {s}")),
        }
    }
}

impl Display for InputCompareFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InputCompareFormat::ShExC => write!(dest, "shexc"),
            InputCompareFormat::ShExJ => write!(dest, "shexj"),
            InputCompareFormat::Turtle => write!(dest, "turtle"),
        }
    }
}

impl MimeType for InputCompareFormat {
    fn mime_type(&self) -> String {
        match &self {
            InputCompareFormat::ShExC => "text/shex".to_string(),
            InputCompareFormat::ShExJ => "application/json".to_string(),
            InputCompareFormat::Turtle => "text/turtle".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {}
