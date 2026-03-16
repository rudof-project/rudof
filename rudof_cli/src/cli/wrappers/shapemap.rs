use crate::cli_wrapper;
use rudof_lib_refactored::formats::ShapeMapFormat;
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

// CLI wrapper for rudof_lib::shapemap_format::ShapeMapFormat.
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