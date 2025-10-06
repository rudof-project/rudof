use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShowNodeMode {
    Outgoing,
    Incoming,
    Both,
}

impl Display for ShowNodeMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShowNodeMode::Outgoing => write!(dest, "outgoing"),
            ShowNodeMode::Incoming => write!(dest, "incoming"),
            ShowNodeMode::Both => write!(dest, "both"),
        }
    }
}
