use crate::cli_wrapper;
use clap::ValueEnum;
use rudof_viz::VizEngine;
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(
    VizEngineCli,
    VizEngine,
    {
        PlantUml,
        GraphViz
    }
);
