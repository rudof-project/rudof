use clap::ValueEnum;
use std::fmt::{Display, Formatter};

use crate::ResultShaclValidationFormat;
use rudof_lib::result_shex_validation_format::ResultShExValidationFormat;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ResultValidationFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
    Compact,
    Details,
    Json,
}

impl ResultValidationFormat {
    pub fn to_shex_result_format(&self) -> ResultShExValidationFormat {
        match self {
            ResultValidationFormat::Turtle => ResultShExValidationFormat::Turtle,
            ResultValidationFormat::NTriples => ResultShExValidationFormat::NTriples,
            ResultValidationFormat::RDFXML => ResultShExValidationFormat::RDFXML,
            ResultValidationFormat::TriG => ResultShExValidationFormat::TriG,
            ResultValidationFormat::N3 => ResultShExValidationFormat::N3,
            ResultValidationFormat::NQuads => ResultShExValidationFormat::NQuads,
            ResultValidationFormat::Compact => ResultShExValidationFormat::Compact,
            ResultValidationFormat::Details => ResultShExValidationFormat::Details,
            ResultValidationFormat::Json => ResultShExValidationFormat::Json,
        }
    }

    pub fn to_shacl_result_format(&self) -> ResultShaclValidationFormat {
        match &self {
            ResultValidationFormat::Turtle => ResultShaclValidationFormat::Turtle,
            ResultValidationFormat::NTriples => ResultShaclValidationFormat::NTriples,
            ResultValidationFormat::RDFXML => ResultShaclValidationFormat::RDFXML,
            ResultValidationFormat::TriG => ResultShaclValidationFormat::TriG,
            ResultValidationFormat::N3 => ResultShaclValidationFormat::N3,
            ResultValidationFormat::NQuads => ResultShaclValidationFormat::NQuads,
            ResultValidationFormat::Compact => ResultShaclValidationFormat::Compact,
            ResultValidationFormat::Details => ResultShaclValidationFormat::Details,
            ResultValidationFormat::Json => ResultShaclValidationFormat::Json,
        }
    }
}

impl Display for ResultValidationFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultValidationFormat::Turtle => write!(dest, "turtle"),
            ResultValidationFormat::NTriples => write!(dest, "ntriples"),
            ResultValidationFormat::RDFXML => write!(dest, "rdfxml"),
            ResultValidationFormat::TriG => write!(dest, "trig"),
            ResultValidationFormat::N3 => write!(dest, "n3"),
            ResultValidationFormat::NQuads => write!(dest, "nquads"),
            ResultValidationFormat::Compact => write!(dest, "compact"),
            ResultValidationFormat::Json => write!(dest, "json"),
            ResultValidationFormat::Details => write!(dest, "details"),
        }
    }
}
