use crate::cli_wrapper;
use rudof_lib_refactored::formats::NodeInspectionMode;
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

// CLI wrapper for rudof_lib::show_node_mode::ShowNodeMode.
cli_wrapper!(
    NodeInspectionModeCli,
    NodeInspectionMode,
    {
        Outgoing,
        Incoming,
        Both,
    }
);
