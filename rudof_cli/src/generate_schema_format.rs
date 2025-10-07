use clap::ValueEnum;
use serde::Deserialize;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Deserialize, Default)]
pub enum GenerateSchemaFormat {
    /// Automatically detect format from file extension
    #[value(name = "auto")]
    #[default]
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
