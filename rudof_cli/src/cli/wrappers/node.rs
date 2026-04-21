use crate::cli_wrapper;
use clap::ValueEnum;
use rudof_lib::formats::NodeInspectionMode;
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(
    NodeInspectionModeCli,
    NodeInspectionMode,
    {
        Outgoing,
        Incoming,
        Both,
    }
);
