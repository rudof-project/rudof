use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// DCTAP available formats
#[derive(Debug, Default, PartialEq)]
pub enum DCTAPFormat {
    #[default]
    CSV,
    XLSX,
    XLSB,
    XLSM,
    XLS,
}

impl FromStr for DCTAPFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(DCTAPFormat::CSV),
            "xlsx" => Ok(DCTAPFormat::XLSX),
            _ => Err(format!("Unsupported DCTAP format {s}")),
        }
    }
}

impl Display for DCTAPFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DCTAPFormat::CSV => write!(dest, "csv"),
            &DCTAPFormat::XLSX => write!(dest, "xlsx"),
            DCTAPFormat::XLSB => write!(dest, "xlsb"),
            DCTAPFormat::XLSM => write!(dest, "xlsm"),
            DCTAPFormat::XLS => write!(dest, "xls"),
        }
    }
}
