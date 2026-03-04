use serde::Deserialize;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Default)]
pub enum GenerateSchemaFormat {
    #[default]
    /// Automatically detect format from file extension
    Auto,

    /// ShEx format
    ShEx,

    /// SHACL format
    Shacl,
}

impl fmt::Display for GenerateSchemaFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateSchemaFormat::Auto => write!(f, "auto"),
            GenerateSchemaFormat::ShEx => write!(f, "shex"),
            GenerateSchemaFormat::Shacl => write!(f, "shacl"),
        }
    }
}

impl FromStr for GenerateSchemaFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(GenerateSchemaFormat::Auto),
            "shex" => Ok(GenerateSchemaFormat::ShEx),
            "shacl" => Ok(GenerateSchemaFormat::Shacl),
            _ => Err(format!("Unknown schema format: '{}'. Supported: auto, shex, shacl", s)),
        }
    }
}
