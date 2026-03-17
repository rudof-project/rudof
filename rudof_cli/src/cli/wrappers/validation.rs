use crate::cli_wrapper;
use crate::cli::wrappers::{
    ShExFormatCli, ShaclFormatCli
};
use rudof_lib_refactored::formats::{
    ValidationMode, ValidationSortByMode, ShaclValidationMode, ResultShaclValidationFormat,
    ShaclValidationSortByMode, ShExValidationSortByMode, ResultShExValidationFormat, ResultValidationFormat,
    ResultPgSchemaValidationFormat
};
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};
use anyhow::{bail, Result as AnyhowResult};

cli_wrapper!(
    ValidationModeCli,
    ValidationMode,
    {
        ShEx,
        Shacl,
        PGSchema
    }
);

cli_wrapper!(
    ValidationSortByModeCli,
    ValidationSortByMode,
    {
        Node,
        Details
    }
);

cli_wrapper!(
    ShaclValidationModeCli,
    ShaclValidationMode,
    {
        Native,
        Sparql
    }
);

cli_wrapper!(
    ResultShaclValidationFormatCli,
    ResultShaclValidationFormat,
    {
        Details,
        Turtle,
        NTriples,
        RdfXml,
        TriG,
        N3,
        NQuads,
        Minimal,
        Compact,
        Json,
        Csv,
    }
);

cli_wrapper!(
    ShaclValidationSortByModeCli,
    ShaclValidationSortByMode,
    {
        Severity,
        Node,
        Component,
        Value,
        Path,
        SourceShape,
        Details,
    }
);

cli_wrapper!(
    ResultValidationFormatCli,
    ResultValidationFormat,
    {
        Turtle,
        NTriples,
        RdfXml,
        TriG,
        N3,
        NQuads,
        Compact,
        Details,
        Json,
        Csv
    }
);

cli_wrapper!(
    ShExValidationSortByModeCli,
    ShExValidationSortByMode,
    {
        Node,
        Shape,
        Status,
        Details
    }
);

cli_wrapper!(
    ResultShExValidationFormatCli,
    ResultShExValidationFormat,
    {
        Details,
        Turtle,
        NTriples,
        RdfXml,
        TriG,
        N3,
        NQuads,
        Compact,
        Json,
        Csv,
    }
);

cli_wrapper!(ResultPgSchemaValidationFormatCli, ResultPgSchemaValidationFormat, { Compact, Details, Json, Csv });

impl From<ValidationSortByModeCli> for ShExValidationSortByModeCli {
    fn from(val: ValidationSortByModeCli) -> Self {
        match val {
            ValidationSortByModeCli::Node => ShExValidationSortByModeCli::Node,
            ValidationSortByModeCli::Details => ShExValidationSortByModeCli::Details,
        }
    }
}

impl From<ValidationSortByModeCli> for ShaclValidationSortByModeCli {
    fn from(val: ValidationSortByModeCli) -> Self {
        match val {
            ValidationSortByModeCli::Node => ShaclValidationSortByModeCli::Node,
            ValidationSortByModeCli::Details => ShaclValidationSortByModeCli::Details,
        }
    }
}

impl From<ResultValidationFormatCli> for ResultShExValidationFormatCli {
    fn from(val: ResultValidationFormatCli) -> Self {
        match val {
            ResultValidationFormatCli::Turtle => ResultShExValidationFormatCli::Turtle,
            ResultValidationFormatCli::NTriples => ResultShExValidationFormatCli::NTriples,
            ResultValidationFormatCli::RdfXml => ResultShExValidationFormatCli::RdfXml,
            ResultValidationFormatCli::TriG => ResultShExValidationFormatCli::TriG,
            ResultValidationFormatCli::N3 => ResultShExValidationFormatCli::N3,
            ResultValidationFormatCli::NQuads => ResultShExValidationFormatCli::NQuads,
            ResultValidationFormatCli::Compact => ResultShExValidationFormatCli::Compact,
            ResultValidationFormatCli::Details => ResultShExValidationFormatCli::Details,
            ResultValidationFormatCli::Json => ResultShExValidationFormatCli::Json,
            ResultValidationFormatCli::Csv => ResultShExValidationFormatCli::Csv,
        }
    }
}

