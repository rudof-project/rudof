use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use thiserror::Error;

use crate::{
    pgschema_format::PgSchemaResultFormat, result_shacl_validation_format::ResultShaclValidationFormat,
    result_shex_validation_format::ResultShExValidationFormat,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(Error, Clone, Debug)]
pub enum ResultFormatError {
    #[error("Format '{format}' is not supported for Property Graph validation. Use: compact, details, json, or csv")]
    UnsupportedForPgSchema { format: String },

    #[error("Unknown result format: {format}")]
    UnknownFormat { format: String },
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

    pub fn to_pgschema_result_format(self) -> Result<PgSchemaResultFormat, ResultFormatError> {
        match self {
            ResultValidationFormat::Compact => Ok(PgSchemaResultFormat::Compact),
            ResultValidationFormat::Details => Ok(PgSchemaResultFormat::Details),
            ResultValidationFormat::Json => Ok(PgSchemaResultFormat::Json),
            ResultValidationFormat::Csv => Ok(PgSchemaResultFormat::Csv),
            other => Err(ResultFormatError::UnsupportedForPgSchema {
                format: other.to_string(),
            }),
        }
    }

    pub fn is_rdf_format(&self) -> bool {
        matches!(
            self,
            ResultValidationFormat::Turtle
                | ResultValidationFormat::NTriples
                | ResultValidationFormat::RdfXml
                | ResultValidationFormat::TriG
                | ResultValidationFormat::N3
                | ResultValidationFormat::NQuads
        )
    }

    pub fn is_pg_format(&self) -> bool {
        matches!(
            self,
            ResultValidationFormat::Compact
                | ResultValidationFormat::Details
                | ResultValidationFormat::Json
                | ResultValidationFormat::Csv
        )
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

impl FromStr for ResultValidationFormat {
    type Err = ResultFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "turtle" => Ok(ResultValidationFormat::Turtle),
            "ntriples" => Ok(ResultValidationFormat::NTriples),
            "rdfxml" => Ok(ResultValidationFormat::RdfXml),
            "trig" => Ok(ResultValidationFormat::TriG),
            "n3" => Ok(ResultValidationFormat::N3),
            "nquads" => Ok(ResultValidationFormat::NQuads),
            "compact" => Ok(ResultValidationFormat::Compact),
            "details" => Ok(ResultValidationFormat::Details),
            "json" => Ok(ResultValidationFormat::Json),
            "csv" => Ok(ResultValidationFormat::Csv),
            other => Err(ResultFormatError::UnknownFormat {
                format: other.to_string(),
            }),
        }
    }
}
