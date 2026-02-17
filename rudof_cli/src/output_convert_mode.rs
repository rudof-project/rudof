use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum OutputConvertMode {
    Sparql,
    ShEx,
    Uml,
    Html,
    Shacl,
}

impl Display for OutputConvertMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OutputConvertMode::Sparql => write!(dest, "sparql"),
            OutputConvertMode::ShEx => write!(dest, "shex"),
            OutputConvertMode::Uml => write!(dest, "uml"),
            OutputConvertMode::Html => write!(dest, "html"),
            OutputConvertMode::Shacl => write!(dest, "shacl"),
        }
    }
}
