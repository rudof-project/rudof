use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ResultServiceFormat {
    Internal,
    Mie,
    Json,
}

impl FromStr for ResultServiceFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "internal" => Ok(ResultServiceFormat::Internal),
            "mie" => Ok(ResultServiceFormat::Mie),
            "json" => Ok(ResultServiceFormat::Json),
            _ => Err(format!(
                "Unknown service result format: '{}'. Valid options are: internal, mie, json",
                s
            )),
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
