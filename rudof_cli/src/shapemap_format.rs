use std::fmt::{Display, Formatter};

use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShapeMapFormat {
    Compact,
    Internal,
}

impl Display for ShapeMapFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShapeMapFormat::Compact => write!(dest, "compact"),
            ShapeMapFormat::Internal => write!(dest, "internal"),
        }
    }
}
