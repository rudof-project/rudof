use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum PgSchemaFormatCli {
    PgSchemaC,
}

impl Display for PgSchemaFormatCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PgSchemaFormatCli::PgSchemaC => "compact",
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ValidationModeCli {
    ShEx,
    Shacl,
    PGSchema,
}

impl Display for ValidationModeCli {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ValidationModeCli::ShEx => write!(dest, "shex"),
            ValidationModeCli::Shacl => write!(dest, "shacl"),
            ValidationModeCli::PGSchema => write!(dest, "pgschema"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum SortByValidateCli {
    Node,
    Details,
}

impl Display for SortByValidateCli {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SortByValidateCli::Node => write!(dest, "node"),
            SortByValidateCli::Details => write!(dest, "details"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum PgSchemaResultFormatCli {
    Compact,
    Details,
    Json,
    Csv,
}

impl Display for PgSchemaResultFormatCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PgSchemaResultFormatCli::Compact => "compact",
            PgSchemaResultFormatCli::Details => "details",
            PgSchemaResultFormatCli::Json => "json",
            PgSchemaResultFormatCli::Csv => "csv",
        };
        write!(f, "{}", s)
    }
}