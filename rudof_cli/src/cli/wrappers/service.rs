use crate::cli_wrapper;
use clap::ValueEnum;
use rudof_lib::formats::ResultServiceFormat;
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
