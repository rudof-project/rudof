use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ResultServiceFormat {
    Internal,
    Mie,
    Json,
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
