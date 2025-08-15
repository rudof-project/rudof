use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum OutputConvertMode {
    SPARQL,
    ShEx,
    UML,
    HTML,
    SHACL,
}

impl Display for OutputConvertMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OutputConvertMode::SPARQL => write!(dest, "sparql"),
            OutputConvertMode::ShEx => write!(dest, "shex"),
            OutputConvertMode::UML => write!(dest, "uml"),
            OutputConvertMode::HTML => write!(dest, "html"),
            OutputConvertMode::SHACL => write!(dest, "shacl"),
        }
    }
}
