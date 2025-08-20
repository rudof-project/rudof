use std::fmt::{Display, Formatter};

use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ValidationMode {
    ShEx,
    SHACL,
}

impl Display for ValidationMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ValidationMode::ShEx => write!(dest, "shex"),
            ValidationMode::SHACL => write!(dest, "shacl"),
        }
    }
}
