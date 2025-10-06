use clap::ValueEnum;
use serde::Deserialize;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Deserialize)]
pub enum GenerateSchemaFormat {
    /// Automatically detect format from file extension
    #[value(name = "auto")]
    Auto,

    /// ShEx format
    #[value(name = "shex")]
    ShEx,

    /// SHACL format
    #[value(name = "shacl")]
    SHACL,
}

impl fmt::Display for GenerateSchemaFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateSchemaFormat::Auto => write!(f, "auto"),
            GenerateSchemaFormat::ShEx => write!(f, "shex"),
            GenerateSchemaFormat::SHACL => write!(f, "shacl"),
        }
    }
}

impl Default for GenerateSchemaFormat {
    fn default() -> Self {
        GenerateSchemaFormat::Auto
    }
}
