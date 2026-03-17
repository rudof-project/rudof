use crate::cli_wrapper;
use rudof_lib_refactored::formats::NodeInspectionMode;
use clap::ValueEnum;
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
