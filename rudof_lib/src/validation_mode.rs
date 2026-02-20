use clap::ValueEnum;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ValidationMode {
    ShEx,
    Shacl,
    PGSchema,
}

impl Display for ValidationMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ValidationMode::ShEx => write!(dest, "shex"),
            ValidationMode::Shacl => write!(dest, "shacl"),
            ValidationMode::PGSchema => write!(dest, "pgschema"),
        }
    }
}

impl FromStr for ValidationMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shex" => Ok(ValidationMode::ShEx),
            "shacl" => Ok(ValidationMode::Shacl),
            "pgschema" => Ok(ValidationMode::PGSchema),
            other => Err(format!("Unknown validation mode: {}", other)),
        }
    }
}
