use crate::errors::PgSchemaError;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// Property Graph schema formats supported by Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum PgSchemaFormat {
    /// PgSchemaC - Compact Property Graph schema syntax (default)
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
    type Err = PgSchemaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "compact" | "pgschemac" => Ok(PgSchemaFormat::PgSchemaC),
            other => Err(PgSchemaError::UnsupportedPgSchemaFormat {
                format: other.to_string(),
            }),
        }
    }
}
