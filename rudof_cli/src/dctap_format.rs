use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum DCTapFormat {
    CSV,
    XLSX,
    XLSB,
    XLSM,
    XLS,
}

impl Display for DCTapFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DCTapFormat::CSV => write!(dest, "csv"),
            DCTapFormat::XLSX => write!(dest, "xlsx"),
            DCTapFormat::XLSB => write!(dest, "xlsb"),
            DCTapFormat::XLSM => write!(dest, "xlsm"),
            DCTapFormat::XLS => write!(dest, "xls"),
        }
    }
}
