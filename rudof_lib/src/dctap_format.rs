use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DCTapFormat {
    Csv,
    Xlsx,
    Xlsb,
    Xlsm,
    Xls,
}

impl Display for DCTapFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DCTapFormat::Csv => write!(dest, "csv"),
            DCTapFormat::Xlsx => write!(dest, "xlsx"),
            DCTapFormat::Xlsb => write!(dest, "xlsb"),
            DCTapFormat::Xlsm => write!(dest, "xlsm"),
            DCTapFormat::Xls => write!(dest, "xls"),
        }
    }
}

impl FromStr for DCTapFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(DCTapFormat::Csv),
            "xlsx" => Ok(DCTapFormat::Xlsx),
            "xlsb" => Ok(DCTapFormat::Xlsb),
            "xlsm" => Ok(DCTapFormat::Xlsm),
            "xls" => Ok(DCTapFormat::Xls),
            _ => Err(format!(
                "Unknown DC-TAP format: '{}'. Supported: csv, xlsx, xlsb, xlsm, xls",
                s
            )),
        }
    }
}
