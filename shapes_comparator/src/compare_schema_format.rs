use crate::ComparatorError;
use shex_ast::ShExFormat;
use rdf::rdf_core::RDFFormat;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum CompareSchemaFormat {
    #[default]
    ShExC,
    ShExJ,
    Turtle,
}

impl FromStr for CompareSchemaFormat {
    type Err = ComparatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shexc" => Ok(CompareSchemaFormat::ShExC),
            "shexj" => Ok(CompareSchemaFormat::ShExJ),
            "turtle" => Ok(CompareSchemaFormat::Turtle),
            _ => Err(ComparatorError::UnknownSchemaFormat(s.to_string())),
        }
    }
}
impl CompareSchemaFormat {
    pub fn to_shex_format(self) -> Result<ShExFormat, ComparatorError> {
        match self {
            CompareSchemaFormat::ShExC => Ok(ShExFormat::ShExC),
            CompareSchemaFormat::ShExJ => Ok(ShExFormat::ShExJ),
            CompareSchemaFormat::Turtle => Ok(ShExFormat::RDFFormat(RDFFormat::Turtle)),
        }
    }
}

impl Display for CompareSchemaFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CompareSchemaFormat::ShExC => write!(dest, "shexc"),
            CompareSchemaFormat::ShExJ => write!(dest, "shexj"),
            CompareSchemaFormat::Turtle => write!(dest, "turtle"),
        }
    }
}
