use clap::ValueEnum;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PgSchemaFormat {
    #[default]
    PgSchemaC,
}

impl Display for PgSchemaFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PgSchemaFormat::PgSchemaC => write!(f, "compact"),
        }
    }
}

impl FromStr for PgSchemaFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "compact" | "pgschemac" => Ok(PgSchemaFormat::PgSchemaC),
            other => Err(format!("Unknown PgSchema format: {}", other)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum, Default)]
pub enum PgSchemaResultFormat {
    #[default]
    Compact,
    Details,
    Json,
    Csv,
}

impl Display for PgSchemaResultFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PgSchemaResultFormat::Compact => "compact",
            PgSchemaResultFormat::Details => "details",
            PgSchemaResultFormat::Json => "json",
            PgSchemaResultFormat::Csv => "csv",
        };
        write!(f, "{}", s)
    }
}
