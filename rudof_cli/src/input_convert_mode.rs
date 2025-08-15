use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum InputConvertMode {
    SHACL,
    ShEx,
    DCTAP,
}

impl Display for InputConvertMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InputConvertMode::SHACL => write!(dest, "shacl"),
            InputConvertMode::ShEx => write!(dest, "shex"),
            InputConvertMode::DCTAP => write!(dest, "dctap"),
        }
    }
}
