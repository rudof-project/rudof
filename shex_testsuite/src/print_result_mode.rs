use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum PrintResultMode {
    Basic,
    FailedSimple,
    Failed,
    Passed,
    NotImplemented,
    All,
}
