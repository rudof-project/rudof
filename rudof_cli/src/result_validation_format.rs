use clap::ValueEnum;
use std::fmt::{Display, Formatter};

use rudof_lib::{
    result_shacl_validation_format::ResultShaclValidationFormat,
    result_shex_validation_format::ResultShExValidationFormat,
};

use crate::PgSchemaResultFormat;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ResultValidationFormat {
    Turtle,
    NTriples,
    RdfXml,
    TriG,
    N3,
    NQuads,
    Compact,
    Details,
    Json,
    Csv,
}

impl ResultValidationFormat {
    pub fn to_shex_result_format(self) -> ResultShExValidationFormat {
        match self {
            ResultValidationFormat::Turtle => ResultShExValidationFormat::Turtle,
            ResultValidationFormat::NTriples => ResultShExValidationFormat::NTriples,
            ResultValidationFormat::RdfXml => ResultShExValidationFormat::RdfXml,
            ResultValidationFormat::TriG => ResultShExValidationFormat::TriG,
            ResultValidationFormat::N3 => ResultShExValidationFormat::N3,
            ResultValidationFormat::NQuads => ResultShExValidationFormat::NQuads,
            ResultValidationFormat::Compact => ResultShExValidationFormat::Compact,
            ResultValidationFormat::Details => ResultShExValidationFormat::Details,
            ResultValidationFormat::Json => ResultShExValidationFormat::Json,
            ResultValidationFormat::Csv => ResultShExValidationFormat::Csv,
        }
    }

    pub fn to_shacl_result_format(self) -> ResultShaclValidationFormat {
        match self {
            ResultValidationFormat::Turtle => ResultShaclValidationFormat::Turtle,
            ResultValidationFormat::NTriples => ResultShaclValidationFormat::NTriples,
            ResultValidationFormat::RdfXml => ResultShaclValidationFormat::RdfXml,
            ResultValidationFormat::TriG => ResultShaclValidationFormat::TriG,
            ResultValidationFormat::N3 => ResultShaclValidationFormat::N3,
            ResultValidationFormat::NQuads => ResultShaclValidationFormat::NQuads,
            ResultValidationFormat::Compact => ResultShaclValidationFormat::Compact,
            ResultValidationFormat::Details => ResultShaclValidationFormat::Details,
            ResultValidationFormat::Json => ResultShaclValidationFormat::Json,
            ResultValidationFormat::Csv => ResultShaclValidationFormat::Csv,
        }
    }

    pub fn to_pgschema_result_format(self) -> PgSchemaResultFormat {
        match self {
            ResultValidationFormat::Compact => PgSchemaResultFormat::Compact,
            ResultValidationFormat::Details => PgSchemaResultFormat::Details,
            ResultValidationFormat::Json => PgSchemaResultFormat::Json,
            ResultValidationFormat::Csv => PgSchemaResultFormat::Csv,
            _ => todo!(),
        }
    }
}

impl Display for ResultValidationFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultValidationFormat::Turtle => write!(dest, "turtle"),
            ResultValidationFormat::NTriples => write!(dest, "ntriples"),
            ResultValidationFormat::RdfXml => write!(dest, "rdfxml"),
            ResultValidationFormat::TriG => write!(dest, "trig"),
            ResultValidationFormat::N3 => write!(dest, "n3"),
            ResultValidationFormat::NQuads => write!(dest, "nquads"),
            ResultValidationFormat::Compact => write!(dest, "compact"),
            ResultValidationFormat::Json => write!(dest, "json"),
            ResultValidationFormat::Details => write!(dest, "details"),
            ResultValidationFormat::Csv => write!(dest, "csv"),
        }
    }
}
