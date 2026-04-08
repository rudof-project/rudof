use crate::cli_wrapper;
use clap::ValueEnum;
use rudof_lib::formats::ShapeMapFormat;
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(
    ShapeMapFormatCli,
    ShapeMapFormat,
    {
        Compact,
        Internal,
        Json,
        Details,
        Csv
    }
);
