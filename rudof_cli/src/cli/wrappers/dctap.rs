use crate::cli_wrapper;
use rudof_lib_refactored::formats::{DCTapFormat, ResultDCTapFormat};
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

// CLI wrapper for rudof_lib::dctap_format::DCTapFormat.
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

// CLI wrapper for rudof_lib::dctap_result_format::DCTapResultFormat.
cli_wrapper!(
    ResultDCTapFormatCli,
    ResultDCTapFormat,
    {
        Internal,
        Json,
    }
);
