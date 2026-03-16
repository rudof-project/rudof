use crate::cli_wrapper;
use rudof_lib_refactored::formats::{RdfConfigFormat, ResultRdfConfigFormat};
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(RdfConfigFormatCli, RdfConfigFormat, { Yaml });

cli_wrapper!(
    ResultRdfConfigFormatCli,
    ResultRdfConfigFormat,
    {
        Internal,
        Yaml,
    }
);