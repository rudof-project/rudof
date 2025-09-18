use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum InputCompareMode {
    SHACL,

    #[default]
    ShEx,
    DCTAP,
    Service,
}

impl Display for InputCompareMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InputCompareMode::SHACL => write!(dest, "shacl"),
            InputCompareMode::ShEx => write!(dest, "shex"),
            InputCompareMode::DCTAP => write!(dest, "dctap"),
            InputCompareMode::Service => write!(dest, "service"),
        }
    }
}
