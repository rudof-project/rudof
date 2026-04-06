use crate::cli_wrapper;
use clap::ValueEnum;
use rudof_lib::formats::PgSchemaFormat;
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(PgSchemaFormatCli, PgSchemaFormat, { PgSchemaC });
