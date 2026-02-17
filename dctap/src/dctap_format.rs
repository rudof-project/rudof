use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// DCTAP available formats
#[derive(Debug, Default, PartialEq)]
pub enum DCTAPFormat {
    #[default]
    Csv,
    Xlsx,
    Xlsb,
    Xlsm,
    Xls,
}

impl FromStr for DCTAPFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(DCTAPFormat::Csv),
            "xlsx" => Ok(DCTAPFormat::Xlsx),
            _ => Err(format!("Unsupported DCTAP format {s}")),
        }
    }
}

impl Display for DCTAPFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DCTAPFormat::Csv => write!(dest, "csv"),
            &DCTAPFormat::Xlsx => write!(dest, "xlsx"),
            DCTAPFormat::Xlsb => write!(dest, "xlsb"),
            DCTAPFormat::Xlsm => write!(dest, "xlsm"),
            DCTAPFormat::Xls => write!(dest, "xls"),
        }
    }
}
