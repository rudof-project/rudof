use crate::cli_wrapper;
use rudof_lib_refactored::formats::PgSchemaFormat;
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(PgSchemaFormatCli, PgSchemaFormat, { PgSchemaC });