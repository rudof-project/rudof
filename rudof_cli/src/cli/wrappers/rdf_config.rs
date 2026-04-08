use crate::cli_wrapper;
use clap::ValueEnum;
use rudof_lib::formats::{RdfConfigFormat, ResultRdfConfigFormat};
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
