use crate::cli_wrapper;
use rudof_lib_refactored::formats::ResultServiceFormat;
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(
    ResultServiceFormatCli,
    ResultServiceFormat,
    {
        Internal,
        Mie,
        Json,
    }
);