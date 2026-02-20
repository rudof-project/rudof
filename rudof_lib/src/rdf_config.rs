use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum RdfConfigFormat {
    #[default]
    Yaml,
}

impl FromStr for RdfConfigFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "yaml" => Ok(RdfConfigFormat::Yaml),
            _ => Err(format!("Unknown RDF config format: '{}'. Supported: yaml", s)),
        }
    }
}

impl Display for RdfConfigFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RdfConfigFormat::Yaml => write!(f, "yaml"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum RdfConfigResultFormat {
    #[default]
    Internal,
    Yaml,
}

impl FromStr for RdfConfigResultFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "internal" => Ok(RdfConfigResultFormat::Internal),
            "yaml" => Ok(RdfConfigResultFormat::Yaml),
            _ => Err(format!(
                "Unknown RDF config result format: '{}'. Supported: internal, yaml",
                s
            )),
        }
    }
}

impl Display for RdfConfigResultFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RdfConfigResultFormat::Internal => write!(f, "internal"),
            RdfConfigResultFormat::Yaml => write!(f, "yaml"),
        }
    }
}
