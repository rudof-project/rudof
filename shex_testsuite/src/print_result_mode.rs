use clap::ValueEnum;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum PrintResultMode {
    Basic,
    Failed,
    Passed,
    NotImplemented,
    All,
}
