use std::fmt::{Display, Formatter};

use shex_validation::ShExFormat;

use crate::ComparatorError;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum CompareSchemaFormat {
    #[default]
    ShExC,
    ShExJ,
    Turtle,
}

impl CompareSchemaFormat {
    pub fn to_shex_format(&self) -> Result<ShExFormat, ComparatorError> {
        match self {
            CompareSchemaFormat::ShExC => Ok(ShExFormat::ShExC),
            CompareSchemaFormat::ShExJ => Ok(ShExFormat::ShExJ),
            CompareSchemaFormat::Turtle => Ok(ShExFormat::Turtle),
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
