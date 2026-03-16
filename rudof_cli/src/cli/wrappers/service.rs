use crate::cli_wrapper;
use rudof_lib_refactored::formats::ResultServiceFormat;
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

// CLI wrapper for rudof_lib::result_service_format::ResultServiceFormat.
cli_wrapper!(
    ResultServiceFormatCli,
    ResultServiceFormat,
    {
        Internal,
        Mie,
        Json,
    }
);