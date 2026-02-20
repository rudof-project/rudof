use clap::ValueEnum;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum DCTapResultFormat {
    Internal,
    Json,
}

impl FromStr for DCTapResultFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "internal" => Ok(DCTapResultFormat::Internal),
            "json" => Ok(DCTapResultFormat::Json),
            _ => Err(format!(
                "Invalid DC-TAP result format: '{}'. Valid values are: internal, json",
                s
            )),
        }
    }
}

impl Display for DCTapResultFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DCTapResultFormat::Internal => write!(dest, "internal"),
            DCTapResultFormat::Json => write!(dest, "json"),
        }
    }
}
