use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum ResultCompareFormat {
    #[default]
    Internal,
    JSON,
}

impl Display for ResultCompareFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultCompareFormat::Internal => write!(dest, "internal"),
            ResultCompareFormat::JSON => write!(dest, "json"),
        }
    }
}
