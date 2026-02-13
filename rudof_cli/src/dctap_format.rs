use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
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
