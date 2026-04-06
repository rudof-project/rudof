use crate::cli_wrapper;
use clap::ValueEnum;
use rudof_lib::formats::GenerationSchemaFormat;
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
