use crate::cli_wrapper;
use rudof_lib_refactored::formats::{DCTapFormat, ResultDCTapFormat};
use clap::ValueEnum;
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
