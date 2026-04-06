use crate::cli_wrapper;
use clap::ValueEnum;
use rudof_lib::formats::{DCTapFormat, ResultDCTapFormat};
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(
    DCTapFormatCli,
    DCTapFormat,
    {
        Csv,
        Xlsx,
        Xlsb,
        Xlsm,
        Xls,
    }
);

cli_wrapper!(
    ResultDCTapFormatCli,
    ResultDCTapFormat,
    {
        Internal,
        Json,
    }
);
