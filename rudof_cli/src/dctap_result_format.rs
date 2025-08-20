use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum DCTapResultFormat {
    Internal,
    JSON,
}

impl Display for DCTapResultFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DCTapResultFormat::Internal => write!(dest, "internal"),
            DCTapResultFormat::JSON => write!(dest, "json"),
        }
    }
}
