use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Backend used for the validation.
///
/// According to the SHACL Recommendation, there exists no concrete method for
/// implementing SHACL. Thus, by choosing your preferred SHACL Validation Mode,
/// the user can select which engine is used for the validation.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum ShaclValidationMode {
    /// Rust native engine using functions implemented with Rust native code
    #[default]
    Native,
    /// SPARQL-based engine using SPARQL queries to validate the data
    #[cfg(feature = "sparql")]
    Sparql,
}

impl Display for ShaclValidationMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaclValidationMode::Native => write!(f, "native"),
            #[cfg(feature = "sparql")]
            ShaclValidationMode::Sparql => write!(f, "sparql"),
        }
    }
}

impl FromStr for ShaclValidationMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "native" => Ok(Self::Native),
            #[cfg(feature = "sparql")]
            "sparql" => Ok(Self::Sparql),
            other => Err(format!("Unsupported SHACL validation mode: {other}")),
        }
    }
}
