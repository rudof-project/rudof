use crate::cli_wrapper;
use rudof_lib_refactored::formats::GenerationSchemaFormat;
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(
    GenerationSchemaFormatCli,
    GenerationSchemaFormat,
    {
        Auto,
        ShEx,
        Shacl,
    }
);