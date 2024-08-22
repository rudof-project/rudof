use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// Different formats supported by DCTAP
pub enum DCTapFormat {
    /// Comma separated values
    CSV,

    /// Excel based format
    XLSX,
}

impl FromStr for DCTapFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(DCTapFormat::CSV),
            "xlsx" => Ok(DCTapFormat::XLSX),
            _ => Err(format!("Unsupported DCTAP format {s}")),
        }
    }
}

impl Display for DCTapFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DCTapFormat::CSV => write!(dest, "csv"),
            &DCTapFormat::XLSX => write!(dest, "xlsx"),
        }
    }
}
