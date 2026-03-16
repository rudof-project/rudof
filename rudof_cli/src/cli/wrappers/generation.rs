use crate::cli_wrapper;
use rudof_lib_refactored::formats::GenerationSchemaFormat;
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

// CLI wrapper for rudof_lib::generate_schema_format::GenerateSchemaFormat.
cli_wrapper!(
    GenerationSchemaFormatCli,
    GenerationSchemaFormat,
    {
        Auto,
        ShEx,
        Shacl,
    }
);