impl From<ResultValidationFormatCli> for ResultShaclValidationFormatCli {
    fn from(val: ResultValidationFormatCli) -> Self {
        match val {
            ResultValidationFormatCli::Turtle => ResultShaclValidationFormatCli::Turtle,
            ResultValidationFormatCli::NTriples => ResultShaclValidationFormatCli::NTriples,
            ResultValidationFormatCli::RdfXml => ResultShaclValidationFormatCli::RdfXml,
            ResultValidationFormatCli::TriG => ResultShaclValidationFormatCli::TriG,
            ResultValidationFormatCli::N3 => ResultShaclValidationFormatCli::N3,
            ResultValidationFormatCli::NQuads => ResultShaclValidationFormatCli::NQuads,
            ResultValidationFormatCli::Compact => ResultShaclValidationFormatCli::Compact,
            ResultValidationFormatCli::Details => ResultShaclValidationFormatCli::Details,
            ResultValidationFormatCli::Json => ResultShaclValidationFormatCli::Json,
            ResultValidationFormatCli::Csv => ResultShaclValidationFormatCli::Csv,
        }
    }
}

impl TryFrom<ShExFormatCli> for ShaclFormatCli {
    type Error = anyhow::Error;

    fn try_from(val: ShExFormatCli) -> AnyhowResult<Self> {
        match val {
            ShExFormatCli::Internal => Ok(ShaclFormatCli::Internal),
            ShExFormatCli::Turtle => Ok(ShaclFormatCli::Turtle),
            ShExFormatCli::NTriples => Ok(ShaclFormatCli::NTriples),
            ShExFormatCli::RdfXml => Ok(ShaclFormatCli::RdfXml),
            ShExFormatCli::TriG => Ok(ShaclFormatCli::TriG),
            ShExFormatCli::N3 => Ok(ShaclFormatCli::N3),
            ShExFormatCli::NQuads => Ok(ShaclFormatCli::NQuads),
            ShExFormatCli::JsonLd => Ok(ShaclFormatCli::JsonLd),
            ShExFormatCli::Simple => bail!("Validation using SHACL mode doesn't support Simple format"),
            ShExFormatCli::ShExC => bail!("Validation using SHACL mode doesn't support ShExC format"),
            ShExFormatCli::ShExJ => bail!("Validation using SHACL mode doesn't support ShExJ format"),
            ShExFormatCli::Json => Ok(ShaclFormatCli::JsonLd),
        }
    }
}

impl TryFrom<ResultValidationFormatCli> for ResultPgSchemaValidationFormatCli {
    type Error = anyhow::Error;

    fn try_from(val: ResultValidationFormatCli) -> AnyhowResult<Self, Self::Error> {
        match val {
            ResultValidationFormatCli::Compact => Ok(ResultPgSchemaValidationFormatCli::Compact),
            ResultValidationFormatCli::Details => Ok(ResultPgSchemaValidationFormatCli::Details),
            ResultValidationFormatCli::Json => Ok(ResultPgSchemaValidationFormatCli::Json),
            ResultValidationFormatCli::Csv => Ok(ResultPgSchemaValidationFormatCli::Csv),
            ResultValidationFormatCli::Turtle => todo!("PGSchema validation doesn't support Turtle result format"),
            ResultValidationFormatCli::NTriples => todo!("PGSchema validation doesn't support NTriples result format"),
            ResultValidationFormatCli::RdfXml => todo!("PGSchema validation doesn't support RDF/XML result format"),
            ResultValidationFormatCli::TriG => todo!("PGSchema validation doesn't support TriG result format"),
            ResultValidationFormatCli::N3 => todo!("PGSchema validation doesn't support N3 result format"),
            ResultValidationFormatCli::NQuads => todo!("PGSchema validation doesn't support NQuads result format"),
        }
    }
}